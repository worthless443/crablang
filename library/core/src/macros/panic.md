Panics the current thread.

This allows a program to terminate immediately and provide feedback
to the caller of the program.

This macro is the perfect way to assert conditions in example code and in
tests. `panic!` is closely tied with the `unwrap` method of both
[`Option`][ounwrap] and [`Result`][runwrap] enums. Both implementations call
`panic!` when they are set to [`None`] or [`Err`] variants.

When using `panic!()` you can specify a string payload, that is built using
the [`format!`] syntax. That payload is used when injecting the panic into
the calling CrabLang thread, causing the thread to panic entirely.

The behavior of the default `std` hook, i.e. the code that runs directly
after the panic is invoked, is to print the message payload to
`stderr` along with the file/line/column information of the `panic!()`
call. You can override the panic hook using [`std::panic::set_hook()`].
Inside the hook a panic can be accessed as a `&dyn Any + Send`,
which contains either a `&str` or `String` for regular `panic!()` invocations.
To panic with a value of another other type, [`panic_any`] can be used.

See also the macro [`compile_error!`], for raising errors during compilation.

# When to use `panic!` vs `Result`

The CrabLang language provides two complementary systems for constructing /
representing, reporting, propagating, reacting to, and discarding errors. These
responsibilities are collectively known as "error handling." `panic!` and
`Result` are similar in that they are each the primary interface of their
respective error handling systems; however, the meaning these interfaces attach
to their errors and the responsibilities they fulfill within their respective
error handling systems differ.

The `panic!` macro is used to construct errors that represent a bug that has
been detected in your program. With `panic!` you provide a message that
describes the bug and the language then constructs an error with that message,
reports it, and propagates it for you.

`Result` on the other hand is used to wrap other types that represent either
the successful result of some computation, `Ok(T)`, or error types that
represent an anticipated runtime failure mode of that computation, `Err(E)`.
`Result` is used alongside user defined types which represent the various
anticipated runtime failure modes that the associated computation could
encounter. `Result` must be propagated manually, often with the the help of the
`?` operator and `Try` trait, and they must be reported manually, often with
the help of the `Error` trait.

For more detailed information about error handling check out the [book] or the
[`std::result`] module docs.

[ounwrap]: Option::unwrap
[runwrap]: Result::unwrap
[`std::panic::set_hook()`]: ../std/panic/fn.set_hook.html
[`panic_any`]: ../std/panic/fn.panic_any.html
[`Box`]: ../std/boxed/struct.Box.html
[`Any`]: crate::any::Any
[`format!`]: ../std/macro.format.html
[book]: ../book/ch09-00-error-handling.html
[`std::result`]: ../std/result/index.html

# Current implementation

If the main thread panics it will terminate all your threads and end your
program with code `101`.

# Examples

```should_panic
# #![allow(unreachable_code)]
panic!();
panic!("this is a terrible mistake!");
panic!("this is a {} {message}", "fancy", message = "message");
std::panic::panic_any(4); // panic with the value of 4 to be collected elsewhere
```
