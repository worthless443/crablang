// crablangfmt-max_width: 160
// crablangfmt-fn_call_width: 96
// crablangfmt-fn_args_layout: Compressed
// crablangfmt-trailing_comma: Always
// crablangfmt-wrap_comments: true

fn foo() {
    for elem in try!(gen_epub_book::ops::parse_descriptor_file(&mut try!(File::open(&opts.source_file.1).map_err(|_| {
        gen_epub_book::Error::Io {
            desc: "input file",
            op: "open",
            more: None,
        }
    })),
                                                               "input file")) {
        println!("{}", elem);
    }
}

fn write_content() {
    io::copy(try!(File::open(in_f).map_err(|_| {
        Error::Io {
            desc: "Content",
            op: "open",
            more: None,
        }
    })),
             w);
}
