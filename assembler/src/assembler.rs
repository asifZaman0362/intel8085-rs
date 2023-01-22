use std::collections::HashMap;
use std::fs::File;
use std::io::{ Read, BufReader };

use crate::error::{ ParseError, ErrorKind };
use crate::token::{ Token, TokenType, Register, TokenStream };


#[derive(Clone, Copy)]
struct Instruction {
    opcode: u8,
    size: u8,
    args: u8,
}

impl Instruction {
    fn new(opcode: u8, size: u8, args: u8) -> Instruction {
        Instruction { opcode, size, args }
    }
}

type SymbolTable = HashMap<String, u16>;

enum ParsedToken {
    Code(u8),
    Symbol(String),
}

fn next_token(
    iterator: &mut std::slice::Iter<Token>,
    expected: Vec<TokenType>,
) -> Result<Token, ParseError> {
    if let Some(token) = iterator.next() {
        if expected.contains(&token.token) {
            return Ok(token.to_owned());
        } else {
            return Err(ParseError {
                position: token.position,
                error: ErrorKind::UnexpectedToken(expected, token.token.clone()),
            });
        }
    } else {
        if expected.contains(&TokenType::End) {
            Ok( Token { position: (0, 0), token: TokenType::End })
        }
        else {
            Err(ParseError {
                position: (0, 0),
                error: ErrorKind::Eof,
            })
        }
    }
}

fn get_register_index(name: &Register, position: (usize, usize)) -> Result<u8, ParseError> {
    match name {
        Register::A => Ok(7),
        Register::B => Ok(0),
        Register::C => Ok(1),
        Register::D => Ok(2),
        Register::E => Ok(3),
        Register::H => Ok(4),
        Register::L => Ok(5),
        Register::M => Ok(6),
        _ => Err(ParseError {
            position,
            error: ErrorKind::InvalidArguments("Register".to_owned(), format!("{:?}", name)),
        }),
    }
}

fn get_register_pair_index(name: &Register, position: (usize, usize)) -> Result<u8, ParseError> {
    match name {
        Register::B => Ok(0),
        Register::D => Ok(1),
        Register::H => Ok(2),
        Register::SP | Register::PSW => Ok(3),
        _ => Err(ParseError {
            position,
            error: ErrorKind::InvalidArguments("Register Pair".to_owned(), format!("{:?}", name)),
        }),
    }
}

fn parse_first_pass(
    iterator: &mut std::slice::Iter<Token>
) -> Result<(Vec<ParsedToken>, SymbolTable), ParseError> {
    let mut byte = 0u16;
    let mut symbol_table = SymbolTable::new();
    let mut stream: Vec<ParsedToken> = vec![];
    let opcodes: HashMap<&str, Instruction> = HashMap::from([
        ("ADD", Instruction::new(0x80, 1, 1)),
        ("ACI", Instruction::new(0xCE, 2, 1)),
        ("ADC", Instruction::new(0x88, 1, 1)),
        ("ADI", Instruction::new(0xC6, 2, 1)),
        ("ANA", Instruction::new(0xA0, 1, 1)),
        ("ANI", Instruction::new(0xE6, 2, 1)),
        ("CALL", Instruction::new(0xCD, 3, 1)),
        ("CC", Instruction::new(0xDC, 3, 1)),
        ("CM", Instruction::new(0xFC, 3, 1)),
        ("CMA", Instruction::new(0x2F, 1, 0)),
        ("CMC", Instruction::new(0x3D, 1, 0)),
        ("CMP", Instruction::new(0xB8, 1, 1)),
        ("CNC", Instruction::new(0xD4, 3, 1)),
        ("CNZ", Instruction::new(0xC4, 3, 1)),
        ("CP", Instruction::new(0xF4, 3, 1)),
        ("CPE", Instruction::new(0xEC, 3, 1)),
        ("CPI", Instruction::new(0xFE, 2, 1)),
        ("CPO", Instruction::new(0xE4, 3, 1)),
        ("CZ", Instruction::new(0xCC, 3, 1)),
        ("DAA", Instruction::new(0x27, 1, 0)),
        ("DAD", Instruction::new(0x09, 1, 1)),
        ("DCR", Instruction::new(0x05, 1, 1)),
        ("DCX", Instruction::new(0x0B, 1, 1)),
        ("DI", Instruction::new(0xF3, 1, 0)),
        ("EI", Instruction::new(0xFB, 1, 0)),
        ("HLT", Instruction::new(0x76, 1, 0)),
        ("IN", Instruction::new(0xDB, 2, 1)),
        ("INR", Instruction::new(0x04, 1, 1)),
        ("INX", Instruction::new(0x03, 1, 1)),
        ("JC", Instruction::new(0xDA, 3, 1)),
        ("JNC", Instruction::new(0xD2, 3, 1)),
        ("JM", Instruction::new(0xFA, 3, 1)),
        ("JMP", Instruction::new(0xC3, 3, 1)),
        ("JNZ", Instruction::new(0xC2, 3, 1)),
        ("JP", Instruction::new(0xF2, 3, 1)),
        ("JPE", Instruction::new(0xEA, 3, 1)),
        ("JPO", Instruction::new(0xE2, 3, 1)),
        ("JZ", Instruction::new(0xCA, 3, 1)),
        ("LDA", Instruction::new(0x3A, 3, 1)),
        ("LDAX", Instruction::new(0xA, 1, 1)),
        ("LHLD", Instruction::new(0x2A, 3, 1)),
        ("LXI", Instruction::new(0x01, 3, 1)),
        ("MOV", Instruction::new(0x40, 1, 2)),
        ("MVI", Instruction::new(0x06, 2, 2)),
        ("NOP", Instruction::new(0x00, 1, 0)),
        ("ORA", Instruction::new(0xB0, 1, 1)),
        ("ORI", Instruction::new(0xF6, 2, 1)),
        ("OUT", Instruction::new(0xD3, 2, 1)),
        ("PCHL", Instruction::new(0xE9, 1, 0)),
        ("POP", Instruction::new(0xC1, 1, 1)),
        ("POP", Instruction::new(0xF1, 1, 1)),
        ("PUSH", Instruction::new(0xC5, 1, 1)),
        ("RAL", Instruction::new(0x17, 1, 0)),
        ("RAR", Instruction::new(0x1F, 1, 0)),
        ("RC", Instruction::new(0xD8, 1, 0)),
        ("RET", Instruction::new(0xC9, 1, 0)),
        ("RIM", Instruction::new(0x20, 1, 0)),
        ("RLC", Instruction::new(0x07, 1, 0)),
        ("RM", Instruction::new(0xF8, 1, 0)),
        ("RNC", Instruction::new(0xD0, 1, 0)),
        ("RNZ", Instruction::new(0xC0, 1, 0)),
        ("RP", Instruction::new(0xF0, 1, 0)),
        ("RPE", Instruction::new(0xE8, 1, 0)),
        ("RPO", Instruction::new(0xE0, 1, 0)),
        ("RRC", Instruction::new(0x0F, 1, 0)),
        ("RST", Instruction::new(0xC7, 1, 1)),
        ("RZ", Instruction::new(0xC8, 1, 0)),
        ("SBB", Instruction::new(0x98, 1, 1)),
        ("SBI", Instruction::new(0xDE, 2, 1)),
        ("SHLD", Instruction::new(0x22, 3, 1)),
        ("SIM", Instruction::new(0x30, 1, 0)),
        ("SPHL", Instruction::new(0xF9, 1, 0)),
        ("STA", Instruction::new(0x32, 3, 1)),
        ("STAX", Instruction::new(0x02, 1, 1)),
        ("STC", Instruction::new(0x37, 1, 0)),
        ("SUB", Instruction::new(0x90, 1, 1)),
        ("SUI", Instruction::new(0xD6, 2, 1)),
        ("XCHG", Instruction::new(0xEB, 1, 0)),
        ("XRA", Instruction::new(0xA8, 1, 1)),
        ("XRI", Instruction::new(0xEE, 2, 1)),
        ("XTHL", Instruction::new(0xE3, 1, 0)),
    ]);
    loop {
        match next_token(
            iterator,
            vec![
                TokenType::Operation("".to_owned()),
                TokenType::Label("".to_owned()),
                TokenType::End,
            ],
        )?
        .token
        {
            TokenType::Operation(operation) => {
                let instruction = opcodes[operation.as_str()];
                byte += instruction.size as u16;
                if instruction.args == 0 {
                    stream.push(ParsedToken::Code(instruction.opcode));
                } else {
                    match operation.as_str() {
                        "SUI" | "SBI" | "ORI" | "CPI" | "ANI" | "ADI" | "ACI" | "IN" | "OUT" => {
                            stream.push(ParsedToken::Code(instruction.opcode));
                            if let TokenType::U8(byte) =
                                next_token(iterator, vec![TokenType::U8(0)])?.token
                            {
                                stream.push(ParsedToken::Code(byte));
                            }
                        }
                        "ADD" | "ADC" | "ANA" | "ORA" | "SUB" | "CMP" | "SBB" | "XRA" => {
                            let mut opcode = instruction.opcode;
                            if let Token {
                                position,
                                token: TokenType::Register(reg),
                            } = next_token(iterator, vec![TokenType::Register(Register::A)])?
                            {
                                opcode += get_register_index(&reg, position)?;
                                stream.push(ParsedToken::Code(opcode));
                            }
                        }
                        "CALL" | "CC" | "CM" | "CNC" | "CNZ" | "CP" | "CPE" | "CPO" | "CZ" | "JC"
                        | "JM" | "JMP" | "JNC" | "JNZ" | "JP" | "JPE" | "JPO" | "JZ" => {
                            stream.push(ParsedToken::Code(instruction.opcode));
                            if let TokenType::Label(label) =
                                next_token(iterator, vec![TokenType::Label("".to_owned())])?.token
                            {
                                stream.push(ParsedToken::Symbol(label));
                            }
                        }
                        "LDA" | "LHLD" | "SHLD" | "STA" => {
                            stream.push(ParsedToken::Code(instruction.opcode));
                            match next_token(iterator, vec![TokenType::U16(0), TokenType::U8(0)])?.token {
                                TokenType::U8(byte) => {
                                    stream.push(ParsedToken::Code(byte));
                                    stream.push(ParsedToken::Code(0));
                                }
                                TokenType::U16(addr) => {
                                    stream.push(ParsedToken::Code((addr << 8 >> 8) as u8));
                                    stream.push(ParsedToken::Code((addr >> 8) as u8));
                                }
                                _ => unreachable!("should never happen!")
                            }
                        }
                        "DAD" => {
                            if let Token {
                                position,
                                token: TokenType::Register(reg),
                            } = next_token(iterator, vec![TokenType::Register(Register::A)])?
                            {
                                stream.push(ParsedToken::Code({
                                    match reg {
                                        Register::B => 0x09,
                                        Register::D => 0x19,
                                        Register::H => 0x29,
                                        Register::SP => 0x39,
                                        _ => {
                                            return Err(ParseError {
                                                position,
                                                error: ErrorKind::InvalidArguments(
                                                    "B, D, H or SP".to_owned(),
                                                    format!("{:?}", reg),
                                                ),
                                            });
                                        }
                                    }
                                }));
                            }
                        }
                        "DCX" => {
                            if let Token {
                                position,
                                token: TokenType::Register(reg),
                            } = next_token(iterator, vec![TokenType::Register(Register::A)])?
                            {
                                stream.push(ParsedToken::Code({
                                    match reg {
                                        Register::B => 0x0b,
                                        Register::D => 0x1b,
                                        Register::H => 0x2b,
                                        Register::SP => 0x3b,
                                        _ => {
                                            return Err(ParseError {
                                                position,
                                                error: ErrorKind::InvalidArguments(
                                                    "B, D, H or SP".to_owned(),
                                                    format!("{:?}", reg),
                                                ),
                                            });
                                        }
                                    }
                                }));
                            }
                        }
                        "INX" => {
                            if let Token {
                                position,
                                token: TokenType::Register(reg),
                            } = next_token(iterator, vec![TokenType::Register(Register::A)])?
                            {
                                stream.push(ParsedToken::Code({
                                    match reg {
                                        Register::B => 0x03,
                                        Register::D => 0x13,
                                        Register::H => 0x23,
                                        Register::SP => 0x33,
                                        _ => {
                                            return Err(ParseError {
                                                position,
                                                error: ErrorKind::InvalidArguments(
                                                    "B, D, H or SP".to_owned(),
                                                    format!("{:?}", reg),
                                                ),
                                            });
                                        }
                                    }
                                }));
                            }
                        }
                        "LXI" => {
                            if let Token {
                                position,
                                token: TokenType::Register(reg),
                            } = next_token(iterator, vec![TokenType::Register(Register::A)])?
                            {
                                let opcode;
                                match reg {
                                    Register::B => opcode = 0x01,
                                    Register::D => opcode = 0x11,
                                    Register::H => opcode = 0x21,
                                    Register::SP => opcode = 0x31,
                                    _ => {
                                        return Err(ParseError {
                                            position,
                                            error: ErrorKind::InvalidArguments(
                                                "B, D, H or SP".to_owned(),
                                                format!("{:?}", reg),
                                            ),
                                        });
                                    }
                                };
                                stream.push(ParsedToken::Code(opcode));
                                next_token(iterator, vec![TokenType::Comma])?;
                                match next_token(iterator, vec![TokenType::U16(0), TokenType::U8(0)])?.token {
                                    TokenType::U8(byte) => {
                                        stream.push(ParsedToken::Code(byte));
                                        stream.push(ParsedToken::Code(0));
                                    }
                                    TokenType::U16(addr) => {
                                        stream.push(ParsedToken::Code((addr << 8 >> 8) as u8));
                                        stream.push(ParsedToken::Code((addr >> 8) as u8));
                                    }
                                    _ => unreachable!("should never happen!")
                                }
                            }
                        }
                        "LDAX" | "STAX" => {
                            let opcode = instruction.opcode;
                            if let Token {
                                position,
                                token: TokenType::Register(reg),
                            } = next_token(iterator, vec![TokenType::Register(Register::A)])?
                            {
                                match reg {
                                    Register::B | Register::D => stream.push(ParsedToken::Code(
                                        opcode + get_register_pair_index(&reg, position)? * 16,
                                    )),
                                    _ => {
                                        return Err(ParseError {
                                            position,
                                            error: ErrorKind::InvalidArguments(
                                                "B or D".to_owned(),
                                                format!("{:?}", reg),
                                            ),
                                        })
                                    }
                                }
                            }
                        }
                        "PUSH" | "POP" => {
                            let opcode = instruction.opcode;
                            if let Token {
                                position,
                                token: TokenType::Register(reg),
                            } = next_token(iterator, vec![TokenType::Register(Register::A)])?
                            {
                                if reg == Register::SP {
                                    return Err(ParseError {
                                        position,
                                        error: ErrorKind::InvalidArguments(
                                            "B, D, H or M".to_owned(),
                                            "SP".to_owned(),
                                        ),
                                    });
                                } else {
                                    stream.push(ParsedToken::Code(
                                        opcode + get_register_pair_index(&reg, position)? * 16,
                                    ));
                                }
                            }
                        }
                        "RST" => {
                            let opcode = instruction.opcode;
                            if let Token {
                                position,
                                token: TokenType::U8(number),
                            } = next_token(iterator, vec![TokenType::U8(0)])?
                            {
                                if !((0..7).contains(&number)) {
                                    return Err(ParseError {
                                        position,
                                        error: ErrorKind::InvalidArguments(
                                            "[0-7]".to_owned(),
                                            format!("{}", number),
                                        ),
                                    });
                                } else {
                                    stream.push(ParsedToken::Code(opcode + 8 * number));
                                }
                            }
                        }
                        "MOV" => {
                            let mut opcode = instruction.opcode;
                            if let Token {
                                position,
                                token: TokenType::Register(reg),
                            } = next_token(iterator, vec![TokenType::Register(Register::A)])?
                            {
                                opcode += 8 * get_register_index(&reg, position)?;
                            }
                            next_token(iterator, vec![TokenType::Comma])?;
                            if let Token {
                                position,
                                token: TokenType::Register(reg),
                            } = next_token(iterator, vec![TokenType::Register(Register::A)])?
                            {
                                opcode += get_register_index(&reg, position)?;
                            }
                            stream.push(ParsedToken::Code(opcode));
                        }
                        "MVI" => {
                            if let Token {
                                position,
                                token: TokenType::Register(reg),
                            } = next_token(iterator, vec![TokenType::Register(Register::A)])?
                            {
                                stream.push(match reg {
                                    Register::A => ParsedToken::Code(0x3e),
                                    Register::B => ParsedToken::Code(0x06),
                                    Register::C => ParsedToken::Code(0x0e),
                                    Register::D => ParsedToken::Code(0x16),
                                    Register::E => ParsedToken::Code(0x1e),
                                    Register::H => ParsedToken::Code(0x26),
                                    Register::L => ParsedToken::Code(0x2e),
                                    Register::M => ParsedToken::Code(0x36),
                                    _ => {
                                        return Err(ParseError {
                                            position,
                                            error: ErrorKind::InvalidArguments(
                                                "Register".to_owned(),
                                                format!("{:?}", reg),
                                            ),
                                        })
                                    }
                                });
                            }
                            next_token(iterator, vec![TokenType::Comma])?;
                            if let TokenType::U8(number) =
                                next_token(iterator, vec![TokenType::U8(0)])?.token
                            {
                                stream.push(ParsedToken::Code(number));
                            }
                        }
                        "INR" => {
                            if let Token {
                                position,
                                token: TokenType::Register(reg),
                            } = next_token(iterator, vec![TokenType::Register(Register::A)])?
                            {
                                stream.push(match reg {
                                    Register::A => ParsedToken::Code(0x3c),
                                    Register::B => ParsedToken::Code(0x04),
                                    Register::C => ParsedToken::Code(0x0c),
                                    Register::D => ParsedToken::Code(0x14),
                                    Register::E => ParsedToken::Code(0x1c),
                                    Register::H => ParsedToken::Code(0x24),
                                    Register::L => ParsedToken::Code(0x2c),
                                    Register::M => ParsedToken::Code(0x34),
                                    _ => {
                                        return Err(ParseError {
                                            position,
                                            error: ErrorKind::InvalidArguments(
                                                "Register".to_owned(),
                                                format!("{:?}", reg),
                                            ),
                                        })
                                    }
                                });
                            }
                        }
                        "DCR" => {
                            if let Token {
                                position,
                                token: TokenType::Register(reg),
                            } = next_token(iterator, vec![TokenType::Register(Register::A)])?
                            {
                                stream.push(match reg {
                                    Register::A => ParsedToken::Code(0x3d),
                                    Register::B => ParsedToken::Code(0x05),
                                    Register::C => ParsedToken::Code(0x0d),
                                    Register::D => ParsedToken::Code(0x15),
                                    Register::E => ParsedToken::Code(0x1d),
                                    Register::H => ParsedToken::Code(0x25),
                                    Register::L => ParsedToken::Code(0x2d),
                                    Register::M => ParsedToken::Code(0x35),
                                    _ => {
                                        return Err(ParseError {
                                            position,
                                            error: ErrorKind::InvalidArguments(
                                                "Register".to_owned(),
                                                format!("{:?}", reg),
                                            ),
                                        })
                                    }
                                });
                            }
                        }
                        _ => unreachable!("should never happen!"),
                    }
                }
            }
            TokenType::Label(label) => {
                next_token(iterator, vec![TokenType::Colon])?;
                symbol_table.insert(label.clone(), byte);
            }
            TokenType::End => break,
            _ => unreachable!("should never happen!"),
        }
    }
    Ok((stream, symbol_table))
}

fn second_pass(symbol_table: &SymbolTable, token_stream: &Vec<ParsedToken>) -> Vec<u8> {
    let mut bytes = vec![];
    for parsed in token_stream {
        match parsed {
            ParsedToken::Code(byte) => bytes.push(*byte),
            ParsedToken::Symbol(symbol) => {
                let word = symbol_table[symbol];
                bytes.push((word << 8 >> 8) as u8);
                bytes.push((word >> 8) as u8);
            }
        }
    }
    bytes
}

fn assemble_tokens(tokens: &mut TokenStream) -> Result<Vec<u8>, ParseError> {
    let (pre, symbol_table) = parse_first_pass(&mut tokens.iter())?;
    Ok(second_pass(&symbol_table, &pre))
}

pub fn assemble_file<P>(filename: P) -> std::io::Result<Result<Vec<u8>, ParseError>> 
where P: AsRef<std::path::Path> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(&file);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    match crate::lexer::tokenize(&buffer) {
        Ok(mut tokens) => Ok(assemble_tokens(&mut tokens)),
        Err(parse_error) => Ok(Err(parse_error))
    }
}
