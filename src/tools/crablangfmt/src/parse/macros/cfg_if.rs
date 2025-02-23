use std::panic::{catch_unwind, AssertUnwindSafe};

use crablangc_ast::ast;
use crablangc_ast::token::{Delimiter, TokenKind};
use crablangc_parse::parser::ForceCollect;
use crablangc_span::symbol::kw;

use crate::parse::macros::build_stream_parser;
use crate::parse::session::ParseSess;

pub(crate) fn parse_cfg_if<'a>(
    sess: &'a ParseSess,
    mac: &'a ast::MacCall,
) -> Result<Vec<ast::Item>, &'static str> {
    match catch_unwind(AssertUnwindSafe(|| parse_cfg_if_inner(sess, mac))) {
        Ok(Ok(items)) => Ok(items),
        Ok(err @ Err(_)) => err,
        Err(..) => Err("failed to parse cfg_if!"),
    }
}

fn parse_cfg_if_inner<'a>(
    sess: &'a ParseSess,
    mac: &'a ast::MacCall,
) -> Result<Vec<ast::Item>, &'static str> {
    let ts = mac.args.tokens.clone();
    let mut parser = build_stream_parser(sess.inner(), ts);

    let mut items = vec![];
    let mut process_if_cfg = true;

    while parser.token.kind != TokenKind::Eof {
        if process_if_cfg {
            if !parser.eat_keyword(kw::If) {
                return Err("Expected `if`");
            }
            // Inner attributes are not actually syntactically permitted here, but we don't
            // care about inner vs outer attributes in this position. Our purpose with this
            // special case parsing of cfg_if macros is to ensure we can correctly resolve
            // imported modules that may have a custom `path` defined.
            //
            // As such, we just need to advance the parser past the attribute and up to
            // to the opening brace.
            // See also https://github.com/crablang/crablang/pull/79433
            parser
                .parse_attribute(crablangc_parse::parser::attr::InnerAttrPolicy::Permitted)
                .map_err(|_| "Failed to parse attributes")?;
        }

        if !parser.eat(&TokenKind::OpenDelim(Delimiter::Brace)) {
            return Err("Expected an opening brace");
        }

        while parser.token != TokenKind::CloseDelim(Delimiter::Brace)
            && parser.token.kind != TokenKind::Eof
        {
            let item = match parser.parse_item(ForceCollect::No) {
                Ok(Some(item_ptr)) => item_ptr.into_inner(),
                Ok(None) => continue,
                Err(err) => {
                    err.cancel();
                    parser.sess.span_diagnostic.reset_err_count();
                    return Err(
                        "Expected item inside cfg_if block, but failed to parse it as an item",
                    );
                }
            };
            if let ast::ItemKind::Mod(..) = item.kind {
                items.push(item);
            }
        }

        if !parser.eat(&TokenKind::CloseDelim(Delimiter::Brace)) {
            return Err("Expected a closing brace");
        }

        if parser.eat(&TokenKind::Eof) {
            break;
        }

        if !parser.eat_keyword(kw::Else) {
            return Err("Expected `else`");
        }

        process_if_cfg = parser.token.is_keyword(kw::If);
    }

    Ok(items)
}
