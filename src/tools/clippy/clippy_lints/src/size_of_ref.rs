use clippy_utils::{diagnostics::span_lint_and_help, path_def_id, ty::peel_mid_ty_refs};
use crablangc_hir::{Expr, ExprKind};
use crablangc_lint::{LateContext, LateLintPass};
use crablangc_session::{declare_lint_pass, declare_tool_lint};
use crablangc_span::sym;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// Checks for calls to `std::mem::size_of_val()` where the argument is
    /// a reference to a reference.
    ///
    /// ### Why is this bad?
    ///
    /// Calling `size_of_val()` with a reference to a reference as the argument
    /// yields the size of the reference-type, not the size of the value behind
    /// the reference.
    ///
    /// ### Example
    /// ```crablang
    /// struct Foo {
    ///     buffer: [u8],
    /// }
    ///
    /// impl Foo {
    ///     fn size(&self) -> usize {
    ///         // Note that `&self` as an argument is a `&&Foo`: Because `self`
    ///         // is already a reference, `&self` is a double-reference.
    ///         // The return value of `size_of_val()` therefor is the
    ///         // size of the reference-type, not the size of `self`.
    ///         std::mem::size_of_val(&self)
    ///     }
    /// }
    /// ```
    /// Use instead:
    /// ```crablang
    /// struct Foo {
    ///     buffer: [u8],
    /// }
    ///
    /// impl Foo {
    ///     fn size(&self) -> usize {
    ///         // Correct
    ///         std::mem::size_of_val(self)
    ///     }
    /// }
    /// ```
    #[clippy::version = "1.68.0"]
    pub SIZE_OF_REF,
    suspicious,
    "Argument to `std::mem::size_of_val()` is a double-reference, which is almost certainly unintended"
}
declare_lint_pass!(SizeOfRef => [SIZE_OF_REF]);

impl LateLintPass<'_> for SizeOfRef {
    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &'_ Expr<'_>) {
        if let ExprKind::Call(path, [arg]) = expr.kind
            && let Some(def_id) = path_def_id(cx, path)
            && cx.tcx.is_diagnostic_item(sym::mem_size_of_val, def_id)
            && let arg_ty = cx.typeck_results().expr_ty(arg)
            && peel_mid_ty_refs(arg_ty).1 > 1
        {
            span_lint_and_help(
                cx,
                SIZE_OF_REF,
                expr.span,
                "argument to `std::mem::size_of_val()` is a reference to a reference",
                None,
                "dereference the argument to `std::mem::size_of_val()` to get the size of the value instead of the size of the reference-type",
            );
        }
    }
}
