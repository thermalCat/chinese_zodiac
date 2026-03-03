An example that demonstrates enum pattern matching. Let me show you it's features:

"Rust’s match is exhaustive. That means:

Every enum variant must be handled.
If you add a new variant later, the compiler forces you to update every match.
You can’t accidentally fall through or forget a case.
You can’t accidentally handle an invalid integer, because invalid integers don’t exist for enums.

This is fundamentally different from C/C++:
A C/C++ enum is just an integer.
A switch can silently miss cases.
You can pass invalid values around.
Adding a new enum value doesn’t break anything… even when it should."

std::io gives you access to stdin() and related functions.
Rust does not have a preprocessor; everything is namespaced and explicit.

#[derive(...)] automatically generates useful traits:
Debug → allows {:?} printing
Clone/Copy → allows copying the enum like a small integer
PartialEq/Eq → allows comparisons (==)

Rust enums are tagged unions with strict type safety.
You cannot accidentally assign an invalid value.

Why rem_euclid?
% in Rust behaves like C: negative numbers produce negative remainders.
rem_euclid always returns a positive remainder, which is what we want for a cycle.

expect() unwraps the result or crashes with a message. The parameter is an exception message, which is a clean syntax - if slightly unfortunate naming convention.

integers use i32 instead of inferred integer size because input.trim().parse() needs a explicit target data type.
i32 is sensible when:

The value is conceptually a 32‑bit signed integer (e.g., a year, a temperature, a small counter).
"You want predictable, cross‑platform behaviour (Rust does not vary integer sizes by architecture)."

