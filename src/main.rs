use ecc_ansi_lib::{ansi, ansi_extend, arg_wrapper};

fn main() {
	print_ansi_introduction();
	print_default_colors();
	print_arg_wrapper_introduction();
}

fn print_ansi_introduction() {
	// Basic usage:
	println!(ansi!("The idea is, that you can add «r»colors«» to your application output easily."));
	println!(ansi!("The '«lo»ansi!()«»' macro provides a bunch of default colors."));
	println!();
	
	// And macros:
	println!(concat!("Colors are triggered by shortcuts wrapped in «». Like '«r»Text«»' turns text ", ansi!("«r»Text«» red")));
	println!(ansi!(concat!("It is possible ", ansi!("to «c»use«» macros "), "inside of «lo»ansi!()«». (Just like with any other macro).")));
	println!();
	
	// Palette extension:
	macro_rules! ansi_extended {
		($format:expr) => {
			ecc_ansi_lib::ansi_extend!($format, custom 100 200 255)
		};
	}
	println!(ansi_extended!("Ecc Ansi Lib version 2 adds the ability to extend the color palette with new «custom»colors«»."));
	println!(ansi_extended!("Also it is now possible to «lo»««»«» escape the color symbol by using two '«lo»«««««»' openers."));
	// Palette replacement:
	macro_rules! ansi_replaced {
		($format:expr) => {
			ecc_ansi_lib::ansi_replace!($format, custom 100 200 255)
		};
	}
	println!(ansi_replaced!("You can also start a new palette, which won't have support for the original colors ««r» <- would panic!() then, but custom «custom»colors«» work."));
	println!();
	
	// Examples of color usage:
	println!(ansi!("Color by RGB «80,255,80»««80,255,80»«»"));
	println!(ansi!("Color by HEX «4FC5F8»««4FC5F8»«»"));
	println!(ansi!("Color by color palette code «w»««w»«»"));
	println!();
	
	// Examples of expanding the color palette:
	println!(ansi_extend!("Custom color «rgb»using RGB«»; «hex»using HEX«»; «var»using variables«».",
		rgb 123 234 80
		hex ff8888
		var_a = 40
		var_b = 90
		var var_a var_b 180
	));
	println!();
}

fn print_default_colors() {
	println!("Default color palette:");
	println!(ansi!(concat!("Bright:",
		" «lr»Red",
		" «lo»Orange",
		" «ly»Yellow",
		" «la»Acid",
		" «lg»Green",
		" «lc»Cyan",
		" «lb»Blue",
		" «lv»Violet",
		" «lp»Pink",
		" «lm»Magenta",
		" «lr»Red",
		" «»",
	)));
	println!(ansi!(concat!("Colors:",
		" «r»Red",
		" «o»Orange",
		" «y»Yellow",
		" «a»Acid",
		" «g»Green",
		" «c»Cyan",
		" «b»Blue",
		" «v»Violet",
		" «p»Pink",
		" «m»Magenta",
		" «r»Red",
		" «»",
	)));
	println!(ansi!(concat!("  Dark:",
		" «dr»Red",
		" «do»Orange",
		" «dy»Yellow",
		" «da»Acid",
		" «dg»Green",
		" «dc»Cyan",
		" «db»Blue",
		" «dv»Violet",
		" «dp»Pink",
		" «dm»Magenta",
		" «dr»Red",
		" «»",
	)));
	
	println!(ansi!(concat!("Grayscale:",
		" «ds»BLACK",
		" «s»BLACK",
		" «ls»BLACK",
		" «dgr»GRAY",
		" «gr»GRAY",
		" «lgr»GRAY",
		" «dw»WHITE",
		" «w»WHITE",
		" «lw»WHITE",
		"«»"
	)));
	println!();
}

fn print_arg_wrapper_introduction() {
	// Basic example of how one can spice up prints.
	macro_rules! colored_println {
		($first:expr $( ,$rest:tt )* ) => {
			// Uses colors cyan and "white". Basically a println!() wrapper.
			println!(arg_wrapper!($first, "c", "w") $( ,$rest )* );
		};
	}
	colored_println!("Introduction for {} macro.", "arg_wrapper!()");
	
	// Basic usage:
	println!(arg_wrapper!(
		"You can highlight arguments in text '{}'. For that you supply a highlight color {} to the macro",
		"y" // Highlight color 'yellow'
	), "{}", "y");
	println!(arg_wrapper!(
		"Besides a {} color you can also provide a color for the {} text {}",
		"y", // Highlight color 'yellow'
		"lo" // Normal text color 'light orange'
	), "highlight", "normal", "«n»");
	println!(arg_wrapper!(
		"Using the {} macro, any argument '{}' will be wrapped in {}.", "lr", "lg"
	), "arg_wrapper!()", "{}", "«y»{}«lo»");
	colored_println!("{0}The macro also ensures, that the color gets reset at start and end of the string literal - this ensures correct colors in any case.{0}", "«»");
	colored_println!("Small note, when you have {}{}, the macro will prevent redundant color codes {}. As they would overwrite each other.", "multiple ", "arguments", "«text»«highlight»");
	println!();
	
	// concat!() disclaimer.
	println!(arg_wrapper!(
		concat!(
			"Btw, it is ",
			"not recommended",
			"to use {}",
			" with {}",
			" as that will cause",
			" redundant color/ANSI codes."
		), "p" // Highlight color pink
	), "concat!()", "arg_wrapper!()");
}
