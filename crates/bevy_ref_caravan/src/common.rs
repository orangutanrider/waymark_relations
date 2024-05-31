use std::str::*;
use proc_macro::TokenStream;

pub(crate) fn compile_error_stream(msg: &str) -> TokenStream {
    let Ok(stream) = TokenStream::from_str(&("compile_error!(".to_owned() + msg + ")")) else {
        panic!()
    };

    return stream;
}