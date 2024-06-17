use proc_macro::*;
use proc_macro::token_stream::IntoIter as TokenIter;

enum EntityWildcard {
    Direct,
    Literal,
    DeRefLiteral,
    Overlap,
    Lifted,
}