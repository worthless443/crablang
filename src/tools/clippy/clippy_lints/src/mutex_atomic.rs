//! Checks for uses of mutex where an atomic value could be used
//!
//! This lint is **allow** by default

use clippy_utils::diagnostics::span_lint;
use clippy_utils::ty::is_type_diagnostic_item;
use crablangc_hir::Expr;
use crablangc_lint::{LateContext, LateLintPass};
use crablangc_middle::ty::{self, Ty};
use crablangc_session::{declare_lint_pass, declare_tool_lint};
use crablangc_span::sym;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for usages of `Mutex<X>` where an atomic will do.
    ///
    /// ### Why is this bad?
    /// Using a mutex just to make access to a plain bool or
    /// reference sequential is shooting flies with cannons.
    /// `std::sync::atomic::AtomicBool` and `std::sync::atomic::AtomicPtr` are leaner and
    /// faster.
    ///
    /// On the other hand, `Mutex`es are, in general, easier to
    /// verify correctness. An atomic does not behave the same as
    /// an equivalent mutex. See [this issue](https://github.com/crablang/crablang-clippy/issues/4295)'s commentary for more details.
    ///
    /// ### Known problems
    /// This lint cannot detect if the mutex is actually used
    /// for waiting before a critical section.
    ///
    /// ### Example
    /// ```crablang
    /// # let y = true;
    /// # use std::sync::Mutex;
    /// let x = Mutex::new(&y);
    /// ```
    ///
    /// Use instead:
    /// ```crablang
    /// # let y = true;
    /// # use std::sync::atomic::AtomicBool;
    /// let x = AtomicBool::new(y);
    /// ```
    #[clippy::version = "pre 1.29.0"]
    pub MUTEX_ATOMIC,
    restriction,
    "using a mutex where an atomic value could be used instead."
}

declare_clippy_lint! {
    /// ### What it does
    /// Checks for usages of `Mutex<X>` where `X` is an integral
    /// type.
    ///
    /// ### Why is this bad?
    /// Using a mutex just to make access to a plain integer
    /// sequential is
    /// shooting flies with cannons. `std::sync::atomic::AtomicUsize` is leaner and faster.
    ///
    /// ### Known problems
    /// This lint cannot detect if the mutex is actually used
    /// for waiting before a critical section.
    ///
    /// ### Example
    /// ```crablang
    /// # use std::sync::Mutex;
    /// let x = Mutex::new(0usize);
    /// ```
    ///
    /// Use instead:
    /// ```crablang
    /// # use std::sync::atomic::AtomicUsize;
    /// let x = AtomicUsize::new(0usize);
    /// ```
    #[clippy::version = "pre 1.29.0"]
    pub MUTEX_INTEGER,
    nursery,
    "using a mutex for an integer type"
}

declare_lint_pass!(Mutex => [MUTEX_ATOMIC, MUTEX_INTEGER]);

impl<'tcx> LateLintPass<'tcx> for Mutex {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        let ty = cx.typeck_results().expr_ty(expr);
        if let ty::Adt(_, subst) = ty.kind() {
            if is_type_diagnostic_item(cx, ty, sym::Mutex) {
                let mutex_param = subst.type_at(0);
                if let Some(atomic_name) = get_atomic_name(mutex_param) {
                    let msg = format!(
                        "consider using an `{atomic_name}` instead of a `Mutex` here; if you just want the locking \
                         behavior and not the internal type, consider using `Mutex<()>`"
                    );
                    match *mutex_param.kind() {
                        ty::Uint(t) if t != ty::UintTy::Usize => span_lint(cx, MUTEX_INTEGER, expr.span, &msg),
                        ty::Int(t) if t != ty::IntTy::Isize => span_lint(cx, MUTEX_INTEGER, expr.span, &msg),
                        _ => span_lint(cx, MUTEX_ATOMIC, expr.span, &msg),
                    };
                }
            }
        }
    }
}

fn get_atomic_name(ty: Ty<'_>) -> Option<&'static str> {
    match ty.kind() {
        ty::Bool => Some("AtomicBool"),
        ty::Uint(_) => Some("AtomicUsize"),
        ty::Int(_) => Some("AtomicIsize"),
        ty::RawPtr(_) => Some("AtomicPtr"),
        _ => None,
    }
}
