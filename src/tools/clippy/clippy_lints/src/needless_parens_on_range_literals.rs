use clippy_utils::{
    diagnostics::span_lint_and_then,
    higher,
    source::{snippet, snippet_with_applicability},
};

use crablangc_ast::ast;
use crablangc_errors::Applicability;
use crablangc_hir::{Expr, ExprKind};

use crablangc_lint::{LateContext, LateLintPass};
use crablangc_session::{declare_lint_pass, declare_tool_lint};

declare_clippy_lint! {
  /// ### What it does
  /// The lint checks for parenthesis on literals in range statements that are
  /// superfluous.
  ///
  /// ### Why is this bad?
  /// Having superfluous parenthesis makes the code less readable
  /// overhead when reading.
  ///
  /// ### Example
  ///
  /// ```crablang
  /// for i in (0)..10 {
  ///   println!("{i}");
  /// }
  /// ```
  ///
  /// Use instead:
  ///
  /// ```crablang
  /// for i in 0..10 {
  ///   println!("{i}");
  /// }
  /// ```
  #[clippy::version = "1.63.0"]
  pub NEEDLESS_PARENS_ON_RANGE_LITERALS,
  style,
  "needless parenthesis on range literals can be removed"
}

declare_lint_pass!(NeedlessParensOnRangeLiterals => [NEEDLESS_PARENS_ON_RANGE_LITERALS]);

fn snippet_enclosed_in_parenthesis(snippet: &str) -> bool {
    snippet.starts_with('(') && snippet.ends_with(')')
}

fn check_for_parens(cx: &LateContext<'_>, e: &Expr<'_>, is_start: bool) {
    if is_start &&
    let ExprKind::Lit(ref literal) = e.kind &&
    let ast::LitKind::Float(_sym, ast::LitFloatType::Unsuffixed) = literal.node
    {
        // don't check floating point literals on the start expression of a range
        return;
    }
    if_chain! {
        if let ExprKind::Lit(ref literal) = e.kind;
        // the indicator that parenthesis surround the literal is that the span of the expression and the literal differ
        if (literal.span.data().hi - literal.span.data().lo) != (e.span.data().hi - e.span.data().lo);
        // inspect the source code of the expression for parenthesis
        if snippet_enclosed_in_parenthesis(&snippet(cx, e.span, ""));
        then {
            let mut applicability = Applicability::MachineApplicable;
            span_lint_and_then(cx, NEEDLESS_PARENS_ON_RANGE_LITERALS, e.span,
                "needless parenthesis on range literals can be removed",
                |diag| {
                    let suggestion = snippet_with_applicability(cx, literal.span, "_", &mut applicability);
                    diag.span_suggestion(e.span, "try", suggestion, applicability);
                });
        }
    }
}

impl<'tcx> LateLintPass<'tcx> for NeedlessParensOnRangeLiterals {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if let Some(higher::Range { start, end, .. }) = higher::Range::hir(expr) {
            if let Some(start) = start {
                check_for_parens(cx, start, true);
            }
            if let Some(end) = end {
                check_for_parens(cx, end, false);
            }
        }
    }
}
