mod context;

use crate::edition_panic::use_panic_2021;
use crate::errors;
use crablangc_ast::ptr::P;
use crablangc_ast::token;
use crablangc_ast::tokenstream::{DelimSpan, TokenStream};
use crablangc_ast::{DelimArgs, Expr, ExprKind, MacCall, MacDelimiter, Path, PathSegment, UnOp};
use crablangc_ast_pretty::ppcrablang;
use crablangc_errors::PResult;
use crablangc_expand::base::{DummyResult, ExtCtxt, MacEager, MacResult};
use crablangc_parse::parser::Parser;
use crablangc_span::symbol::{sym, Ident, Symbol};
use crablangc_span::{Span, DUMMY_SP};
use thin_vec::thin_vec;

pub fn expand_assert<'cx>(
    cx: &'cx mut ExtCtxt<'_>,
    span: Span,
    tts: TokenStream,
) -> Box<dyn MacResult + 'cx> {
    let Assert { cond_expr, custom_message } = match parse_assert(cx, span, tts) {
        Ok(assert) => assert,
        Err(mut err) => {
            err.emit();
            return DummyResult::any(span);
        }
    };

    // `core::panic` and `std::panic` are different macros, so we use call-site
    // context to pick up whichever is currently in scope.
    let call_site_span = cx.with_call_site_ctxt(span);

    let panic_path = || {
        if use_panic_2021(span) {
            // On edition 2021, we always call `$crate::panic::panic_2021!()`.
            Path {
                span: call_site_span,
                segments: cx
                    .std_path(&[sym::panic, sym::panic_2021])
                    .into_iter()
                    .map(|ident| PathSegment::from_ident(ident))
                    .collect(),
                tokens: None,
            }
        } else {
            // Before edition 2021, we call `panic!()` unqualified,
            // such that it calls either `std::panic!()` or `core::panic!()`.
            Path::from_ident(Ident::new(sym::panic, call_site_span))
        }
    };

    // Simply uses the user provided message instead of generating custom outputs
    let expr = if let Some(tokens) = custom_message {
        let then = cx.expr(
            call_site_span,
            ExprKind::MacCall(P(MacCall {
                path: panic_path(),
                args: P(DelimArgs {
                    dspan: DelimSpan::from_single(call_site_span),
                    delim: MacDelimiter::Parenthesis,
                    tokens,
                }),
                prior_type_ascription: None,
            })),
        );
        expr_if_not(cx, call_site_span, cond_expr, then, None)
    }
    // If `generic_assert` is enabled, generates rich captured outputs
    //
    // FIXME(c410-f3r) See https://github.com/crablang/crablang/issues/96949
    else if let Some(features) = cx.ecfg.features && features.generic_assert {
        context::Context::new(cx, call_site_span).build(cond_expr, panic_path())
    }
    // If `generic_assert` is not enabled, only outputs a literal "assertion failed: ..."
    // string
    else {
        // Pass our own message directly to $crate::panicking::panic(),
        // because it might contain `{` and `}` that should always be
        // passed literally.
        let then = cx.expr_call_global(
            call_site_span,
            cx.std_path(&[sym::panicking, sym::panic]),
            thin_vec![cx.expr_str(
                DUMMY_SP,
                Symbol::intern(&format!(
                    "assertion failed: {}",
                    ppcrablang::expr_to_string(&cond_expr).escape_debug()
                )),
            )],
        );
        expr_if_not(cx, call_site_span, cond_expr, then, None)
    };

    MacEager::expr(expr)
}

struct Assert {
    cond_expr: P<Expr>,
    custom_message: Option<TokenStream>,
}

// if !{ ... } { ... } else { ... }
fn expr_if_not(
    cx: &ExtCtxt<'_>,
    span: Span,
    cond: P<Expr>,
    then: P<Expr>,
    els: Option<P<Expr>>,
) -> P<Expr> {
    cx.expr_if(span, cx.expr(span, ExprKind::Unary(UnOp::Not, cond)), then, els)
}

fn parse_assert<'a>(cx: &mut ExtCtxt<'a>, sp: Span, stream: TokenStream) -> PResult<'a, Assert> {
    let mut parser = cx.new_parser_from_tts(stream);

    if parser.token == token::Eof {
        return Err(cx.create_err(errors::AssertRequiresBoolean { span: sp }));
    }

    let cond_expr = parser.parse_expr()?;

    // Some crates use the `assert!` macro in the following form (note extra semicolon):
    //
    // assert!(
    //     my_function();
    // );
    //
    // Emit an error about semicolon and suggest removing it.
    if parser.token == token::Semi {
        cx.emit_err(errors::AssertRequiresExpression { span: sp, token: parser.token.span });
        parser.bump();
    }

    // Some crates use the `assert!` macro in the following form (note missing comma before
    // message):
    //
    // assert!(true "error message");
    //
    // Emit an error and suggest inserting a comma.
    let custom_message =
        if let token::Literal(token::Lit { kind: token::Str, .. }) = parser.token.kind {
            let comma = parser.prev_token.span.shrink_to_hi();
            cx.emit_err(errors::AssertMissingComma { span: parser.token.span, comma });

            parse_custom_message(&mut parser)
        } else if parser.eat(&token::Comma) {
            parse_custom_message(&mut parser)
        } else {
            None
        };

    if parser.token != token::Eof {
        return parser.unexpected();
    }

    Ok(Assert { cond_expr, custom_message })
}

fn parse_custom_message(parser: &mut Parser<'_>) -> Option<TokenStream> {
    let ts = parser.parse_tokens();
    if !ts.is_empty() { Some(ts) } else { None }
}
