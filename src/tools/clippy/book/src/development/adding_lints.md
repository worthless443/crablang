# Adding a new lint

You are probably here because you want to add a new lint to Clippy. If this is
the first time you're contributing to Clippy, this document guides you through
creating an example lint from scratch.

To get started, we will create a lint that detects functions called `foo`,
because that's clearly a non-descriptive name.

- [Adding a new lint](#adding-a-new-lint)
  - [Setup](#setup)
  - [Getting Started](#getting-started)
    - [Defining Our Lint](#defining-our-lint)
      - [Standalone](#standalone)
      - [Specific Type](#specific-type)
      - [Tests Location](#tests-location)
  - [Testing](#testing)
    - [Cargo lints](#cargo-lints)
  - [CrabLangfix tests](#crablangfix-tests)
  - [Testing manually](#testing-manually)
  - [Lint declaration](#lint-declaration)
  - [Lint registration](#lint-registration)
  - [Lint passes](#lint-passes)
  - [Emitting a lint](#emitting-a-lint)
  - [Adding the lint logic](#adding-the-lint-logic)
  - [Specifying the lint's minimum supported CrabLang version (MSRV)](#specifying-the-lints-minimum-supported-crablang-version-msrv)
  - [Author lint](#author-lint)
  - [Print HIR lint](#print-hir-lint)
  - [Documentation](#documentation)
  - [Running crablangfmt](#running-crablangfmt)
  - [Debugging](#debugging)
  - [PR Checklist](#pr-checklist)
  - [Adding configuration to a lint](#adding-configuration-to-a-lint)
  - [Cheat Sheet](#cheat-sheet)

## Setup

See the [Basics](basics.md#get-the-code) documentation.

## Getting Started

There is a bit of boilerplate code that needs to be set up when creating a new
lint. Fortunately, you can use the Clippy dev tools to handle this for you. We
are naming our new lint `foo_functions` (lints are generally written in snake
case), and we don't need type information, so it will have an early pass type
(more on this later). If you're unsure if the name you chose fits the lint,
take a look at our [lint naming guidelines][lint_naming].

## Defining Our Lint
To get started, there are two ways to define our lint.

### Standalone
Command: `cargo dev new_lint --name=foo_functions --pass=early --category=pedantic`
(category will default to nursery if not provided)

This command will create a new file: `clippy_lints/src/foo_functions.rs`, as well
as [register the lint](#lint-registration).

### Specific Type
Command: `cargo dev new_lint --name=foo_functions --type=functions --category=pedantic`

This command will create a new file: `clippy_lints/src/{type}/foo_functions.rs`.

Notice how this command has a `--type` flag instead of `--pass`. Unlike a standalone
definition, this lint won't be registered in the traditional sense. Instead, you will
call your lint from within the type's lint pass, found in `clippy_lints/src/{type}/mod.rs`.

A "type" is just the name of a directory in `clippy_lints/src`, like `functions` in
the example command. These are groupings of lints with common behaviors, so if your
lint falls into one, it would be best to add it to that type.

### Tests Location
Both commands will create a file: `tests/ui/foo_functions.rs`. For cargo lints,
two project hierarchies (fail/pass) will be created by default under `tests/ui-cargo`.

Next, we'll open up these files and add our lint!

## Testing

Let's write some tests first that we can execute while we iterate on our lint.

Clippy uses UI tests for testing. UI tests check that the output of Clippy is
exactly as expected. Each test is just a plain CrabLang file that contains the code
we want to check. The output of Clippy is compared against a `.stderr` file.
Note that you don't have to create this file yourself, we'll get to generating
the `.stderr` files further down.

We start by opening the test file created at `tests/ui/foo_functions.rs`.

Update the file with some examples to get started:

```crablang
#![allow(unused)]
#![warn(clippy::foo_functions)]

// Impl methods
struct A;
impl A {
    pub fn fo(&self) {}
    pub fn foo(&self) {}
    pub fn food(&self) {}
}

// Default trait methods
trait B {
    fn fo(&self) {}
    fn foo(&self) {}
    fn food(&self) {}
}

// Plain functions
fn fo() {}
fn foo() {}
fn food() {}

fn main() {
    // We also don't want to lint method calls
    foo();
    let a = A;
    a.foo();
}
```

Now we can run the test with `TESTNAME=foo_functions cargo uitest`, currently
this test is meaningless though.

While we are working on implementing our lint, we can keep running the UI test.
That allows us to check if the output is turning into what we want.

Once we are satisfied with the output, we need to run `cargo dev bless` to
update the `.stderr` file for our lint. Please note that, we should run
`TESTNAME=foo_functions cargo uitest` every time before running `cargo dev
bless`. Running `TESTNAME=foo_functions cargo uitest` should pass then. When we
commit our lint, we need to commit the generated `.stderr` files, too. In
general, you should only commit files changed by `cargo dev bless` for the
specific lint you are creating/editing. Note that if the generated files are
empty, they should be removed.

> _Note:_ you can run multiple test files by specifying a comma separated list:
> `TESTNAME=foo_functions,test2,test3`.

### Cargo lints

For cargo lints, the process of testing differs in that we are interested in the
`Cargo.toml` manifest file. We also need a minimal crate associated with that
manifest.

If our new lint is named e.g. `foo_categories`, after running `cargo dev
new_lint --name=foo_categories --type=cargo --category=cargo` we will find by
default two new crates, each with its manifest file:

* `tests/ui-cargo/foo_categories/fail/Cargo.toml`: this file should cause the
  new lint to raise an error.
* `tests/ui-cargo/foo_categories/pass/Cargo.toml`: this file should not trigger
  the lint.

If you need more cases, you can copy one of those crates (under
`foo_categories`) and rename it.

The process of generating the `.stderr` file is the same, and prepending the
`TESTNAME` variable to `cargo uitest` works too.

## CrabLangfix tests

If the lint you are working on is making use of structured suggestions, the test
file should include a `// run-crablangfix` comment at the top. This will
additionally run [crablangfix] for that test. CrabLangfix will apply the suggestions
from the lint to the code of the test file and compare that to the contents of a
`.fixed` file.

Use `cargo dev bless` to automatically generate the `.fixed` file after running
the tests.

[crablangfix]: https://github.com/crablang/crablangfix

## Testing manually

Manually testing against an example file can be useful if you have added some
`println!`s and the test suite output becomes unreadable. To try Clippy with
your local modifications, run

```
cargo dev lint input.rs
```

from the working copy root. With tests in place, let's have a look at
implementing our lint now.

## Lint declaration

Let's start by opening the new file created in the `clippy_lints` crate at
`clippy_lints/src/foo_functions.rs`. That's the crate where all the lint code
is. This file has already imported some initial things we will need:

```crablang
use crablangc_lint::{EarlyLintPass, EarlyContext};
use crablangc_session::{declare_lint_pass, declare_tool_lint};
use crablangc_ast::ast::*;
```

The next step is to update the lint declaration. Lints are declared using the
[`declare_clippy_lint!`][declare_clippy_lint] macro, and we just need to update
the auto-generated lint declaration to have a real description, something like
this:

```crablang
declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```crablang
    /// // example code
    /// ```
    #[clippy::version = "1.29.0"]
    pub FOO_FUNCTIONS,
    pedantic,
    "function named `foo`, which is not a descriptive name"
}
```

* The section of lines prefixed with `///` constitutes the lint documentation
  section. This is the default documentation style and will be displayed [like
  this][example_lint_page]. To render and open this documentation locally in a
  browser, run `cargo dev serve`.
* The `#[clippy::version]` attribute will be rendered as part of the lint
  documentation. The value should be set to the current CrabLang version that the
  lint is developed in, it can be retrieved by running `crablangc -vV` in the
  crablang-clippy directory. The version is listed under *release*. (Use the version
  without the `-nightly`) suffix.
* `FOO_FUNCTIONS` is the name of our lint. Be sure to follow the [lint naming
  guidelines][lint_naming] here when naming your lint. In short, the name should
  state the thing that is being checked for and read well when used with
  `allow`/`warn`/`deny`.
* `pedantic` sets the lint level to `Allow`. The exact mapping can be found
  [here][category_level_mapping]
* The last part should be a text that explains what exactly is wrong with the
  code

The rest of this file contains an empty implementation for our lint pass, which
in this case is `EarlyLintPass` and should look like this:

```crablang
// clippy_lints/src/foo_functions.rs

// .. imports and lint declaration ..

declare_lint_pass!(FooFunctions => [FOO_FUNCTIONS]);

impl EarlyLintPass for FooFunctions {}
```

[declare_clippy_lint]: https://github.com/crablang/crablang-clippy/blob/557f6848bd5b7183f55c1e1522a326e9e1df6030/clippy_lints/src/lib.rs#L60
[example_lint_page]: https://crablang.github.io/crablang-clippy/master/index.html#redundant_closure
[lint_naming]: https://crablang.github.io/rfcs/0344-conventions-galore.html#lints
[category_level_mapping]: https://github.com/crablang/crablang-clippy/blob/557f6848bd5b7183f55c1e1522a326e9e1df6030/clippy_lints/src/lib.rs#L110

## Lint registration

When using `cargo dev new_lint`, the lint is automatically registered and
nothing more has to be done.

When declaring a new lint by hand and `cargo dev update_lints` is used, the lint
pass may have to be registered manually in the `register_plugins` function in
`clippy_lints/src/lib.rs`:

```crablang
store.register_early_pass(|| Box::new(foo_functions::FooFunctions));
```

As one may expect, there is a corresponding `register_late_pass` method
available as well. Without a call to one of `register_early_pass` or
`register_late_pass`, the lint pass in question will not be run.

One reason that `cargo dev update_lints` does not automate this step is that
multiple lints can use the same lint pass, so registering the lint pass may
already be done when adding a new lint. Another reason that this step is not
automated is that the order that the passes are registered determines the order
the passes actually run, which in turn affects the order that any emitted lints
are output in.

## Lint passes

Writing a lint that only checks for the name of a function means that we only
have to deal with the AST and don't have to deal with the type system at all.
This is good, because it makes writing this particular lint less complicated.

We have to make this decision with every new Clippy lint. It boils down to using
either [`EarlyLintPass`][early_lint_pass] or [`LateLintPass`][late_lint_pass].

In short, the `LateLintPass` has access to type information while the
`EarlyLintPass` doesn't. If you don't need access to type information, use the
`EarlyLintPass`. The `EarlyLintPass` is also faster. However linting speed
hasn't really been a concern with Clippy so far.

Since we don't need type information for checking the function name, we used
`--pass=early` when running the new lint automation and all the imports were
added accordingly.

[early_lint_pass]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_lint/trait.EarlyLintPass.html
[late_lint_pass]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_lint/trait.LateLintPass.html

## Emitting a lint

With UI tests and the lint declaration in place, we can start working on the
implementation of the lint logic.

Let's start by implementing the `EarlyLintPass` for our `FooFunctions`:

```crablang
impl EarlyLintPass for FooFunctions {
    fn check_fn(&mut self, cx: &EarlyContext<'_>, fn_kind: FnKind<'_>, span: Span, _: NodeId) {
        // TODO: Emit lint here
    }
}
```

We implement the [`check_fn`][check_fn] method from the
[`EarlyLintPass`][early_lint_pass] trait. This gives us access to various
information about the function that is currently being checked. More on that in
the next section. Let's worry about the details later and emit our lint for
*every* function definition first.

Depending on how complex we want our lint message to be, we can choose from a
variety of lint emission functions. They can all be found in
[`clippy_utils/src/diagnostics.rs`][diagnostics].

`span_lint_and_help` seems most appropriate in this case. It allows us to
provide an extra help message and we can't really suggest a better name
automatically. This is how it looks:

```crablang
impl EarlyLintPass for FooFunctions {
    fn check_fn(&mut self, cx: &EarlyContext<'_>, fn_kind: FnKind<'_>, span: Span, _: NodeId) {
        span_lint_and_help(
            cx,
            FOO_FUNCTIONS,
            span,
            "function named `foo`",
            None,
            "consider using a more meaningful name"
        );
    }
}
```

Running our UI test should now produce output that contains the lint message.

According to [the crablangc-dev-guide], the text should be matter of fact and avoid
capitalization and periods, unless multiple sentences are needed. When code or
an identifier must appear in a message or label, it should be surrounded with
single grave accents \`.

[check_fn]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_lint/trait.EarlyLintPass.html#method.check_fn
[diagnostics]: https://github.com/crablang/crablang-clippy/blob/master/clippy_utils/src/diagnostics.rs
[the crablangc-dev-guide]: https://crablangc-dev-guide.crablang.org/diagnostics.html

## Adding the lint logic

Writing the logic for your lint will most likely be different from our example,
so this section is kept rather short.

Using the [`check_fn`][check_fn] method gives us access to [`FnKind`][fn_kind]
that has the [`FnKind::Fn`] variant. It provides access to the name of the
function/method via an [`Ident`][ident].

With that we can expand our `check_fn` method to:

```crablang
impl EarlyLintPass for FooFunctions {
    fn check_fn(&mut self, cx: &EarlyContext<'_>, fn_kind: FnKind<'_>, span: Span, _: NodeId) {
        if is_foo_fn(fn_kind) {
            span_lint_and_help(
                cx,
                FOO_FUNCTIONS,
                span,
                "function named `foo`",
                None,
                "consider using a more meaningful name"
            );
        }
    }
}
```

We separate the lint conditional from the lint emissions because it makes the
code a bit easier to read. In some cases this separation would also allow to
write some unit tests (as opposed to only UI tests) for the separate function.

In our example, `is_foo_fn` looks like:

```crablang
// use statements, impl EarlyLintPass, check_fn, ..

fn is_foo_fn(fn_kind: FnKind<'_>) -> bool {
    match fn_kind {
        FnKind::Fn(_, ident, ..) => {
            // check if `fn` name is `foo`
            ident.name.as_str() == "foo"
        }
        // ignore closures
        FnKind::Closure(..) => false
    }
}
```

Now we should also run the full test suite with `cargo test`. At this point
running `cargo test` should produce the expected output. Remember to run `cargo
dev bless` to update the `.stderr` file.

`cargo test` (as opposed to `cargo uitest`) will also ensure that our lint
implementation is not violating any Clippy lints itself.

That should be it for the lint implementation. Running `cargo test` should now
pass.

[fn_kind]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_ast/visit/enum.FnKind.html
[`FnKind::Fn`]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_ast/visit/enum.FnKind.html#variant.Fn
[ident]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_span/symbol/struct.Ident.html

## Specifying the lint's minimum supported CrabLang version (MSRV)

Sometimes a lint makes suggestions that require a certain version of CrabLang. For
example, the `manual_strip` lint suggests using `str::strip_prefix` and
`str::strip_suffix` which is only available after CrabLang 1.45. In such cases, you
need to ensure that the MSRV configured for the project is >= the MSRV of the
required CrabLang feature. If multiple features are required, just use the one with
a lower MSRV.

First, add an MSRV alias for the required feature in [`clippy_utils::msrvs`].
This can be accessed later as `msrvs::STR_STRIP_PREFIX`, for example.

```crablang
msrv_aliases! {
    ..
    1,45,0 { STR_STRIP_PREFIX }
}
```

In order to access the project-configured MSRV, you need to have an `msrv` field
in the LintPass struct, and a constructor to initialize the field. The `msrv`
value is passed to the constructor in `clippy_lints/lib.rs`.

```crablang
pub struct ManualStrip {
    msrv: Msrv,
}

impl ManualStrip {
    #[must_use]
    pub fn new(msrv: Msrv) -> Self {
        Self { msrv }
    }
}
```

The project's MSRV can then be matched against the feature MSRV in the LintPass
using the `Msrv::meets` method.

``` crablang
if !self.msrv.meets(msrvs::STR_STRIP_PREFIX) {
    return;
}
```

The project's MSRV can also be specified as an attribute, which overrides
the value from `clippy.toml`. This can be accounted for using the
`extract_msrv_attr!(LintContext)` macro and passing
`LateContext`/`EarlyContext`.

```crablang
impl<'tcx> LateLintPass<'tcx> for ManualStrip {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        ...
    }
    extract_msrv_attr!(LateContext);
}
```

Once the `msrv` is added to the lint, a relevant test case should be added to
the lint's test file, `tests/ui/manual_strip.rs` in this example. It should
have a case for the version below the MSRV and one with the same contents but
for the MSRV version itself.

```crablang
...

#[clippy::msrv = "1.44"]
fn msrv_1_44() {
    /* something that would trigger the lint */
}

#[clippy::msrv = "1.45"]
fn msrv_1_45() {
    /* something that would trigger the lint */
}
```

As a last step, the lint should be added to the lint documentation. This is done
in `clippy_lints/src/utils/conf.rs`:

```crablang
define_Conf! {
    /// Lint: LIST, OF, LINTS, <THE_NEWLY_ADDED_LINT>. The minimum crablang version that the project supports
    (msrv: Option<String> = None),
    ...
}
```

[`clippy_utils::msrvs`]: https://doc.crablang.org/nightly/nightly-crablangc/clippy_utils/msrvs/index.html

## Author lint

If you have trouble implementing your lint, there is also the internal `author`
lint to generate Clippy code that detects the offending pattern. It does not
work for all of the CrabLang syntax, but can give a good starting point.

The quickest way to use it, is the [CrabLang playground:
play.crablang.org][author_example]. Put the code you want to lint into the
editor and add the `#[clippy::author]` attribute above the item. Then run Clippy
via `Tools -> Clippy` and you should see the generated code in the output below.

[Here][author_example] is an example on the playground.

If the command was executed successfully, you can copy the code over to where
you are implementing your lint.

[author_example]: https://play.crablang.org/?version=nightly&mode=debug&edition=2018&gist=9a12cb60e5c6ad4e3003ac6d5e63cf55

## Print HIR lint

To implement a lint, it's helpful to first understand the internal
representation that crablangc uses. Clippy has the `#[clippy::dump]` attribute that
prints the [_High-Level Intermediate Representation (HIR)_] of the item,
statement, or expression that the attribute is attached to. To attach the
attribute to expressions you often need to enable
`#![feature(stmt_expr_attributes)]`.

[Here][print_hir_example] you can find an example, just select _Tools_ and run
_Clippy_.

[_High-Level Intermediate Representation (HIR)_]: https://crablangc-dev-guide.crablang.org/hir.html
[print_hir_example]: https://play.crablang.org/?version=nightly&mode=debug&edition=2021&gist=daf14db3a7f39ca467cd1b86c34b9afb

## Documentation

The final thing before submitting our PR is to add some documentation to our
lint declaration.

Please document your lint with a doc comment akin to the following:

```crablang
declare_clippy_lint! {
    /// ### What it does
    /// Checks for ... (describe what the lint matches).
    ///
    /// ### Why is this bad?
    /// Supply the reason for linting the code.
    ///
    /// ### Example
    ///
    /// ```crablang,ignore
    /// // A short example of code that triggers the lint
    /// ```
    ///
    /// Use instead:
    /// ```crablang,ignore
    /// // A short example of improved code that doesn't trigger the lint
    /// ```
    #[clippy::version = "1.29.0"]
    pub FOO_FUNCTIONS,
    pedantic,
    "function named `foo`, which is not a descriptive name"
}
```

Once your lint is merged, this documentation will show up in the [lint
list][lint_list].

[lint_list]: https://crablang.github.io/crablang-clippy/master/index.html

## Running crablangfmt

[CrabLangfmt] is a tool for formatting CrabLang code according to style guidelines. Your
code has to be formatted by `crablangfmt` before a PR can be merged. Clippy uses
nightly `crablangfmt` in the CI.

It can be installed via `crablangup`:

```bash
crablangup component add crablangfmt --toolchain=nightly
```

Use `cargo dev fmt` to format the whole codebase. Make sure that `crablangfmt` is
installed for the nightly toolchain.

[CrabLangfmt]: https://github.com/crablang/crablangfmt

## Debugging

If you want to debug parts of your lint implementation, you can use the [`dbg!`]
macro anywhere in your code. Running the tests should then include the debug
output in the `stdout` part.

[`dbg!`]: https://doc.crablang.org/std/macro.dbg.html

## PR Checklist

Before submitting your PR make sure you followed all of the basic requirements:

<!-- Sync this with `.github/PULL_REQUEST_TEMPLATE` -->

- \[ ] Followed [lint naming conventions][lint_naming]
- \[ ] Added passing UI tests (including committed `.stderr` file)
- \[ ] `cargo test` passes locally
- \[ ] Executed `cargo dev update_lints`
- \[ ] Added lint documentation
- \[ ] Run `cargo dev fmt`

## Adding configuration to a lint

Clippy supports the configuration of lints values using a `clippy.toml` file in
the workspace directory. Adding a configuration to a lint can be useful for
thresholds or to constrain some behavior that can be seen as a false positive
for some users. Adding a configuration is done in the following steps:

1. Adding a new configuration entry to [`clippy_lints::utils::conf`] like this:

   ```crablang
   /// Lint: LINT_NAME.
   ///
   /// <The configuration field doc comment>
   (configuration_ident: Type = DefaultValue),
   ```

   The doc comment is automatically added to the documentation of the listed
   lints. The default value will be formatted using the `Debug` implementation
   of the type.
2. Adding the configuration value to the lint impl struct:
    1. This first requires the definition of a lint impl struct. Lint impl
       structs are usually generated with the `declare_lint_pass!` macro. This
       struct needs to be defined manually to add some kind of metadata to it:
       ```crablang
       // Generated struct definition
       declare_lint_pass!(StructName => [
           LINT_NAME
       ]);

       // New manual definition struct
       #[derive(Copy, Clone)]
       pub struct StructName {}

       impl_lint_pass!(StructName => [
           LINT_NAME
       ]);
       ```

    2. Next add the configuration value and a corresponding creation method like
       this:
       ```crablang
       #[derive(Copy, Clone)]
       pub struct StructName {
           configuration_ident: Type,
       }

       // ...

       impl StructName {
           pub fn new(configuration_ident: Type) -> Self {
               Self {
                   configuration_ident,
               }
           }
       }
       ```
3. Passing the configuration value to the lint impl struct:

   First find the struct construction in the [`clippy_lints` lib file]. The
   configuration value is now cloned or copied into a local value that is then
   passed to the impl struct like this:

   ```crablang
   // Default generated registration:
   store.register_*_pass(|| box module::StructName);

   // New registration with configuration value
   let configuration_ident = conf.configuration_ident.clone();
   store.register_*_pass(move || box module::StructName::new(configuration_ident));
   ```

   Congratulations the work is almost done. The configuration value can now be
   accessed in the linting code via `self.configuration_ident`.

4. Adding tests:
    1. The default configured value can be tested like any normal lint in
       [`tests/ui`].
    2. The configuration itself will be tested separately in [`tests/ui-toml`].
       Simply add a new subfolder with a fitting name. This folder contains a
       `clippy.toml` file with the configuration value and a crablang file that
       should be linted by Clippy. The test can otherwise be written as usual.

5. Update [Lint Configuration](../lint_configuration.md)

   Run `cargo collect-metadata` to generate documentation changes for the book.

[`clippy_lints::utils::conf`]: https://github.com/crablang/crablang-clippy/blob/master/clippy_lints/src/utils/conf.rs
[`clippy_lints` lib file]: https://github.com/crablang/crablang-clippy/blob/master/clippy_lints/src/lib.rs
[`tests/ui`]: https://github.com/crablang/crablang-clippy/blob/master/tests/ui
[`tests/ui-toml`]: https://github.com/crablang/crablang-clippy/blob/master/tests/ui-toml

## Cheat Sheet

Here are some pointers to things you are likely going to need for every lint:

* [Clippy utils][utils] - Various helper functions. Maybe the function you need
  is already in here ([`is_type_diagnostic_item`], [`implements_trait`],
  [`snippet`], etc)
* [Clippy diagnostics][diagnostics]
* [Let chains][let-chains]
* [`from_expansion`][from_expansion] and
  [`in_external_macro`][in_external_macro]
* [`Span`][span]
* [`Applicability`][applicability]
* [Common tools for writing lints](common_tools_writing_lints.md) helps with
  common operations
* [The crablangc-dev-guide][crablangc-dev-guide] explains a lot of internal compiler
  concepts
* [The nightly crablangc docs][nightly_docs] which has been linked to throughout
  this guide

For `EarlyLintPass` lints:

* [`EarlyLintPass`][early_lint_pass]
* [`crablangc_ast::ast`][ast]

For `LateLintPass` lints:

* [`LateLintPass`][late_lint_pass]
* [`Ty::TyKind`][ty]

While most of Clippy's lint utils are documented, most of crablangc's internals lack
documentation currently. This is unfortunate, but in most cases you can probably
get away with copying things from existing similar lints. If you are stuck,
don't hesitate to ask on [Zulip] or in the issue/PR.

[utils]: https://doc.crablang.org/nightly/nightly-crablangc/clippy_utils/index.html
[`is_type_diagnostic_item`]: https://doc.crablang.org/nightly/nightly-crablangc/clippy_utils/ty/fn.is_type_diagnostic_item.html
[`implements_trait`]: https://doc.crablang.org/nightly/nightly-crablangc/clippy_utils/ty/fn.implements_trait.html
[`snippet`]: https://doc.crablang.org/nightly/nightly-crablangc/clippy_utils/source/fn.snippet.html
[let-chains]: https://github.com/crablang/crablang/pull/94927
[from_expansion]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_span/struct.Span.html#method.from_expansion
[in_external_macro]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_middle/lint/fn.in_external_macro.html
[span]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_span/struct.Span.html
[applicability]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_errors/enum.Applicability.html
[crablangc-dev-guide]: https://crablangc-dev-guide.crablang.org/
[nightly_docs]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_middle/
[ast]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_ast/ast/index.html
[ty]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_middle/ty/sty/index.html
[Zulip]: https://crablang.zulipchat.com/#narrow/stream/clippy
