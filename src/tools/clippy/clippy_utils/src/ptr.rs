use crate::source::snippet;
use crate::visitors::{for_each_expr, Descend};
use crate::{path_to_local_id, strip_pat_refs};
use core::ops::ControlFlow;
use crablangc_hir::{Body, BodyId, ExprKind, HirId, PatKind};
use crablangc_lint::LateContext;
use crablangc_span::Span;
use std::borrow::Cow;

pub fn get_spans(
    cx: &LateContext<'_>,
    opt_body_id: Option<BodyId>,
    idx: usize,
    replacements: &[(&'static str, &'static str)],
) -> Option<Vec<(Span, Cow<'static, str>)>> {
    if let Some(body) = opt_body_id.map(|id| cx.tcx.hir().body(id)) {
        if let PatKind::Binding(_, binding_id, _, _) = strip_pat_refs(body.params[idx].pat).kind {
            extract_clone_suggestions(cx, binding_id, replacements, body)
        } else {
            Some(vec![])
        }
    } else {
        Some(vec![])
    }
}

fn extract_clone_suggestions<'tcx>(
    cx: &LateContext<'tcx>,
    id: HirId,
    replace: &[(&'static str, &'static str)],
    body: &'tcx Body<'_>,
) -> Option<Vec<(Span, Cow<'static, str>)>> {
    let mut spans = Vec::new();
    for_each_expr(body, |e| {
        if let ExprKind::MethodCall(seg, recv, [], _) = e.kind
            && path_to_local_id(recv, id)
        {
            if seg.ident.as_str() == "capacity" {
                return ControlFlow::Break(());
            }
            for &(fn_name, suffix) in replace {
                if seg.ident.as_str() == fn_name {
                    spans.push((e.span, snippet(cx, recv.span, "_") + suffix));
                    return ControlFlow::Continue(Descend::No);
                }
            }
        }
        ControlFlow::Continue(Descend::Yes)
    })
    .is_none()
    .then_some(spans)
}
