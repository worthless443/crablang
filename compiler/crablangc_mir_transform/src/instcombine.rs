//! Performs various peephole optimizations.

use crate::MirPass;
use crablangc_hir::Mutability;
use crablangc_middle::mir::{
    BinOp, Body, CastKind, Constant, ConstantKind, LocalDecls, Operand, Place, ProjectionElem,
    Rvalue, SourceInfo, Statement, StatementKind, SwitchTargets, Terminator, TerminatorKind, UnOp,
};
use crablangc_middle::ty::layout::ValidityRequirement;
use crablangc_middle::ty::util::IntTypeExt;
use crablangc_middle::ty::{self, ParamEnv, SubstsRef, Ty, TyCtxt};
use crablangc_span::symbol::Symbol;
use crablangc_target::abi::FieldIdx;

pub struct InstCombine;

impl<'tcx> MirPass<'tcx> for InstCombine {
    fn is_enabled(&self, sess: &crablangc_session::Session) -> bool {
        sess.mir_opt_level() > 0
    }

    fn run_pass(&self, tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
        let ctx = InstCombineContext {
            tcx,
            local_decls: &body.local_decls,
            param_env: tcx.param_env_reveal_all_normalized(body.source.def_id()),
        };
        for block in body.basic_blocks.as_mut() {
            for statement in block.statements.iter_mut() {
                match statement.kind {
                    StatementKind::Assign(box (_place, ref mut rvalue)) => {
                        ctx.combine_bool_cmp(&statement.source_info, rvalue);
                        ctx.combine_ref_deref(&statement.source_info, rvalue);
                        ctx.combine_len(&statement.source_info, rvalue);
                        ctx.combine_cast(&statement.source_info, rvalue);
                    }
                    _ => {}
                }
            }

            ctx.combine_primitive_clone(
                &mut block.terminator.as_mut().unwrap(),
                &mut block.statements,
            );
            ctx.combine_intrinsic_assert(
                &mut block.terminator.as_mut().unwrap(),
                &mut block.statements,
            );
            ctx.combine_duplicate_switch_targets(&mut block.terminator.as_mut().unwrap());
        }
    }
}

struct InstCombineContext<'tcx, 'a> {
    tcx: TyCtxt<'tcx>,
    local_decls: &'a LocalDecls<'tcx>,
    param_env: ParamEnv<'tcx>,
}

impl<'tcx> InstCombineContext<'tcx, '_> {
    fn should_combine(&self, source_info: &SourceInfo, rvalue: &Rvalue<'tcx>) -> bool {
        self.tcx.consider_optimizing(|| {
            format!("InstCombine - Rvalue: {:?} SourceInfo: {:?}", rvalue, source_info)
        })
    }

    /// Transform boolean comparisons into logical operations.
    fn combine_bool_cmp(&self, source_info: &SourceInfo, rvalue: &mut Rvalue<'tcx>) {
        match rvalue {
            Rvalue::BinaryOp(op @ (BinOp::Eq | BinOp::Ne), box (a, b)) => {
                let new = match (op, self.try_eval_bool(a), self.try_eval_bool(b)) {
                    // Transform "Eq(a, true)" ==> "a"
                    (BinOp::Eq, _, Some(true)) => Some(Rvalue::Use(a.clone())),

                    // Transform "Ne(a, false)" ==> "a"
                    (BinOp::Ne, _, Some(false)) => Some(Rvalue::Use(a.clone())),

                    // Transform "Eq(true, b)" ==> "b"
                    (BinOp::Eq, Some(true), _) => Some(Rvalue::Use(b.clone())),

                    // Transform "Ne(false, b)" ==> "b"
                    (BinOp::Ne, Some(false), _) => Some(Rvalue::Use(b.clone())),

                    // Transform "Eq(false, b)" ==> "Not(b)"
                    (BinOp::Eq, Some(false), _) => Some(Rvalue::UnaryOp(UnOp::Not, b.clone())),

                    // Transform "Ne(true, b)" ==> "Not(b)"
                    (BinOp::Ne, Some(true), _) => Some(Rvalue::UnaryOp(UnOp::Not, b.clone())),

                    // Transform "Eq(a, false)" ==> "Not(a)"
                    (BinOp::Eq, _, Some(false)) => Some(Rvalue::UnaryOp(UnOp::Not, a.clone())),

                    // Transform "Ne(a, true)" ==> "Not(a)"
                    (BinOp::Ne, _, Some(true)) => Some(Rvalue::UnaryOp(UnOp::Not, a.clone())),

                    _ => None,
                };

                if let Some(new) = new && self.should_combine(source_info, rvalue) {
                    *rvalue = new;
                }
            }

            _ => {}
        }
    }

    fn try_eval_bool(&self, a: &Operand<'_>) -> Option<bool> {
        let a = a.constant()?;
        if a.literal.ty().is_bool() { a.literal.try_to_bool() } else { None }
    }

    /// Transform "&(*a)" ==> "a".
    fn combine_ref_deref(&self, source_info: &SourceInfo, rvalue: &mut Rvalue<'tcx>) {
        if let Rvalue::Ref(_, _, place) = rvalue {
            if let Some((base, ProjectionElem::Deref)) = place.as_ref().last_projection() {
                if rvalue.ty(self.local_decls, self.tcx) != base.ty(self.local_decls, self.tcx).ty {
                    return;
                }

                if !self.should_combine(source_info, rvalue) {
                    return;
                }

                *rvalue = Rvalue::Use(Operand::Copy(Place {
                    local: base.local,
                    projection: self.tcx.mk_place_elems(base.projection),
                }));
            }
        }
    }

    /// Transform "Len([_; N])" ==> "N".
    fn combine_len(&self, source_info: &SourceInfo, rvalue: &mut Rvalue<'tcx>) {
        if let Rvalue::Len(ref place) = *rvalue {
            let place_ty = place.ty(self.local_decls, self.tcx).ty;
            if let ty::Array(_, len) = *place_ty.kind() {
                if !self.should_combine(source_info, rvalue) {
                    return;
                }

                let literal = ConstantKind::from_const(len, self.tcx);
                let constant = Constant { span: source_info.span, literal, user_ty: None };
                *rvalue = Rvalue::Use(Operand::Constant(Box::new(constant)));
            }
        }
    }

    fn combine_cast(&self, _source_info: &SourceInfo, rvalue: &mut Rvalue<'tcx>) {
        if let Rvalue::Cast(kind, operand, cast_ty) = rvalue {
            let operand_ty = operand.ty(self.local_decls, self.tcx);
            if operand_ty == *cast_ty {
                *rvalue = Rvalue::Use(operand.clone());
            } else if *kind == CastKind::Transmute {
                // Transmuting an integer to another integer is just a signedness cast
                if let (ty::Int(int), ty::Uint(uint)) | (ty::Uint(uint), ty::Int(int)) = (operand_ty.kind(), cast_ty.kind())
                    && int.bit_width() == uint.bit_width()
                {
                    // The width check isn't strictly necessary, as different widths
                    // are UB and thus we'd be allowed to turn it into a cast anyway.
                    // But let's keep the UB around for codegen to exploit later.
                    // (If `CastKind::Transmute` ever becomes *not* UB for mismatched sizes,
                    // then the width check is necessary for big-endian correctness.)
                    *kind = CastKind::IntToInt;
                    return;
                }

                // Transmuting a fieldless enum to its repr is a discriminant read
                if let ty::Adt(adt_def, ..) = operand_ty.kind()
                    && adt_def.is_enum()
                    && adt_def.is_payloadfree()
                    && let Some(place) = operand.place()
                    && let Some(repr_int) = adt_def.repr().int
                    && repr_int.to_ty(self.tcx) == *cast_ty
                {
                    *rvalue = Rvalue::Discriminant(place);
                    return;
                }

                // Transmuting a transparent struct/union to a field's type is a projection
                if let ty::Adt(adt_def, substs) = operand_ty.kind()
                    && adt_def.repr().transparent()
                    && (adt_def.is_struct() || adt_def.is_union())
                    && let Some(place) = operand.place()
                {
                    let variant = adt_def.non_enum_variant();
                    for (i, field) in variant.fields.iter().enumerate() {
                        let field_ty = field.ty(self.tcx, substs);
                        if field_ty == *cast_ty {
                            let place = place.project_deeper(&[ProjectionElem::Field(FieldIdx::from_usize(i), *cast_ty)], self.tcx);
                            let operand = if operand.is_move() { Operand::Move(place) } else { Operand::Copy(place) };
                            *rvalue = Rvalue::Use(operand);
                            return;
                        }
                    }
                }
            }
        }
    }

    fn combine_primitive_clone(
        &self,
        terminator: &mut Terminator<'tcx>,
        statements: &mut Vec<Statement<'tcx>>,
    ) {
        let TerminatorKind::Call { func, args, destination, target, .. } = &mut terminator.kind
        else { return };

        // It's definitely not a clone if there are multiple arguments
        if args.len() != 1 {
            return;
        }

        let Some(destination_block) = *target
        else { return };

        // Only bother looking more if it's easy to know what we're calling
        let Some((fn_def_id, fn_substs)) = func.const_fn_def()
        else { return };

        // Clone needs one subst, so we can cheaply rule out other stuff
        if fn_substs.len() != 1 {
            return;
        }

        // These types are easily available from locals, so check that before
        // doing DefId lookups to figure out what we're actually calling.
        let arg_ty = args[0].ty(self.local_decls, self.tcx);

        let ty::Ref(_region, inner_ty, Mutability::Not) = *arg_ty.kind()
        else { return };

        if !inner_ty.is_trivially_pure_clone_copy() {
            return;
        }

        let trait_def_id = self.tcx.trait_of_item(fn_def_id);
        if trait_def_id.is_none() || trait_def_id != self.tcx.lang_items().clone_trait() {
            return;
        }

        if !self.tcx.consider_optimizing(|| {
            format!(
                "InstCombine - Call: {:?} SourceInfo: {:?}",
                (fn_def_id, fn_substs),
                terminator.source_info
            )
        }) {
            return;
        }

        let Some(arg_place) = args.pop().unwrap().place()
        else { return };

        statements.push(Statement {
            source_info: terminator.source_info,
            kind: StatementKind::Assign(Box::new((
                *destination,
                Rvalue::Use(Operand::Copy(
                    arg_place.project_deeper(&[ProjectionElem::Deref], self.tcx),
                )),
            ))),
        });
        terminator.kind = TerminatorKind::Goto { target: destination_block };
    }

    fn combine_duplicate_switch_targets(&self, terminator: &mut Terminator<'tcx>) {
        let TerminatorKind::SwitchInt { targets, .. } = &mut terminator.kind
        else { return };

        let otherwise = targets.otherwise();
        if targets.iter().any(|t| t.1 == otherwise) {
            *targets = SwitchTargets::new(
                targets.iter().filter(|t| t.1 != otherwise),
                targets.otherwise(),
            );
        }
    }

    fn combine_intrinsic_assert(
        &self,
        terminator: &mut Terminator<'tcx>,
        _statements: &mut Vec<Statement<'tcx>>,
    ) {
        let TerminatorKind::Call { func, target, .. } = &mut terminator.kind  else { return; };
        let Some(target_block) = target else { return; };
        let func_ty = func.ty(self.local_decls, self.tcx);
        let Some((intrinsic_name, substs)) = resolve_crablang_intrinsic(self.tcx, func_ty) else {
            return;
        };
        // The intrinsics we are interested in have one generic parameter
        if substs.is_empty() {
            return;
        }
        let ty = substs.type_at(0);

        let known_is_valid = intrinsic_assert_panics(self.tcx, self.param_env, ty, intrinsic_name);
        match known_is_valid {
            // We don't know the layout or it's not validity assertion at all, don't touch it
            None => {}
            Some(true) => {
                // If we know the assert panics, indicate to later opts that the call diverges
                *target = None;
            }
            Some(false) => {
                // If we know the assert does not panic, turn the call into a Goto
                terminator.kind = TerminatorKind::Goto { target: *target_block };
            }
        }
    }
}

fn intrinsic_assert_panics<'tcx>(
    tcx: TyCtxt<'tcx>,
    param_env: ty::ParamEnv<'tcx>,
    ty: Ty<'tcx>,
    intrinsic_name: Symbol,
) -> Option<bool> {
    let requirement = ValidityRequirement::from_intrinsic(intrinsic_name)?;
    Some(!tcx.check_validity_requirement((requirement, param_env.and(ty))).ok()?)
}

fn resolve_crablang_intrinsic<'tcx>(
    tcx: TyCtxt<'tcx>,
    func_ty: Ty<'tcx>,
) -> Option<(Symbol, SubstsRef<'tcx>)> {
    if let ty::FnDef(def_id, substs) = *func_ty.kind() {
        if tcx.is_intrinsic(def_id) {
            return Some((tcx.item_name(def_id), substs));
        }
    }
    None
}
