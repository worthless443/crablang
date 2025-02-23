use log::trace;

use crablangc_target::abi::{Abi, Size};

use crate::borrow_tracker::{AccessKind, GlobalStateInner, ProtectorKind, RetagFields};
use crablangc_middle::{
    mir::{Mutability, RetagKind},
    ty::{
        self,
        layout::{HasParamEnv, LayoutOf},
        Ty,
    },
};

use crate::*;

mod diagnostics;
mod perms;
mod tree;
mod unimap;
use perms::Permission;
pub use tree::Tree;

pub type AllocState = Tree;

pub fn err_tb_ub<'tcx>(msg: String) -> InterpError<'tcx> {
    err_machine_stop!(TerminationInfo::TreeBorrowsUb { msg })
}

impl<'tcx> Tree {
    /// Create a new allocation, i.e. a new tree
    pub fn new_allocation(
        id: AllocId,
        size: Size,
        state: &mut GlobalStateInner,
        _kind: MemoryKind<machine::MiriMemoryKind>,
        machine: &MiriMachine<'_, 'tcx>,
    ) -> Self {
        let tag = state.base_ptr_tag(id, machine); // Fresh tag for the root
        Tree::new(tag, size)
    }

    /// Check that an access on the entire range is permitted, and update
    /// the tree.
    pub fn before_memory_access(
        &mut self,
        access_kind: AccessKind,
        alloc_id: AllocId,
        prov: ProvenanceExtra,
        range: AllocRange,
        machine: &MiriMachine<'_, 'tcx>,
    ) -> InterpResult<'tcx> {
        trace!(
            "{} with tag {:?}: {:?}, size {}",
            access_kind,
            prov,
            Pointer::new(alloc_id, range.start),
            range.size.bytes(),
        );
        // TODO: for now we bail out on wildcard pointers. Eventually we should
        // handle them as much as we can.
        let tag = match prov {
            ProvenanceExtra::Concrete(tag) => tag,
            ProvenanceExtra::Wildcard => return Ok(()),
        };
        let global = machine.borrow_tracker.as_ref().unwrap();
        self.perform_access(access_kind, tag, range, global)
    }

    /// Check that this pointer has permission to deallocate this range.
    pub fn before_memory_deallocation(
        &mut self,
        _alloc_id: AllocId,
        prov: ProvenanceExtra,
        range: AllocRange,
        machine: &MiriMachine<'_, 'tcx>,
    ) -> InterpResult<'tcx> {
        // TODO: for now we bail out on wildcard pointers. Eventually we should
        // handle them as much as we can.
        let tag = match prov {
            ProvenanceExtra::Concrete(tag) => tag,
            ProvenanceExtra::Wildcard => return Ok(()),
        };
        let global = machine.borrow_tracker.as_ref().unwrap();
        self.dealloc(tag, range, global)
    }

    pub fn expose_tag(&mut self, _tag: BorTag) {
        // TODO
    }
}

/// Policy for a new borrow.
#[derive(Debug, Clone, Copy)]
struct NewPermission {
    /// Whether this borrow requires a read access on its parent.
    /// `perform_read_access` is `true` for all pointers marked `dereferenceable`.
    perform_read_access: bool,
    /// Which permission should the pointer start with.
    initial_state: Permission,
    /// Whether this pointer is part of the arguments of a function call.
    /// `protector` is `Some(_)` for all pointers marked `noalias`.
    protector: Option<ProtectorKind>,
}

impl<'tcx> NewPermission {
    /// Determine NewPermission of the reference from the type of the pointee.
    fn from_ref_ty(
        pointee: Ty<'tcx>,
        mutability: Mutability,
        kind: RetagKind,
        cx: &crate::MiriInterpCx<'_, 'tcx>,
    ) -> Option<Self> {
        let ty_is_freeze = pointee.is_freeze(*cx.tcx, cx.param_env());
        let ty_is_unpin = pointee.is_unpin(*cx.tcx, cx.param_env());
        let initial_state = match mutability {
            Mutability::Mut if ty_is_unpin => Permission::new_unique_2phase(ty_is_freeze),
            Mutability::Not if ty_is_freeze => Permission::new_frozen(),
            // Raw pointers never enter this function so they are not handled.
            // However raw pointers are not the only pointers that take the parent
            // tag, this also happens for `!Unpin` `&mut`s and interior mutable
            // `&`s, which are excluded above.
            _ => return None,
        };
        // This field happens to be redundant since right now we always do a read,
        // but it could be useful in the future.
        let perform_read_access = true;

        let protector = (kind == RetagKind::FnEntry).then_some(ProtectorKind::StrongProtector);
        Some(Self { perform_read_access, initial_state, protector })
    }

    // Boxes are not handled by `from_ref_ty`, they need special behavior
    // implemented here.
    fn from_box_ty(
        ty: Ty<'tcx>,
        kind: RetagKind,
        cx: &crate::MiriInterpCx<'_, 'tcx>,
    ) -> Option<Self> {
        let pointee = ty.builtin_deref(true).unwrap().ty;
        pointee.is_unpin(*cx.tcx, cx.param_env()).then_some(()).map(|()| {
            // Regular `Unpin` box, give it `noalias` but only a weak protector
            // because it is valid to deallocate it within the function.
            let ty_is_freeze = ty.is_freeze(*cx.tcx, cx.param_env());
            Self {
                perform_read_access: true,
                initial_state: Permission::new_unique_2phase(ty_is_freeze),
                protector: (kind == RetagKind::FnEntry).then_some(ProtectorKind::WeakProtector),
            }
        })
    }
}

/// Retagging/reborrowing.
/// Policy on which permission to grant to each pointer should be left to
/// the implementation of NewPermission.
impl<'mir: 'ecx, 'tcx: 'mir, 'ecx> EvalContextPrivExt<'mir, 'tcx, 'ecx>
    for crate::MiriInterpCx<'mir, 'tcx>
{
}
trait EvalContextPrivExt<'mir: 'ecx, 'tcx: 'mir, 'ecx>: crate::MiriInterpCxExt<'mir, 'tcx> {
    /// Returns the `AllocId` the reborrow was done in, if there is some actual
    /// memory associated with this pointer. Returns `None` if there is no actual
    /// memory allocated. Also checks that the reborrow of size `ptr_size` is
    /// within bounds of the allocation.
    ///
    /// Also returns the tag that the pointer should get, which is essentially
    /// `if new_perm.is_some() { new_tag } else { parent_tag }` along with
    /// some logging (always) and fake reads (if `new_perm` is
    /// `Some(NewPermission { perform_read_access: true }`).
    fn tb_reborrow(
        &mut self,
        place: &MPlaceTy<'tcx, Provenance>, // parent tag extracted from here
        ptr_size: Size,
        new_perm: Option<NewPermission>,
        new_tag: BorTag,
    ) -> InterpResult<'tcx, Option<(AllocId, BorTag)>> {
        let this = self.eval_context_mut();

        // It is crucial that this gets called on all code paths, to ensure we track tag creation.
        let log_creation = |this: &MiriInterpCx<'mir, 'tcx>,
                            loc: Option<(AllocId, Size, ProvenanceExtra)>| // alloc_id, base_offset, orig_tag
         -> InterpResult<'tcx> {
            let global = this.machine.borrow_tracker.as_ref().unwrap().borrow();
            let ty = place.layout.ty;
            if global.tracked_pointer_tags.contains(&new_tag) {
                let kind_str = format!("{new_perm:?} (pointee type {ty})");
                this.emit_diagnostic(NonHaltingDiagnostic::CreatedPointerTag(
                    new_tag.inner(),
                    Some(kind_str),
                    loc.map(|(alloc_id, base_offset, orig_tag)| (alloc_id, alloc_range(base_offset, ptr_size), orig_tag)),
                ));
            }
            drop(global); // don't hold that reference any longer than we have to
            Ok(())
        };

        let (alloc_id, base_offset, parent_prov) = if ptr_size > Size::ZERO {
            this.ptr_get_alloc_id(place.ptr)?
        } else {
            match this.ptr_try_get_alloc_id(place.ptr) {
                Ok(data) => data,
                Err(_) => {
                    // This pointer doesn't come with an AllocId, so there's no
                    // memory to do retagging in.
                    trace!(
                        "reborrow of size 0: reference {:?} derived from {:?} (pointee {})",
                        new_tag,
                        place.ptr,
                        place.layout.ty,
                    );
                    log_creation(this, None)?;
                    return Ok(None);
                }
            }
        };
        let orig_tag = match parent_prov {
            ProvenanceExtra::Wildcard => return Ok(None), // TODO: handle wildcard pointers
            ProvenanceExtra::Concrete(tag) => tag,
        };

        // Protection against trying to get a reference to a vtable:
        // vtables do not have an alloc_extra so the call to
        // `get_alloc_extra` that follows fails.
        let (alloc_size, _align, alloc_kind) = this.get_alloc_info(alloc_id);
        if ptr_size == Size::ZERO && !matches!(alloc_kind, AllocKind::LiveData) {
            return Ok(Some((alloc_id, orig_tag)));
        }

        log_creation(this, Some((alloc_id, base_offset, parent_prov)))?;

        // Ensure we bail out if the pointer goes out-of-bounds (see miri#1050).
        if base_offset + ptr_size > alloc_size {
            throw_ub!(PointerOutOfBounds {
                alloc_id,
                alloc_size,
                ptr_offset: this.target_usize_to_isize(base_offset.bytes()),
                ptr_size,
                msg: CheckInAllocMsg::InboundsTest
            });
        }

        trace!(
            "reborrow: reference {:?} derived from {:?} (pointee {}): {:?}, size {}",
            new_tag,
            orig_tag,
            place.layout.ty,
            Pointer::new(alloc_id, base_offset),
            ptr_size.bytes()
        );

        let Some(new_perm) = new_perm else { return Ok(Some((alloc_id, orig_tag))); };

        if let Some(protect) = new_perm.protector {
            // We register the protection in two different places.
            // This makes creating a protector slower, but checking whether a tag
            // is protected faster.
            this.frame_mut().extra.borrow_tracker.as_mut().unwrap().protected_tags.push(new_tag);
            this.machine
                .borrow_tracker
                .as_mut()
                .expect("We should have borrow tracking data")
                .get_mut()
                .protected_tags
                .insert(new_tag, protect);
        }

        let alloc_extra = this.get_alloc_extra(alloc_id)?;
        let range = alloc_range(base_offset, ptr_size);
        let mut tree_borrows = alloc_extra.borrow_tracker_tb().borrow_mut();

        if new_perm.perform_read_access {
            // Count this reborrow as a read access
            let global = &this.machine.borrow_tracker.as_ref().unwrap();
            tree_borrows.perform_access(AccessKind::Read, orig_tag, range, global)?;
            if let Some(data_race) = alloc_extra.data_race.as_ref() {
                data_race.read(alloc_id, range, &this.machine)?;
            }
        }

        // Record the parent-child pair in the tree.
        tree_borrows.new_child(orig_tag, new_tag, new_perm.initial_state, range)?;
        Ok(Some((alloc_id, new_tag)))
    }

    /// Retags an indidual pointer, returning the retagged version.
    fn tb_retag_reference(
        &mut self,
        val: &ImmTy<'tcx, Provenance>,
        new_perm: Option<NewPermission>,
    ) -> InterpResult<'tcx, ImmTy<'tcx, Provenance>> {
        let this = self.eval_context_mut();
        // We want a place for where the ptr *points to*, so we get one.
        let place = this.ref_to_mplace(val)?;

        // Get a lower bound of the size of this place.
        // (When `extern type` are involved, use the size of the known prefix.)
        let size = this
            .size_and_align_of_mplace(&place)?
            .map(|(size, _)| size)
            .unwrap_or(place.layout.size);

        // This new tag is not guaranteed to actually be used.
        //
        // If you run out of tags, consider the following optimization: adjust `tb_reborrow`
        // so that rather than taking as input a fresh tag and deciding whether it uses this
        // one or the parent it instead just returns whether a new tag should be created.
        // This will avoid creating tags than end up never being used.
        let new_tag = this.machine.borrow_tracker.as_mut().unwrap().get_mut().new_ptr();

        // Compute the actual reborrow.
        let reborrowed = this.tb_reborrow(&place, size, new_perm, new_tag)?;

        // Adjust pointer.
        let new_place = place.map_provenance(|p| {
            p.map(|prov| {
                match reborrowed {
                    Some((alloc_id, actual_tag)) => {
                        // If `reborrow` could figure out the AllocId of this ptr, hard-code it into the new one.
                        // Even if we started out with a wildcard, this newly retagged pointer is tied to that allocation.
                        Provenance::Concrete { alloc_id, tag: actual_tag }
                    }
                    None => {
                        // Looks like this has to stay a wildcard pointer.
                        assert!(matches!(prov, Provenance::Wildcard));
                        Provenance::Wildcard
                    }
                }
            })
        });

        // Return new pointer.
        Ok(ImmTy::from_immediate(new_place.to_ref(this), val.layout))
    }
}

impl<'mir, 'tcx: 'mir> EvalContextExt<'mir, 'tcx> for crate::MiriInterpCx<'mir, 'tcx> {}
pub trait EvalContextExt<'mir, 'tcx: 'mir>: crate::MiriInterpCxExt<'mir, 'tcx> {
    /// Retag a pointer. References are passed to `from_ref_ty` and
    /// raw pointers are never reborrowed.
    fn tb_retag_ptr_value(
        &mut self,
        kind: RetagKind,
        val: &ImmTy<'tcx, Provenance>,
    ) -> InterpResult<'tcx, ImmTy<'tcx, Provenance>> {
        let this = self.eval_context_mut();
        let new_perm = if let &ty::Ref(_, pointee, mutability) = val.layout.ty.kind() {
            NewPermission::from_ref_ty(pointee, mutability, kind, this)
        } else {
            None
        };
        this.tb_retag_reference(val, new_perm)
    }

    /// Retag all pointers that are stored in this place.
    fn tb_retag_place_contents(
        &mut self,
        kind: RetagKind,
        place: &PlaceTy<'tcx, Provenance>,
    ) -> InterpResult<'tcx> {
        let this = self.eval_context_mut();
        let retag_fields = this.machine.borrow_tracker.as_mut().unwrap().get_mut().retag_fields;
        let mut visitor = RetagVisitor { ecx: this, kind, retag_fields };
        return visitor.visit_value(place);

        // The actual visitor.
        struct RetagVisitor<'ecx, 'mir, 'tcx> {
            ecx: &'ecx mut MiriInterpCx<'mir, 'tcx>,
            kind: RetagKind,
            retag_fields: RetagFields,
        }
        impl<'ecx, 'mir, 'tcx> RetagVisitor<'ecx, 'mir, 'tcx> {
            #[inline(always)] // yes this helps in our benchmarks
            fn retag_ptr_inplace(
                &mut self,
                place: &PlaceTy<'tcx, Provenance>,
                new_perm: Option<NewPermission>,
            ) -> InterpResult<'tcx> {
                let val = self.ecx.read_immediate(&self.ecx.place_to_op(place)?)?;
                let val = self.ecx.tb_retag_reference(&val, new_perm)?;
                self.ecx.write_immediate(*val, place)?;
                Ok(())
            }
        }
        impl<'ecx, 'mir, 'tcx> MutValueVisitor<'mir, 'tcx, MiriMachine<'mir, 'tcx>>
            for RetagVisitor<'ecx, 'mir, 'tcx>
        {
            type V = PlaceTy<'tcx, Provenance>;

            #[inline(always)]
            fn ecx(&mut self) -> &mut MiriInterpCx<'mir, 'tcx> {
                self.ecx
            }

            fn visit_box(&mut self, place: &PlaceTy<'tcx, Provenance>) -> InterpResult<'tcx> {
                let new_perm = NewPermission::from_box_ty(place.layout.ty, self.kind, self.ecx);
                self.retag_ptr_inplace(place, new_perm)
            }

            fn visit_value(&mut self, place: &PlaceTy<'tcx, Provenance>) -> InterpResult<'tcx> {
                // If this place is smaller than a pointer, we know that it can't contain any
                // pointers we need to retag, so we can stop recursion early.
                // This optimization is crucial for ZSTs, because they can contain way more fields
                // than we can ever visit.
                if place.layout.is_sized() && place.layout.size < self.ecx.pointer_size() {
                    return Ok(());
                }

                // Check the type of this value to see what to do with it (retag, or recurse).
                match place.layout.ty.kind() {
                    &ty::Ref(_, pointee, mutability) => {
                        let new_perm =
                            NewPermission::from_ref_ty(pointee, mutability, self.kind, self.ecx);
                        self.retag_ptr_inplace(place, new_perm)?;
                    }
                    ty::RawPtr(_) => {
                        // We definitely do *not* want to recurse into raw pointers -- wide raw
                        // pointers have fields, and for dyn Trait pointees those can have reference
                        // type!
                        // We also do not want to reborrow them.
                    }
                    ty::Adt(adt, _) if adt.is_box() => {
                        // Recurse for boxes, they require some tricky handling and will end up in `visit_box` above.
                        // (Yes this means we technically also recursively retag the allocator itself
                        // even if field retagging is not enabled. *shrug*)
                        self.walk_value(place)?;
                    }
                    _ => {
                        // Not a reference/pointer/box. Only recurse if configured appropriately.
                        let recurse = match self.retag_fields {
                            RetagFields::No => false,
                            RetagFields::Yes => true,
                            RetagFields::OnlyScalar => {
                                // Matching `ArgAbi::new` at the time of writing, only fields of
                                // `Scalar` and `ScalarPair` ABI are considered.
                                matches!(place.layout.abi, Abi::Scalar(..) | Abi::ScalarPair(..))
                            }
                        };
                        if recurse {
                            self.walk_value(place)?;
                        }
                    }
                }

                Ok(())
            }
        }
    }

    /// After a stack frame got pushed, retag the return place so that we are sure
    /// it does not alias with anything.
    ///
    /// This is a HACK because there is nothing in MIR that would make the retag
    /// explicit. Also see <https://github.com/crablang/crablang/issues/71117>.
    fn tb_retag_return_place(&mut self) -> InterpResult<'tcx> {
        let this = self.eval_context_mut();
        //this.debug_hint_location();
        let return_place = &this.frame().return_place;
        if return_place.layout.is_zst() {
            // There may not be any memory here, nothing to do.
            return Ok(());
        }
        // We need this to be in-memory to use tagged pointers.
        let return_place = this.force_allocation(&return_place.clone())?;

        // We have to turn the place into a pointer to use the existing code.
        // (The pointer type does not matter, so we use a raw pointer.)
        let ptr_layout = this.layout_of(this.tcx.mk_mut_ptr(return_place.layout.ty))?;
        let val = ImmTy::from_immediate(return_place.to_ref(this), ptr_layout);
        // Reborrow it. With protection! That is part of the point.
        // FIXME: do we truly want a 2phase borrow here?
        let new_perm = Some(NewPermission {
            initial_state: Permission::new_unique_2phase(/*freeze*/ false),
            perform_read_access: true,
            protector: Some(ProtectorKind::StrongProtector),
        });
        let val = this.tb_retag_reference(&val, new_perm)?;
        // And use reborrowed pointer for return place.
        let return_place = this.ref_to_mplace(&val)?;
        this.frame_mut().return_place = return_place.into();

        Ok(())
    }

    /// Mark the given tag as exposed. It was found on a pointer with the given AllocId.
    fn tb_expose_tag(&mut self, alloc_id: AllocId, tag: BorTag) -> InterpResult<'tcx> {
        let this = self.eval_context_mut();

        // Function pointers and dead objects don't have an alloc_extra so we ignore them.
        // This is okay because accessing them is UB anyway, no need for any Tree Borrows checks.
        // NOT using `get_alloc_extra_mut` since this might be a read-only allocation!
        let (_size, _align, kind) = this.get_alloc_info(alloc_id);
        match kind {
            AllocKind::LiveData => {
                // This should have alloc_extra data, but `get_alloc_extra` can still fail
                // if converting this alloc_id from a global to a local one
                // uncovers a non-supported `extern static`.
                let alloc_extra = this.get_alloc_extra(alloc_id)?;
                trace!("Stacked Borrows tag {tag:?} exposed in {alloc_id:?}");
                alloc_extra.borrow_tracker_tb().borrow_mut().expose_tag(tag);
            }
            AllocKind::Function | AllocKind::VTable | AllocKind::Dead => {
                // No tree borrows on these allocations.
            }
        }
        Ok(())
    }

    /// Display the tree.
    fn print_tree(&mut self, alloc_id: AllocId, show_unnamed: bool) -> InterpResult<'tcx> {
        let this = self.eval_context_mut();
        let alloc_extra = this.get_alloc_extra(alloc_id)?;
        let tree_borrows = alloc_extra.borrow_tracker_tb().borrow();
        let borrow_tracker = &this.machine.borrow_tracker.as_ref().unwrap().borrow();
        tree_borrows.print_tree(&borrow_tracker.protected_tags, show_unnamed)
    }

    /// Give a name to the pointer, usually the name it has in the source code (for debugging).
    /// The name given is `name` and the pointer that receives it is the `nth_parent`
    /// of `ptr` (with 0 representing `ptr` itself)
    fn tb_give_pointer_debug_name(
        &mut self,
        ptr: Pointer<Option<Provenance>>,
        nth_parent: u8,
        name: &str,
    ) -> InterpResult<'tcx> {
        let this = self.eval_context_mut();
        let (tag, alloc_id) = match ptr.provenance {
            Some(Provenance::Concrete { tag, alloc_id }) => (tag, alloc_id),
            _ => {
                eprintln!("Can't give the name {name} to Wildcard pointer");
                return Ok(());
            }
        };
        let alloc_extra = this.get_alloc_extra(alloc_id)?;
        let mut tree_borrows = alloc_extra.borrow_tracker_tb().borrow_mut();
        tree_borrows.give_pointer_debug_name(tag, nth_parent, name)
    }
}
