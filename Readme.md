# EccAnsiLib

A simple ANSI library, with the intended use-case to make it easier for me to use ANSI in console output of my projects.

Provides following macros:

- `ansi_rgb!(r, g, b)`, generates an RGB ANSI sting literal
- `ansi_reset!()` generates a reset ANSI string literal
- `ansi!(concat!("Colored«y»Text with", " «r»colorful«123,6,255» ", " stuff«»"))` a procedural macro, replacing color codes in `«»` with a string literal with ANSI colors. It supports (emulates) the concat macro.\
  `«»` is a short for reset. `«0,123,255»` is an RGB ANSI code. `«code»` contains a letter sequence representing a default color. The codes are fit to my needs and might be adjusted in the future.
- `arg_wrapper!("format literal", "w", "123,123,123")` wraps every `{}` argument of the provided format literal with a highlight and a normal color. The colors are in the same format as `ansi!()`, but without the `«»`. First argument is the normal text color, second argument is to highlight `{}`.
- `arg_wrapper!("format literal", "w")` highlights `{}`, with the provided color code.

For the specific color codes, look them up in the `/ecc_ansi_lib_proc/src/lib.rs : parse_format()` function.
