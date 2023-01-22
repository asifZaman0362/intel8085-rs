mod simulator;
mod instructions;

#[cfg(test)]
mod tests {

    use super::*;

    static TEST_LOC: &str = "/Users/asifzaman/src/rust/intel8085-rs/tests/";

    fn setup_sim(sim: &mut simulator::Microcontroller, filename: &str) -> std::io::Result<()> {
        sim.clear_memory();
        sim.clear_registers();
        let code = match assembler::assembler::assemble_file(TEST_LOC.to_owned() + filename)? {
            Ok(code) => code,
            Err(parse_error) => panic!("{parse_error}")
        };
        sim.load_code(&code, 0).unwrap();
        Ok(())
    }

    #[test]
    fn test_fib() -> std::io::Result<()> {
        let mut sim = simulator::Microcontroller::new();
        setup_sim(&mut sim, "fib.asm")?;
        sim.set_data_at(Some(0x3030), 0x33);
        sim.start();
        let written = sim.get_data_at(Some(0x3031));
        Ok(assert_eq!(written, 0x37))
    }

    #[test]
    fn test_sum() -> std::io::Result<()> {
        let mut sim = simulator::Microcontroller::new();
        setup_sim(&mut sim, "add.asm")?;
        sim.set_data_at(Some(0x20), 0x30);
        sim.set_data_at(Some(0x21), 0x31);
        sim.start();
        let written = sim.get_data_at(Some(0x22));
        Ok(assert_eq!(written, 0x61))
    }

    #[test]
    fn test_series() -> std::io::Result<()> {
        let mut sim = simulator::Microcontroller::new();
        setup_sim(&mut sim, "series.asm")?;
        sim.set_data_at(Some(0x30), 0x4);
        sim.set_data_at(Some(0x31), 0x1);
        sim.set_data_at(Some(0x32), 0x2);
        sim.set_data_at(Some(0x33), 0x3);
        sim.set_data_at(Some(0x34), 0x4);
        sim.start();
        let written = sim.get_data_at(Some(0x70));
        Ok(assert_eq!(written, 0x0A))
    }

    #[test]
    fn test_sum_16bit() -> std::io::Result<()> {
        let mut sim = simulator::Microcontroller::new();
        setup_sim(&mut sim, "16bitadd.asm")?;
        sim.set_data_at(Some(0x5000), 0x34);
        sim.set_data_at(Some(0x5001), 0x12);
        sim.set_data_at(Some(0x5002), 0x78);
        sim.set_data_at(Some(0x5003), 0x56);
        sim.start();
        let written = (sim.get_data_at(Some(0x5005)) as u16) << 8 |
            (sim.get_data_at(Some(0x5004)) as u16);
        Ok(assert_eq!(written, 0x68AC))
    }

    #[test]
    fn test_sort_asc() -> std::io::Result<()> {
        let mut sim = simulator::Microcontroller::new();
        setup_sim(&mut sim, "sort_asc.asm")?;
        sim.set_data_at(Some(0x5000), 0x05);
        sim.set_data_at(Some(0x5001), 0x04);
        sim.set_data_at(Some(0x5002), 0x02);
        sim.set_data_at(Some(0x5003), 0x05);
        sim.set_data_at(Some(0x5004), 0x03);
        sim.set_data_at(Some(0x5005), 0x01);
        sim.start();
        assert_eq!(sim.get_data_at(Some(0x5001)), 0x01);
        assert_eq!(sim.get_data_at(Some(0x5002)), 0x02);
        assert_eq!(sim.get_data_at(Some(0x5003)), 0x03);
        assert_eq!(sim.get_data_at(Some(0x5004)), 0x04);
        Ok(assert_eq!(sim.get_data_at(Some(0x5005)), 0x05))
    }

    #[test]
    fn test_sort_dsc() -> std::io::Result<()> {
        let mut sim = simulator::Microcontroller::new();
        setup_sim(&mut sim, "sort_dsc.asm")?;
        sim.set_data_at(Some(0x5000), 0x05);
        sim.set_data_at(Some(0x5001), 0x04);
        sim.set_data_at(Some(0x5002), 0x02);
        sim.set_data_at(Some(0x5003), 0x05);
        sim.set_data_at(Some(0x5004), 0x03);
        sim.set_data_at(Some(0x5005), 0x01);
        sim.start();
        assert_eq!(sim.get_data_at(Some(0x5001)), 0x05);
        assert_eq!(sim.get_data_at(Some(0x5002)), 0x04);
        assert_eq!(sim.get_data_at(Some(0x5003)), 0x03);
        assert_eq!(sim.get_data_at(Some(0x5004)), 0x02);
        Ok(assert_eq!(sim.get_data_at(Some(0x5005)), 0x01))
    }

    #[test]
    fn test_even_odd() -> std::io::Result<()> {
        let mut sim = simulator::Microcontroller::new();
        setup_sim(&mut sim, "even_odd.asm")?;
        sim.set_data_at(Some(0x5000), 0x04);
        sim.start();
        Ok(assert_eq!(sim.get_data_at(Some(0x5001)), 0x00))
    }

    #[test]
    fn test_largest() -> std::io::Result<()> {
        let mut sim = simulator::Microcontroller::new();
        setup_sim(&mut sim, "largest.asm")?;
        sim.set_data_at(Some(0x5000), 0x05);
        sim.set_data_at(Some(0x5001), 0x04);
        sim.set_data_at(Some(0x5002), 0x02);
        sim.set_data_at(Some(0x5003), 0x05);
        sim.set_data_at(Some(0x5004), 0x03);
        sim.set_data_at(Some(0x5005), 0x01);
        sim.start();
        Ok(assert_eq!(sim.get_data_at(Some(0x4999)), 0x5))
    }

    #[test]
    fn test_mul() -> std::io::Result<()> {
        let mut sim = simulator::Microcontroller::new();
        setup_sim(&mut sim, "mul.asm")?;
        sim.set_data_at(Some(0x5000), 0x0a);
        sim.set_data_at(Some(0x5001), 0x05);
        sim.start();
        Ok(assert_eq!(sim.get_data_at(Some(0x5003)), 0x32))
    }

    #[test]
    fn test_div() -> std::io::Result<()> {
        let mut sim = simulator::Microcontroller::new();
        setup_sim(&mut sim, "div.asm")?;
        sim.set_data_at(Some(0x5000), 0x10);
        sim.set_data_at(Some(0x5001), 0x05);
        sim.start();
        assert_eq!(sim.get_data_at(Some(0x5003)), 0x03);
        Ok(assert_eq!(sim.get_data_at(Some(0x5002)), 0x01))
    }

}
