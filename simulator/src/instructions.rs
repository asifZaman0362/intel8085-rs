use crate::simulator::Register;
use crate::simulator::Microcontroller;
use crate::simulator::Flag;

trait Arith<T> {
    fn sub(&self, other: T) -> T;
    fn add(&self, other: T) -> T;
}

impl Arith<u8> for u8 {
    fn sub(&self, other: Self) -> Self {
        (*self as u16 + (!other + 1) as u16) as u8
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
    let ac = (((a & 0b00001111) + c) + (other & 0b00001111)) > 0b00001001;
    let c = (a as u16 + other as u16 + c as u16) > 255;
    controller.set_register(Register::A, sum).unwrap();
    controller.update_flags(ac, c);
}

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

fn add(controller: &mut Microcontroller, reg: Register) {
    let b = controller.get_register(reg).unwrap();
    _add(controller, b, 0);
}

fn adc(controller: &mut Microcontroller, reg: Register) {
    let b = controller.get_register(reg).unwrap();
    _add(controller, b, 1);
}

fn adi(controller : &mut Microcontroller) {
    controller.fetch();
    let b = controller.instruction_register;
    _add(controller, b, 0);
}

fn aci(controller : &mut Microcontroller) {
    controller.fetch();
    let b = controller.instruction_register;
    _add(controller, b, 1);
}

fn _sub(controller : &mut Microcontroller, other: u8, carry: u8) {
    let a = controller.get_register(Register::A).unwrap();
    let c = controller.check_flag(Flag::Carry) as u8 * carry;
    let sum = a.sub(other).sub(c);
    let ac = (((a & 0b00001111) - c) - (other & 0b00001111)) > 0b00001001;
    let c = (a as i32 - other as i32 - c as i32) < 0;
    controller.set_register(Register::A, sum).unwrap();
    controller.update_flags(ac, c);
}

fn sub(controller: &mut Microcontroller, reg: Register, carry: u8) {
    let b = controller.get_register(reg).unwrap();
    _sub(controller, b, carry);
}

fn sbi(controller : &mut Microcontroller) {
    controller.fetch();
    let b = controller.instruction_register;
    _sub(controller, b, 1);
}

fn cmp(controller : &mut Microcontroller, reg: Register) {
    let a = controller.get_register(Register::A).unwrap();
    sub(controller, reg, 0);
    controller.set_register(Register::A, a).unwrap();
}

fn cpi(controller : &mut Microcontroller) {
    let a = controller.get_register(Register::A).unwrap();
    sbi(controller);
    controller.set_register(Register::A, a).unwrap();
}

fn mov(controller: &mut Microcontroller, to: Register, from: Register) {
    let data = controller.get_register(from).unwrap();
    controller.set_register(to, data).unwrap();
}

fn mvi(controller: &mut Microcontroller, to: Register) {
    controller.fetch();
    let data = controller.instruction_register;
    controller.set_register(to, data).unwrap();
}

fn ora(controller : &mut Microcontroller, other : Register) {
    let mut val = controller.get_register(Register::A).unwrap();
    val |= controller.get_register(other).unwrap();
    controller.set_register(Register::A, val).unwrap();
    controller.update_flags_logical();
}

fn ori(controller : &mut Microcontroller) {
    let mut val = controller.get_register(Register::A).unwrap();
    controller.fetch();
    val |= controller.instruction_register;
    controller.set_register(Register::A, val).unwrap();
    controller.update_flags_logical();
}

fn ana(controller : &mut Microcontroller, other : Register) {
    let mut val = controller.get_register(Register::A).unwrap();
    val &= controller.get_register(other).unwrap();
    controller.set_register(Register::A, val).unwrap();
    controller.update_flags_logical();
}

fn ani(controller : &mut Microcontroller) {
    let mut val = controller.get_register(Register::A).unwrap();
    controller.fetch();
    val &= controller.instruction_register;
    controller.set_register(Register::A, val).unwrap();
    controller.update_flags_logical();
}

fn xra(controller : &mut Microcontroller, other : Register) {
    let mut val = controller.get_register(Register::A).unwrap();
    val ^= controller.get_register(other).unwrap();
    controller.set_register(Register::A, val).unwrap();
    controller.update_flags_logical();
}

fn xri(controller : &mut Microcontroller) {
    controller.fetch();
    let mut val = controller.get_register(Register::A).unwrap();
    val ^= controller.instruction_register;
    controller.set_register(Register::A, val).unwrap();
    controller.update_flags_logical();
}

fn inr(controller: &mut Microcontroller, reg: Register) {
    let val = controller.get_register(reg).unwrap();
    let new_val = val.add(1);
    let c = val == 255;
    controller.set_register(reg, new_val);
    controller.set_flag(Flag::Zero, new_val == 0);
    controller.set_flag(Flag::Carry, c);
    controller.set_flag(Flag::Sign, new_val > 127);
    controller.set_flag(Flag::Parity, Microcontroller::check_parity(new_val));
}

fn dcr(controller: &mut Microcontroller, reg: Register) {
    let val = controller.get_register(reg).unwrap();
    let new_val = val.sub(1);
    let c = val == 255;
    controller.set_register(reg, new_val);
    controller.set_flag(Flag::Zero, new_val == 0);
    controller.set_flag(Flag::Carry, c);
    controller.set_flag(Flag::Sign, new_val > 127);
    controller.set_flag(Flag::Parity, Microcontroller::check_parity(new_val));
}

fn inx(controller: &mut Microcontroller, reg: Register) {
    let val = controller.get_register_pair(reg).unwrap();
    let new_val = val.add(1);
    let c = val == 65535;
    controller.set_register_pair(reg, new_val);
    controller.set_flag(Flag::Zero, new_val == 0);
    controller.set_flag(Flag::Carry, c);
    controller.set_flag(Flag::Sign, new_val > 32768);
    controller.set_flag(Flag::Parity, Microcontroller::check_parity_16(new_val));
}

fn dcx(controller: &mut Microcontroller, reg: Register) {
    let val = controller.get_register_pair(reg).unwrap();
    let new_val = val.sub(1);
    let c = val == 65535;
    controller.set_register_pair(reg, new_val);
    controller.set_flag(Flag::Zero, new_val == 0);
    controller.set_flag(Flag::Carry, c);
    controller.set_flag(Flag::Sign, new_val > 32768);
    controller.set_flag(Flag::Parity, Microcontroller::check_parity_16(new_val));
}

fn lxi(controller: &mut Microcontroller, reg: Register) {
    let high = controller.fetch();
    let low = controller.fetch();
    let val = (high as u16) << 8 | low as u16;
    controller.set_register_pair(reg, val);
}

fn lda(controller: &mut Microcontroller) {
    let high = controller.fetch();
    let low = controller.fetch();
    let addr = (high as u16) << 8 | low as u16;
    controller.set_register(Register::A, controller.get_data_at(Some(addr)));
}

fn ldax(controller: &mut Microcontroller, reg: Register) {
    let addr = controller.get_register_pair(reg).unwrap();
    controller.set_register(Register::A, controller.get_data_at(Some(addr)));
}

fn lhld(controller: &mut Microcontroller) {
    let high = controller.fetch();
    let low = controller.fetch();
    let addr = (high as u16) << 8 | low as u16;
    controller.set_register(Register::L, controller.get_data_at(Some(addr)));
    controller.set_register(Register::H, controller.get_data_at(Some(addr.add(1))));
}

fn sta(controller: &mut Microcontroller) {
    let high = controller.fetch();
    let low = controller.fetch();
    let addr = (high as u16) << 8 | low as u16;
    controller.set_data_at(Some(addr), controller.get_register(Register::A).unwrap());
}

fn stax(controller: &mut Microcontroller, reg: Register) {
    let addr = controller.get_register_pair(reg).unwrap();
    controller.set_data_at(Some(addr), controller.get_register(Register::A).unwrap());
}

fn shld(controller: &mut Microcontroller) {
    let high = controller.fetch();
    let low = controller.fetch();
    let addr = (high as u16) << 8 | low as u16;
    controller.set_data_at(Some(addr), controller.get_register(Register::L).unwrap());
    controller.set_data_at(Some(addr.add(1)), controller.get_register(Register::H).unwrap());
}

pub type Instruction = fn(&mut Microcontroller);

pub static SUB_A: Instruction = |controller| sub(controller, Register::A, 0);
pub static SUB_B: Instruction = |controller| sub(controller, Register::B, 0);
pub static SUB_C: Instruction = |controller| sub(controller, Register::C, 0);
pub static SUB_D: Instruction = |controller| sub(controller, Register::D, 0);
pub static SUB_E: Instruction = |controller| sub(controller, Register::E, 0);
pub static SUB_H: Instruction = |controller| sub(controller, Register::H, 0);
pub static SUB_L: Instruction = |controller| sub(controller, Register::L, 0);
pub static SUB_M: Instruction = |controller| sub(controller, Register::M, 0);
pub static SBB_A: Instruction = |controller| sub(controller, Register::A, 1);
pub static SBB_B: Instruction = |controller| sub(controller, Register::B, 1);
pub static SBB_C: Instruction = |controller| sub(controller, Register::C, 1);
pub static SBB_D: Instruction = |controller| sub(controller, Register::D, 1);
pub static SBB_E: Instruction = |controller| sub(controller, Register::E, 1);
pub static SBB_H: Instruction = |controller| sub(controller, Register::H, 1);
pub static SBB_L: Instruction = |controller| sub(controller, Register::L, 1);
pub static SBB_M: Instruction = |controller| sub(controller, Register::M, 1);
pub static ADD_A: Instruction = |controller| add(controller, Register::A);
pub static ADD_B: Instruction = |controller| add(controller, Register::B);
pub static ADD_C: Instruction = |controller| add(controller, Register::C);
pub static ADD_D: Instruction = |controller| add(controller, Register::D);
pub static ADD_E: Instruction = |controller| add(controller, Register::E);
pub static ADD_H: Instruction = |controller| add(controller, Register::H);
pub static ADD_L: Instruction = |controller| add(controller, Register::L);
pub static ADD_M: Instruction = |controller| add(controller, Register::M);
pub static ADC_A: Instruction = |controller| adc(controller, Register::A);
pub static ADC_B: Instruction = |controller| adc(controller, Register::B);
pub static ADC_C: Instruction = |controller| adc(controller, Register::C);
pub static ADC_D: Instruction = |controller| adc(controller, Register::D);
pub static ADC_E: Instruction = |controller| adc(controller, Register::E);
pub static ADC_H: Instruction = |controller| adc(controller, Register::H);
pub static ADC_L: Instruction = |controller| adc(controller, Register::L);
pub static ADC_M: Instruction = |controller| adc(controller, Register::M);
pub static DAD_B: Instruction = |controller| dadd(controller, Register::B);
pub static DAD_D: Instruction = |controller| dadd(controller, Register::D);
pub static DAD_H: Instruction = |controller| dadd(controller, Register::H);
pub static DAD_SP: Instruction = |controller| dadd(controller, Register::SP);
pub static ADI: Instruction = |controller| adi(controller);
pub static ACI: Instruction = |controller| aci(controller);
pub static SBI: Instruction = |controller| sbi(controller);
pub static MOV_AA: Instruction = |controller| mov(controller, Register::A, Register::A);
pub static MOV_AB: Instruction = |controller| mov(controller, Register::A, Register::B);
pub static MOV_AC: Instruction = |controller| mov(controller, Register::A, Register::C);
pub static MOV_AD: Instruction = |controller| mov(controller, Register::A, Register::D);
pub static MOV_AE: Instruction = |controller| mov(controller, Register::A, Register::E);
pub static MOV_AH: Instruction = |controller| mov(controller, Register::A, Register::H);
pub static MOV_AL: Instruction = |controller| mov(controller, Register::A, Register::L);
pub static MOV_AM: Instruction = |controller| mov(controller, Register::A, Register::M);
pub static MOV_BA: Instruction = |controller| mov(controller, Register::B, Register::A);
pub static MOV_BB: Instruction = |controller| mov(controller, Register::B, Register::B);
pub static MOV_BC: Instruction = |controller| mov(controller, Register::B, Register::C);
pub static MOV_BD: Instruction = |controller| mov(controller, Register::B, Register::D);
pub static MOV_BE: Instruction = |controller| mov(controller, Register::B, Register::E);
pub static MOV_BH: Instruction = |controller| mov(controller, Register::B, Register::H);
pub static MOV_BL: Instruction = |controller| mov(controller, Register::B, Register::L);
pub static MOV_BM: Instruction = |controller| mov(controller, Register::B, Register::M);
pub static MOV_CA: Instruction = |controller| mov(controller, Register::C, Register::A);
pub static MOV_CB: Instruction = |controller| mov(controller, Register::C, Register::B);
pub static MOV_CC: Instruction = |controller| mov(controller, Register::C, Register::C);
pub static MOV_CD: Instruction = |controller| mov(controller, Register::C, Register::D);
pub static MOV_CE: Instruction = |controller| mov(controller, Register::C, Register::E);
pub static MOV_CH: Instruction = |controller| mov(controller, Register::C, Register::H);
pub static MOV_CL: Instruction = |controller| mov(controller, Register::C, Register::L);
pub static MOV_CM: Instruction = |controller| mov(controller, Register::C, Register::M);
pub static MOV_DA: Instruction = |controller| mov(controller, Register::D, Register::A);
pub static MOV_DB: Instruction = |controller| mov(controller, Register::D, Register::B);
pub static MOV_DC: Instruction = |controller| mov(controller, Register::D, Register::C);
pub static MOV_DD: Instruction = |controller| mov(controller, Register::D, Register::D);
pub static MOV_DE: Instruction = |controller| mov(controller, Register::D, Register::E);
pub static MOV_DH: Instruction = |controller| mov(controller, Register::D, Register::H);
pub static MOV_DL: Instruction = |controller| mov(controller, Register::D, Register::L);
pub static MOV_DM: Instruction = |controller| mov(controller, Register::D, Register::M);
pub static MOV_EA: Instruction = |controller| mov(controller, Register::E, Register::A);
pub static MOV_EB: Instruction = |controller| mov(controller, Register::E, Register::B);
pub static MOV_EC: Instruction = |controller| mov(controller, Register::E, Register::C);
pub static MOV_ED: Instruction = |controller| mov(controller, Register::E, Register::D);
pub static MOV_EE: Instruction = |controller| mov(controller, Register::E, Register::E);
pub static MOV_EH: Instruction = |controller| mov(controller, Register::E, Register::H);
pub static MOV_EL: Instruction = |controller| mov(controller, Register::E, Register::L);
pub static MOV_EM: Instruction = |controller| mov(controller, Register::E, Register::M);
pub static MOV_HA: Instruction = |controller| mov(controller, Register::H, Register::A);
pub static MOV_HB: Instruction = |controller| mov(controller, Register::H, Register::B);
pub static MOV_HC: Instruction = |controller| mov(controller, Register::H, Register::C);
pub static MOV_HD: Instruction = |controller| mov(controller, Register::H, Register::D);
pub static MOV_HE: Instruction = |controller| mov(controller, Register::H, Register::E);
pub static MOV_HH: Instruction = |controller| mov(controller, Register::H, Register::H);
pub static MOV_HL: Instruction = |controller| mov(controller, Register::H, Register::L);
pub static MOV_HM: Instruction = |controller| mov(controller, Register::H, Register::M);
pub static MOV_LA: Instruction = |controller| mov(controller, Register::L, Register::A);
pub static MOV_LB: Instruction = |controller| mov(controller, Register::L, Register::B);
pub static MOV_LC: Instruction = |controller| mov(controller, Register::L, Register::C);
pub static MOV_LD: Instruction = |controller| mov(controller, Register::L, Register::D);
pub static MOV_LE: Instruction = |controller| mov(controller, Register::L, Register::E);
pub static MOV_LH: Instruction = |controller| mov(controller, Register::L, Register::H);
pub static MOV_LL: Instruction = |controller| mov(controller, Register::L, Register::L);
pub static MOV_LM: Instruction = |controller| mov(controller, Register::L, Register::M);
pub static MOV_MA: Instruction = |controller| mov(controller, Register::M, Register::A);
pub static MOV_MB: Instruction = |controller| mov(controller, Register::M, Register::B);
pub static MOV_MC: Instruction = |controller| mov(controller, Register::M, Register::C);
pub static MOV_MD: Instruction = |controller| mov(controller, Register::M, Register::D);
pub static MOV_ME: Instruction = |controller| mov(controller, Register::M, Register::E);
pub static MOV_MH: Instruction = |controller| mov(controller, Register::M, Register::H);
pub static MOV_ML: Instruction = |controller| mov(controller, Register::M, Register::L);
pub static MVI_A: Instruction = |controller| mvi(controller, Register::A);
pub static MVI_B: Instruction = |controller| mvi(controller, Register::B);
pub static MVI_C: Instruction = |controller| mvi(controller, Register::C);
pub static MVI_D: Instruction = |controller| mvi(controller, Register::D);
pub static MVI_E: Instruction = |controller| mvi(controller, Register::E);
pub static MVI_H: Instruction = |controller| mvi(controller, Register::H);
pub static MVI_L: Instruction = |controller| mvi(controller, Register::L);
pub static MVI_M: Instruction = |controller| mvi(controller, Register::M);
pub static HLT: Instruction = |controller| controller.stop();
pub static CMA: Instruction = |controller| {
    controller.set_register(Register::A, !controller.get_register(Register::A).unwrap());
};
pub static CMC: Instruction = |controller| {
    controller.set_flag(Flag::Carry, !controller.check_flag(Flag::Carry));
};
pub static ORA_A: Instruction = |controller| ora(controller, Register::A);
pub static ORA_B: Instruction = |controller| ora(controller, Register::B);
pub static ORA_C: Instruction = |controller| ora(controller, Register::C);
pub static ORA_D: Instruction = |controller| ora(controller, Register::D);
pub static ORA_E: Instruction = |controller| ora(controller, Register::E);
pub static ORA_H: Instruction = |controller| ora(controller, Register::H);
pub static ORA_L: Instruction = |controller| ora(controller, Register::L);
pub static ORA_M: Instruction = |controller| ora(controller, Register::M);
pub static ORI: Instruction = |controller| ori(controller);
pub static XRA_A: Instruction = |controller| xra(controller, Register::A);
pub static XRA_B: Instruction = |controller| xra(controller, Register::B);
pub static XRA_C: Instruction = |controller| xra(controller, Register::C);
pub static XRA_D: Instruction = |controller| xra(controller, Register::D);
pub static XRA_E: Instruction = |controller| xra(controller, Register::E);
pub static XRA_H: Instruction = |controller| xra(controller, Register::H);
pub static XRA_L: Instruction = |controller| xra(controller, Register::L);
pub static XRA_M: Instruction = |controller| xra(controller, Register::M);
pub static XRI: Instruction = |controller| xri(controller);
pub static ANA_A: Instruction = |controller| ana(controller, Register::A);
pub static ANA_B: Instruction = |controller| ana(controller, Register::B);
pub static ANA_C: Instruction = |controller| ana(controller, Register::C);
pub static ANA_D: Instruction = |controller| ana(controller, Register::D);
pub static ANA_E: Instruction = |controller| ana(controller, Register::E);
pub static ANA_H: Instruction = |controller| ana(controller, Register::H);
pub static ANA_L: Instruction = |controller| ana(controller, Register::L);
pub static ANA_M: Instruction = |controller| ana(controller, Register::M);
pub static ANI: Instruction = |controller| ani(controller);
pub static CMP_A: Instruction = |controller| cmp(controller, Register::A);
pub static CMP_B: Instruction = |controller| cmp(controller, Register::B);
pub static CMP_C: Instruction = |controller| cmp(controller, Register::C);
pub static CMP_D: Instruction = |controller| cmp(controller, Register::D);
pub static CMP_E: Instruction = |controller| cmp(controller, Register::E);
pub static CMP_H: Instruction = |controller| cmp(controller, Register::H);
pub static CMP_L: Instruction = |controller| cmp(controller, Register::L);
pub static CMP_M: Instruction = |controller| cmp(controller, Register::M);
pub static INR_A: Instruction = |controller| inr(controller, Register::A);
pub static INR_B: Instruction = |controller| inr(controller, Register::B);
pub static INR_C: Instruction = |controller| inr(controller, Register::C);
pub static INR_D: Instruction = |controller| inr(controller, Register::D);
pub static INR_E: Instruction = |controller| inr(controller, Register::E);
pub static INR_H: Instruction = |controller| inr(controller, Register::H);
pub static INR_L: Instruction = |controller| inr(controller, Register::L);
pub static INR_M: Instruction = |controller| inr(controller, Register::M);
pub static INX_B: Instruction = |controller| inx(controller, Register::B);
pub static INX_D: Instruction = |controller| inx(controller, Register::D);
pub static INX_H: Instruction = |controller| inx(controller, Register::H);
pub static INX_SP: Instruction = |controller| inx(controller, Register::SP);
pub static DCR_A: Instruction = |controller| inr(controller, Register::A);
pub static DCR_B: Instruction = |controller| inr(controller, Register::B);
pub static DCR_C: Instruction = |controller| inr(controller, Register::C);
pub static DCR_D: Instruction = |controller| inr(controller, Register::D);
pub static DCR_E: Instruction = |controller| inr(controller, Register::E);
pub static DCR_H: Instruction = |controller| inr(controller, Register::H);
pub static DCR_L: Instruction = |controller| inr(controller, Register::L);
pub static DCR_M: Instruction = |controller| inr(controller, Register::M);
pub static DCX_B: Instruction = |controller| inx(controller, Register::B);
pub static DCX_D: Instruction = |controller| inx(controller, Register::D);
pub static DCX_H: Instruction = |controller| inx(controller, Register::SP);
pub static DCX_SP: Instruction = |controller| inx(controller, Register::SP);
pub static LDA: Instruction = |controller| lda(controller);
pub static LHLD: Instruction = |controller| lhld(controller);
pub static STA: Instruction = |controller| sta(controller);
pub static STAX_B: Instruction = |controller| stax(controller, Register::B);
pub static STAX_D: Instruction = |controller| stax(controller, Register::D);
pub static SHLD: Instruction = |controller| shld(controller);
pub static LXI_B: Instruction = |controller| lxi(controller, Register::B);
pub static LXI_D: Instruction = |controller| lxi(controller, Register::D);
pub static LXI_H: Instruction = |controller| lxi(controller, Register::H);
pub static LXI_SP: Instruction = |controller| lxi(controller, Register::SP);
pub static LDAX_B: Instruction = |controller| lxi(controller, Register::B);
pub static LDAX_D: Instruction = |controller| lxi(controller, Register::D);
pub static STC: Instruction = |controller| controller.set_flag(Flag::Carry, true);
pub static NOOP: Instruction = |_| {};
