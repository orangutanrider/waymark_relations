use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

pub(crate) struct EntityPreProcess {
    pub(crate) farm: TokenStream,
    pub(crate) suffix: Ident,
} 
impl EntityPreProcess {
    pub(crate) fn create_pre_processing_code(&self) -> TokenStream {
        
        todo!()
    }
}