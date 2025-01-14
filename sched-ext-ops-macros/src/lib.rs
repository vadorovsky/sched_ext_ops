use proc_macro::TokenStream;
use syn::parse_macro_input;

mod expand;
use expand::SchedExtOpsArgs;

/// Creates a new `SchedExtOps` instance.
///
/// The main purpose of this macro is to ensure the correctness of the name
/// (as a char array) during compile time.
#[proc_macro]
pub fn sched_ext_ops(args: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as SchedExtOpsArgs);
    expand::sched_ext_ops(args)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
