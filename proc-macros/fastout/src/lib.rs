use proc_macro::TokenStream;
use watt::WasmMacro;

static MACRO: WasmMacro = WasmMacro::new(include_bytes!(concat!(
    env!("OUT_DIR"),
    "/fastout_impl.wasm",
)));

#[proc_macro_attribute]
pub fn fastout(args: TokenStream, item: TokenStream) -> TokenStream {
    MACRO.proc_macro_attribute("fastout", args, item)
}
