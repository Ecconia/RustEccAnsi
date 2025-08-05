use crate::palette::Palette;
use proc_macro::TokenStream;
use std::iter::Peekable;
use std::str::FromStr;

pub fn ansi_impl(input: TokenStream) -> TokenStream {
	let mut iterator = input.into_iter();
	
	// The first argument is the "string literal", or something that contains/yields string literals.
	let format_string = crate::helpers::collect_first_argument(&mut iterator);
	// Now collect all remaining tokens (the color palette) as string and parse them into a Palette.
	let palette = Palette::from_string_tokens(iterator.map(|token_tree| token_tree.to_string()).collect::<Vec<_>>());
	
	// Replace all color-symbols in the format string.
	let output = apply_ansi(&format_string, &palette);
	
	// Finally return the format as TokenStream again.
	// This conversion to string is performed, as copying/modifying the original token stream is kind of huge and error-prone effort.
	TokenStream::from_str(&output).unwrap()
}

fn apply_ansi(format: &str, palette: &Palette) -> String {
	let mut output = String::with_capacity(format.len());
	let mut iterator = format.chars().into_iter().peekable();
	
	while let Some(char) = iterator.next() {
		// Iterate over each character in the format:
		output.push(char); // Add each to the output (including opening '"' symbols).
		if char == '"' {
			// When a string literal opening was found, process it.
			process_string_literal(&mut iterator, &palette, &mut output)
		}
	}
	
	output
}

fn process_string_literal<T: Iterator<Item = char>>(iterator: &mut Peekable<T>, palette: &Palette, output: &mut String) {
	loop {
		let string_literal_char = iterator.next().unwrap_or_else(|| panic!("Unterminated string literal."));
		match string_literal_char {
			'\\' => {
				// Encountered an escaping symbol. Disregard whatever the next symbol would be. In a well-formed code, this should work just fine.
				output.push('\\');
				output.push(iterator.next().unwrap_or_else(|| panic!("Unterminated string literal.")));
			}
			'"' => {
				// Encountered string literal closer.
				output.push('"');
				break;
			}
			'«' => process_color_format(iterator, palette, output),
			// In all other cases just keep the symbol as-is.
			_ => output.push(string_literal_char),
		}
	}
}

fn process_color_format<T: Iterator<Item = char>>(iterator: &mut Peekable<T>, palette: &Palette, output: &mut String) {
	// Check if this is an escaped argument (starting with '««'), if so just ignore it and continue with the string literal.
	let next = *iterator.peek().unwrap_or_else(|| panic!("Unterminated string literal ('«»'). Output: '{output}'"));
	if next == '«' {
		iterator.next().unwrap(); // Consume the peeked symbol.
		output.push('«'); // At the full opener (as addition below is not executed).
		return;
	}
	
	let mut color_format_buffer = String::new();
	// Now collect all color format code characters (until it stops).
	loop {
		let string_literal_argument_char = iterator.next().unwrap_or_else(|| panic!("Unterminated color format code ('«»'). Output: '{output}'"));
		if string_literal_argument_char == '»' {
			break;
		}
		color_format_buffer.push(string_literal_argument_char);
	}
	// Resolve and append the ANSI color.
	output.push_str(&palette.lookup(&color_format_buffer));
}
