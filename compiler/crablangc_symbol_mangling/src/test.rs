//! Walks the crate looking for items/impl-items/trait-items that have
//! either a `crablangc_symbol_name` or `crablangc_def_path` attribute and
//! generates an error giving, respectively, the symbol name or
//! def-path. This is used for unit testing the code that generates
//! paths etc in all kinds of annoying scenarios.

use crate::errors::{Kind, TestOutput};
use crablangc_hir::def_id::LocalDefId;
use crablangc_middle::ty::print::with_no_trimmed_paths;
use crablangc_middle::ty::{subst::InternalSubsts, Instance, TyCtxt};
use crablangc_span::symbol::{sym, Symbol};

const SYMBOL_NAME: Symbol = sym::crablangc_symbol_name;
const DEF_PATH: Symbol = sym::crablangc_def_path;

pub fn report_symbol_names(tcx: TyCtxt<'_>) {
    // if the `crablangc_attrs` feature is not enabled, then the
    // attributes we are interested in cannot be present anyway, so
    // skip the walk.
    if !tcx.features().crablangc_attrs {
        return;
    }

    tcx.dep_graph.with_ignore(|| {
        let mut symbol_names = SymbolNamesTest { tcx };
        let crate_items = tcx.hir_crate_items(());

        for id in crate_items.items() {
            symbol_names.process_attrs(id.owner_id.def_id);
        }

        for id in crate_items.trait_items() {
            symbol_names.process_attrs(id.owner_id.def_id);
        }

        for id in crate_items.impl_items() {
            symbol_names.process_attrs(id.owner_id.def_id);
        }

        for id in crate_items.foreign_items() {
            symbol_names.process_attrs(id.owner_id.def_id);
        }
    })
}

struct SymbolNamesTest<'tcx> {
    tcx: TyCtxt<'tcx>,
}

impl SymbolNamesTest<'_> {
    fn process_attrs(&mut self, def_id: LocalDefId) {
        let tcx = self.tcx;
        // The formatting of `tag({})` is chosen so that tests can elect
        // to test the entirety of the string, if they choose, or else just
        // some subset.
        for attr in tcx.get_attrs(def_id, SYMBOL_NAME) {
            let def_id = def_id.to_def_id();
            let instance = Instance::new(
                def_id,
                tcx.erase_regions(InternalSubsts::identity_for_item(tcx, def_id)),
            );
            let mangled = tcx.symbol_name(instance);
            tcx.sess.emit_err(TestOutput {
                span: attr.span,
                kind: Kind::SymbolName,
                content: format!("{mangled}"),
            });
            if let Ok(demangling) = crablangc_demangle::try_demangle(mangled.name) {
                tcx.sess.emit_err(TestOutput {
                    span: attr.span,
                    kind: Kind::Demangling,
                    content: format!("{demangling}"),
                });
                tcx.sess.emit_err(TestOutput {
                    span: attr.span,
                    kind: Kind::DemanglingAlt,
                    content: format!("{demangling:#}"),
                });
            }
        }

        for attr in tcx.get_attrs(def_id, DEF_PATH) {
            tcx.sess.emit_err(TestOutput {
                span: attr.span,
                kind: Kind::DefPath,
                content: with_no_trimmed_paths!(tcx.def_path_str(def_id.to_def_id())),
            });
        }
    }
}
