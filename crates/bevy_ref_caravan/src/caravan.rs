mod root_step;
mod entity_step;
mod query_step;
mod bindings_step;
mod next_step;

use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;
use crate::syntax_out::exit_rule_default;

pub(crate) struct Caravan {
    iter: TokenIter, // Iteration across the input token stream.
    package: TokenStream, // Token stream output, iteratively built upon at each step.
    depth: u32, // Recursion/Scope depth, initiated via delimiters/nesting.

    //step_read: TokenStream, // A token stream gathered from the input. Like a window into what is currently being read. It is used for error highlighting. 
}

impl Caravan {
    pub(crate) fn start(iter: TokenIter) -> Self {
        return Self {
            iter,
            package: TokenStream::new(),
            depth: 0,
        }
    }

    fn new(
        iter: TokenIter, 
        package: TokenStream, 
        depth: u32,
    ) -> Self {
        return Self {
            iter,
            package,
            depth,
        }
    }

    fn next(&mut self) -> Option<TokenTree> {
        return self.iter.next()
    }

    fn unpack(&mut self) -> TokenStream {
        return self.package.to_owned()
    }

    fn repack(&mut self, new_package: TokenStream) {
        self.package = new_package;
    }

    fn next_depth(&self) -> u32 {
        return self.depth + 1
    } 

    fn pack(&mut self, stream: TokenStream) {
        self.package.extend(stream)
    }
} 