use clippy_utils::diagnostics::span_lint;
use crablangc_ast::ast::{Crate, ItemKind, ModKind};
use crablangc_lint::{EarlyContext, EarlyLintPass};
use crablangc_session::{declare_lint_pass, declare_tool_lint};

declare_clippy_lint! {
    /// ### What it does
    /// Checks for various things we like to keep tidy in clippy.
    ///
    /// ### Why is this bad?
    /// We like to pretend we're an example of tidy code.
    ///
    /// ### Example
    /// Wrong ordering of the util::paths constants.
    pub CLIPPY_LINTS_INTERNAL,
    internal,
    "various things that will negatively affect your clippy experience"
}

declare_lint_pass!(ClippyLintsInternal => [CLIPPY_LINTS_INTERNAL]);

impl EarlyLintPass for ClippyLintsInternal {
    fn check_crate(&mut self, cx: &EarlyContext<'_>, krate: &Crate) {
        if let Some(utils) = krate.items.iter().find(|item| item.ident.name.as_str() == "utils") {
            if let ItemKind::Mod(_, ModKind::Loaded(ref items, ..)) = utils.kind {
                if let Some(paths) = items.iter().find(|item| item.ident.name.as_str() == "paths") {
                    if let ItemKind::Mod(_, ModKind::Loaded(ref items, ..)) = paths.kind {
                        let mut last_name: Option<&str> = None;
                        for item in items {
                            let name = item.ident.as_str();
                            if let Some(last_name) = last_name {
                                if *last_name > *name {
                                    span_lint(
                                        cx,
                                        CLIPPY_LINTS_INTERNAL,
                                        item.span,
                                        "this constant should be before the previous constant due to lexical \
                                         ordering",
                                    );
                                }
                            }
                            last_name = Some(name);
                        }
                    }
                }
            }
        }
    }
}
