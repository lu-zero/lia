use syntax::parse::token::{Token as RsToken};

#[derive(Debug)]
pub enum LiaToken {
    RustToken(RsToken),
    Var,
    Function,
    Return,
    If,
    While,
    For
}

impl LiaToken {
    pub fn from_rust_token(t: RsToken) -> LiaToken {
        if let RsToken::Ident(ident) = t {
            let s = ident.name.as_str();

            // No better way to go from InternedString -> &str?
            match unsafe { s.slice_unchecked(0, s.len()) } {
                "function" => LiaToken::Function,
                "var" => LiaToken::Var,
                "return" => LiaToken::Return,
                "if" => LiaToken::If,
                "while" => LiaToken::While,
                "for" => LiaToken::For,
                _ => LiaToken::RustToken(t)
            }
        } else {
            LiaToken::RustToken(t)
        }
    }
}
