pub use ecc_ansi_lib_proc::ansi_impl;
pub use ecc_ansi_lib_proc::arg_wrapper_impl;

/// Generates an RGB ANSI foreground color code.
#[macro_export]
macro_rules! ansi_rgb {
	($r:expr, $g:expr, $b:expr) => {
		concat!("\u{1B}[38;2;", $r, ";", $g, ";", $b, "m")
	};
}

/// Simply generates an ANSI reset code. Can be used in combination with concat!().
#[macro_export]
macro_rules! ansi_reset {
	() => {
		"\u{001B}[m"
	};
}

/// This macro allows you to format «color» codes to ANSI colors using the default palette provided by this mod (see below).
///
/// Create a different version of this macro if you like to use a different color palette.
#[macro_export]
macro_rules! ansi {
	($format:expr) => {
		// Essentially forward to 'ansi_escape' to prevent having to redefine the default color palette.
		// But as extending with no additional palette colors works - this is fine.
		ecc_ansi_lib::ansi_extend!($format,)
	};
}

/// This macro is meant to be used to expand the default color palette with custom colors.
/// It contains/defines the default palette.
#[macro_export]
macro_rules! ansi_extend {
	($format:expr, $( $palette:tt )*) => {
		ecc_ansi_lib::ansi_impl!(
			$format,
			// Default color table:
			// There probably are more scientific & correct methods to assign colors.
			// This color palette was however created by the Ecconia's eye calibration...
			
			// Anyway, to make base colors darker prefix with 'd', to make them brighter/lighter prefix with 'l'.
			
			lr 255 100 100
			r  255   0   0 // Red
			dr 150   0   0
			
			lo 255 150 50
			o  255 100  0 // Orange
			do 150  40  0
			
			ly 255 255 120
			y  255 255   0 // Yellow
			dy 150 150   0
			
			la 160 255 80
			a  130 255  0 // Acid
			da  70 150  0
			
			lg 80 255 80
			g   0 255  0 // Green
			dg  0 150  0
			
			// I do not really have an idea what the color here is called.
			// Or what it would be used for. My eyes are not trained for it.
			
			lc 120 255 255
			c    0 255 255 // Cyan
			dc   0 180 180
			
			// Also here no clue for this color.
			
			lb 50 120 255
			b   0   0 255 // Blue
			db  0   0 150
			
			lv 180 70 255
			v  150  0 255 // Violet
			dv 100  0 200
			
			lp 255 120 255
			p  255   0 255 // Pink
			dp 150   0 150
			
			lm 255 70 180
			m  255  0 150 // Magenta
			dm 200  0 100
			
			// Grayscale:
			ds    0   0   0
			s    20  20  20 // Black (DE: "Schwarz")
			ls   30  30  30
			dgr  60  60  60
			gr  100 100 100 // Gray
			lgr 150 150 150
			dw  180 180 180
			w   220 220 220 // White
			lw  255 255 255
			
			// Append the extra values (or nothing):
			$( $palette )*
		)
	};
}

/// This macro allows to provide your own palette and replace the original default (if you do not want to depend on ecc_ansi_lib_proc crate directly).
#[macro_export]
macro_rules! ansi_replace {
	($format:expr, $( $palette:tt )*) => {
		ecc_ansi_lib::ansi_impl!($format, $( $palette )*)
	};
}

/// This macro allows to wrap arguments with color codes of the default color palette.
/// You can provide 2 or 3 arguments:
/// - arg_wrapper!("string literal or expression with {} arguments", "argument highlight color")
/// - arg_wrapper!("string literal or expression with {} arguments", "argument highlight color", "normal text color")
///
/// Create your own version of this macro to use a different version of ansi!() with a different palette.
#[macro_export]
macro_rules! arg_wrapper {
	($format:expr, $highlight:literal) => {
		// It is important, that the ansi!() macro is an argument of arg_wrapper_impl!().
		// As arg_wrapper_impl!() must be evaluated first, for ansi!() to colorize it's output.
		// If ansi!() was first, then it would not find any color codes and colorize nothing.
		ecc_ansi_lib::arg_wrapper_impl!(ecc_ansi_lib::ansi!($format), $highlight)
	};
	($format:expr, $highlight:literal, $normal:literal) => {
		// Same as above ansi!() as argument.
		ecc_ansi_lib::arg_wrapper_impl!(ecc_ansi_lib::ansi!($format), $highlight, $normal)
	};
}
