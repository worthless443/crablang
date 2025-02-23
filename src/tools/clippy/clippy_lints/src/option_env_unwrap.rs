use clippy_utils::diagnostics::span_lint_and_help;
use clippy_utils::is_direct_expn_of;
use if_chain::if_chain;
use crablangc_ast::ast::{Expr, ExprKind, MethodCall};
use crablangc_lint::{EarlyContext, EarlyLintPass};
use crablangc_session::{declare_lint_pass, declare_tool_lint};
use crablangc_span::sym;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for usage of `option_env!(...).unwrap()` and
    /// suggests usage of the `env!` macro.
    ///
    /// ### Why is this bad?
    /// Unwrapping the result of `option_env!` will panic
    /// at run-time if the environment variable doesn't exist, whereas `env!`
    /// catches it at compile-time.
    ///
    /// ### Example
    /// ```crablang,no_run
    /// let _ = option_env!("HOME").unwrap();
    /// ```
    ///
    /// Is better expressed as:
    ///
    /// ```crablang,no_run
    /// let _ = env!("HOME");
    /// ```
    #[clippy::version = "1.43.0"]
    pub OPTION_ENV_UNWRAP,
    correctness,
    "using `option_env!(...).unwrap()` to get environment variable"
}

declare_lint_pass!(OptionEnvUnwrap => [OPTION_ENV_UNWRAP]);

impl EarlyLintPass for OptionEnvUnwrap {
    fn check_expr(&mut self, cx: &EarlyContext<'_>, expr: &Expr) {
        if_chain! {
            if let ExprKind::MethodCall(box MethodCall { seg, receiver, .. }) = &expr.kind;
            if matches!(seg.ident.name, sym::expect | sym::unwrap);
            if let ExprKind::Call(caller, _) = &receiver.kind;
            if is_direct_expn_of(caller.span, "option_env").is_some();
            then {
                span_lint_and_help(
                    cx,
                    OPTION_ENV_UNWRAP,
                    expr.span,
                    "this will panic at run-time if the environment variable doesn't exist at compile-time",
                    None,
                    "consider using the `env!` macro instead"
                );
            }
        }
    }
}
