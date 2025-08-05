#[cfg(test)]
mod ansi {
	use ecc_ansi_lib_proc::ansi_impl;
	
	macro_rules! ansi_test {
		(
			$name:ident
			in $input:expr,
			ex $expect:literal
			$( $palette:tt )*
		) => {
			#[test]
			fn $name() {
				println!("Input: '{}'", stringify!($input));
				let result = ansi_impl!($input, $( $palette )*);
				println!("Result: '{}'", result.replace("\u{1B}", "\\u{1B}"));
				println!("Expect: '{}'", $expect);
				println!(ansi_impl!($input, $( $palette )*));
				assert_eq!(result, $expect);
			}
		};
	}
	
	macro_rules! ignore_other {
		($drop1:tt, $arg:expr, $drop2:tt) => {
			$arg
		};
	}
	
	ansi_test!(generic_color_inputs
		in "Okay\"«255,123,0»RBB«»««»«d23467»HEX«»",
		ex "Okay\"\u{1B}[38;2;255;123;0mRBB\u{1B}[m«»\u{1B}[38;2;210;52;103mHEX\u{1B}[m"
	);
	
	// Simple "only consider inside string literal" test.
	// Not sure how to even provide that symbol properly outside of string literals without Rust choking on it.
	ansi_test!(hex_palette_entry_with_junk
		in ignore_other!('«', "«r»Hi!«»", '«'),
		ex "\u{1B}[38;2;255;0;0mHi!\u{1B}[m"
		r ff0000 // Hex
	);
	ansi_test!(rgb_palette_entry_with_concat
		in concat!('«', "«r»Hi!«»", '«'),
		ex "«\u{1B}[38;2;255;0;0mHi!\u{1B}[m«"
		r 255 0 0 // RGB
	);
	ansi_test!(variable_palette_entry
		in "«r»Hi!«»",
		ex "\u{1B}[38;2;255;0;0mHi!\u{1B}[m"
		max = 255 zero = 0 // Variable creation
		r max zero zero // Variable usage in RGB definitions
	);
	
	// Test escaping?
	// Only the opener is escaped '««' closer are just used as encountered '»'
	ansi_test!(ansi_escaping
		in "«««r»»»«»H««»i!««»«»",
		ex "«\u{1B}[38;2;255;0;0m»»\u{1B}[mH«»i!«»\u{1B}[m"
		r ff0000
	);
	
	// Not sure what else to test, this one is much more simplistic than arg_wrapper.
}

#[cfg(test)]
mod arg_wrapper {
	use ecc_ansi_lib_proc::arg_wrapper_impl;
	
	// TODO: Maybe also check if the formatted output is correct (with arguments)?
	macro_rules! arg_wrapper_test {
		(
			$name:ident
			in $input:expr, $highlight:literal, $normal:literal
			ex $expect:literal
			$( ,$arg:literal )*
		) => {
			#[test]
			fn $name() {
				println!("Input: '{}'", stringify!($input));
				println!("With color options: '{}' / '{}'", $highlight, $normal);
				let result = arg_wrapper_impl!($input, $highlight, $normal);
				println!("Result: '{}'", result);
				println!("Expect: '{}'", $expect);
				println!(arg_wrapper_impl!($input, $highlight, $normal) $( ,$arg )*);
				assert_eq!(result, $expect);
			}
		};
		(
			$name:ident
			in $input:expr, $highlight:literal
			ex $expect:literal
			$( ,$arg:literal )*
		) => {
			#[test]
			fn $name() {
				println!("Input: '{}'", stringify!($input));
				println!("With color options: '{}'", $highlight);
				let result = arg_wrapper_impl!($input, $highlight);
				println!("Result: '{}'", result);
				println!("Expect: '{}'", $expect);
				println!(arg_wrapper_impl!($input, $highlight) $( ,$arg )*);
				assert_eq!(result, $expect);
			}
		};
	}
	
	// How does it handle input without arguments?
	arg_wrapper_test!(no_arg_default
		in "Nothing to highlight. Without normal color. No final reset (cause pointless)! Watch out when adding custom «» colors in this cases.", "h"
		ex "«»Nothing to highlight. Without normal color. No final reset (cause pointless)! Watch out when adding custom «» colors in this cases."
	);
	arg_wrapper_test!(no_arg_normal
		in "Nothing to highlight. With normal color.", "h", "n"
		ex "«n»Nothing to highlight. With normal color.«»"
	);
	
	// How does it handle with a single argument?
	arg_wrapper_test!(one_arg_default
		in "Arg {}. No final reset here as well!", "highlight"
		ex "«»Arg «highlight»{}«». No final reset here as well!"
		, "arg1"
	);
	arg_wrapper_test!(one_arg_normal
		in "Arg {}.", "highlight", "normal"
		ex "«normal»Arg «highlight»{}«normal».«»"
		, "arg1"
	);
	
	// How does it handle edge-touching arguments?
	// The idea here is, that we ditch any «» which is not really required.
	//  Only when an actual color change is needed, we apply it.
	arg_wrapper_test!(three_args_bordering_separated_default
		in "{}_sep_{}_sep_{}", "h"
		ex "«h»{}«»_sep_«h»{}«»_sep_«h»{}«»"
		, "_arg1_", "_arg2_", "_arg3_"
	);
	arg_wrapper_test!(three_args_bordering_separated_normal
		in "{}_sep_{}_sep_{}", "h", "n"
		ex "«h»{}«n»_sep_«h»{}«n»_sep_«h»{}«»"
		, "_arg1_", "_arg2_", "_arg3_"
	);
	arg_wrapper_test!(three_args_bordering_default
		in "{}{}{}", "h"
		ex "«h»{}{}{}«»"
		, "_arg1_", "_arg2_", "_arg3_"
	);
	arg_wrapper_test!(three_args_bordering_normal
		in "{}{}{}", "h", "n"
		ex "«h»{}{}{}«»"
		, "_arg1_", "_arg2_", "_arg3_"
	);
	
	// This is highly inefficient and do not recommend doing this, but it is functional (which counts).
	// Downside, arg_wrapper operates on string literals. It cannot treat them as one - thus we get redundant «» codes.
	
	// This case here specifically rips apart '{}'. The only reason why this works - is that the code is not expecting another character.
	// One could argue, that this is wrong. But with well-formed string literals this is not an issue. I argue single { or } in string literals is not well-formed.
	// Do not rely on this being functional forever. In general - only concat if you really have to.
	arg_wrapper_test!(concat_malformed_normal
		in concat!("{}_", "{", "}", "_{}"), "h", "n"
		ex "«h»{}«n»_«»«h»{}«»«n»_«h»{}«»"
		, "arg1", "arg2", "arg3"
	);
	arg_wrapper_test!(concat_normal
		in concat!("{}", "{}", "{}"), "h", "n"
		ex "«h»{}«»«h»{}«»«h»{}«»"
		, "arg1", "arg2", "arg3"
	);
	
	// Argument escaping!
	arg_wrapper_test!(escaped_quotes_normal
		in "Arg \"{}\".", "highlight", "normal"
		ex "«normal»Arg \"«highlight»{}«normal»\".«»"
		, "quoted"
	);
	arg_wrapper_test!(escaped_quotes_default
		in "Arg \"{}\".", "highlight"
		ex "«»Arg \"«highlight»{}«»\"."
		, "quoted"
	);
	arg_wrapper_test!(escaped_arguments_extra_normal
		in "\"Arg {{{{{}}}}}.\"", "highlight", "normal"
		ex "«normal»\"Arg {{{{«highlight»{}«normal»}}}}.\"«»"
		, "quoted"
	);
	arg_wrapper_test!(escaped_arguments_default
		in "\"Arg {{{}}}.\"", "highlight"
		ex "«»\"Arg {{«highlight»{}«»}}.\""
		, "quoted"
	);
	arg_wrapper_test!(escaped_arguments_bordering_default
		in "{{{}}}\"{{{}}}", "highlight"
		ex "«»{{«highlight»{}«»}}\"{{«highlight»{}«»}}"
		, "arg1", "arg2"
	);
}
