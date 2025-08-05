# EccAnsiLib

A simple ANSI library, with the intended use-case to make it easier for me to use ANSI in console output of my projects.

Provides following macros:

- `ansi_reset!()` generates a reset ANSI string literal.
- `ansi_rgb!(r, g, b)`, generates an RGB ANSI sting literal.
- `ansi!("Colored«y»Text with «r»colorful«123,6,255» elements«»")` a procedural macro, replacing color codes with `«»` in string literals with ANSI colors.
  `«»` is a short for reset. `«0,123,255»` is an RGB ANSI code. `«code»` contains a letter sequence representing a color from the default color palette.
- `arg_wrapper!("format literal", "w", "gr")` wraps every `{}` argument of the provided format literal with a highlight and a normal color.
  The colors are in the same format as `ansi!()`, but without the `«»`.
  First argument is the highlight color for `{}` arguments. Second argument is optional and the normal text color, when omitted the color is reset to your terminal default.

You can find a bunch of examples in `src/main.rs`, including a usage of all colors. Run the `main.rs` and look at how it works.

## Use in your project:

Add this dependency:
```toml
[dependencies]
ecc_ansi_lib = { git = "https://github.com/Ecconia/RustEccAnsi", tag = "v2.0.0" }
```
(Yep, this is mostly for myself to copy/paste).

## Changelog:

Version 1:
- Providing `ansi!()` and `arg_wrapper!()`.
- Emulating `concat!()` behavior in `ansi!()` (because I did not know any better). And not handling other macros as argument (besides itself).
- Hardcoded color palette.

Version 2:
- Macros:
  - Now only applying color codes inside of string literals.
  - Now first argument of both macros are properly accept all tokens forming a single expression. This allows to supply any sort of macro as first argument.
  - Now possible to escape color code `«` with `««` in `ansi!()`.
  - arg_wrapper!():
    - Now honoring escaped arguments `{{` in `arg_wrapper!()`.
    - Now second argument is the highlight color and the third optional is the normal color.
    - Now no longer converting color codes in `arg_wrapper_impl!()`. This is now an extra step taken care of by `arg_wrapper!()`. Doing this allows to change which ansi color palette is applied.
    - Now no longer generates redundant (consecutive) color codes (`«»`) within the same string literal.
- Color palette:
  - Reworked color values.
  - Renamed color `Turquoise` to `Cyan`.
  - Added colors `Magenta`, `Orange`.
  - Ensure every color has light/dark variants.
  - Added the concept of variables (`variable = 123`) allowing to change multiple color components at once.
- No longer depending on crates 'syn' and 'quote'. Now tokens are handled via Rust API.
- There now are tests for the proc macros.
- It is now possible to use a custom color palette or extend the existing one, by providing it to `ansi_impl!()` and wrapping that method manually.

## Future ideas:

- Control background color
- Control bold text
- Color palette:
  - 3 letter Hex colors?
  - Hex colors with # prefix? (Less confusion)
  - HSV and other color schemes? Makes it much easier to set up color palettes.
