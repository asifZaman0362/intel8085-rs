mod error;
mod lexer;
pub mod assembler;
mod token;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembler() -> std::io::Result<()> {
        let result = assembler::assemble_file("code.asm")?;
        match result {
            Ok(bytes) => {
                print!("{{");
                for byte in bytes {
                    print!(" {:0x} ", byte);
                }
                println!("}}");
            }
            Err(parse_error) => println!("{}", parse_error)
        }
        Ok(())
    }
}
