use proc_macro::TokenStream;

mod helpers;
mod arg_wrapper;
mod ansi;
mod palette;

// TBI: Figure out if there is a better way to expose or re-expose macros on library level.

#[proc_macro]
pub fn arg_wrapper_impl(input: TokenStream) -> TokenStream {
	arg_wrapper::arg_wrapper_impl(input)
}

#[proc_macro]
pub fn ansi_impl(input: TokenStream) -> TokenStream {
	ansi::ansi_impl(input)
}
