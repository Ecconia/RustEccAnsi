use proc_macro::{Delimiter, TokenTree};

/// Peeks the next chars, TRUE is returned when they match a provided string reference.
pub(crate) fn matches_string<T: Iterator<Item = char> + Clone>(iterator: &T, matches: &str) -> bool {
	let mut iterator_clone = iterator.clone();
	
	for c in matches.chars() {
		let next_c = iterator_clone.next();
		if next_c.is_none() {
			return false;
		}
		let next_c = next_c.unwrap();
		if next_c != c {
			return false;
		}
	}
	
	true
}

pub(crate) fn expect_string_literal<T: Iterator<Item = TokenTree>>(iterator: &mut T) -> Result<String, String> {
	let mut token = match iterator.next() {
		Some(token) => token,
		None => return Err("Expected string literal argument (token), but there are no more arguments (tokens)".to_string()),
	};
	
	// Unwrap Group { delimiter: None } wrappings. Rust macros sometimes group a single token for reasons...
	while let TokenTree::Group(group) = &token {
		match group.delimiter() {
			Delimiter::None => {},
			_ => return Err(format!("Expected string literal argument (token), but encountered group with delimiter {:?}.", group.delimiter())),
		}
		let sub_tokens = group.stream().into_iter().collect::<Vec<_>>();
		if sub_tokens.len() != 1 {
			return Err("Expected string literal argument (token), but there was a Group".to_string());
		}
		token = sub_tokens.into_iter().next().unwrap();
	}
	
	// Finally, expect the actual literal token.
	let literal = match token {
		TokenTree::Literal(literal) => literal,
		_ => return Err(format!("Second argument must be a string literal. Got: {token:?}")),
	};
	let string = literal.to_string();
	
	// Ensure it is a string literal.
	if !string.starts_with('"') || !string.ends_with('"') {
		panic!("Second argument must be a string literal, got >>{string}<<");
	}
	
	// Remove quotation & return.
	Ok(string[1..(string.len() - 1)].to_string())
}

pub(crate) fn collect_first_argument<T: Iterator<Item = TokenTree>>(iterator: &mut T) -> String {
	// Collect all tokens which are part of the first argument.
	// Once a comma is encountered, the first argument is completed.
	// Commas wrapped in any pair of brackets are not considered as they are part of TokenTree::Group sub-stream.
	let mut format_string = String::new();
	while let Some(token_tree) = iterator.next() {
		// Check if the current token is a comma - then return (as all argument-tokens had been gathered).
		if let TokenTree::Punct(punct) = &token_tree {
			if punct.as_char() == ',' {
				return format_string;
			}
		}
		// Every other token - add as string.
		// Important here is to not add a spacer. Things will break like the '::' symbol if separated.
		// TBI: Maybe some tokens cannot be concatenated like this. Valid Rust code however "should" be fine? Works for my test/use cases.
		format_string.push_str(&token_tree.to_string());
	}
	
	panic!("Expected first argument followed by a comma. No comma found. Collected argument so far is >>{format_string}<<");
}
