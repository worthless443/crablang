#[cfg(feature="master")]
use gccjit::{VarAttribute, FnAttribute};
use crablangc_codegen_ssa::traits::PreDefineMethods;
use crablangc_hir::def_id::{DefId, LOCAL_CRATE};
use crablangc_middle::middle::codegen_fn_attrs::CodegenFnAttrFlags;
use crablangc_middle::mir::mono::{Linkage, Visibility};
use crablangc_middle::ty::{self, Instance, TypeVisitableExt};
use crablangc_middle::ty::layout::{FnAbiOf, LayoutOf};

use crate::attributes;
use crate::base;
use crate::context::CodegenCx;
use crate::type_of::LayoutGccExt;

impl<'gcc, 'tcx> PreDefineMethods<'tcx> for CodegenCx<'gcc, 'tcx> {
    #[cfg_attr(not(feature="master"), allow(unused_variables))]
    fn predefine_static(&self, def_id: DefId, _linkage: Linkage, visibility: Visibility, symbol_name: &str) {
        let attrs = self.tcx.codegen_fn_attrs(def_id);
        let instance = Instance::mono(self.tcx, def_id);
        let ty = instance.ty(self.tcx, ty::ParamEnv::reveal_all());
        let gcc_type = self.layout_of(ty).gcc_type(self);

        let is_tls = attrs.flags.contains(CodegenFnAttrFlags::THREAD_LOCAL);
        let global = self.define_global(symbol_name, gcc_type, is_tls, attrs.link_section);
        #[cfg(feature="master")]
        global.add_attribute(VarAttribute::Visibility(base::visibility_to_gcc(visibility)));

        // TODO(antoyo): set linkage.
        self.instances.borrow_mut().insert(instance, global);
    }

    #[cfg_attr(not(feature="master"), allow(unused_variables))]
    fn predefine_fn(&self, instance: Instance<'tcx>, linkage: Linkage, visibility: Visibility, symbol_name: &str) {
        assert!(!instance.substs.needs_infer());

        let fn_abi = self.fn_abi_of_instance(instance, ty::List::empty());
        self.linkage.set(base::linkage_to_gcc(linkage));
        let decl = self.declare_fn(symbol_name, &fn_abi);
        //let attrs = self.tcx.codegen_fn_attrs(instance.def_id());

        attributes::from_fn_attrs(self, decl, instance);

        // If we're compiling the compiler-builtins crate, e.g., the equivalent of
        // compiler-rt, then we want to implicitly compile everything with hidden
        // visibility as we're going to link this object all over the place but
        // don't want the symbols to get exported.
        if linkage != Linkage::Internal
            && linkage != Linkage::Private
            && self.tcx.is_compiler_builtins(LOCAL_CRATE)
        {
            #[cfg(feature="master")]
            decl.add_attribute(FnAttribute::Visibility(gccjit::Visibility::Hidden));
        }
        else {
            #[cfg(feature="master")]
            decl.add_attribute(FnAttribute::Visibility(base::visibility_to_gcc(visibility)));
        }

        // TODO(antoyo): call set_link_section() to allow initializing argc/argv.
        // TODO(antoyo): set unique comdat.
        // TODO(antoyo): use inline attribute from there in linkage.set() above.

        self.functions.borrow_mut().insert(symbol_name.to_string(), decl);
        self.function_instances.borrow_mut().insert(instance, unsafe { std::mem::transmute(decl) });
    }
}
