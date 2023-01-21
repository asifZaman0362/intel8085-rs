static MEMORY_LOWER_LIMIT: usize = 1024;
static MEMORY_UPPER_LIMIT: usize = 64000;

#[derive(Clone, Copy)]
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

pub struct Microcontroller {
    reg_a: u8,
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_h: u8,
    reg_l: u8,
    stack_pointer: (u8, u8),
    flags: u8,
    pub program_counter: u16,
    pub instruction_register: u8,
    memory: [u8; 65535],
    io: [u8; 255],
    interrupts: bool,
    running: bool
}

pub enum Flag {
    Parity,
    Sign,
    Zero,
    Carry,
    AuxCarry,
}

impl Microcontroller {
    pub fn new() -> Microcontroller {
        use crate::instructions;
        let op_table        : [instructions::Instruction; 256] = [
            instructions::NOOP, // 0
            instructions::LXI_B, // 1
            instructions::STAX_B, // 2
            instructions::INX_B, // 3
            instructions::INR_B, // 4
            instructions::DCR_B, // 5
            instructions::MVI_B, // 6
            instructions::RLC, // 7
            instructions::NOOP, // 8
            instructions::DAD_B, // 9
            instructions::LDAX_B, // a
            instructions::DCX_B, // b
            instructions::INR_C, // c
            instructions::DCR_C, // d
            instructions::MVI_C, // e
            instructions::RRC, // f
            instructions::NOOP, // 10
            instructions::LXI_D, // 11
            instructions::STAX_D, // 12
            instructions::INX_D, // 13
            instructions::INR_D, // 14
            instructions::DCR_D, // 15
            instructions::MVI_D, // 16
            instructions::RAL, // 17
            instructions::NOOP, // 18
            instructions::DAD_D, // 19
            instructions::LDAX_D, // 1a
            instructions::DCX_D, // 1b
            instructions::INR_E, // 1c
            instructions::DCR_E, // 1d
            instructions::MVI_E, // 1e
            instructions::RAR, // 1f
            instructions::RIM, // 20
            instructions::LXI_H, // 21
            instructions::SHLD, // 22
            instructions::INX_H, // 23
            instructions::INR_H, // 24
            instructions::DCR_H, // 25
            instructions::MVI_H, // 26
            instructions::DAA, // 27
            instructions::NOOP, // 28
            instructions::DAD_D, // 29
            instructions::LHLD, // 2a
            instructions::DCX_H, // 2b
            instructions::INR_L, // 2c
            instructions::DCR_L, // 2d
            instructions::MVI_L, // 2e
            instructions::CMA, // 2f
            instructions::SIM, // 30
            instructions::LXI_SP, // 31
            instructions::STA, // 32
            instructions::INX_SP, // 33
            instructions::INR_M, // 34
            instructions::DCR_M, // 35
            instructions::MVI_M, // 36
            instructions::STC, // 37
            instructions::NOOP, // 38
            instructions::DAD_SP, // 39
            instructions::LDA, // 3a
            instructions::DCX_SP, // 3b
            instructions::INR_A, // 3c
            instructions::DCR_A, // 3d
            instructions::MVI_A, // 3e
            instructions::CMC, // 3f
            instructions::MOV_BB, // 40
            instructions::MOV_BC, // 41
            instructions::MOV_BD, // 42
            instructions::MOV_BE, // 43
            instructions::MOV_BH, // 44
            instructions::MOV_BL, // 45
            instructions::MOV_BM, // 46
            instructions::MOV_BA, // 47
            instructions::MOV_CB, // 48
            instructions::MOV_CC, // 49
            instructions::MOV_CD, // 4a
            instructions::MOV_CE, // 4b
            instructions::MOV_CH, // 4c
            instructions::MOV_CL, // 4d
            instructions::MOV_CM, // 4e
            instructions::MOV_CA, // 4f
            instructions::MOV_DB, // 50
            instructions::MOV_DC, // 51
            instructions::MOV_DD, // 52
            instructions::MOV_DE, // 53
            instructions::MOV_DH, // 54
            instructions::MOV_DL, // 55
            instructions::MOV_DM, // 56
            instructions::MOV_DA, // 57
            instructions::MOV_EB, // 58
            instructions::MOV_EC, // 59
            instructions::MOV_ED, // 5a
            instructions::MOV_EE, // 5b
            instructions::MOV_EH, // 5c
            instructions::MOV_EL, // 5d
            instructions::MOV_EM, // 5e
            instructions::MOV_EA, // 5f
            instructions::MOV_HB, // 60
            instructions::MOV_HC, // 61
            instructions::MOV_HD, // 62
            instructions::MOV_HE, // 63
            instructions::MOV_HH, // 64
            instructions::MOV_HL, // 65
            instructions::MOV_HM, // 66
            instructions::MOV_HA, // 67
            instructions::MOV_LB, // 68
            instructions::MOV_LC, // 69
            instructions::MOV_LD, // 6a
            instructions::MOV_LE, // 6b
            instructions::MOV_LH, // 6c
            instructions::MOV_LL, // 6d
            instructions::MOV_LM, // 6e
            instructions::MOV_LA, // 6f
            instructions::MOV_MB, // 70
            instructions::MOV_MC, // 71
            instructions::MOV_MD, // 72
            instructions::MOV_ME, // 73
            instructions::MOV_MH, // 74
            instructions::MOV_ML, // 75
            instructions::HLT, // 76
            instructions::MOV_MA, // 77
            instructions::MOV_AB, // 78
            instructions::MOV_AC, // 79
            instructions::MOV_AD, // 7a
            instructions::MOV_AE, // 7b
            instructions::MOV_AH, // 7c
            instructions::MOV_AL, // 7d
            instructions::MOV_AM, // 7e
            instructions::MOV_AA, // 7f
            instructions::ADD_B, // 80
            instructions::ADD_C, // 81
            instructions::ADD_D, // 82
            instructions::ADD_E, // 83
            instructions::ADD_H, // 84
            instructions::ADD_L, // 85
            instructions::ADD_M, // 86
            instructions::ADD_A, // 87
            instructions::ADC_B, // 88
            instructions::ADC_C, // 89
            instructions::ADC_D, // 8a
            instructions::ADC_E, // 8b
            instructions::ADC_H, // 8c
            instructions::ADC_L, // 8d
            instructions::ADC_M, // 8e
            instructions::ADC_A, // 8f
            instructions::SUB_B, // 90
            instructions::SUB_C, // 91
            instructions::SUB_D, // 92
            instructions::SUB_E, // 93
            instructions::SUB_H, // 94
            instructions::SUB_L, // 95
            instructions::SUB_M, // 96
            instructions::SUB_A, // 97
            instructions::SBB_B, // 98
            instructions::SBB_C, // 99
            instructions::SBB_D, // 9a
            instructions::SBB_E, // 9b
            instructions::SBB_H, // 9c
            instructions::SBB_L, // 9d
            instructions::SBB_M, // 9e
            instructions::SBB_A, // 9f
            instructions::ANA_B, // a0
            instructions::ANA_C, // a1
            instructions::ANA_D, // a2
            instructions::ANA_E, // a3
            instructions::ANA_H, // a4
            instructions::ANA_L, // a5
            instructions::ANA_M, // a6
            instructions::ANA_A, // a7
            instructions::XRA_B, // a8
            instructions::XRA_C, // a9
            instructions::XRA_D, // aa
            instructions::XRA_E, // ab
            instructions::XRA_H, // ac
            instructions::XRA_L, // ad
            instructions::XRA_M, // ae
            instructions::XRA_A, // af
            instructions::ORA_B, // b0
            instructions::ORA_C, // b1
            instructions::ORA_D, // b2
            instructions::ORA_E, // b3
            instructions::ORA_H, // b4
            instructions::ORA_L, // b5
            instructions::ORA_M, // b6
            instructions::ORA_A, // b7
            instructions::CMP_B, // b8
            instructions::CMP_C, // b9
            instructions::CMP_D, // ba
            instructions::CMP_E, // bb
            instructions::CMP_H, // bc
            instructions::CMP_L, // bd
            instructions::CMP_M, // be
            instructions::CMP_A, // bf
            instructions::RNZ, // c0
            instructions::POP_B, // c1
            instructions::JNZ, // c2
            instructions::JMP, // c3
            instructions::CNZ, // c4
            instructions::PUSH_B, // c5
            instructions::ADI, // c6
            instructions::RST_0, // c7
            instructions::RZ, // c8
            instructions::RET, // c9
            instructions::JZ, // ca
            instructions::NOOP, // cb
            instructions::CZ, // cc
            instructions::CALL, // cd
            instructions::ACI, // ce
            instructions::RST_1, // cf
            instructions::RNC, // d0
            instructions::POP_D, // d1
            instructions::JNC, // d2
            instructions::OUTPUT, // d3
            instructions::CNC, // d4
            instructions::PUSH_D, // d5
            instructions::SUI, // d6
            instructions::RST_2, // d7
            instructions::RC, // d8
            instructions::NOOP, // d9
            instructions::JC, // da
            instructions::INPUT, // db
            instructions::CC, // dc
            instructions::NOOP, // dd
            instructions::SBI, // de
            instructions::RST_3, // df
            instructions::RPO, // e0
            instructions::POP_H, // e1
            instructions::JPO, // e2
            instructions::XTHL, // e3
            instructions::CPO, // e4
            instructions::PUSH_H, // e5
            instructions::ANI, // e6
            instructions::RST_4, // e7
            instructions::RPE, // e8
            instructions::PCHL, // e9
            instructions::JPE, // ea
            instructions::XCHG, // eb
            instructions::CPE, // ec
            instructions::NOOP, // ed
            instructions::XRI, // ee
            instructions::RST_5, // ef
            instructions::RP, // f0
            instructions::POP_PSW, // f1
            instructions::JP, // f2
            instructions::DI, // f3
            instructions::CP, // f4
            instructions::PUSH_PSW, // f5
            instructions::ORI, // f6
            instructions::RST_6, // f7
            instructions::RM, // f8
            instructions::SPHL, // f9
            instructions::JM, // fa
            instructions::EI, // fb
            instructions::CM, // fc
            instructions::NOOP, // fd
            instructions::CPI, // fe
            instructions::NOOP  // ff
        ];
        Microcontroller {
            reg_a: 0,
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,
            stack_pointer: (0, 0),
            flags: 0,
            program_counter: 0,
            instruction_register: 0,
            memory: [0u8; 65535],
            io: [0u8; 255],
            interrupts: false,
            running: false
        }
    }

    pub fn get_register(&self, reg: Register) -> Result<u8, &'static str> {
        use Register::{A, B, C, D, E, H, L, M};
        match reg {
            A => Ok(self.reg_a),
            B => Ok(self.reg_b),
            C => Ok(self.reg_c),
            D => Ok(self.reg_d),
            E => Ok(self.reg_e),
            H => Ok(self.reg_h),
            L => Ok(self.reg_l),
            M => Ok(self.get_data_at(None)),
            _ => Err("Cannot get single byte data from special registers"),
        }
    }

    pub fn get_register_pair(&self, reg: Register) -> Result<u16, &'static str> {
        use Register::{B, D, H, SP, PSW};
        match reg {
            B => Ok((self.reg_b as u16) << 8 | self.reg_c as u16),
            D => Ok((self.reg_d as u16) << 8 | self.reg_e as u16),
            H => Ok((self.reg_h as u16) << 8 | self.reg_l as u16),
            SP => Ok((self.stack_pointer.0 as u16) << 8 | self.stack_pointer.1 as u16),
            PSW => Ok((self.reg_a as u16) << 8 | self.flags as u16),
            _ => Err("Not a register pair"),
        }
    }

    pub fn get_data_at(&self, location: Option<u16>) -> u8 {
        if let Some(location) = location {
            self.memory[location as usize]
        } else {
            let location = (self.reg_h as u16) << 8 | self.reg_l as u16;
            self.memory[location as usize]
        }
    }

    pub fn set_data_at(&mut self, location: Option<u16>, data: u8) {
        if let Some(location) = location {
            self.memory[location as usize] = data;
        } else {
            let location = (self.reg_h as u16) << 8 | self.reg_l as u16;
            self.memory[location as usize] = data;
        }
    }

    pub fn set_register(&mut self, register: Register, data: u8) -> Result<(), &'static str> {
        use Register::{A, B, C, D, E, H, L, M};
        match register {
            A => self.reg_a = data,
            B => self.reg_b = data,
            C => self.reg_c = data,
            D => self.reg_d = data,
            E => self.reg_e = data,
            H => self.reg_h = data,
            L => self.reg_l = data,
            M => self.set_data_at(None, data),
            _ => {
                return Err("not an 8bit register");
            }
        }
        Ok(())
    }

    pub fn set_register_pair(&mut self, register: Register, data: u16) -> Result<(), &'static str> {
        use Register::{B, D, H, SP};
        match register {
            B => {
                self.reg_b = (data << 8 >> 8) as u8;
                self.reg_c = (data >> 8) as u8;
            }
            D => {
                self.reg_d = (data << 8 >> 8) as u8;
                self.reg_e = (data >> 8) as u8;
            }
            H => {
                self.reg_h = (data << 8 >> 8) as u8;
                self.reg_l = (data >> 8) as u8;
            }
            SP => {
                self.stack_pointer.0 = (data << 8 >> 8) as u8;
                self.stack_pointer.1 = (data >> 8) as u8;
            }
            _ => {
                return Err("not a register pair");
            }
        }
        Ok(())
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        use Flag::{AuxCarry, Carry, Parity, Sign, Zero};
        let mask = match flag {
            Sign => 0b10000000,
            Zero => 0b01000000,
            AuxCarry => 0b00010000,
            Parity => 0b00000100,
            Carry => 0b00000001,
        };
        if value {
            self.flags &= !mask;
        } else {
            self.flags |= mask;
        }
    }

    pub fn check_flag(&self, flag: Flag) -> bool {
        use Flag::{AuxCarry, Carry, Parity, Sign, Zero};
        let mask = match flag {
            Sign => 0b10000000,
            Zero => 0b01000000,
            AuxCarry => 0b00010000,
            Parity => 0b00000100,
            Carry => 0b00000001,
        };
        self.flags & mask == mask
    }

    pub fn load_code(&mut self, code: &[u8], load_point: u16) -> Result<(), String> {
        let pc = load_point;
        let load_point = load_point as usize;
        if load_point < MEMORY_LOWER_LIMIT {
            Err(format!("Cant load at {load_point} as bytes till {MEMORY_LOWER_LIMIT} are reserved"))
        }
        else if code.len() + load_point > MEMORY_UPPER_LIMIT {
            Err(format!("Code does not fit inside memory when loaded at {load_point}"))
        } else {
            Ok({
                self.program_counter = pc;
                for i in 0..code.len() {
                    self.memory[i + load_point] = code[i];
                }
            })
        }
    }

    pub fn tick(&mut self) -> Result<(), &'static str> {
        if self.running {
            Ok({
                self.fetch();
                self.execute();
            })
        } else {
            Err("Microcontroller not started!")
        }
    }

    pub fn start(&mut self) {
        self.tick();
    }

    pub fn fetch(&mut self) -> u8 {
        self.instruction_register = self.get_data_at(Some(self.program_counter));
        self.program_counter += 1;
        self.instruction_register
    }

    pub fn execute(&mut self) {
        // TODO: execute instruction
    }

    pub fn check_parity(x: u8) -> bool {
        let mut y = x;
        y ^= y >> 4;
        y ^= y >> 2;
        y ^= y >> 1;
        ((!y) & 1) != 0
    }

    pub fn check_parity_16(x: u16) -> bool {
        let mut y = x;
        y ^= y >> 8;
        y ^= y >> 4;
        y ^= y >> 2;
        y ^= y >> 1;
        ((!y) & 1) != 0
    }

    pub fn update_flags(&mut self, ac: bool, c: bool) {
        self.set_flag(Flag::Carry, c);
        self.set_flag(Flag::AuxCarry, ac);
        self.set_flag(Flag::Zero, self.reg_a == 0);
        self.set_flag(Flag::Sign, self.reg_a > 127);
        self.set_flag(Flag::Parity, Microcontroller::check_parity(self.reg_a));
    }

    pub fn update_flags_logical(&mut self) {
        self.set_flag(Flag::Zero, self.reg_a == 0);
        self.set_flag(Flag::Sign, self.reg_a > 127);
        self.set_flag(Flag::Parity, self.reg_a > 127);
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn clear_memory(&mut self) {
        self.memory.fill(0b0);
    }

    pub fn clear_registers(&mut self) {
        self.reg_a = 0;
        self.reg_b = 0;
        self.reg_c = 0;
        self.reg_d = 0;
        self.reg_e = 0;
        self.reg_h = 0;
        self.reg_l = 0;
        self.stack_pointer = (0, 0);
        self.flags = 0;
        self.instruction_register = 0;
    }

    pub fn enable_interrupts(&mut self) {
        self.interrupts = true;
    }

    pub fn disable_interrupts(&mut self) {
        self.interrupts = false;
    }

    pub fn write_io(&mut self, addr: u16, byte: u8) {
    }

    pub fn read_io(&mut self, addr: u16) -> u8 {
        0
    }

}
