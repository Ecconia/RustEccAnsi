use proc_macro::TokenStream;
use std::collections::HashMap;
use std::num::ParseIntError;

use quote::quote;
use syn::{Expr, Lit, LitStr};
use syn::parse::Parser;
use syn::parse_macro_input;
use syn::punctuated::Punctuated;
use syn::Token;

//Internal helpers:

macro_rules! ansi_reset {
	( ) => {
		"\u{1B}[m"
	};
}

fn rgb(r: u8, g: u8, b: u8) -> String {
	format!("\u{1B}[38;2;{r};{g};{b}m")
}

// This macro eats two formats:
// - arg_wrapper("format string with {}", "highlight color code")
// - arg_wrapper("format string with {}", "normal color code", "highlight color code")
#[proc_macro]
pub fn arg_wrapper(input: TokenStream) -> TokenStream {
	let parser = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
	let argument_expr = parser.parse(input).unwrap();
	if argument_expr.len() < 2 || argument_expr.len() > 3 {
		panic!("Incorrect amount of arguments got {}, only accepting 2 or 3.", argument_expr.len());
	}
	let argument_count = argument_expr.len();
	let mut argument_literals = Vec::new();
	for arg in argument_expr {
		argument_literals.push(extract_string_literal(arg));
	}
	
	let output = if argument_count == 2 {
		wrap(
			argument_literals[0].to_string(),
			ansi_reset!().to_string(),
			parse_format(&argument_literals[1]),
		)
	} else {
		wrap(
			argument_literals[0].to_string(),
			parse_format(&argument_literals[1]),
			parse_format(&argument_literals[2]),
		)
	};
	
	(quote! {
		#output
	}).into()
}

fn extract_string_literal(mut expression: Expr) -> String {
	//Primitive group unwrap:
	while let Expr::Group(group) = expression {
		expression = *group.expr;
	}
	
	//Honestly, like I have no clue how they expect me to evaluate the concat! macro.
	// Well I gonna do it in a yolo way myself. As well macros are bundled, but I want evalutated arguments...
	if let Expr::Macro(mac_ro) = expression {
		let last = mac_ro.mac.path.segments.last().map(|a| a.ident.to_string());
		if last.is_none() {
			panic!("Weird no path macro...");
		}
		let last = last.unwrap();
		if last.eq("concat") {
			let mut buffer = String::new();
			let tokens = mac_ro.mac.tokens;
			let parser = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
			let argument_expr = parser.parse(tokens.try_into().unwrap()).unwrap();
			
			for argument in argument_expr {
				buffer.push_str(&extract_string_literal(argument)[..]);
			}
			return buffer;
		}
		if last.eq("ansi") {
			let tokens = mac_ro.mac.tokens;
			let parser = Punctuated::<Expr, Token![,]>::parse_separated_nonempty;
			let argument_expr = parser.parse(tokens.try_into().unwrap()).unwrap();
			
			if argument_expr.len() != 1 {
				panic!("ansi! macro only supports a single argument.");
			}
			let string = extract_string_literal(argument_expr.into_iter().next().unwrap());
			return inner_ansi(string);
		}
		panic!("Non concat/ansi macro, cannot handle.");
	}
	
	//Actual checking of underlaying expression:
	if let Expr::Lit(lit) = expression {
		let literal = lit.lit;
		if let Lit::Str(string_literal) = literal {
			string_literal.value()
		} else {
			panic!("Expected string literal, got: {:?}", literal);
		}
	} else {
		panic!("Expected literal, got: {:?}", expression);
	}
}

fn wrap(format: String, normal: String, highlight: String) -> String {
	if format.is_empty() {
		return format;
	}
	let mut output = String::with_capacity(format.len());
	if format.bytes().collect::<Vec<u8>>()[0] != b'{' {
		output.push_str(&normal[..]);
	}
	let mut was_close = false;
	//TODO: Add escaping support.
	for char in format.chars() {
		if char == '{' {
			if !was_close {
				output.push_str(&highlight[..]);
			}
			was_close = false;
		} else if char == '}' {
			was_close = true;
		} else if was_close {
			output.push_str(&normal[..]);
			was_close = false;
		}
		output.push(char);
	}
	output.push_str(ansi_reset!());
	
	output
}

#[proc_macro]
pub fn ansi(input: TokenStream) -> TokenStream {
	let input = parse_macro_input!(input as LitStr);
	let str_value = input.value();
	
	let output = inner_ansi(str_value);
	
	(quote! {
		#output
	}).into()
}

fn inner_ansi(input: String) -> String {
	let mut output = String::with_capacity(input.len());
	let mut buffer = String::new();
	let mut inside = false;
	//TODO: Add escaping support.
	for char in input.chars() {
		if inside {
			if char == '»' {
				output.push_str(&parse_format(&buffer)[..]);
				buffer.clear();
				inside = false;
			} else {
				buffer.push(char);
			}
		} else if char == '«' {
			inside = true;
		} else {
			output.push(char);
		}
	}
	//Always reset at the end, to not pollute the terminal.
	// output.push_str(ansi_reset!());
	
	output
}

/*
	empty = reset
	
	l_ = light-<other>
	d_ = dark-<other>
	
	b = blue
	r = red
	y = yellow
	g = green
	
	s = black / "schwarz" / my rules
	w = white
	
	o = orange
	v = purple / violet / vhatever
	p = pink
	
	r,g,b = ~RGB~ *magic*
 */
fn parse_format(format: &String) -> String {
	if format.is_empty() {
		return ansi_reset!().to_string();
	}
	
	let mut map = HashMap::new();
	map.insert("r".to_string(), rgb(255, 0, 0)); // Red
	map.insert("g".to_string(), rgb(0, 255, 0)); // Green
	map.insert("b".to_string(), rgb(0, 0, 255)); // Blue
	
	let dark = 180;
	map.insert("dr".to_string(), rgb(dark, 0, 0));
	map.insert("dg".to_string(), rgb(0, dark, 0));
	map.insert("db".to_string(), rgb(0, 0, dark));
	
	let bright = 80;
	map.insert("lr".to_string(), rgb(255, bright, bright));
	map.insert("lg".to_string(), rgb(bright, 255, bright));
	map.insert("lb".to_string(), rgb(bright, bright, 255));
	
	map.insert("y".to_string(), rgb(255, 255, 0)); // Yellow
	map.insert("p".to_string(), rgb(255, 0, 255)); // Pink
	map.insert("t".to_string(), rgb(0, 255, 255)); // Turquoise
	
	map.insert("dy".to_string(), rgb(dark, dark, 0));
	map.insert("dp".to_string(), rgb(dark, 0, dark));
	map.insert("dt".to_string(), rgb(0, dark, dark));
	
	let bright = 120;
	map.insert("ly".to_string(), rgb(255, 255, bright));
	map.insert("lp".to_string(), rgb(255, bright, 255));
	map.insert("lt".to_string(), rgb(bright, 255, 255));
	
	map.insert("lv".to_string(), rgb(180, 70, 255));
	map.insert("v".to_string(), rgb(150, 0, 255)); // Violet
	
	map.insert("s".to_string(), rgb(0, 0, 0)); // Black ("Schwarz")
	map.insert("ls".to_string(), rgb(40, 40, 40));
	map.insert("dgr".to_string(), rgb(100, 100, 100));
	map.insert("gr".to_string(), rgb(150, 150, 150)); // Gray
	map.insert("lgr".to_string(), rgb(200, 200, 200));
	map.insert("dw".to_string(), rgb(230, 230, 230));
	map.insert("w".to_string(), rgb(255, 255, 255)); // White
	
	if let Some(v) = map.get(format) {
		return v.clone();
	}
	
	//Attempt to parse RGB (as hex):
	if format.bytes().len() == 6 {
		let parsed = u32::from_str_radix(format, 16);
		if let Ok(value) = parsed {
			return rgb(
				(value >> 16) as u8,
				(value >> 8) as u8,
				value as u8,
			);
		}
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
	panic!("Could not parse ansi!() shortcut: '{}'", format);
}
