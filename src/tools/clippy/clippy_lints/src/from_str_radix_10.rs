use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::is_integer_literal;
use clippy_utils::sugg::Sugg;
use clippy_utils::ty::{is_type_diagnostic_item, is_type_lang_item};
use if_chain::if_chain;
use crablangc_errors::Applicability;
use crablangc_hir::{def, Expr, ExprKind, LangItem, PrimTy, QPath, TyKind};
use crablangc_lint::{LateContext, LateLintPass};
use crablangc_middle::ty::Ty;
use crablangc_session::{declare_lint_pass, declare_tool_lint};
use crablangc_span::symbol::sym;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// Checks for function invocations of the form `primitive::from_str_radix(s, 10)`
    ///
    /// ### Why is this bad?
    ///
    /// This specific common use case can be rewritten as `s.parse::<primitive>()`
    /// (and in most cases, the turbofish can be removed), which reduces code length
    /// and complexity.
    ///
    /// ### Known problems
    ///
    /// This lint may suggest using (&<expression>).parse() instead of <expression>.parse() directly
    /// in some cases, which is correct but adds unnecessary complexity to the code.
    ///
    /// ### Example
    /// ```ignore
    /// let input: &str = get_input();
    /// let num = u16::from_str_radix(input, 10)?;
    /// ```
    /// Use instead:
    /// ```ignore
    /// let input: &str = get_input();
    /// let num: u16 = input.parse()?;
    /// ```
    #[clippy::version = "1.52.0"]
    pub FROM_STR_RADIX_10,
    style,
    "from_str_radix with radix 10"
}

declare_lint_pass!(FromStrRadix10 => [FROM_STR_RADIX_10]);

impl<'tcx> LateLintPass<'tcx> for FromStrRadix10 {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, exp: &Expr<'tcx>) {
        if_chain! {
            if let ExprKind::Call(maybe_path, [src, radix]) = &exp.kind;
            if let ExprKind::Path(QPath::TypeRelative(ty, pathseg)) = &maybe_path.kind;

            // check if the first part of the path is some integer primitive
            if let TyKind::Path(ty_qpath) = &ty.kind;
            let ty_res = cx.qpath_res(ty_qpath, ty.hir_id);
            if let def::Res::PrimTy(prim_ty) = ty_res;
            if matches!(prim_ty, PrimTy::Int(_) | PrimTy::Uint(_));

            // check if the second part of the path indeed calls the associated
            // function `from_str_radix`
            if pathseg.ident.name.as_str() == "from_str_radix";

            // check if the second argument is a primitive `10`
            if is_integer_literal(radix, 10);

            then {
                let expr = if let ExprKind::AddrOf(_, _, expr) = &src.kind {
                    let ty = cx.typeck_results().expr_ty(expr);
                    if is_ty_stringish(cx, ty) {
                        expr
                    } else {
                        &src
                    }
                } else {
                    &src
                };

                let sugg = Sugg::hir_with_applicability(
                    cx,
                    expr,
                    "<string>",
                    &mut Applicability::MachineApplicable
                ).maybe_par();

                span_lint_and_sugg(
                    cx,
                    FROM_STR_RADIX_10,
                    exp.span,
                    "this call to `from_str_radix` can be replaced with a call to `str::parse`",
                    "try",
                    format!("{sugg}.parse::<{}>()", prim_ty.name_str()),
                    Applicability::MaybeIncorrect
                );
            }
        }
    }
}

/// Checks if a Ty is `String` or `&str`
fn is_ty_stringish(cx: &LateContext<'_>, ty: Ty<'_>) -> bool {
    is_type_lang_item(cx, ty, LangItem::String) || is_type_diagnostic_item(cx, ty, sym::str)
}
