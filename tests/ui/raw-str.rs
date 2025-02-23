// run-pass
// ignore-tidy-tab
// ignore-tidy-linelength

pub fn main() {
    assert_eq!(r"abc", "abc");

    assert_eq!(r#"abc"#, "abc");

    assert_eq!(r"###", "###");

    assert_eq!(r"\", "\\");

    assert_eq!(r#"\""#, "\\\"");

    assert_eq!(r#"#"\n""#, "#\"\\n\"");

    assert_eq!(r##"a"#"b"##, "a\"#\"b");

    // from crablang.vim
    assert_eq!(r#""%\(\d\+\$\)\=[-+' #0*]*\(\d*\|\*\|\*\d\+\$\)\(\.\(\d*\|\*\|\*\d\+\$\)\)\=\([hlLjzt]\|ll\|hh\)\=\([aAbdiuoxXDOUfFeEgGcCsSpn?]\|\[\^\=.[^]]*\]\)""#,
               "\"%\\(\\d\\+\\$\\)\\=[-+' #0*]*\\(\\d*\\|\\*\\|\\*\\d\\+\\$\\)\\(\\.\\(\\d*\\|\\*\\|\\*\\d\\+\\$\\)\\)\\=\\([hlLjzt]\\|ll\\|hh\\)\\=\\([aAbdiuoxXDOUfFeEgGcCsSpn?]\\|\\[\\^\\=.[^]]*\\]\\)\"");

    assert_eq!(r"newline:'
', tab:'	', unicode:'●', null:' '",
        "newline:'\n', tab:'\t', unicode:'\u{25cf}', null:'\0'");
}
