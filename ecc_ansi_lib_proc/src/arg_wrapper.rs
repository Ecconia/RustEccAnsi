use proc_macro::{TokenStream, TokenTree};
use std::iter::Peekable;
use std::str::FromStr;
use crate::helpers::{collect_first_argument, expect_string_literal, matches_string};

// This macro eats two formats:
// - arg_wrapper_impl!(<first argument, should contain string literals>, <string literal for argument highlight color>)
// - arg_wrapper_impl!(<first argument, should contain string literals>, <string literal for argument highlight color>, <string literal for normal text color>)
// Each string literal passed as first argument to this maro will have its arguments wrapped with the color codes provided to this macro.
// - First color argument is the color used to highlight arguments.
// - Second color argument will be used for non-argument text. If omitted the 'reset' color is used (whatever the terminal users as default).
// Each string literal will have its text color reset at the end. And the normal color applied at its beginning.
// Macro will prevent redundant color-codes within a string literal ('«»«»').
// Result must be sent through ansi!() to actually apply the color codes. This is done externally to allow the usage of custom color palettes.
pub fn arg_wrapper_impl(input: TokenStream) -> TokenStream {
	let mut iterator = input.into_iter();
	
	// The first argument is the "string literal", or something that contains/yields string literals.
	let format = collect_first_argument(&mut iterator);
	// The second arguments is the highlight color as string literal.
	let highlight_color = match expect_string_literal(&mut iterator) {
		Ok(value) => value,
		Err(message) => panic!("Could not parse second (highlight color) argument: {message}"),
	};
	// The OPTIONAL third argument is the normal text color as string literal.
	let normal_color = if let Some(token) = iterator.next() {
		if let TokenTree::Punct(punctuation) = token {
			if punctuation.as_char() != ',' {
				panic!("If a third argument is supplied the separator after the second argument should be a comma.");
			}
		} else {
			panic!("Only allowing another string literal after the second argument, separated by a comma.");
		}
		
		match expect_string_literal(&mut iterator) {
			Ok(value) => value,
			Err(message) => panic!("Could not parse second (highlight color) argument: {message}"),
		}
	} else {
		"".to_string()
	};
	
	// Actually wrap all arguments with «» color codes.
	let output = wrap_arguments_with_color_codes(
		format,
		format!("«{}»", normal_color),
		format!("«{}»", highlight_color),
	);
	
	// Finally return the format as TokenStream again.
	// This conversion to string is performed, as copying/modifying the original token stream is kind of huge and error-prone effort.
	TokenStream::from_str(&output).unwrap()
}

fn wrap_arguments_with_color_codes(format: String, normal: String, highlight: String) -> String {
	if format.is_empty() {
		return format;
	}
	
	let mut output = String::with_capacity(format.len());
	let mut iterator = format.chars().peekable();
	
	while let Some(char) = iterator.next() {
		// Iterate over each character in the format:
		output.push(char); // Add each to the output (including opening '"' symbols).
		if char == '"' {
			// When a string literal opening was found, process it.
			process_string_literal(&mut iterator, &mut output, &normal, &highlight)
		}
	}
	
	output
}

fn process_string_literal<T: Iterator<Item = char> + Clone>(iterator: &mut Peekable<T>, output: &mut String, normal: &str, highlight: &str) {
	// If string literal does not start with an argument, add the normal text color color-code.
	let first_string_literal_char = *iterator.peek().unwrap_or_else(|| panic!("Unterminated string literal. Only encountered opener so far."));
	// If {, but not {{, then skip adding the normal color.
	if !(first_string_literal_char == '{' && !matches_string(iterator, "{{")) {
		output.push_str(&normal);
	}
	
	// Process every char, until the string literal closes.
	let mut previously_finished_argument = false;
	loop {
		let mut just_finished_argument = false;
		let string_literal_char = iterator.next().unwrap_or_else(|| panic!("Unterminated string literal."));
		match string_literal_char {
			'\\' => {
				// Encountered an escaping symbol. Disregard whatever the next symbol would be. In a well-formed code, this should work just fine.
				output.push('\\');
				output.push(iterator.next().unwrap_or_else(|| panic!("Unterminated string literal.")));
			}
			'"' => {
				// Encountered string literal closer.
				if previously_finished_argument || normal != "«»" {
					// Always append a color-reset. (If string literals are merged, this can be redundant).
					// Except: There is no need to reset though, when the normal color is reset anyway.
					//         But if there just was an argument, we got to reset again. As the argument
					//          termination won't reset when it detects literal termination.
					output.push_str("«»");
				}
				output.push('"');
				break;
			}
			// If an argument-starter is encountered, handle that.
			'{' => {
				// Check if this is an escaped argument (starting with '{{'), if so just ignore it and continue with the string literal.
				let peeked = *iterator.peek().unwrap_or_else(|| panic!("Unterminated string literal argument ('{{}}'). Output: '{output}'"));
				if peeked == '{' {
					// Is escaped!
					iterator.next().unwrap(); // Consume the peeked symbol.
					output.push_str("{{"); // At the full opener.
				} else {
					// Not escaped - actual argument.
					process_string_literal_argument(iterator, output, normal, highlight, previously_finished_argument);
					just_finished_argument = true;
				}
			},
			// In all other cases just keep the symbol as-is.
			_ => output.push(string_literal_char),
		}
		previously_finished_argument = just_finished_argument;
	}
}

fn process_string_literal_argument<T: Iterator<Item = char> + Clone>(iterator: &mut Peekable<T>, output: &mut String, normal: &str, highlight: &str, just_finished_an_argument: bool) {
	// 100% inside an argument now. Prefix it with the highlight color.
	if !just_finished_an_argument {
		// Do not put a highlight color, if we still are using the highlight color (cause an argument finished right before this one).
		output.push_str(&highlight);
	}
	output.push('{');
	
	// Now loop over all characters in the argument (until it stops).
	loop {
		let string_literal_argument_char = iterator.next().unwrap_or_else(|| panic!("Unterminated string literal argument ('{{}}'). Output: '{output}'"));
		output.push(string_literal_argument_char); // Add any char to the output, nothing will be color-prefixed here.
		// Encountered a (potential) closing char, handle it.
		if string_literal_argument_char == '}' {
			// The next char is important to know if this is escaped.
			let next_char = *iterator.peek().unwrap_or_else(|| panic!("Unterminated string literal after argument."));
			if !(next_char == '"' || (next_char == '{' && !matches_string(iterator, "{{"))) {
				// Always reset the color after an argument.
				// Except: The next character is '"' (then «» is added by the literal string termination).
				// Except: There is another argument following. Meaning '{' but not '{{' follows.
				output.push_str(&normal);
			}
			break;
		}
	}
}
