#[macro_export]
macro_rules! ansi_rgb {
	( $r:tt, $g:tt, $b:tt) => {
		concat!("\u{1B}[38;2;", $r, ";", $g, ";", $b, "m")
	};
}

#[macro_export]
macro_rules! ansi_reset {
	( ) => {
		"\u{001B}[m"
	};
}

pub use ecc_ansi_lib_proc::ansi;
pub use ecc_ansi_lib_proc::arg_wrapper;
