#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    M,
    SP,
    PSW,
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Operation(String),
    U8(u8),
    U16(u16),
    Label(String),
    Comma,
    Colon,
    Register(Register),
    End
}

impl PartialEq for TokenType {
    fn eq(&self, other: &TokenType) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Eq for TokenType {}

#[derive(Debug, Clone)]
pub struct Token {
    pub position: (usize, usize),
    pub token: TokenType,
}

pub struct TokenStream {
    pub tokens: Vec<Token>,
}

impl TokenStream {
    pub fn iter(&mut self) -> std::slice::Iter<Token> {
        self.tokens.iter()
    }
}

use std::fmt::Display;

impl Display for TokenStream {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\nTokens: {\n")?;
        for token in &self.tokens {
            f.write_fmt(format_args!(
                "\t{:4}:{}\t: {:?};\n",
                token.position.0, token.position.1, token.token
            ))?;
        }
        f.write_str("}\n")?;
        Ok(())
    }
}
