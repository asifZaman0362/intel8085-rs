use crate::token::{ TokenType, Token, TokenStream, Register };
use crate::error::{ ParseError, ErrorKind };

static KEYWORDS: &[&str] = &[
    "ADD", "ACI", "ADC", "ADI", "ANA", "ANI", "CALL", "CC", "CM", "CMA", "CMC", "CMP", "CNC",
    "CNZ", "CP", "CPE", "CPI", "CPO", "CZ", "DAA", "DAD", "DCR", "DCX", "DI", "EI", "HLT", "IN",
    "INR", "INX", "JC", "JNC", "JM", "JMP", "JNZ", "JP", "JPE", "JPO", "JZ", "LDA", "LDAX", "LHLD",
    "LXI", "MOV", "MVI", "NOP", "ORA", "ORI", "OUT", "PCHL", "POP", "POP", "PUSH", "RAL", "RAR",
    "RC", "RET", "RIM", "RLC", "RM", "RNC", "RNZ", "RP", "RPE", "RPO", "RRC", "RST", "RZ", "SBB",
    "SBI", "SHLD", "SIM", "SPHL", "STA", "STAX", "STC", "SUB", "SUI", "XCHG", "XRA", "XRI", "XTHL",
];

fn is_valid_identifier(lexeme: &str) -> bool {
    for (i, char) in lexeme.as_bytes().iter().enumerate() {
        if i == 0 {
            if !(char.is_ascii_alphabetic() || *char == b'_') {
                return false;
            }
        } else if !(char.is_ascii_alphanumeric() || *char == b'_') {
            return false;
        }
    }
    true
}

fn is_keyword(token: &str) -> bool {
    KEYWORDS.contains(&token)
}

fn is_numeric(lexeme: &str) -> bool {
    for char in lexeme.as_bytes() {
        if *char < b'0' || *char > b'9' {
            return false;
        }
    }
    true
}

fn make_token(line_number: usize, col_num: usize, lexeme: &str) -> Result<Token, ParseError> {
    if lexeme.to_lowercase().ends_with('h') || lexeme.to_lowercase().ends_with('k') {
        if let Ok(number) = u16::from_str_radix(&lexeme[..lexeme.len() - 1], 16) {
            if number < 256 {
                return Ok(Token {
                    position: (line_number, col_num),
                    token: TokenType::U8(number as u8),
                });
            } else {
                return Ok(Token {
                    position: (line_number, col_num),
                    token: TokenType::U16(number),
                });
            }
        } else {
            return Err(ParseError {
                position: (line_number, col_num),
                error: ErrorKind::NumberError(lexeme.to_owned()),
            });
        }
    } else if let Ok(number) = u16::from_str_radix(&lexeme, 10) {
        if number < 256 {
            return Ok(Token {
                position: (line_number, col_num),
                token: TokenType::U8(number as u8),
            });
        } else {
            return Ok(Token {
                position: (line_number, col_num),
                token: TokenType::U16(number),
            });
        }
    } else if is_numeric(lexeme) {
        return Err(ParseError {
            position: (line_number, col_num),
            error: ErrorKind::NumberError(lexeme.to_owned()),
        });
    } else if is_keyword(&lexeme) {
        return Ok(Token {
            position: (line_number, col_num),
            token: TokenType::Operation(lexeme.to_owned()),
        });
    } else if ["A", "B", "C", "D", "H", "L", "SP", "PSW"].contains(&lexeme.to_uppercase().as_str())
    {
        return Ok(Token {
            position: (line_number, col_num),
            token: match lexeme {
                "A" => TokenType::Register(Register::A),
                "B" => TokenType::Register(Register::B),
                "C" => TokenType::Register(Register::C),
                "D" => TokenType::Register(Register::D),
                "H" => TokenType::Register(Register::H),
                "L" => TokenType::Register(Register::L),
                "SP" => TokenType::Register(Register::SP),
                "E" => TokenType::Register(Register::E),
                "M" => TokenType::Register(Register::M),
                "PSW" => TokenType::Register(Register::PSW),
                _ => unreachable!("this is not supposed to happen!"),
            },
        });
    } else if is_valid_identifier(lexeme) {
        return Ok(Token {
            position: (line_number, col_num),
            token: TokenType::Label(lexeme.to_owned()),
        });
    }
    Err(ParseError {
        position: (line_number, col_num),
        error: ErrorKind::UnexpectedLexeme(lexeme.to_owned()),
    })
}

pub fn tokenize(code: &str) -> Result<TokenStream, ParseError> {
    let mut start = 0usize;
    let mut line_number = 1;
    let mut tokens = vec![];
    let mut col_num = 1usize;
    let mut last_col = 0usize;
    for (i, char) in code.as_bytes().iter().enumerate() {
        if [b' ', b'\t', b',', b':', b'\n', b'\r'].contains(&char) {
            if start == i {
                start = i + 1;
                col_num = (start - last_col) + 1;
                continue;
            }
            tokens.push(make_token(line_number, col_num, &code[start..i])?);
            if *char == b',' {
                tokens.push(Token {
                    token: TokenType::Comma,
                    position: (line_number, (i - last_col) + 1),
                });
            } else if *char == b':' {
                tokens.push(Token {
                    position: (line_number, (i - last_col) + 1),
                    token: TokenType::Colon,
                });
            } else if *char == b'\n' || *char == b'\r' {
                line_number += 1;
                last_col = i + 1;
            }
            start = i + 1;
            col_num = (start - last_col) + 1;
        }
    }
    Ok(TokenStream { tokens })
}
