use std::collections::HashMap;
use std::iter::Peekable;
use std::num::ParseIntError;
use std::str::FromStr;

macro_rules! ansi_reset {
	() => {
		"\u{1B}[m"
	};
}

fn rgb(r: u8, g: u8, b: u8) -> String {
	format!("\u{1B}[38;2;{r};{g};{b}m")
}

pub(crate) struct Palette {
	palette: HashMap<String, String>,
}

impl Palette {
	// TBI: Consider lazily evaluating the palette only when no other color input is available. Can save compilation time.
	// TODO: Measure how long parsing the palette actually takes.
	pub(crate) fn from_string_tokens(palette_tokens: Vec<String>) -> Palette {
		let mut variables = HashMap::new();
		let mut palette = HashMap::new();
		
		let mut iterator = palette_tokens.into_iter().peekable();
		while let Some(key) = iterator.next() {
			if !key.chars().all(|char| char == '_' || char.is_ascii_alphabetic()) {
				panic!("Variables/Color-Keys must only consist of ascii letters or underscore. Got '{key}'");
			}
			
			let next = iterator.peek().unwrap_or_else(|| panic!("Got opening color-key/variable '{key}', but no values/assignment token."));
			if next == "=" {
				iterator.next().unwrap(); // Yep is assignment, drop '='.
				// New variable:
				let value = iterator.next().unwrap_or_else(|| panic!("Got opening variable assignment, but no value token to assign."));
				let value = u8::from_str(&value).unwrap_or_else(|e| panic!("Could not parse unsigned byte value of variable assignment (variable '{key}'; value '{value}'). Error: {e}"));
				variables.insert(key, value);
			} else {
				let color = Self::parse_color_value(&mut iterator, &variables, &key);
				palette.insert(key, color);
			}
		}
		
		Self {
			palette,
		}
	}
	
	fn parse_color_value<T: Iterator<Item=String>>(iterator: &mut Peekable<T>, variables: &HashMap<String, u8>, key: &str) -> String {
		let first_argument = iterator.peek().unwrap();
		if first_argument.len() == 6 && !variables.contains_key(first_argument) {
			// Argument has length of 6, thus it is not a byte.
			// Argument is not a variable.
			// Thus, it must be a hex color.
			match Self::parse_hex(first_argument) {
				Ok(value) => {
					iterator.next().unwrap(); // Drop hex value from iterator.
					return value
				},
				Err(e) => panic!("Could not parse hex input '{first_argument}'. Error: {e}"),
			}
		}
		
		// Read 3 numbers/variables:
		let r = Self::parse_color_channel(iterator, variables, "RED", key);
		let g = Self::parse_color_channel(iterator, variables, "GREEN", key);
		let b = Self::parse_color_channel(iterator, variables, "BLUE", key);
		rgb(r, g, b)
	}
	
	fn parse_color_channel<T: Iterator<Item=String>>(iterator: &mut Peekable<T>, variables: &HashMap<String, u8>, channel: &str, key: &str) -> u8 {
		match iterator.next() {
			None => panic!("Got color format, but no {channel} color channel. For color '{key}'"),
			Some(literal) => {
				if let Some(b) = variables.get(&literal) {
					*b
				} else {
					u8::from_str(&literal).unwrap_or_else(|e| panic!("Could not parse unsigned byte value of {channel} color channel (variable '{key}'). Error: {e}"))
				}
			}
		}
	}
	
	fn parse_hex(format: &str) -> Result<String, ParseIntError> {
		u32::from_str_radix(format, 16).map(|value| rgb(
			(value >> 16) as u8,
			(value >> 8) as u8,
			value as u8,
		))
	}
	
	/*
		Currently supported:
		- "" => Ansi reset
		- Lookup into the palette map
		- 6-Character hex color codes
		- "R, G, B" format for custom RGB values
	 */
	pub(crate) fn lookup(&self, mut format: &str) -> String {
		format = format.trim();
		
		// Empty => ANSI reset
		if format.is_empty() {
			return ansi_reset!().to_string();
		}
		
		// Lookup in palette:
		if let Some(v) = self.palette.get(format) {
			return v.clone();
		}
		
		//Attempt to parse RGB (as hex):
		if format.bytes().len() == 6 {
			if let Ok(value) = Self::parse_hex(format) {
				return value;
			}
			// Do not handle the error - it might be some other format.
		}
		
		//Attempt to parse RGB (as R,G,B):
		let parts: Vec<&str> = format.split(',').collect();
		if parts.len() == 3 {
			//Assume got RGB parts in vector.
			use std::str::FromStr;
			let numbers: Result<Vec<u8>, ParseIntError> = parts.iter().map(|a| a.trim()).map(u8::from_str).collect();
			if let Err(err) = numbers {
				panic!("Could not parse R,B,G as component is not byte: {}", err);
			}
			let numbers = numbers.unwrap();
			return rgb(
				numbers[0],
				numbers[1],
				numbers[2],
			);
		}
		
		//No match:
		panic!("Could not parse ANSI color format: '{}'", format);
	}
}
