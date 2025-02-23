# Contributing to Clippy

Hello fellow CrabLangacean! Great to see your interest in compiler internals and lints!

**First**: if you're unsure or afraid of _anything_, just ask or submit the issue or pull request anyway. You won't be
yelled at for giving it your best effort. The worst that can happen is that you'll be politely asked to change
something. We appreciate any sort of contributions, and don't want a wall of rules to get in the way of that.

Clippy welcomes contributions from everyone. There are many ways to contribute to Clippy and the following document
explains how you can contribute and how to get started.  If you have any questions about contributing or need help with
anything, feel free to ask questions on issues or visit the `#clippy` on [Zulip].

All contributors are expected to follow the [CrabLang Code of Conduct].

- [Contributing to Clippy](#contributing-to-clippy)
  - [The Clippy book](#the-clippy-book)
  - [High level approach](#high-level-approach)
  - [Finding something to fix/improve](#finding-something-to-fiximprove)
  - [Getting code-completion for crablangc internals to work](#getting-code-completion-for-crablangc-internals-to-work)
    - [IntelliJ CrabLang](#intellij-crablang)
    - [CrabLang Analyzer](#crablang-analyzer)
  - [How Clippy works](#how-clippy-works)
  - [Issue and PR triage](#issue-and-pr-triage)
  - [Bors and Homu](#bors-and-homu)
  - [Contributions](#contributions)
  - [License](#license)

[Zulip]: https://crablang.zulipchat.com/#narrow/stream/clippy
[CrabLang Code of Conduct]: https://www.crablang.org/policies/code-of-conduct

## The Clippy book

If you're new to Clippy and don't know where to start, the [Clippy book] includes
a [developer guide] and is a good place to start your journey.

[Clippy book]: https://doc.crablang.org/nightly/clippy/index.html
[developer guide]: https://doc.crablang.org/nightly/clippy/development/index.html

## High level approach

1. Find something to fix/improve
2. Change code (likely some file in `clippy_lints/src/`)
3. Follow the instructions in the [Basics docs](book/src/development/basics.md)
   to get set up
4. Run `cargo test` in the root directory and wiggle code until it passes
5. Open a PR (also can be done after 2. if you run into problems)

## Finding something to fix/improve

All issues on Clippy are mentored, if you want help simply ask someone from the
Clippy team directly by mentioning them in the issue or over on [Zulip]. All
currently active team members can be found
[here](https://github.com/crablang/crablang-clippy/blob/master/triagebot.toml#L18)

Some issues are easier than others. The [`good-first-issue`] label can be used to find the easy
issues. You can use `@crablangbot claim` to assign the issue to yourself.

There are also some abandoned PRs, marked with [`S-inactive-closed`].
Pretty often these PRs are nearly completed and just need some extra steps
(formatting, addressing review comments, ...) to be merged. If you want to
complete such a PR, please leave a comment in the PR and open a new one based
on it.

Issues marked [`T-AST`] involve simple matching of the syntax tree structure,
and are generally easier than [`T-middle`] issues, which involve types
and resolved paths.

[`T-AST`] issues will generally need you to match against a predefined syntax structure.
To figure out how this syntax structure is encoded in the AST, it is recommended to run
`crablangc -Z unpretty=ast-tree` on an example of the structure and compare with the [nodes in the AST docs].
Usually the lint will end up to be a nested series of matches and ifs, [like so][deep-nesting].
But we can make it nest-less by using [let chains], [like this][nest-less].

[`E-medium`] issues are generally pretty easy too, though it's recommended you work on an [`good-first-issue`]
first. Sometimes they are only somewhat involved code wise, but not difficult per-se.
Note that [`E-medium`] issues may require some knowledge of Clippy internals or some
debugging to find the actual problem behind the issue.

[`T-middle`] issues can be more involved and require verifying types. The [`ty`] module contains a
lot of methods that are useful, though one of the most useful would be `expr_ty` (gives the type of
an AST expression). `match_def_path()` in Clippy's `utils` module can also be useful.

[`good-first-issue`]: https://github.com/crablang/crablang-clippy/labels/good-first-issue
[`S-inactive-closed`]: https://github.com/crablang/crablang-clippy/pulls?q=is%3Aclosed+label%3AS-inactive-closed
[`T-AST`]: https://github.com/crablang/crablang-clippy/labels/T-AST
[`T-middle`]: https://github.com/crablang/crablang-clippy/labels/T-middle
[`E-medium`]: https://github.com/crablang/crablang-clippy/labels/E-medium
[`ty`]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_middle/ty
[nodes in the AST docs]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_ast/ast/
[deep-nesting]: https://github.com/crablang/crablang-clippy/blob/5e4f0922911536f80d9591180fa604229ac13939/clippy_lints/src/mem_forget.rs#L31-L45
[let chains]: https://github.com/crablang/crablang/pull/94927
[nest-less]: https://github.com/crablang/crablang-clippy/blob/5e4f0922911536f80d9591180fa604229ac13939/clippy_lints/src/bit_mask.rs#L133-L159

## Getting code-completion for crablangc internals to work

### IntelliJ CrabLang
Unfortunately, [`IntelliJ CrabLang`][IntelliJ_crablang_homepage] does not (yet?) understand how Clippy uses compiler-internals
using `extern crate` and it also needs to be able to read the source files of the crablangc-compiler which are not
available via a `crablangup` component at the time of writing.
To work around this, you need to have a copy of the [crablangc-repo][crablangc_repo] available which can be obtained via
`git clone https://github.com/crablang/crablang/`.
Then you can run a `cargo dev` command to automatically make Clippy use the crablangc-repo via path-dependencies
which `IntelliJ CrabLang` will be able to understand.
Run `cargo dev setup intellij --repo-path <repo-path>` where `<repo-path>` is a path to the crablangc repo
you just cloned.
The command will add path-dependencies pointing towards crablangc-crates inside the crablangc repo to
Clippy's `Cargo.toml`s and should allow `IntelliJ CrabLang` to understand most of the types that Clippy uses.
Just make sure to remove the dependencies again before finally making a pull request!

[crablangc_repo]: https://github.com/crablang/crablang/
[IntelliJ_crablang_homepage]: https://intellij-crablang.github.io/

### CrabLang Analyzer
For [`crablang-analyzer`][ra_homepage] to work correctly make sure that in the `crablang-analyzer` configuration you set

```json
{ "crablang-analyzer.crablangc.source": "discover" }
```

You should be able to see information on things like `Expr` or `EarlyContext` now if you hover them, also
a lot more type hints.

To have `crablang-analyzer` also work in the `clippy_dev` and `lintcheck` crates, add the following configuration

```json
{
    "crablang-analyzer.linkedProjects": [
        "./Cargo.toml",
        "clippy_dev/Cargo.toml",
        "lintcheck/Cargo.toml",
    ]
}
```

[ra_homepage]: https://crablang-analyzer.github.io/

## How Clippy works

[`clippy_lints/src/lib.rs`][lint_crate_entry] imports all the different lint modules and registers in the [`LintStore`].
For example, the [`else_if_without_else`][else_if_without_else] lint is registered like this:

```crablang
// ./clippy_lints/src/lib.rs

// ...
pub mod else_if_without_else;
// ...

pub fn register_plugins(store: &mut crablangc_lint::LintStore, sess: &Session, conf: &Conf) {
    // ...
    store.register_early_pass(|| box else_if_without_else::ElseIfWithoutElse);
    // ...

    store.register_group(true, "clippy::restriction", Some("clippy_restriction"), vec![
        // ...
        LintId::of(&else_if_without_else::ELSE_IF_WITHOUT_ELSE),
        // ...
    ]);
}
```

The [`crablangc_lint::LintStore`][`LintStore`] provides two methods to register lints:
[register_early_pass][reg_early_pass] and [register_late_pass][reg_late_pass]. Both take an object
that implements an [`EarlyLintPass`][early_lint_pass] or [`LateLintPass`][late_lint_pass] respectively. This is done in
every single lint. It's worth noting that the majority of `clippy_lints/src/lib.rs` is autogenerated by `cargo dev
update_lints`. When you are writing your own lint, you can use that script to save you some time.

```crablang
// ./clippy_lints/src/else_if_without_else.rs

use crablangc_lint::{EarlyLintPass, EarlyContext};

// ...

pub struct ElseIfWithoutElse;

// ...

impl EarlyLintPass for ElseIfWithoutElse {
    // ... the functions needed, to make the lint work
}
```

The difference between `EarlyLintPass` and `LateLintPass` is that the methods of the `EarlyLintPass` trait only provide
AST information. The methods of the `LateLintPass` trait are executed after type checking and contain type information
via the `LateContext` parameter.

That's why the `else_if_without_else` example uses the `register_early_pass` function. Because the
[actual lint logic][else_if_without_else] does not depend on any type information.

[lint_crate_entry]: https://github.com/crablang/crablang-clippy/blob/master/clippy_lints/src/lib.rs
[else_if_without_else]: https://github.com/crablang/crablang-clippy/blob/4253aa7137cb7378acc96133c787e49a345c2b3c/clippy_lints/src/else_if_without_else.rs
[`LintStore`]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_lint/struct.LintStore.html
[reg_early_pass]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_lint/struct.LintStore.html#method.register_early_pass
[reg_late_pass]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_lint/struct.LintStore.html#method.register_late_pass
[early_lint_pass]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_lint/trait.EarlyLintPass.html
[late_lint_pass]: https://doc.crablang.org/nightly/nightly-crablangc/crablangc_lint/trait.LateLintPass.html

## Issue and PR triage

Clippy is following the [CrabLang triage procedure][triage] for issues and pull
requests.

However, we are a smaller project with all contributors being volunteers
currently. Between writing new lints, fixing issues, reviewing pull requests and
responding to issues there may not always be enough time to stay on top of it
all.

Our highest priority is fixing [crashes][l-crash] and [bugs][l-bug], for example
an ICE in a popular crate that many other crates depend on. We don't
want Clippy to crash on your code and we want it to be as reliable as the
suggestions from CrabLang compiler errors.

We have prioritization labels and a sync-blocker label, which are described below.
- [P-low][p-low]: Requires attention (fix/response/evaluation) by a team member but isn't urgent.
- [P-medium][p-medium]: Should be addressed by a team member until the next sync.
- [P-high][p-high]: Should be immediately addressed and will require an out-of-cycle sync or a backport.
- [L-sync-blocker][l-sync-blocker]: An issue that "blocks" a sync.
Or rather: before the sync this should be addressed,
e.g. by removing a lint again, so it doesn't hit beta/stable.

## Bors and Homu

We use a bot powered by [Homu][homu] to help automate testing and landing of pull
requests in Clippy. The bot's username is @bors.

You can find the Clippy bors queue [here][homu_queue].

If you have @bors permissions, you can find an overview of the available
commands [here][homu_instructions].

[triage]: https://forge.crablang.org/release/triage-procedure.html
[l-crash]: https://github.com/crablang/crablang-clippy/labels/L-crash
[l-bug]: https://github.com/crablang/crablang-clippy/labels/L-bug
[p-low]: https://github.com/crablang/crablang-clippy/labels/P-low
[p-medium]: https://github.com/crablang/crablang-clippy/labels/P-medium
[p-high]: https://github.com/crablang/crablang-clippy/labels/P-high
[l-sync-blocker]: https://github.com/crablang/crablang-clippy/labels/L-sync-blocker
[homu]: https://github.com/crablang/homu
[homu_instructions]: https://bors.crablang.org/
[homu_queue]: https://bors.crablang.org/queue/clippy

## Contributions

Contributions to Clippy should be made in the form of GitHub pull requests. Each pull request will
be reviewed by a core contributor (someone with permission to land patches) and either landed in the
main tree or given feedback for changes that would be required.

All PRs should include a `changelog` entry with a short comment explaining the change. The rule of thumb is basically,
"what do you believe is important from an outsider's perspective?" Often, PRs are only related to a single property of a
lint, and then it's good to mention that one. Otherwise, it's better to include too much detail than too little.

Clippy's [changelog] is created from these comments. Every release, someone gets all commits from bors with a
`changelog: XYZ` entry and combines them into the changelog. This is a manual process.

Examples:
- New lint
  ```
  changelog: new lint: [`missing_trait_methods`]
  ```
- False positive fix
  ```
  changelog: Fix [`unused_peekable`] false positive when peeked in a closure or called as `f(&mut peekable)`
  ```
- Purely internal change
  ```
  changelog: none
  ```

Note this it is fine for a PR to include multiple `changelog` entries, e.g.:
```
changelog: Something 1
changelog: Something 2
changelog: Something 3
```

[changelog]: CHANGELOG.md

## License

All code in this repository is under the [Apache-2.0] or the [MIT] license.

<!-- adapted from https://github.com/servo/servo/blob/master/CONTRIBUTING.md -->

[Apache-2.0]: https://www.apache.org/licenses/LICENSE-2.0
[MIT]: https://opensource.org/licenses/MIT
