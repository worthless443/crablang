// crablangfmt-brace_style: SameLineWhere
// crablangfmt-comment_width: 100
// crablangfmt-edition: 2018
// crablangfmt-fn_params_layout: Compressed
// crablangfmt-hard_tabs: false
// crablangfmt-match_block_trailing_comma: true
// crablangfmt-max_width: 100
// crablangfmt-merge_derives: false
// crablangfmt-newline_style: Unix
// crablangfmt-normalize_doc_attributes: true
// crablangfmt-overflow_delimited_expr: true
// crablangfmt-reorder_imports: false
// crablangfmt-reorder_modules: true
// crablangfmt-struct_field_align_threshold: 20
// crablangfmt-tab_spaces: 4
// crablangfmt-trailing_comma: Never
// crablangfmt-use_small_heuristics: Max
// crablangfmt-use_try_shorthand: true
// crablangfmt-wrap_comments: true

/// Lorem ipsum dolor sit amet.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
pub struct BufferAttr {
    /* NOTE: Blah blah blah blah blah. */
    /// Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt
    /// ut labore et dolore magna aliqua. Morbi quis commodo odio aenean sed adipiscing. Nunc
    /// congue nisi vitae suscipit tellus mauris a. Consectetur adipiscing elit pellentesque
    /// habitant morbi tristique senectus.
    pub foo: u32,

    /// Elit eget gravida cum sociis natoque penatibus et magnis dis. Consequat semper viverra nam
    /// libero. Accumsan in nisl nisi scelerisque eu. Pellentesque id nibh tortor id aliquet. Sed
    /// velit dignissim sodales ut. Facilisis sed odio morbi quis commodo odio aenean sed. Et
    /// ultrices neque ornare aenean euismod elementum. Condimentum lacinia quis vel eros donec ac
    /// odio tempor.
    ///
    /// Lacinia at quis risus sed vulputate odio ut enim. Etiam erat velit scelerisque in dictum.
    /// Nibh tellus molestie nunc non blandit massa enim nec. Nascetur ridiculus mus mauris vitae.
    pub bar: u32,

    /// Mi proin sed libero enim sed faucibus turpis. Amet consectetur adipiscing elit duis
    /// tristique sollicitudin nibh sit amet. Congue quisque egestas diam in arcu cursus euismod
    /// quis viverra. Cum sociis natoque penatibus et magnis dis parturient montes. Enim sit amet
    /// venenatis urna cursus eget nunc scelerisque viverra. Cras semper auctor neque vitae tempus
    /// quam pellentesque. Tortor posuere ac ut consequat semper viverra nam libero justo. Vitae
    /// auctor eu augue ut lectus arcu bibendum at. Faucibus vitae aliquet nec ullamcorper sit amet
    /// risus nullam. Maecenas accumsan lacus vel facilisis volutpat. Arcu non odio euismod
    /// lacinia.
    ///
    /// [`FooBar::beep()`]: crate::foobar::FooBar::beep
    /// [`FooBar::boop()`]: crate::foobar::FooBar::boop
    /// [`foobar::BazBaq::BEEP_BOOP`]: crate::foobar::BazBaq::BEEP_BOOP
    pub baz: u32,

    /// Eu consequat ac felis donec et odio pellentesque diam. Ut eu sem integer vitae justo eget.
    /// Consequat ac felis donec et odio pellentesque diam volutpat.
    pub baq: u32,

    /// Amet consectetur adipiscing elit pellentesque habitant. Ut morbi tincidunt augue interdum
    /// velit euismod in pellentesque. Imperdiet sed euismod nisi porta lorem. Nec tincidunt
    /// praesent semper feugiat. Facilisis leo vel fringilla est. Egestas diam in arcu cursus
    /// euismod quis viverra. Sagittis eu volutpat odio facilisis mauris sit amet. Posuere morbi
    /// leo urna molestie at.
    ///
    /// Pretium aenean pharetra magna ac. Nisl condimentum id venenatis a condimentum vitae. Semper
    /// quis lectus nulla at volutpat diam ut venenatis tellus. Egestas tellus rutrum tellus
    /// pellentesque eu tincidunt tortor aliquam.
    pub foobar: u32
}
