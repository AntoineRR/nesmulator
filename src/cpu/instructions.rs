// Implements the CPU instructions

// ===== IMPORTS =====

use super::enums::AdressingMode as am;
use super::cpu::CPU;

// ===== CPU INSTRUCTION STRUCT =====

pub struct CpuInstruction {
    pub name: &'static str,
    pub opcode: u8,
    pub execute: fn(&mut CPU, am),
    pub adressing_mode: am,
    pub cycles: u8,
    pub bytes: u8,
}

// ===== GLOBAL CONSTANT =====

// The whole set of instructions of the CPU.
// Some instructions are not valid, and some are not documented and not implemented here.
// The unimplemented instructions return an error with the ERR method.
// Their name is written in lower case
#[allow(dead_code)]
pub const INSTRUCTIONS: [CpuInstruction;256] =
[
    CpuInstruction {name: "BRK", opcode: 0x00, execute: CPU::brk, adressing_mode: am::Implicit, cycles: 7, bytes: 1},
    CpuInstruction {name: "ORA", opcode: 0x01, execute: CPU::ora, adressing_mode: am::IndirectX, cycles: 6, bytes: 2},
    CpuInstruction {name: "???", opcode: 0x02, execute: CPU::err, adressing_mode: am::NoMode, cycles: 1, bytes: 1},
    CpuInstruction {name: "slo", opcode: 0x03, execute: CPU::err, adressing_mode: am::IndirectX, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0x04, execute: CPU::err, adressing_mode: am::ZeroPage, cycles: 1, bytes: 1},
    CpuInstruction {name: "ORA", opcode: 0x05, execute: CPU::ora, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "ASL", opcode: 0x06, execute: CPU::asl, adressing_mode: am::ZeroPage, cycles: 5, bytes: 2},
    CpuInstruction {name: "slo", opcode: 0x07, execute: CPU::err, adressing_mode: am::ZeroPage, cycles: 1, bytes: 1},
    CpuInstruction {name: "PHP", opcode: 0x08, execute: CPU::php, adressing_mode: am::Implicit, cycles: 3, bytes: 1},
    CpuInstruction {name: "ORA", opcode: 0x09, execute: CPU::ora, adressing_mode: am::Immediate, cycles: 2, bytes: 2},
    CpuInstruction {name: "ASL", opcode: 0x0A, execute: CPU::asl, adressing_mode: am::Accumulator, cycles: 2, bytes: 1},
    CpuInstruction {name: "anc", opcode: 0x0B, execute: CPU::err, adressing_mode: am::Immediate, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0x0C, execute: CPU::err, adressing_mode: am::Absolute, cycles: 1, bytes: 1},
    CpuInstruction {name: "ORA", opcode: 0x0D, execute: CPU::ora, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "ASL", opcode: 0x0E, execute: CPU::asl, adressing_mode: am::Absolute, cycles: 6, bytes: 3},
    CpuInstruction {name: "slo", opcode: 0x0F, execute: CPU::err, adressing_mode: am::Absolute, cycles: 1, bytes: 1},

    CpuInstruction {name: "BPL", opcode: 0x10, execute: CPU::bpl, adressing_mode: am::Relative, cycles: 2, bytes: 2},
    CpuInstruction {name: "ORA", opcode: 0x11, execute: CPU::ora, adressing_mode: am::IndirectY, cycles: 5, bytes: 2},
    CpuInstruction {name: "???", opcode: 0x12, execute: CPU::err, adressing_mode: am::NoMode, cycles: 1, bytes: 1},
    CpuInstruction {name: "slo", opcode: 0x13, execute: CPU::err, adressing_mode: am::IndirectY, cycles: 1, bytes: 1},
    CpuInstruction {name: "slo", opcode: 0x14, execute: CPU::err, adressing_mode: am::ZeroPageX, cycles: 1, bytes: 1},
    CpuInstruction {name: "ORA", opcode: 0x15, execute: CPU::ora, adressing_mode: am::ZeroPageX, cycles: 4, bytes: 2},
    CpuInstruction {name: "ASL", opcode: 0x16, execute: CPU::asl, adressing_mode: am::ZeroPageX, cycles: 6, bytes: 2},
    CpuInstruction {name: "slo", opcode: 0x17, execute: CPU::err, adressing_mode: am::ZeroPageX, cycles: 1, bytes: 1},
    CpuInstruction {name: "CLC", opcode: 0x18, execute: CPU::clc, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "ORA", opcode: 0x19, execute: CPU::ora, adressing_mode: am::AbsoluteY, cycles: 4, bytes: 3},
    CpuInstruction {name: "nop", opcode: 0x1A, execute: CPU::err, adressing_mode: am::Implicit, cycles: 1, bytes: 1},
    CpuInstruction {name: "slo", opcode: 0x1B, execute: CPU::err, adressing_mode: am::AbsoluteY, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0x1C, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},
    CpuInstruction {name: "ORA", opcode: 0x1D, execute: CPU::ora, adressing_mode: am::AbsoluteX, cycles: 4, bytes: 3},
    CpuInstruction {name: "ASL", opcode: 0x1E, execute: CPU::asl, adressing_mode: am::AbsoluteX, cycles: 7, bytes: 3},
    CpuInstruction {name: "slo", opcode: 0x1F, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},

    CpuInstruction {name: "JSR", opcode: 0x20, execute: CPU::jsr, adressing_mode: am::Absolute, cycles: 6, bytes: 3},
    CpuInstruction {name: "AND", opcode: 0x21, execute: CPU::and, adressing_mode: am::IndirectX, cycles: 6, bytes: 2},
    CpuInstruction {name: "???", opcode: 0x22, execute: CPU::err, adressing_mode: am::NoMode, cycles: 1, bytes: 1},
    CpuInstruction {name: "rla", opcode: 0x23, execute: CPU::err, adressing_mode: am::IndirectX, cycles: 1, bytes: 1},
    CpuInstruction {name: "BIT", opcode: 0x24, execute: CPU::bit, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "AND", opcode: 0x25, execute: CPU::and, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "ROL", opcode: 0x26, execute: CPU::rol, adressing_mode: am::ZeroPage, cycles: 5, bytes: 2},
    CpuInstruction {name: "rla", opcode: 0x27, execute: CPU::err, adressing_mode: am::ZeroPage, cycles: 1, bytes: 1},
    CpuInstruction {name: "PLP", opcode: 0x28, execute: CPU::plp, adressing_mode: am::Implicit, cycles: 4, bytes: 1},
    CpuInstruction {name: "AND", opcode: 0x29, execute: CPU::and, adressing_mode: am::Immediate, cycles: 2, bytes: 2},
    CpuInstruction {name: "ROL", opcode: 0x2A, execute: CPU::rol, adressing_mode: am::Accumulator, cycles: 2, bytes: 1},
    CpuInstruction {name: "anc", opcode: 0x2B, execute: CPU::err, adressing_mode: am::Immediate, cycles: 1, bytes: 1},
    CpuInstruction {name: "BIT", opcode: 0x2C, execute: CPU::bit, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "AND", opcode: 0x2D, execute: CPU::and, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "ROL", opcode: 0x2E, execute: CPU::rol, adressing_mode: am::Absolute, cycles: 6, bytes: 3},
    CpuInstruction {name: "rla", opcode: 0x2F, execute: CPU::err, adressing_mode: am::Absolute, cycles: 1, bytes: 1},

    CpuInstruction {name: "BMI", opcode: 0x30, execute: CPU::bmi, adressing_mode: am::Relative, cycles: 2, bytes: 2},
    CpuInstruction {name: "AND", opcode: 0x31, execute: CPU::and, adressing_mode: am::IndirectY, cycles: 5, bytes: 2},
    CpuInstruction {name: "???", opcode: 0x32, execute: CPU::err, adressing_mode: am::NoMode, cycles: 1, bytes: 1},
    CpuInstruction {name: "rla", opcode: 0x33, execute: CPU::err, adressing_mode: am::IndirectY, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0x34, execute: CPU::err, adressing_mode: am::ZeroPageX, cycles: 1, bytes: 1},
    CpuInstruction {name: "AND", opcode: 0x35, execute: CPU::and, adressing_mode: am::ZeroPageX, cycles: 4, bytes: 2},
    CpuInstruction {name: "ROL", opcode: 0x36, execute: CPU::rol, adressing_mode: am::ZeroPageX, cycles: 6, bytes: 2},
    CpuInstruction {name: "rla", opcode: 0x37, execute: CPU::err, adressing_mode: am::ZeroPageX, cycles: 1, bytes: 1},
    CpuInstruction {name: "SEC", opcode: 0x38, execute: CPU::sec, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "AND", opcode: 0x39, execute: CPU::and, adressing_mode: am::AbsoluteY, cycles: 4, bytes: 3},
    CpuInstruction {name: "nop", opcode: 0x3A, execute: CPU::err, adressing_mode: am::Implicit, cycles: 1, bytes: 1},
    CpuInstruction {name: "rla", opcode: 0x3B, execute: CPU::err, adressing_mode: am::AbsoluteY, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0x3C, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},
    CpuInstruction {name: "AND", opcode: 0x3D, execute: CPU::and, adressing_mode: am::AbsoluteX, cycles: 4, bytes: 3},
    CpuInstruction {name: "ROL", opcode: 0x3E, execute: CPU::rol, adressing_mode: am::AbsoluteX, cycles: 7, bytes: 3},
    CpuInstruction {name: "rla", opcode: 0x3F, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},

    CpuInstruction {name: "RTI", opcode: 0x40, execute: CPU::rti, adressing_mode: am::Implicit, cycles: 6, bytes: 1},
    CpuInstruction {name: "EOR", opcode: 0x41, execute: CPU::eor, adressing_mode: am::IndirectX, cycles: 6, bytes: 2},
    CpuInstruction {name: "???", opcode: 0x42, execute: CPU::err, adressing_mode: am::NoMode, cycles: 1, bytes: 1},
    CpuInstruction {name: "sre", opcode: 0x43, execute: CPU::err, adressing_mode: am::IndirectX, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0x44, execute: CPU::err, adressing_mode: am::ZeroPage, cycles: 1, bytes: 1},
    CpuInstruction {name: "EOR", opcode: 0x45, execute: CPU::eor, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "LSR", opcode: 0x46, execute: CPU::lsr, adressing_mode: am::ZeroPage, cycles: 5, bytes: 2},
    CpuInstruction {name: "sre", opcode: 0x47, execute: CPU::err, adressing_mode: am::ZeroPage, cycles: 1, bytes: 1},
    CpuInstruction {name: "PHA", opcode: 0x48, execute: CPU::pha, adressing_mode: am::Implicit, cycles: 3, bytes: 1},
    CpuInstruction {name: "EOR", opcode: 0x49, execute: CPU::eor, adressing_mode: am::Immediate, cycles: 2, bytes: 2},
    CpuInstruction {name: "LSR", opcode: 0x4A, execute: CPU::lsr, adressing_mode: am::Accumulator, cycles: 2, bytes: 1},
    CpuInstruction {name: "asr", opcode: 0x4B, execute: CPU::err, adressing_mode: am::Immediate, cycles: 1, bytes: 1},
    CpuInstruction {name: "JMP", opcode: 0x4C, execute: CPU::jmp, adressing_mode: am::Absolute, cycles: 3, bytes: 3},
    CpuInstruction {name: "EOR", opcode: 0x4D, execute: CPU::eor, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "LSR", opcode: 0x4E, execute: CPU::lsr, adressing_mode: am::Absolute, cycles: 6, bytes: 3},
    CpuInstruction {name: "sre", opcode: 0x4F, execute: CPU::err, adressing_mode: am::Absolute, cycles: 1, bytes: 1},

    CpuInstruction {name: "BVC", opcode: 0x50, execute: CPU::bvc, adressing_mode: am::Relative, cycles: 2, bytes: 2},
    CpuInstruction {name: "EOR", opcode: 0x51, execute: CPU::eor, adressing_mode: am::IndirectY, cycles: 5, bytes: 2},
    CpuInstruction {name: "???", opcode: 0x52, execute: CPU::err, adressing_mode: am::NoMode, cycles: 1, bytes: 1},
    CpuInstruction {name: "sre", opcode: 0x53, execute: CPU::err, adressing_mode: am::IndirectY, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0x54, execute: CPU::err, adressing_mode: am::ZeroPageX, cycles: 1, bytes: 1},
    CpuInstruction {name: "EOR", opcode: 0x55, execute: CPU::eor, adressing_mode: am::ZeroPageX, cycles: 4, bytes: 2},
    CpuInstruction {name: "LSR", opcode: 0x56, execute: CPU::lsr, adressing_mode: am::ZeroPageX, cycles: 6, bytes: 2},
    CpuInstruction {name: "sre", opcode: 0x57, execute: CPU::err, adressing_mode: am::ZeroPageX, cycles: 1, bytes: 1},
    CpuInstruction {name: "CLI", opcode: 0x58, execute: CPU::cli, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "EOR", opcode: 0x59, execute: CPU::eor, adressing_mode: am::AbsoluteY, cycles: 4, bytes: 3},
    CpuInstruction {name: "nop", opcode: 0x5A, execute: CPU::err, adressing_mode: am::Implicit, cycles: 1, bytes: 1},
    CpuInstruction {name: "sre", opcode: 0x5B, execute: CPU::err, adressing_mode: am::AbsoluteY, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0x5C, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},
    CpuInstruction {name: "EOR", opcode: 0x5D, execute: CPU::eor, adressing_mode: am::AbsoluteX, cycles: 4, bytes: 3},
    CpuInstruction {name: "LSR", opcode: 0x5E, execute: CPU::lsr, adressing_mode: am::AbsoluteX, cycles: 7, bytes: 3},
    CpuInstruction {name: "sre", opcode: 0x5F, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},

    CpuInstruction {name: "RTS", opcode: 0x60, execute: CPU::rts, adressing_mode: am::Implicit, cycles: 6, bytes: 1},
    CpuInstruction {name: "ADC", opcode: 0x61, execute: CPU::adc, adressing_mode: am::IndirectX, cycles: 6, bytes: 2},
    CpuInstruction {name: "???", opcode: 0x62, execute: CPU::err, adressing_mode: am::NoMode, cycles: 1, bytes: 1},
    CpuInstruction {name: "rra", opcode: 0x63, execute: CPU::err, adressing_mode: am::IndirectX, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0x64, execute: CPU::err, adressing_mode: am::ZeroPage, cycles: 1, bytes: 1},
    CpuInstruction {name: "ADC", opcode: 0x65, execute: CPU::adc, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "ROR", opcode: 0x66, execute: CPU::ror, adressing_mode: am::ZeroPage, cycles: 5, bytes: 2},
    CpuInstruction {name: "rra", opcode: 0x67, execute: CPU::err, adressing_mode: am::ZeroPage, cycles: 1, bytes: 1},
    CpuInstruction {name: "PLA", opcode: 0x68, execute: CPU::pla, adressing_mode: am::Implicit, cycles: 4, bytes: 1},
    CpuInstruction {name: "ADC", opcode: 0x69, execute: CPU::adc, adressing_mode: am::Immediate, cycles: 2, bytes: 2},
    CpuInstruction {name: "ROR", opcode: 0x6A, execute: CPU::ror, adressing_mode: am::Accumulator, cycles: 2, bytes: 1},
    CpuInstruction {name: "arr", opcode: 0x6B, execute: CPU::err, adressing_mode: am::Immediate, cycles: 1, bytes: 1},
    CpuInstruction {name: "JMP", opcode: 0x6C, execute: CPU::jmp, adressing_mode: am::Indirect, cycles: 5, bytes: 3},
    CpuInstruction {name: "ADC", opcode: 0x6D, execute: CPU::adc, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "ROR", opcode: 0x6E, execute: CPU::ror, adressing_mode: am::Absolute, cycles: 6, bytes: 3},
    CpuInstruction {name: "rra", opcode: 0x6F, execute: CPU::err, adressing_mode: am::Absolute, cycles: 1, bytes: 1},

    CpuInstruction {name: "BVS", opcode: 0x70, execute: CPU::bvs, adressing_mode: am::Relative, cycles: 2, bytes: 2},
    CpuInstruction {name: "ADC", opcode: 0x71, execute: CPU::adc, adressing_mode: am::IndirectY, cycles: 5, bytes: 2},
    CpuInstruction {name: "???", opcode: 0x72, execute: CPU::err, adressing_mode: am::NoMode, cycles: 1, bytes: 1},
    CpuInstruction {name: "rra", opcode: 0x73, execute: CPU::err, adressing_mode: am::IndirectY, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0x74, execute: CPU::err, adressing_mode: am::ZeroPageX, cycles: 1, bytes: 1},
    CpuInstruction {name: "ADC", opcode: 0x75, execute: CPU::adc, adressing_mode: am::ZeroPageX, cycles: 4, bytes: 2},
    CpuInstruction {name: "ROR", opcode: 0x76, execute: CPU::ror, adressing_mode: am::ZeroPageX, cycles: 6, bytes: 2},
    CpuInstruction {name: "rra", opcode: 0x77, execute: CPU::err, adressing_mode: am::ZeroPageX, cycles: 1, bytes: 1},
    CpuInstruction {name: "SEI", opcode: 0x78, execute: CPU::sei, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "ADC", opcode: 0x79, execute: CPU::adc, adressing_mode: am::AbsoluteY, cycles: 4, bytes: 3},
    CpuInstruction {name: "nop", opcode: 0x7A, execute: CPU::err, adressing_mode: am::Implicit, cycles: 1, bytes: 1},
    CpuInstruction {name: "rra", opcode: 0x7B, execute: CPU::err, adressing_mode: am::AbsoluteY, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0x7C, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},
    CpuInstruction {name: "ADC", opcode: 0x7D, execute: CPU::adc, adressing_mode: am::AbsoluteX, cycles: 4, bytes: 3},
    CpuInstruction {name: "ROR", opcode: 0x7E, execute: CPU::ror, adressing_mode: am::AbsoluteX, cycles: 7, bytes: 3},
    CpuInstruction {name: "rra", opcode: 0x7F, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},

    CpuInstruction {name: "nop", opcode: 0x80, execute: CPU::err, adressing_mode: am::Implicit, cycles: 1, bytes: 1},
    CpuInstruction {name: "STA", opcode: 0x81, execute: CPU::sta, adressing_mode: am::IndirectX, cycles: 6, bytes: 2},
    CpuInstruction {name: "nop", opcode: 0x82, execute: CPU::err, adressing_mode: am::NoMode, cycles: 1, bytes: 1},
    CpuInstruction {name: "sax", opcode: 0x83, execute: CPU::err, adressing_mode: am::IndirectX, cycles: 1, bytes: 1},
    CpuInstruction {name: "STY", opcode: 0x84, execute: CPU::sty, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "STA", opcode: 0x85, execute: CPU::sta, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "STX", opcode: 0x86, execute: CPU::stx, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "sax", opcode: 0x87, execute: CPU::err, adressing_mode: am::ZeroPage, cycles: 1, bytes: 1},
    CpuInstruction {name: "DEY", opcode: 0x88, execute: CPU::dey, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0x89, execute: CPU::err, adressing_mode: am::Immediate, cycles: 1, bytes: 1},
    CpuInstruction {name: "TXA", opcode: 0x8A, execute: CPU::txa, adressing_mode: am::Accumulator, cycles: 2, bytes: 1},
    CpuInstruction {name: "ane", opcode: 0x8B, execute: CPU::err, adressing_mode: am::Immediate, cycles: 1, bytes: 1},
    CpuInstruction {name: "STY", opcode: 0x8C, execute: CPU::sty, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "STA", opcode: 0x8D, execute: CPU::sta, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "STX", opcode: 0x8E, execute: CPU::stx, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "sax", opcode: 0x8F, execute: CPU::err, adressing_mode: am::Absolute, cycles: 1, bytes: 1},

    CpuInstruction {name: "BCC", opcode: 0x90, execute: CPU::bcc, adressing_mode: am::Relative, cycles: 2, bytes: 2},
    CpuInstruction {name: "STA", opcode: 0x91, execute: CPU::sta, adressing_mode: am::IndirectY, cycles: 6, bytes: 2},
    CpuInstruction {name: "???", opcode: 0x92, execute: CPU::err, adressing_mode: am::NoMode, cycles: 1, bytes: 1},
    CpuInstruction {name: "sha", opcode: 0x93, execute: CPU::err, adressing_mode: am::IndirectY, cycles: 1, bytes: 1},
    CpuInstruction {name: "STY", opcode: 0x94, execute: CPU::sty, adressing_mode: am::ZeroPageX, cycles: 4, bytes: 2},
    CpuInstruction {name: "STA", opcode: 0x95, execute: CPU::sta, adressing_mode: am::ZeroPageX, cycles: 4, bytes: 2},
    CpuInstruction {name: "STX", opcode: 0x96, execute: CPU::stx, adressing_mode: am::ZeroPageY, cycles: 4, bytes: 2},
    CpuInstruction {name: "sax", opcode: 0x97, execute: CPU::err, adressing_mode: am::ZeroPageX, cycles: 1, bytes: 1},
    CpuInstruction {name: "TYA", opcode: 0x98, execute: CPU::tya, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "STA", opcode: 0x99, execute: CPU::sta, adressing_mode: am::AbsoluteY, cycles: 5, bytes: 3},
    CpuInstruction {name: "TXS", opcode: 0x9A, execute: CPU::txs, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "shs", opcode: 0x9B, execute: CPU::err, adressing_mode: am::AbsoluteY, cycles: 1, bytes: 1},
    CpuInstruction {name: "shy", opcode: 0x9C, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},
    CpuInstruction {name: "STA", opcode: 0x9D, execute: CPU::sta, adressing_mode: am::AbsoluteX, cycles: 5, bytes: 3},
    CpuInstruction {name: "shx", opcode: 0x9E, execute: CPU::asl, adressing_mode: am::AbsoluteX, cycles: 7, bytes: 3},
    CpuInstruction {name: "sha", opcode: 0x9F, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},

    CpuInstruction {name: "LDY", opcode: 0xA0, execute: CPU::ldy, adressing_mode: am::Immediate, cycles: 2, bytes: 2},
    CpuInstruction {name: "LDA", opcode: 0xA1, execute: CPU::lda, adressing_mode: am::IndirectX, cycles: 6, bytes: 2},
    CpuInstruction {name: "LDX", opcode: 0xA2, execute: CPU::ldx, adressing_mode: am::Immediate, cycles: 2, bytes: 2},
    CpuInstruction {name: "lax", opcode: 0xA3, execute: CPU::err, adressing_mode: am::IndirectX, cycles: 1, bytes: 1},
    CpuInstruction {name: "LDY", opcode: 0xA4, execute: CPU::ldy, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "LDA", opcode: 0xA5, execute: CPU::lda, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "LDX", opcode: 0xA6, execute: CPU::ldx, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "lax", opcode: 0xA7, execute: CPU::err, adressing_mode: am::ZeroPage, cycles: 1, bytes: 1},
    CpuInstruction {name: "TAY", opcode: 0xA8, execute: CPU::tay, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "LDA", opcode: 0xA9, execute: CPU::lda, adressing_mode: am::Immediate, cycles: 2, bytes: 2},
    CpuInstruction {name: "TAX", opcode: 0xAA, execute: CPU::tax, adressing_mode: am::Accumulator, cycles: 2, bytes: 1},
    CpuInstruction {name: "lxa", opcode: 0xAB, execute: CPU::err, adressing_mode: am::Immediate, cycles: 1, bytes: 1},
    CpuInstruction {name: "LDY", opcode: 0xAC, execute: CPU::ldy, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "LDA", opcode: 0xAD, execute: CPU::lda, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "LDX", opcode: 0xAE, execute: CPU::ldx, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "lax", opcode: 0xAF, execute: CPU::err, adressing_mode: am::Absolute, cycles: 1, bytes: 1},

    CpuInstruction {name: "BCS", opcode: 0xB0, execute: CPU::bcs, adressing_mode: am::Relative, cycles: 2, bytes: 2},
    CpuInstruction {name: "LDA", opcode: 0xB1, execute: CPU::lda, adressing_mode: am::IndirectY, cycles: 5, bytes: 2},
    CpuInstruction {name: "???", opcode: 0xB2, execute: CPU::err, adressing_mode: am::NoMode, cycles: 1, bytes: 1},
    CpuInstruction {name: "lax", opcode: 0xB3, execute: CPU::err, adressing_mode: am::IndirectY, cycles: 1, bytes: 1},
    CpuInstruction {name: "LDY", opcode: 0xB4, execute: CPU::ldy, adressing_mode: am::ZeroPageX, cycles: 4, bytes: 2},
    CpuInstruction {name: "LDA", opcode: 0xB5, execute: CPU::lda, adressing_mode: am::ZeroPageX, cycles: 4, bytes: 2},
    CpuInstruction {name: "LDX", opcode: 0xB6, execute: CPU::ldx, adressing_mode: am::ZeroPageY, cycles: 4, bytes: 2},
    CpuInstruction {name: "lax", opcode: 0xB7, execute: CPU::err, adressing_mode: am::ZeroPageY, cycles: 1, bytes: 1},
    CpuInstruction {name: "CLV", opcode: 0xB8, execute: CPU::clv, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "LDA", opcode: 0xB9, execute: CPU::lda, adressing_mode: am::AbsoluteY, cycles: 4, bytes: 3},
    CpuInstruction {name: "TSX", opcode: 0xBA, execute: CPU::tsx, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "las", opcode: 0xBB, execute: CPU::err, adressing_mode: am::AbsoluteY, cycles: 1, bytes: 1},
    CpuInstruction {name: "LDY", opcode: 0xBC, execute: CPU::ldy, adressing_mode: am::AbsoluteX, cycles: 4, bytes: 3},
    CpuInstruction {name: "LDA", opcode: 0xBD, execute: CPU::lda, adressing_mode: am::AbsoluteX, cycles: 4, bytes: 3},
    CpuInstruction {name: "LDX", opcode: 0xBE, execute: CPU::ldx, adressing_mode: am::AbsoluteY, cycles: 4, bytes: 3},
    CpuInstruction {name: "lax", opcode: 0xBF, execute: CPU::err, adressing_mode: am::AbsoluteY, cycles: 1, bytes: 1},

    CpuInstruction {name: "CPY", opcode: 0xC0, execute: CPU::cpy, adressing_mode: am::Immediate, cycles: 2, bytes: 2},
    CpuInstruction {name: "CMP", opcode: 0xC1, execute: CPU::cmp, adressing_mode: am::IndirectX, cycles: 6, bytes: 2},
    CpuInstruction {name: "nop", opcode: 0xC2, execute: CPU::err, adressing_mode: am::Immediate, cycles: 1, bytes: 1},
    CpuInstruction {name: "dcp", opcode: 0xC3, execute: CPU::err, adressing_mode: am::IndirectX, cycles: 1, bytes: 1},
    CpuInstruction {name: "CPY", opcode: 0xC4, execute: CPU::cpy, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "CMP", opcode: 0xC5, execute: CPU::cmp, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "DEC", opcode: 0xC6, execute: CPU::dec, adressing_mode: am::ZeroPage, cycles: 5, bytes: 2},
    CpuInstruction {name: "dcp", opcode: 0xC7, execute: CPU::err, adressing_mode: am::ZeroPage, cycles: 1, bytes: 1},
    CpuInstruction {name: "INY", opcode: 0xC8, execute: CPU::iny, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "CMP", opcode: 0xC9, execute: CPU::cmp, adressing_mode: am::Immediate, cycles: 2, bytes: 2},
    CpuInstruction {name: "DEX", opcode: 0xCA, execute: CPU::dex, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "sbx", opcode: 0xCB, execute: CPU::err, adressing_mode: am::Immediate, cycles: 1, bytes: 1},
    CpuInstruction {name: "CPY", opcode: 0xCC, execute: CPU::cpy, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "CMP", opcode: 0xCD, execute: CPU::cmp, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "DEC", opcode: 0xCE, execute: CPU::dec, adressing_mode: am::Absolute, cycles: 6, bytes: 3},
    CpuInstruction {name: "dcp", opcode: 0xCF, execute: CPU::err, adressing_mode: am::Absolute, cycles: 1, bytes: 1},

    CpuInstruction {name: "BNE", opcode: 0xD0, execute: CPU::bne, adressing_mode: am::Relative, cycles: 2, bytes: 2},
    CpuInstruction {name: "CMP", opcode: 0xD1, execute: CPU::cmp, adressing_mode: am::IndirectY, cycles: 5, bytes: 2},
    CpuInstruction {name: "???", opcode: 0xD2, execute: CPU::err, adressing_mode: am::NoMode, cycles: 1, bytes: 1},
    CpuInstruction {name: "dcp", opcode: 0xD3, execute: CPU::err, adressing_mode: am::IndirectY, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0xD4, execute: CPU::err, adressing_mode: am::ZeroPageX, cycles: 1, bytes: 1},
    CpuInstruction {name: "CMP", opcode: 0xD5, execute: CPU::cmp, adressing_mode: am::ZeroPageX, cycles: 4, bytes: 2},
    CpuInstruction {name: "DEC", opcode: 0xD6, execute: CPU::dec, adressing_mode: am::ZeroPageX, cycles: 6, bytes: 2},
    CpuInstruction {name: "dcp", opcode: 0xD7, execute: CPU::err, adressing_mode: am::ZeroPageX, cycles: 1, bytes: 1},
    CpuInstruction {name: "CLD", opcode: 0xD8, execute: CPU::cld, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "CMP", opcode: 0xD9, execute: CPU::cmp, adressing_mode: am::AbsoluteY, cycles: 4, bytes: 3},
    CpuInstruction {name: "nop", opcode: 0xDA, execute: CPU::err, adressing_mode: am::Implicit, cycles: 1, bytes: 1},
    CpuInstruction {name: "dcp", opcode: 0xDB, execute: CPU::err, adressing_mode: am::AbsoluteY, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0xDC, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},
    CpuInstruction {name: "CMP", opcode: 0xDD, execute: CPU::cmp, adressing_mode: am::AbsoluteX, cycles: 4, bytes: 3},
    CpuInstruction {name: "DEC", opcode: 0xDE, execute: CPU::dec, adressing_mode: am::AbsoluteX, cycles: 7, bytes: 3},
    CpuInstruction {name: "dcp", opcode: 0xDF, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},

    CpuInstruction {name: "CPX", opcode: 0xE0, execute: CPU::cpx, adressing_mode: am::Immediate, cycles: 2, bytes: 2},
    CpuInstruction {name: "SBC", opcode: 0xE1, execute: CPU::sbc, adressing_mode: am::IndirectX, cycles: 6, bytes: 2},
    CpuInstruction {name: "nop", opcode: 0xE2, execute: CPU::err, adressing_mode: am::Immediate, cycles: 1, bytes: 1},
    CpuInstruction {name: "isb", opcode: 0xE3, execute: CPU::err, adressing_mode: am::IndirectX, cycles: 1, bytes: 1},
    CpuInstruction {name: "CPX", opcode: 0xE4, execute: CPU::cpx, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "SBC", opcode: 0xE5, execute: CPU::sbc, adressing_mode: am::ZeroPage, cycles: 3, bytes: 2},
    CpuInstruction {name: "INC", opcode: 0xE6, execute: CPU::inc, adressing_mode: am::ZeroPage, cycles: 5, bytes: 2},
    CpuInstruction {name: "isb", opcode: 0xE7, execute: CPU::err, adressing_mode: am::ZeroPage, cycles: 1, bytes: 1},
    CpuInstruction {name: "INX", opcode: 0xE8, execute: CPU::inx, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "SBC", opcode: 0xE9, execute: CPU::sbc, adressing_mode: am::Immediate, cycles: 2, bytes: 2},
    CpuInstruction {name: "NOP", opcode: 0xEA, execute: CPU::nop, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "sbc", opcode: 0xEB, execute: CPU::err, adressing_mode: am::Immediate, cycles: 1, bytes: 1},
    CpuInstruction {name: "CPX", opcode: 0xEC, execute: CPU::cpx, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "SBC", opcode: 0xED, execute: CPU::sbc, adressing_mode: am::Absolute, cycles: 4, bytes: 3},
    CpuInstruction {name: "INC", opcode: 0xEE, execute: CPU::inc, adressing_mode: am::Absolute, cycles: 6, bytes: 3},
    CpuInstruction {name: "isb", opcode: 0xEF, execute: CPU::err, adressing_mode: am::Absolute, cycles: 1, bytes: 1},

    CpuInstruction {name: "BEQ", opcode: 0xF0, execute: CPU::beq, adressing_mode: am::Relative, cycles: 2, bytes: 2},
    CpuInstruction {name: "SBC", opcode: 0xF1, execute: CPU::sbc, adressing_mode: am::IndirectY, cycles: 5, bytes: 2},
    CpuInstruction {name: "???", opcode: 0xF2, execute: CPU::err, adressing_mode: am::NoMode, cycles: 1, bytes: 1},
    CpuInstruction {name: "isb", opcode: 0xF3, execute: CPU::err, adressing_mode: am::IndirectY, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0xF4, execute: CPU::err, adressing_mode: am::ZeroPageX, cycles: 1, bytes: 1},
    CpuInstruction {name: "SBC", opcode: 0xF5, execute: CPU::sbc, adressing_mode: am::ZeroPageX, cycles: 4, bytes: 2},
    CpuInstruction {name: "INC", opcode: 0xF6, execute: CPU::inc, adressing_mode: am::ZeroPageX, cycles: 6, bytes: 2},
    CpuInstruction {name: "isb", opcode: 0xF7, execute: CPU::err, adressing_mode: am::ZeroPageX, cycles: 1, bytes: 1},
    CpuInstruction {name: "SED", opcode: 0xF8, execute: CPU::sed, adressing_mode: am::Implicit, cycles: 2, bytes: 1},
    CpuInstruction {name: "SBC", opcode: 0xF9, execute: CPU::sbc, adressing_mode: am::AbsoluteY, cycles: 4, bytes: 3},
    CpuInstruction {name: "nop", opcode: 0xFA, execute: CPU::err, adressing_mode: am::Implicit, cycles: 1, bytes: 1},
    CpuInstruction {name: "isb", opcode: 0xFB, execute: CPU::err, adressing_mode: am::AbsoluteY, cycles: 1, bytes: 1},
    CpuInstruction {name: "nop", opcode: 0xFC, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},
    CpuInstruction {name: "SBC", opcode: 0xFD, execute: CPU::sbc, adressing_mode: am::AbsoluteX, cycles: 4, bytes: 3},
    CpuInstruction {name: "INC", opcode: 0xFE, execute: CPU::inc, adressing_mode: am::AbsoluteX, cycles: 7, bytes: 3},
    CpuInstruction {name: "isb", opcode: 0xFF, execute: CPU::err, adressing_mode: am::AbsoluteX, cycles: 1, bytes: 1},
];