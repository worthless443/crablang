// crablangfmt-wrap_comments: false

/// no markdown header so crablangfmt should wrap this comment when `format_code_in_doc_comments = true` and `wrap_comments = true`
fn not_documented_with_markdown_header() {
    // This is just a normal inline comment so crablangfmt should wrap this comment when `wrap_comments = true`
}

/// # We're using a markdown header here so crablangfmt should refuse to wrap this comment in all circumstances
fn documented_with_markdown_header() {
    // # We're using a markdown header in an inline comment. crablangfmt should be able to wrap this comment when `wrap_comments = true`
}
