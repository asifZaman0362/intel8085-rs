use crate::simulator::Register;
use crate::simulator::Microcontroller;
use crate::simulator::Flag;

trait Arith<T> {
    fn sub(&self, other: T) -> T;
    fn add(&self, other: T) -> T;
}

impl Arith<u8> for u8 {
    fn sub(&self, other: Self) -> Self {
        if other == 0 {
            *self
        } else {
            (*self as u16 + (!other + 1) as u16) as u8
        }
    }
    fn add(&self, other: Self) -> Self {
        (*self as u16 + other as u16) as u8
    }
}

impl Arith<u16> for u16 {
    fn sub(&self, other: Self) -> Self {
        (*self as u32 + (!other + 1) as u32) as u16
    }
    fn add(&self, other: Self) -> Self {
        (*self as u32 + other as u32) as u16
    }
}

fn _add(controller: &mut Microcontroller, other: u8, carry: u8) {
    let a = controller.get_register(Register::A).unwrap();
    let c = controller.check_flag(Flag::Carry) as u8 * carry;
    let sum = a.add(other).add(c);
    let ac = (((a & 0b00001111).add(c)).add(other & 0b00001111)) > 0b00001001;
    let c = (a as u16 + other as u16 + c as u16) > 255;
    controller.set_register(Register::A, sum).unwrap();
    controller.update_flags(ac, c);
}

#[allow(dead_code)]
fn dadd(controller : &mut Microcontroller, reg: Register) {
    let a = controller.get_register_pair(Register::H).unwrap();
    let b = controller.get_register_pair(reg).unwrap();
    let c = controller.check_flag(Flag::Carry) as u8;
    let sum = a.add(b).add(c as u16);
    let ac = (((a & 0b00001111) + c as u16) + (b & 0b00001111)) > 0b00001001;
    let c = (a as u32 + b as u32 + c as u32) > 65535;
    controller.set_flag(Flag::Carry, c);
    controller.set_flag(Flag::AuxCarry, ac);
    controller.set_register_pair(Register::H, sum).unwrap();
}

#[allow(dead_code)]
fn add(controller: &mut Microcontroller, reg: Register) {
    let b = controller.get_register(reg).unwrap();
    _add(controller, b, 0);
}

#[allow(dead_code)]
fn adc(controller: &mut Microcontroller, reg: Register) {
    let b = controller.get_register(reg).unwrap();
    _add(controller, b, 1);
}

#[allow(dead_code)]
fn adi(controller : &mut Microcontroller) {
    controller.fetch();
    let b = controller.instruction_register;
    _add(controller, b, 0);
}

#[allow(dead_code)]
fn aci(controller : &mut Microcontroller) {
    controller.fetch();
    let b = controller.instruction_register;
    _add(controller, b, 1);
}

#[allow(dead_code)]
fn _sub(controller : &mut Microcontroller, other: u8, carry: u8) {
    let a = controller.get_register(Register::A).unwrap();
    let c = controller.check_flag(Flag::Carry) as u8 * carry;
    let sum = a.sub(other).sub(c);
    let ac = (((a & 0b00001111).sub(c)).sub(other & 0b00001111)) > 0b00001001;
    let c = (a.sub(other).sub(c)) > 127;
    controller.set_register(Register::A, sum).unwrap();
    controller.update_flags(ac, c);
}

#[allow(dead_code)]
fn sub(controller: &mut Microcontroller, reg: Register, carry: u8) {
    let b = controller.get_register(reg).unwrap();
    _sub(controller, b, carry);
}

#[allow(dead_code)]
fn sbi(controller : &mut Microcontroller) {
    controller.fetch();
    let b = controller.instruction_register;
    _sub(controller, b, 1);
}

#[allow(dead_code)]
fn cmp(controller : &mut Microcontroller, reg: Register) {
    let a = controller.get_register(Register::A).unwrap();
    sub(controller, reg, 0);
    controller.set_register(Register::A, a).unwrap();
}

#[allow(dead_code)]
fn cpi(controller : &mut Microcontroller) {
    let a = controller.get_register(Register::A).unwrap();
    sbi(controller);
    controller.set_register(Register::A, a).unwrap();
}

#[allow(dead_code)]
fn mov(controller: &mut Microcontroller, to: Register, from: Register) {
    let data = controller.get_register(from).unwrap();
    controller.set_register(to, data).unwrap();
}

#[allow(dead_code)]
fn mvi(controller: &mut Microcontroller, to: Register) {
    controller.fetch();
    let data = controller.instruction_register;
    controller.set_register(to, data).unwrap();
}

#[allow(dead_code)]
fn ora(controller : &mut Microcontroller, other : Register) {
    let mut val = controller.get_register(Register::A).unwrap();
    val |= controller.get_register(other).unwrap();
    controller.set_register(Register::A, val).unwrap();
    controller.update_flags_logical();
}

#[allow(dead_code)]
fn ori(controller : &mut Microcontroller) {
    let mut val = controller.get_register(Register::A).unwrap();
    controller.fetch();
    val |= controller.instruction_register;
    controller.set_register(Register::A, val).unwrap();
    controller.update_flags_logical();
}

#[allow(dead_code)]
fn ana(controller : &mut Microcontroller, other : Register) {
    let mut val = controller.get_register(Register::A).unwrap();
    val &= controller.get_register(other).unwrap();
    controller.set_register(Register::A, val).unwrap();
    controller.update_flags_logical();
}

#[allow(dead_code)]
fn ani(controller : &mut Microcontroller) {
    let mut val = controller.get_register(Register::A).unwrap();
    controller.fetch();
    val &= controller.instruction_register;
    controller.set_register(Register::A, val).unwrap();
    controller.update_flags_logical();
}

#[allow(dead_code)]
fn xra(controller : &mut Microcontroller, other : Register) {
    let mut val = controller.get_register(Register::A).unwrap();
    val ^= controller.get_register(other).unwrap();
    controller.set_register(Register::A, val).unwrap();
    controller.update_flags_logical();
}

#[allow(dead_code)]
fn xri(controller : &mut Microcontroller) {
    controller.fetch();
    let mut val = controller.get_register(Register::A).unwrap();
    val ^= controller.instruction_register;
    controller.set_register(Register::A, val).unwrap();
    controller.update_flags_logical();
}

#[allow(dead_code)]
fn inr(controller: &mut Microcontroller, reg: Register) {
    let val = controller.get_register(reg).unwrap();
    let new_val = val.add(1);
    let c = val == 255;
    controller.set_register(reg, new_val).unwrap();
    controller.set_flag(Flag::Zero, new_val == 0);
    controller.set_flag(Flag::Carry, c);
    controller.set_flag(Flag::Sign, new_val > 127);
    controller.set_flag(Flag::Parity, Microcontroller::check_parity(new_val));
}

#[allow(dead_code)]
fn dcr(controller: &mut Microcontroller, reg: Register) {
    let val = controller.get_register(reg).unwrap();
    let new_val = val.sub(1);
    let c = val == 0;
    controller.set_register(reg, new_val).unwrap();
    controller.set_flag(Flag::Zero, new_val == 0);
    controller.set_flag(Flag::Carry, c);
    controller.set_flag(Flag::Sign, new_val > 127);
    controller.set_flag(Flag::Parity, Microcontroller::check_parity(new_val));
}

#[allow(dead_code)]
fn inx(controller: &mut Microcontroller, reg: Register) {
    let val = controller.get_register_pair(reg).unwrap();
    let new_val = val.add(1);
    let c = val == 65535;
    controller.set_register_pair(reg, new_val).unwrap();
    controller.set_flag(Flag::Zero, new_val == 0);
    controller.set_flag(Flag::Carry, c);
    controller.set_flag(Flag::Sign, new_val > 32768);
    controller.set_flag(Flag::Parity, Microcontroller::check_parity_16(new_val));
}

#[allow(dead_code)]
fn dcx(controller: &mut Microcontroller, reg: Register) {
    let val = controller.get_register_pair(reg).unwrap();
    let new_val = val.sub(1);
    let c = val == 65535;
    controller.set_register_pair(reg, new_val).unwrap();
    controller.set_flag(Flag::Zero, new_val == 0);
    controller.set_flag(Flag::Carry, c);
    controller.set_flag(Flag::Sign, new_val > 32768);
    controller.set_flag(Flag::Parity, Microcontroller::check_parity_16(new_val));
}

#[allow(dead_code)]
fn lxi(controller: &mut Microcontroller, reg: Register) {
    let low = controller.fetch();
    let high = controller.fetch();
    let val = (high as u16) << 8 | low as u16;
    controller.set_register_pair(reg, val).unwrap();
}

#[allow(dead_code)]
fn lda(controller: &mut Microcontroller) {
    let low = controller.fetch();
    let high = controller.fetch();
    let addr = (high as u16) << 8 | low as u16;
    controller.set_register(Register::A, controller.get_data_at(Some(addr))).unwrap();
}

#[allow(dead_code)]
fn ldax(controller: &mut Microcontroller, reg: Register) {
    let addr = controller.get_register_pair(reg).unwrap();
    controller.set_register(Register::A, controller.get_data_at(Some(addr))).unwrap();
}

#[allow(dead_code)]
fn lhld(controller: &mut Microcontroller) {
    let low = controller.fetch();
    let high = controller.fetch();
    let addr = (high as u16) << 8 | low as u16;
    controller.set_register(Register::L, controller.get_data_at(Some(addr))).unwrap();
    controller.set_register(Register::H, controller.get_data_at(Some(addr.add(1)))).unwrap();
}

#[allow(dead_code)]
fn sta(controller: &mut Microcontroller) {
    let low = controller.fetch();
    let high = controller.fetch();
    let addr = (high as u16) << 8 | low as u16;
    controller.set_data_at(Some(addr), controller.get_register(Register::A).unwrap());
}

#[allow(dead_code)]
fn stax(controller: &mut Microcontroller, reg: Register) {
    let addr = controller.get_register_pair(reg).unwrap();
    controller.set_data_at(Some(addr), controller.get_register(Register::A).unwrap());
}

#[allow(dead_code)]
fn shld(controller: &mut Microcontroller) {
    let low = controller.fetch();
    let high = controller.fetch();
    let addr = (high as u16) << 8 | low as u16;
    controller.set_data_at(Some(addr), controller.get_register(Register::L).unwrap());
    controller.set_data_at(Some(addr.add(1)), controller.get_register(Register::H).unwrap());
}

#[allow(dead_code)]
fn call(controller: &mut Microcontroller, skip: bool) {
    if skip {
        controller.fetch();
        controller.fetch();
    }
    let stp = controller.get_register_pair(Register::SP).unwrap();
    controller.set_register_pair(Register::SP, stp.add(2)).unwrap();
    controller.set_data_at(Some(stp), (controller.program_counter >> 8) as u8);
    controller.set_data_at(Some(stp.sub(1)), (controller.program_counter << 8 >> 8) as u8);
    let dest = (controller.fetch() as u16) << 8 | controller.fetch() as u16;
    controller.program_counter = dest;
}

#[allow(dead_code)]
fn ret(controller : &mut Microcontroller) {
    let addr = controller.get_register_pair(Register::SP).unwrap();
    let pc = ((controller.get_data_at(Some(addr.add(1))) as u16) << 8)
                | controller.get_data_at(Some(addr)) as u16;
    controller.program_counter = pc;
    controller.set_register_pair(Register::SP, addr + 2).unwrap();
}

#[allow(dead_code)]
fn jmp(controller: &mut Microcontroller, skip: bool) {
    let low = controller.fetch();
    let high = controller.fetch();
    let addr = (high as u16) << 8 | low as u16;
    if !skip {
        controller.program_counter = addr;
    }
}

#[allow(dead_code)]
fn sphl(controller: &mut Microcontroller) {
    controller.set_register_pair(
        Register::SP,
        controller.get_register_pair(Register::H).unwrap()
    ).unwrap();
}

#[allow(dead_code)]
fn pchl(controller: &mut Microcontroller) {
    controller.program_counter = controller.get_register_pair(Register::H).unwrap();
}

#[allow(dead_code)]
fn xchg(controller: &mut Microcontroller) {
    let d = controller.get_register_pair(Register::D).unwrap();
    let h = controller.get_register_pair(Register::H).unwrap();
    controller.set_register_pair(Register::H, d).unwrap();
    controller.set_register_pair(Register::D, h).unwrap();
}

#[allow(dead_code)]
fn rar(controller: &mut Microcontroller) {
    let mut val = controller.get_register(Register::A).unwrap();
    let carry = val & 0b00000001 == 0b00000001;
    let prev_carry = controller.check_flag(Flag::Carry);
    controller.set_flag(Flag::Carry, carry);
    val >>= 1;
    val |= match prev_carry {
        true => 0b10000000,
        false => 0b00000000
    };
    controller.set_register(Register::A, val).unwrap();
}

#[allow(dead_code)]
fn ral(controller: &mut Microcontroller) {
    let mut val = controller.get_register(Register::A).unwrap();
    let carry = val & 0b10000000 == 0b10000000;
    let prev_carry = controller.check_flag(Flag::Carry);
    controller.set_flag(Flag::Carry, carry);
    val <<= 1;
    val |= match prev_carry {
        true => 0b00000001,
        false => 0b00000000
    };
    controller.set_register(Register::A, val).unwrap();
}

#[allow(dead_code)]
fn reset(controller: &mut Microcontroller, x: u8) {
    let stp = controller.get_register_pair(Register::SP).unwrap();
    controller.set_register_pair(Register::SP, stp.add(2)).unwrap();
    controller.set_data_at(Some(stp.sub(1)), (controller.program_counter << 8 >> 8) as u8);
    controller.set_data_at(Some(stp), (controller.program_counter >> 8) as u8);
    controller.program_counter = (x * 8) as u16;
}

#[allow(dead_code)]
fn pop(controller: &mut Microcontroller, reg: Register) {
    let addr = controller.get_register_pair(Register::SP).unwrap();
    let val = ((controller.get_data_at(Some(addr.add(1))) as u16) << 8) 
                + controller.get_data_at(Some(addr)) as u16;
    controller.set_register_pair(Register::SP, addr + 2).unwrap();
    controller.set_register_pair(reg, val).unwrap();
}

#[allow(dead_code)]
fn push(controller: &mut Microcontroller, reg: Register) {
    let addr = controller.get_register_pair(Register::SP).unwrap();
    controller.set_register_pair(Register::SP, addr - 2).unwrap();
    let value = controller.get_register_pair(reg).unwrap();
    controller.set_data_at(Some(addr.sub(1)), (value >> 8) as u8);
    controller.set_data_at(Some(addr), (value << 8 >> 8) as u8);
}

#[allow(dead_code)]
pub fn xthl(controller : &mut Microcontroller) {
    let addr = controller.get_register_pair(Register::SP).unwrap();
    let xl = controller.get_register(Register::L).unwrap();
    let xh = controller.get_register(Register::H).unwrap();
    let l = controller.get_data_at(Some(addr));
    let h = controller.get_data_at(Some(addr.add(1)));
    controller.set_register(Register::H, h).unwrap();
    controller.set_register(Register::L, l).unwrap();
    controller.set_data_at(Some(addr), xh);
    controller.set_data_at(Some(addr.add(1)), xl);
}

#[allow(dead_code)]
fn rlc(controller : &mut Microcontroller) {
    let mut val = controller.get_register(Register::A).unwrap();
    let carry = val & 0b10000000 == 0b10000000;
    controller.set_flag(Flag::Carry, carry);
    val <<= 1;
    val |= match carry {
        true => 0b00000001,
        false => 0b00000000
    };
    controller.set_register(Register::A, val).unwrap();
}

#[allow(dead_code)]
fn rrc(controller : &mut Microcontroller) {
    let mut val = controller.get_register(Register::A).unwrap();
    let carry = val & 0b00000001 == 0b00000001;
    controller.set_flag(Flag::Carry, carry);
    val >>= 1;
    val |= match carry {
        true => 0b10000000,
        false => 0b00000000
    };
    controller.set_register(Register::A, val).unwrap();
}

#[allow(dead_code)]
fn daa(controller : &mut Microcontroller) {
    if controller.check_flag(Flag::AuxCarry) {
        _add(controller, 6, 0);
    }
    if controller.check_flag(Flag::Carry) {
        _add(controller, 0b01100000, 0);
    }
}

#[allow(dead_code)]
fn rim(_controller: &mut Microcontroller) {
    // TODO: Implement
}

#[allow(dead_code)]
fn sim(_controller: &mut Microcontroller) {
    // TODO: Implement
}

#[allow(dead_code)]
fn output(controller: &mut Microcontroller) {
    let addr = (controller.fetch() as u16) << 8 | controller.fetch() as u16;
    let val = controller.get_register(Register::A).unwrap();
    controller.write_io(addr, val);
}

#[allow(dead_code)]
fn input(controller: &mut Microcontroller) {
    let addr = (controller.fetch() as u16) << 8 | controller.fetch() as u16;
    let val = controller.read_io(addr);
    controller.set_register(Register::A, val).unwrap();
}

#[allow(dead_code)]
pub type Instruction = fn(&mut Microcontroller);

#[allow(dead_code)]
pub static SUB_A: Instruction = |controller| sub(controller, Register::A, 0);
#[allow(dead_code)]
pub static SUB_B: Instruction = |controller| sub(controller, Register::B, 0);
#[allow(dead_code)]
pub static SUB_C: Instruction = |controller| sub(controller, Register::C, 0);
#[allow(dead_code)]
pub static SUB_D: Instruction = |controller| sub(controller, Register::D, 0);
#[allow(dead_code)]
pub static SUB_E: Instruction = |controller| sub(controller, Register::E, 0);
#[allow(dead_code)]
pub static SUB_H: Instruction = |controller| sub(controller, Register::H, 0);
#[allow(dead_code)]
pub static SUB_L: Instruction = |controller| sub(controller, Register::L, 0);
#[allow(dead_code)]
pub static SUB_M: Instruction = |controller| sub(controller, Register::M, 0);
#[allow(dead_code)]
pub static SBB_A: Instruction = |controller| sub(controller, Register::A, 1);
#[allow(dead_code)]
pub static SBB_B: Instruction = |controller| sub(controller, Register::B, 1);
#[allow(dead_code)]
pub static SBB_C: Instruction = |controller| sub(controller, Register::C, 1);
#[allow(dead_code)]
pub static SBB_D: Instruction = |controller| sub(controller, Register::D, 1);
#[allow(dead_code)]
pub static SBB_E: Instruction = |controller| sub(controller, Register::E, 1);
#[allow(dead_code)]
pub static SBB_H: Instruction = |controller| sub(controller, Register::H, 1);
#[allow(dead_code)]
pub static SBB_L: Instruction = |controller| sub(controller, Register::L, 1);
#[allow(dead_code)]
pub static SBB_M: Instruction = |controller| sub(controller, Register::M, 1);
#[allow(dead_code)]
pub static ADD_A: Instruction = |controller| add(controller, Register::A);
#[allow(dead_code)]
pub static ADD_B: Instruction = |controller| add(controller, Register::B);
#[allow(dead_code)]
pub static ADD_C: Instruction = |controller| add(controller, Register::C);
#[allow(dead_code)]
pub static ADD_D: Instruction = |controller| add(controller, Register::D);
#[allow(dead_code)]
pub static ADD_E: Instruction = |controller| add(controller, Register::E);
#[allow(dead_code)]
pub static ADD_H: Instruction = |controller| add(controller, Register::H);
#[allow(dead_code)]
pub static ADD_L: Instruction = |controller| add(controller, Register::L);
#[allow(dead_code)]
pub static ADD_M: Instruction = |controller| add(controller, Register::M);
#[allow(dead_code)]
pub static ADC_A: Instruction = |controller| adc(controller, Register::A);
#[allow(dead_code)]
pub static ADC_B: Instruction = |controller| adc(controller, Register::B);
#[allow(dead_code)]
pub static ADC_C: Instruction = |controller| adc(controller, Register::C);
#[allow(dead_code)]
pub static ADC_D: Instruction = |controller| adc(controller, Register::D);
#[allow(dead_code)]
pub static ADC_E: Instruction = |controller| adc(controller, Register::E);
#[allow(dead_code)]
pub static ADC_H: Instruction = |controller| adc(controller, Register::H);
#[allow(dead_code)]
pub static ADC_L: Instruction = |controller| adc(controller, Register::L);
#[allow(dead_code)]
pub static ADC_M: Instruction = |controller| adc(controller, Register::M);
#[allow(dead_code)]
pub static DAD_B: Instruction = |controller| dadd(controller, Register::B);
#[allow(dead_code)]
pub static DAD_D: Instruction = |controller| dadd(controller, Register::D);
#[allow(dead_code)]
pub static DAD_H: Instruction = |controller| dadd(controller, Register::H);
#[allow(dead_code)]
pub static DAD_SP: Instruction = |controller| dadd(controller, Register::SP);
#[allow(dead_code)]
pub static ADI: Instruction = |controller| adi(controller);
#[allow(dead_code)]
pub static ACI: Instruction = |controller| aci(controller);
#[allow(dead_code)]
pub static SBI: Instruction = |controller| sbi(controller);
#[allow(dead_code)]
pub static MOV_AA: Instruction = |controller| mov(controller, Register::A, Register::A);
#[allow(dead_code)]
pub static MOV_AB: Instruction = |controller| mov(controller, Register::A, Register::B);
#[allow(dead_code)]
pub static MOV_AC: Instruction = |controller| mov(controller, Register::A, Register::C);
#[allow(dead_code)]
pub static MOV_AD: Instruction = |controller| mov(controller, Register::A, Register::D);
#[allow(dead_code)]
pub static MOV_AE: Instruction = |controller| mov(controller, Register::A, Register::E);
#[allow(dead_code)]
pub static MOV_AH: Instruction = |controller| mov(controller, Register::A, Register::H);
#[allow(dead_code)]
pub static MOV_AL: Instruction = |controller| mov(controller, Register::A, Register::L);
#[allow(dead_code)]
pub static MOV_AM: Instruction = |controller| mov(controller, Register::A, Register::M);
#[allow(dead_code)]
pub static MOV_BA: Instruction = |controller| mov(controller, Register::B, Register::A);
#[allow(dead_code)]
pub static MOV_BB: Instruction = |controller| mov(controller, Register::B, Register::B);
#[allow(dead_code)]
pub static MOV_BC: Instruction = |controller| mov(controller, Register::B, Register::C);
#[allow(dead_code)]
pub static MOV_BD: Instruction = |controller| mov(controller, Register::B, Register::D);
#[allow(dead_code)]
pub static MOV_BE: Instruction = |controller| mov(controller, Register::B, Register::E);
#[allow(dead_code)]
pub static MOV_BH: Instruction = |controller| mov(controller, Register::B, Register::H);
#[allow(dead_code)]
pub static MOV_BL: Instruction = |controller| mov(controller, Register::B, Register::L);
#[allow(dead_code)]
pub static MOV_BM: Instruction = |controller| mov(controller, Register::B, Register::M);
#[allow(dead_code)]
pub static MOV_CA: Instruction = |controller| mov(controller, Register::C, Register::A);
#[allow(dead_code)]
pub static MOV_CB: Instruction = |controller| mov(controller, Register::C, Register::B);
#[allow(dead_code)]
pub static MOV_CC: Instruction = |controller| mov(controller, Register::C, Register::C);
#[allow(dead_code)]
pub static MOV_CD: Instruction = |controller| mov(controller, Register::C, Register::D);
#[allow(dead_code)]
pub static MOV_CE: Instruction = |controller| mov(controller, Register::C, Register::E);
#[allow(dead_code)]
pub static MOV_CH: Instruction = |controller| mov(controller, Register::C, Register::H);
#[allow(dead_code)]
pub static MOV_CL: Instruction = |controller| mov(controller, Register::C, Register::L);
#[allow(dead_code)]
pub static MOV_CM: Instruction = |controller| mov(controller, Register::C, Register::M);
#[allow(dead_code)]
pub static MOV_DA: Instruction = |controller| mov(controller, Register::D, Register::A);
#[allow(dead_code)]
pub static MOV_DB: Instruction = |controller| mov(controller, Register::D, Register::B);
#[allow(dead_code)]
pub static MOV_DC: Instruction = |controller| mov(controller, Register::D, Register::C);
#[allow(dead_code)]
pub static MOV_DD: Instruction = |controller| mov(controller, Register::D, Register::D);
#[allow(dead_code)]
pub static MOV_DE: Instruction = |controller| mov(controller, Register::D, Register::E);
#[allow(dead_code)]
pub static MOV_DH: Instruction = |controller| mov(controller, Register::D, Register::H);
#[allow(dead_code)]
pub static MOV_DL: Instruction = |controller| mov(controller, Register::D, Register::L);
#[allow(dead_code)]
pub static MOV_DM: Instruction = |controller| mov(controller, Register::D, Register::M);
#[allow(dead_code)]
pub static MOV_EA: Instruction = |controller| mov(controller, Register::E, Register::A);
#[allow(dead_code)]
pub static MOV_EB: Instruction = |controller| mov(controller, Register::E, Register::B);
#[allow(dead_code)]
pub static MOV_EC: Instruction = |controller| mov(controller, Register::E, Register::C);
#[allow(dead_code)]
pub static MOV_ED: Instruction = |controller| mov(controller, Register::E, Register::D);
#[allow(dead_code)]
pub static MOV_EE: Instruction = |controller| mov(controller, Register::E, Register::E);
#[allow(dead_code)]
pub static MOV_EH: Instruction = |controller| mov(controller, Register::E, Register::H);
#[allow(dead_code)]
pub static MOV_EL: Instruction = |controller| mov(controller, Register::E, Register::L);
#[allow(dead_code)]
pub static MOV_EM: Instruction = |controller| mov(controller, Register::E, Register::M);
#[allow(dead_code)]
pub static MOV_HA: Instruction = |controller| mov(controller, Register::H, Register::A);
#[allow(dead_code)]
pub static MOV_HB: Instruction = |controller| mov(controller, Register::H, Register::B);
#[allow(dead_code)]
pub static MOV_HC: Instruction = |controller| mov(controller, Register::H, Register::C);
#[allow(dead_code)]
pub static MOV_HD: Instruction = |controller| mov(controller, Register::H, Register::D);
#[allow(dead_code)]
pub static MOV_HE: Instruction = |controller| mov(controller, Register::H, Register::E);
#[allow(dead_code)]
pub static MOV_HH: Instruction = |controller| mov(controller, Register::H, Register::H);
#[allow(dead_code)]
pub static MOV_HL: Instruction = |controller| mov(controller, Register::H, Register::L);
#[allow(dead_code)]
pub static MOV_HM: Instruction = |controller| mov(controller, Register::H, Register::M);
#[allow(dead_code)]
pub static MOV_LA: Instruction = |controller| mov(controller, Register::L, Register::A);
#[allow(dead_code)]
pub static MOV_LB: Instruction = |controller| mov(controller, Register::L, Register::B);
#[allow(dead_code)]
pub static MOV_LC: Instruction = |controller| mov(controller, Register::L, Register::C);
#[allow(dead_code)]
pub static MOV_LD: Instruction = |controller| mov(controller, Register::L, Register::D);
#[allow(dead_code)]
pub static MOV_LE: Instruction = |controller| mov(controller, Register::L, Register::E);
#[allow(dead_code)]
pub static MOV_LH: Instruction = |controller| mov(controller, Register::L, Register::H);
#[allow(dead_code)]
pub static MOV_LL: Instruction = |controller| mov(controller, Register::L, Register::L);
#[allow(dead_code)]
pub static MOV_LM: Instruction = |controller| mov(controller, Register::L, Register::M);
#[allow(dead_code)]
pub static MOV_MA: Instruction = |controller| mov(controller, Register::M, Register::A);
#[allow(dead_code)]
pub static MOV_MB: Instruction = |controller| mov(controller, Register::M, Register::B);
#[allow(dead_code)]
pub static MOV_MC: Instruction = |controller| mov(controller, Register::M, Register::C);
#[allow(dead_code)]
pub static MOV_MD: Instruction = |controller| mov(controller, Register::M, Register::D);
#[allow(dead_code)]
pub static MOV_ME: Instruction = |controller| mov(controller, Register::M, Register::E);
#[allow(dead_code)]
pub static MOV_MH: Instruction = |controller| mov(controller, Register::M, Register::H);
#[allow(dead_code)]
pub static MOV_ML: Instruction = |controller| mov(controller, Register::M, Register::L);
#[allow(dead_code)]
pub static MVI_A: Instruction = |controller| mvi(controller, Register::A);
#[allow(dead_code)]
pub static MVI_B: Instruction = |controller| mvi(controller, Register::B);
#[allow(dead_code)]
pub static MVI_C: Instruction = |controller| mvi(controller, Register::C);
#[allow(dead_code)]
pub static MVI_D: Instruction = |controller| mvi(controller, Register::D);
#[allow(dead_code)]
pub static MVI_E: Instruction = |controller| mvi(controller, Register::E);
#[allow(dead_code)]
pub static MVI_H: Instruction = |controller| mvi(controller, Register::H);
#[allow(dead_code)]
pub static MVI_L: Instruction = |controller| mvi(controller, Register::L);
#[allow(dead_code)]
pub static MVI_M: Instruction = |controller| mvi(controller, Register::M);
#[allow(dead_code)]
pub static HLT: Instruction = |controller| controller.stop();
#[allow(dead_code)]
pub static CMA: Instruction = |controller| {
    controller.set_register(Register::A, !controller.get_register(Register::A).unwrap()).unwrap();
};
#[allow(dead_code)]
pub static CMC: Instruction = |controller| {
    controller.set_flag(Flag::Carry, !controller.check_flag(Flag::Carry));
};
#[allow(dead_code)]
pub static ORA_A: Instruction = |controller| ora(controller, Register::A);
#[allow(dead_code)]
pub static ORA_B: Instruction = |controller| ora(controller, Register::B);
#[allow(dead_code)]
pub static ORA_C: Instruction = |controller| ora(controller, Register::C);
#[allow(dead_code)]
pub static ORA_D: Instruction = |controller| ora(controller, Register::D);
#[allow(dead_code)]
pub static ORA_E: Instruction = |controller| ora(controller, Register::E);
#[allow(dead_code)]
pub static ORA_H: Instruction = |controller| ora(controller, Register::H);
#[allow(dead_code)]
pub static ORA_L: Instruction = |controller| ora(controller, Register::L);
#[allow(dead_code)]
pub static ORA_M: Instruction = |controller| ora(controller, Register::M);
#[allow(dead_code)]
pub static ORI: Instruction = |controller| ori(controller);
#[allow(dead_code)]
pub static XRA_A: Instruction = |controller| xra(controller, Register::A);
#[allow(dead_code)]
pub static XRA_B: Instruction = |controller| xra(controller, Register::B);
#[allow(dead_code)]
pub static XRA_C: Instruction = |controller| xra(controller, Register::C);
#[allow(dead_code)]
pub static XRA_D: Instruction = |controller| xra(controller, Register::D);
#[allow(dead_code)]
pub static XRA_E: Instruction = |controller| xra(controller, Register::E);
#[allow(dead_code)]
pub static XRA_H: Instruction = |controller| xra(controller, Register::H);
#[allow(dead_code)]
pub static XRA_L: Instruction = |controller| xra(controller, Register::L);
#[allow(dead_code)]
pub static XRA_M: Instruction = |controller| xra(controller, Register::M);
#[allow(dead_code)]
pub static XRI: Instruction = |controller| xri(controller);
#[allow(dead_code)]
pub static ANA_A: Instruction = |controller| ana(controller, Register::A);
#[allow(dead_code)]
pub static ANA_B: Instruction = |controller| ana(controller, Register::B);
#[allow(dead_code)]
pub static ANA_C: Instruction = |controller| ana(controller, Register::C);
#[allow(dead_code)]
pub static ANA_D: Instruction = |controller| ana(controller, Register::D);
#[allow(dead_code)]
pub static ANA_E: Instruction = |controller| ana(controller, Register::E);
#[allow(dead_code)]
pub static ANA_H: Instruction = |controller| ana(controller, Register::H);
#[allow(dead_code)]
pub static ANA_L: Instruction = |controller| ana(controller, Register::L);
#[allow(dead_code)]
pub static ANA_M: Instruction = |controller| ana(controller, Register::M);
#[allow(dead_code)]
pub static ANI: Instruction = |controller| ani(controller);
#[allow(dead_code)]
pub static CMP_A: Instruction = |controller| cmp(controller, Register::A);
#[allow(dead_code)]
pub static CMP_B: Instruction = |controller| cmp(controller, Register::B);
#[allow(dead_code)]
pub static CMP_C: Instruction = |controller| cmp(controller, Register::C);
#[allow(dead_code)]
pub static CMP_D: Instruction = |controller| cmp(controller, Register::D);
#[allow(dead_code)]
pub static CMP_E: Instruction = |controller| cmp(controller, Register::E);
#[allow(dead_code)]
pub static CMP_H: Instruction = |controller| cmp(controller, Register::H);
#[allow(dead_code)]
pub static CMP_L: Instruction = |controller| cmp(controller, Register::L);
#[allow(dead_code)]
pub static CMP_M: Instruction = |controller| cmp(controller, Register::M);
#[allow(dead_code)]
pub static INR_A: Instruction = |controller| inr(controller, Register::A);
#[allow(dead_code)]
pub static INR_B: Instruction = |controller| inr(controller, Register::B);
#[allow(dead_code)]
pub static INR_C: Instruction = |controller| inr(controller, Register::C);
#[allow(dead_code)]
pub static INR_D: Instruction = |controller| inr(controller, Register::D);
#[allow(dead_code)]
pub static INR_E: Instruction = |controller| inr(controller, Register::E);
#[allow(dead_code)]
pub static INR_H: Instruction = |controller| inr(controller, Register::H);
#[allow(dead_code)]
pub static INR_L: Instruction = |controller| inr(controller, Register::L);
#[allow(dead_code)]
pub static INR_M: Instruction = |controller| inr(controller, Register::M);
#[allow(dead_code)]
pub static INX_B: Instruction = |controller| inx(controller, Register::B);
#[allow(dead_code)]
pub static INX_D: Instruction = |controller| inx(controller, Register::D);
#[allow(dead_code)]
pub static INX_H: Instruction = |controller| inx(controller, Register::H);
#[allow(dead_code)]
pub static INX_SP: Instruction = |controller| inx(controller, Register::SP);
#[allow(dead_code)]
pub static DCR_A: Instruction = |controller| dcr(controller, Register::A);
#[allow(dead_code)]
pub static DCR_B: Instruction = |controller| dcr(controller, Register::B);
#[allow(dead_code)]
pub static DCR_C: Instruction = |controller| dcr(controller, Register::C);
#[allow(dead_code)]
pub static DCR_D: Instruction = |controller| dcr(controller, Register::D);
#[allow(dead_code)]
pub static DCR_E: Instruction = |controller| dcr(controller, Register::E);
#[allow(dead_code)]
pub static DCR_H: Instruction = |controller| dcr(controller, Register::H);
#[allow(dead_code)]
pub static DCR_L: Instruction = |controller| dcr(controller, Register::L);
#[allow(dead_code)]
pub static DCR_M: Instruction = |controller| dcr(controller, Register::M);
#[allow(dead_code)]
pub static DCX_B: Instruction = |controller| dcx(controller, Register::B);
#[allow(dead_code)]
pub static DCX_D: Instruction = |controller| dcx(controller, Register::D);
#[allow(dead_code)]
pub static DCX_H: Instruction = |controller| dcx(controller, Register::H);
#[allow(dead_code)]
pub static DCX_SP: Instruction = |controller| dcx(controller, Register::SP);
#[allow(dead_code)]
pub static LDA: Instruction = |controller| lda(controller);
#[allow(dead_code)]
pub static LHLD: Instruction = |controller| lhld(controller);
#[allow(dead_code)]
pub static STA: Instruction = |controller| sta(controller);
#[allow(dead_code)]
pub static STAX_B: Instruction = |controller| stax(controller, Register::B);
#[allow(dead_code)]
pub static STAX_D: Instruction = |controller| stax(controller, Register::D);
#[allow(dead_code)]
pub static SHLD: Instruction = |controller| shld(controller);
#[allow(dead_code)]
pub static LXI_B: Instruction = |controller| lxi(controller, Register::B);
#[allow(dead_code)]
pub static LXI_D: Instruction = |controller| lxi(controller, Register::D);
#[allow(dead_code)]
pub static LXI_H: Instruction = |controller| lxi(controller, Register::H);
#[allow(dead_code)]
pub static LXI_SP: Instruction = |controller| lxi(controller, Register::SP);
#[allow(dead_code)]
pub static LDAX_B: Instruction = |controller| lxi(controller, Register::B);
#[allow(dead_code)]
pub static LDAX_D: Instruction = |controller| lxi(controller, Register::D);
#[allow(dead_code)]
pub static STC: Instruction = |controller| controller.set_flag(Flag::Carry, true);
#[allow(dead_code)]
pub static JMP: Instruction = |controller| jmp(controller, false);
#[allow(dead_code)]
pub static JP: Instruction = |controller| jmp(controller, controller.check_flag(Flag::Sign));
#[allow(dead_code)]
pub static JM: Instruction = |controller| jmp(controller, !controller.check_flag(Flag::Sign));
#[allow(dead_code)]
pub static JC: Instruction = |controller| jmp(controller, !controller.check_flag(Flag::Carry));
#[allow(dead_code)]
pub static JNC: Instruction = |controller| jmp(controller, controller.check_flag(Flag::Carry));
#[allow(dead_code)]
pub static JZ: Instruction = |controller| jmp(controller, !controller.check_flag(Flag::Zero));
#[allow(dead_code)]
pub static JNZ: Instruction = |controller| jmp(controller, controller.check_flag(Flag::Zero));
#[allow(dead_code)]
pub static JPO: Instruction = |controller| jmp(controller, controller.check_flag(Flag::Parity));
#[allow(dead_code)]
pub static JPE: Instruction = |controller| jmp(controller, !controller.check_flag(Flag::Parity));
#[allow(dead_code)]
pub static CALL: Instruction = |controller| call(controller, false);
#[allow(dead_code)]
pub static CM: Instruction = |controller| {
    if controller.check_flag(Flag::Sign) { call(controller, false); }
    else { call(controller, true) }
};
#[allow(dead_code)]
pub static CP: Instruction = |controller| {
    if controller.check_flag(Flag::Sign) { call(controller, false); }
    else { call(controller, true) }
};
#[allow(dead_code)]
pub static CC: Instruction = |controller| {
    if controller.check_flag(Flag::Carry) { call(controller, false); }
    else { call(controller, true) }
};
#[allow(dead_code)]
pub static CNC: Instruction = |controller| {
    if !controller.check_flag(Flag::Zero) { call(controller, false); }
    else { call(controller, true) }
};
#[allow(dead_code)]
pub static CZ: Instruction = |controller| {
    if controller.check_flag(Flag::Zero) { call(controller, false); }
    else { call(controller, true) }
};
#[allow(dead_code)] pub static CNZ: Instruction = |controller| {
    if !controller.check_flag(Flag::Carry) { call(controller, false); }
    else { call(controller, true) }
};
#[allow(dead_code)]
pub static CPE: Instruction = |controller| {
    if controller.check_flag(Flag::Parity) { call(controller, false); }
    else { call(controller, true) }
};
#[allow(dead_code)]
pub static CPO: Instruction = |controller| {
    if !controller.check_flag(Flag::Parity) { call(controller, false); }
    else { call(controller, true) }
};
#[allow(dead_code)]
pub static RET: Instruction = |controller| ret(controller);
#[allow(dead_code)]
pub static RP: Instruction = |controller| {
    if !controller.check_flag(Flag::Sign) { ret(controller); }
};
#[allow(dead_code)]
pub static RM: Instruction = |controller| {
    if controller.check_flag(Flag::Sign) { ret(controller); }
};
#[allow(dead_code)]
pub static RC: Instruction = |controller| {
    if controller.check_flag(Flag::Carry) { ret(controller); }
};
#[allow(dead_code)]
pub static RNC: Instruction = |controller| {
    if !controller.check_flag(Flag::Carry) { ret(controller); }
};
#[allow(dead_code)]
pub static RZ: Instruction = |controller| {
    if controller.check_flag(Flag::Zero) { ret(controller); }
};
#[allow(dead_code)]
pub static RNZ: Instruction = |controller| {
    if !controller.check_flag(Flag::Zero) { ret(controller); }
};
#[allow(dead_code)]
pub static RPE: Instruction = |controller| {
    if controller.check_flag(Flag::Parity) { ret(controller); }
};
#[allow(dead_code)]
pub static RPO: Instruction = |controller| {
    if !controller.check_flag(Flag::Parity) { ret(controller); }
};
#[allow(dead_code)]
pub static RST_0: Instruction = |controller| reset(controller, 0);
#[allow(dead_code)]
pub static RST_1: Instruction = |controller| reset(controller, 1);
#[allow(dead_code)]
pub static RST_2: Instruction = |controller| reset(controller, 2);
#[allow(dead_code)]
pub static RST_3: Instruction = |controller| reset(controller, 3);
#[allow(dead_code)]
pub static RST_4: Instruction = |controller| reset(controller, 4);
#[allow(dead_code)]
pub static RST_5: Instruction = |controller| reset(controller, 5);
#[allow(dead_code)]
pub static RST_6: Instruction = |controller| reset(controller, 6);
#[allow(dead_code)]
pub static RAL: Instruction = |controller| ral(controller);
#[allow(dead_code)]
pub static RAR: Instruction = |controller| rar(controller);
#[allow(dead_code)]
pub static PUSH_B: Instruction = |controller| push(controller, Register::B);
#[allow(dead_code)]
pub static PUSH_D: Instruction = |controller| push(controller, Register::D);
#[allow(dead_code)]
pub static PUSH_H: Instruction = |controller| push(controller, Register::H);
#[allow(dead_code)]
pub static PUSH_PSW: Instruction = |controller| push(controller, Register::PSW);
#[allow(dead_code)]
pub static POP_B: Instruction = |controller| pop(controller, Register::B);
#[allow(dead_code)]
pub static POP_D: Instruction = |controller| pop(controller, Register::D);
#[allow(dead_code)]
pub static POP_H: Instruction = |controller| pop(controller, Register::H);
#[allow(dead_code)]
pub static POP_PSW: Instruction = |controller| pop(controller, Register::PSW);
#[allow(dead_code)]
pub static SPHL: Instruction = |controller| sphl(controller);
#[allow(dead_code)]
pub static PCHL: Instruction = |controller| pchl(controller);
#[allow(dead_code)]
pub static XCHG: Instruction = |controller| xchg(controller);
#[allow(dead_code)]
pub static XTHL: Instruction = |controller| xthl(controller);
#[allow(dead_code)]
pub static RLC: Instruction = |controller| rlc(controller);
#[allow(dead_code)]
pub static RRC: Instruction = |controller| rrc(controller);
#[allow(dead_code)]
pub static RIM: Instruction = |controller| rim(controller);
#[allow(dead_code)]
pub static SIM: Instruction = |controller| sim(controller);
#[allow(dead_code)]
pub static INPUT: Instruction = |controller| input(controller);
#[allow(dead_code)]
pub static OUTPUT: Instruction = |controller| output(controller);
#[allow(dead_code)]
pub static DI: Instruction = |controller| controller.disable_interrupts();
#[allow(dead_code)]
pub static EI: Instruction = |controller| controller.enable_interrupts();
#[allow(dead_code)]
pub static SUI: Instruction = |controller| sbi(controller);
#[allow(dead_code)]
pub static CPI: Instruction = |controller| cpi(controller);
#[allow(dead_code)]
pub static DAA: Instruction = |controller| daa(controller);
#[allow(dead_code)]
pub static NOOP: Instruction = |_| {};
