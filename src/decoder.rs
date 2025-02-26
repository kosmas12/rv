pub type Rindex = usize;
pub type RDindex = Rindex;
pub type RS1index = Rindex;
pub type RS2index = Rindex;

pub type RS1value = u32;
pub type RS2value = u32;

type Iimmediate = u32;
type Simmediate = u32;
type Bimmediate = u32;
type Uimmediate = u32;
type Jimmediate = u32;

type Funct3 = u32;
type Funct7 = u32;

fn immediate_i(instruction: u32) -> Iimmediate {
    (instruction >> 20) as Iimmediate
}

fn immediate_s(instruction: u32) -> Simmediate {
    ((instruction >> 25) << 5) | ((instruction >> 7) & 0b1_1111) as Simmediate
}

fn immediate_b(instruction: u32) -> Bimmediate {
    let bits_4_1 = (instruction >> 8) & 0b1111;
    let bits_10_5 = (instruction >> 25) & 0b11_1111;
    let bits_11 = (instruction >> 7) & 0b1;
    let bits_12 = (instruction >> 31) & 0b1;
    ((bits_12 << 12) | (bits_11 << 11) | (bits_10_5 << 5) | (bits_4_1 << 1)) as Bimmediate
}

fn immediate_u(instruction: u32) -> Uimmediate {
    (instruction & 0b1111_1111_1111_1111_1111_0000_0000_0000) as Uimmediate
}

fn immediate_j(instruction: u32) -> Jimmediate {
    let bits_10_1 = (instruction >> 21) & 0b11_1111_1111;
    let bits_11 = (instruction >> 20) & 0b1;
    let bits_19_12 = (instruction >> 12) & 0b1111_1111;
    let bits_20 = (instruction >> 31) & 0b1;
    ((bits_20 << 20) | (bits_19_12 << 12) | (bits_11 << 11) | (bits_10_1 << 1)) as Jimmediate
}

fn rs1(instruction: u32) -> RS1index {
    ((instruction >> 15) & 0b1_1111) as RS1index
}

fn rs2(instruction: u32) -> RS2index {
    ((instruction >> 20) & 0b1_1111) as RS2index
}

fn rd(instruction: u32) -> RDindex {
    ((instruction >> 7) & 0b1_1111) as RDindex
}

fn funct3(instruction: u32) -> Funct3 {
    ((instruction >> 12) & 0b111) as Funct3
}

fn funct7(instruction: u32) -> Funct7 {
    ((instruction >> 25) & 0b111_1111) as Funct7
}

macro_rules! isBaseInstructionSet {
    ($inst:expr) => {
        ($inst & 0b11) == 0b11
    };
}

macro_rules! OpUpperBits {
    ($inst:expr) => {
        ($inst >> 5) & 0b11
    };
}

macro_rules! OpLowerBits {
    ($inst:expr) => {
        ($inst >> 2) & 0b111
    };
}

#[derive(Debug)]
pub enum OpCode {
    LOAD,
    LOADFP,
    CUSTOM0,
    MISCMEM,
    OPIMM,
    AUIPC,
    OPIMM32,
    LEN48,
    STORE,
    STOREFP,
    CUSTOM1,
    AMO,
    OP,
    LUI,
    OP32,
    LEN64,
    MADD,
    MSUB,
    NMSUB,
    NMADD,
    OPFP,
    RESERVED1,
    CUSTOM2,
    LEN482,
    BRANCH,
    JALR,
    RESERVED2,
    JAL,
    SYSTEM,
    RESERVED3,
    CUSTOM3,
    LEN80,
}

#[derive(Debug)]
pub enum Instruction {
    /* RV32I */
    LUI(RDindex, Uimmediate),
    AUIPC(RDindex, Uimmediate),
    JAL(RDindex, Jimmediate),
    JALR(RDindex, RS1index, Iimmediate),
    BEQ(RS1index, RS2index, Bimmediate),
    BNE(RS1index, RS2index, Bimmediate),
    BLT(RS1index, RS2index, Bimmediate),
    BGE(RS1index, RS2index, Bimmediate),
    BLTU(RS1index, RS2index, Bimmediate),
    BGEU(RS1index, RS2index, Bimmediate),
    LB(RDindex, RS1index, Iimmediate),
    LH(RDindex, RS1index, Iimmediate),
    LW(RDindex, RS1index, Iimmediate),
    LBU(RDindex, RS1index, Iimmediate),
    LHU(RDindex, RS1index, Iimmediate),
    SB(RS1index, RS2index, Simmediate),
    SH(RS1index, RS2index, Simmediate),
    SW(RS1index, RS2index, Simmediate),
    ADDI(RDindex, RS1index, Iimmediate),
    SLTI(RDindex, RS1index, Iimmediate),
    SLTIU(RDindex, RS1index, Iimmediate),
    XORI(RDindex, RS1index, Iimmediate),
    ORI(RDindex, RS1index, Iimmediate),
    ANDI(RDindex, RS1index, Iimmediate),
    SLLI(RDindex, RS1index, Iimmediate),
    SRLI(RDindex, RS1index, Iimmediate),
    SRAI(RDindex, RS1index, Iimmediate),
    ADD(RDindex, RS1index, RS2index),
    SUB(RDindex, RS1index, RS2index),
    SLL(RDindex, RS1index, RS2index),
    SLT(RDindex, RS1index, RS2index),
    SLTU(RDindex, RS1index, RS2index),
    XOR(RDindex, RS1index, RS2index),
    SRL(RDindex, RS1index, RS2index),
    SRA(RDindex, RS1index, RS2index),
    OR(RDindex, RS1index, RS2index),
    AND(RDindex, RS1index, RS2index),
    FENCE(RDindex, RS1index, Iimmediate),
    ECALL(),
    EBREAK(),
    MRET(),
    /* Zicsr */
    CSRRW(RDindex, RS1index, Iimmediate),
    CSRRS(RDindex, RS1index, Iimmediate),
    CSRRC(RDindex, RS1index, Iimmediate),
    CSRRWI(RDindex, RS1index, Iimmediate),
    CSRRSI(RDindex, RS1index, Iimmediate),
    CSRRCI(RDindex, RS1index, Iimmediate),
    /* M */
    MUL(RDindex, RS1index, RS2index),
    MULH(RDindex, RS1index, RS2index),
    MULHSU(RDindex, RS1index, RS2index),
    MULHU(RDindex, RS1index, RS2index),
    DIV(RDindex, RS1index, RS2index),
    DIVU(RDindex, RS1index, RS2index),
    REM(RDindex, RS1index, RS2index),
    REMU(RDindex, RS1index, RS2index),
}

impl Instruction {
    pub fn is_zicsr(&self) -> bool {
        matches!(
            self,
            Self::CSRRCI(..)
                | Self::CSRRW(..)
                | Self::CSRRS(..)
                | Self::CSRRC(..)
                | Self::CSRRWI(..)
                | Self::CSRRSI(..)
        )
    }
    pub fn is_m(&self) -> bool {
        matches!(
            self,
            Self::MUL(..)
                | Self::MULH(..)
                | Self::MULHSU(..)
                | Self::MULHU(..)
                | Self::DIV(..)
                | Self::DIVU(..)
                | Self::REM(..)
                | Self::REMU(..)
        )
    }
}

fn get_opcode(instruction: u32) -> Result<OpCode, &'static str> {
    match OpUpperBits!(instruction) {
        0b00 => match OpLowerBits!(instruction) {
            0b000 => Ok(OpCode::LOAD),
            0b001 => Ok(OpCode::LOADFP),
            0b010 => Ok(OpCode::CUSTOM0),
            0b011 => Ok(OpCode::MISCMEM),
            0b100 => Ok(OpCode::OPIMM),
            0b101 => Ok(OpCode::AUIPC),
            0b110 => Ok(OpCode::OPIMM32),
            0b111 => Ok(OpCode::LEN48),
            _ => Err("Shouldn't happen"),
        },
        0b01 => match OpLowerBits!(instruction) {
            0b000 => Ok(OpCode::STORE),
            0b001 => Ok(OpCode::STOREFP),
            0b010 => Ok(OpCode::CUSTOM1),
            0b011 => Ok(OpCode::AMO),
            0b100 => Ok(OpCode::OP),
            0b101 => Ok(OpCode::LUI),
            0b110 => Ok(OpCode::OP32),
            0b111 => Ok(OpCode::LEN64),
            _ => Err("Shouldn't happen"),
        },
        0b10 => match OpLowerBits!(instruction) {
            0b000 => Ok(OpCode::MADD),
            0b001 => Ok(OpCode::MSUB),
            0b010 => Ok(OpCode::NMSUB),
            0b011 => Ok(OpCode::NMADD),
            0b100 => Ok(OpCode::OPFP),
            0b101 => Ok(OpCode::RESERVED1),
            0b110 => Ok(OpCode::CUSTOM2),
            0b111 => Ok(OpCode::LEN482),
            _ => Err("Shouldn't happen"),
        },
        0b11 => match OpLowerBits!(instruction) {
            0b000 => Ok(OpCode::BRANCH),
            0b001 => Ok(OpCode::JALR),
            0b010 => Ok(OpCode::RESERVED2),
            0b011 => Ok(OpCode::JAL),
            0b100 => Ok(OpCode::SYSTEM),
            0b101 => Ok(OpCode::RESERVED3),
            0b110 => Ok(OpCode::CUSTOM3),
            0b111 => Ok(OpCode::LEN80),
            _ => Err("Shouldn't happen"),
        },
        _ => Err("Wrong upper bits"),
    }
}

#[allow(clippy::too_many_lines)]
pub fn decode(instruction: u32) -> Result<Instruction, &'static str> {
    if !isBaseInstructionSet!(instruction) {
        return Err("Invalid base instruction type");
    }
    let op = get_opcode(instruction)?;

    match op {
        OpCode::LOAD => {
            /* All LOAD are I-Type instructions */
            let rd_index: RDindex = rd(instruction);
            let rs1: RS1index = rs1(instruction);
            let i_imm: Iimmediate = immediate_i(instruction);
            match funct3(instruction) {
                0b000 => Ok(Instruction::LB(rd_index, rs1, i_imm)),
                0b001 => Ok(Instruction::LH(rd_index, rs1, i_imm)),
                0b010 => Ok(Instruction::LW(rd_index, rs1, i_imm)),
                0b100 => Ok(Instruction::LBU(rd_index, rs1, i_imm)),
                0b101 => Ok(Instruction::LHU(rd_index, rs1, i_imm)),
                _ => Err("Invalid funct3 I-Type"),
            }
        }
        OpCode::LOADFP => todo!(),
        OpCode::CUSTOM0 => todo!(),
        OpCode::MISCMEM => {
            let rd_index: RDindex = rd(instruction);
            let rs1: RS1index = rs1(instruction);
            let i_imm: Iimmediate = immediate_i(instruction);
            Ok(Instruction::FENCE(rd_index, rs1, i_imm))
        }
        OpCode::OPIMM => {
            /* All OPIMM are I-Type instructions */
            let rd_index: RDindex = rd(instruction);
            let rs1: RS1index = rs1(instruction);
            let i_imm: Iimmediate = immediate_i(instruction);
            match funct3(instruction) {
                0b000 => Ok(Instruction::ADDI(rd_index, rs1, i_imm)),
                0b010 => Ok(Instruction::SLTI(rd_index, rs1, i_imm)),
                0b011 => Ok(Instruction::SLTIU(rd_index, rs1, i_imm)),
                0b100 => Ok(Instruction::XORI(rd_index, rs1, i_imm)),
                0b110 => Ok(Instruction::ORI(rd_index, rs1, i_imm)),
                0b111 => Ok(Instruction::ANDI(rd_index, rs1, i_imm)),
                0b001 => Ok(Instruction::SLLI(rd_index, rs1, i_imm)),
                0b101 => {
                    if i_imm == 0 {
                        Ok(Instruction::SRLI(rd_index, rs1, i_imm))
                    } else {
                        Ok(Instruction::SRAI(rd_index, rs1, i_imm))
                    }
                }
                _ => Err("Invalid funct3 I-Type"),
            }
        }
        OpCode::AUIPC => {
            /* U Type */
            let rd_index: RDindex = rd(instruction);
            let u_imm: Uimmediate = immediate_u(instruction);
            Ok(Instruction::AUIPC(rd_index, u_imm))
        }
        OpCode::OPIMM32 => todo!(),
        OpCode::LEN48 => todo!(),
        OpCode::STORE => {
            /* STOREs are S-Type */
            let rs1: RS1index = rs1(instruction);
            let rs2: RS2index = rs2(instruction);
            let s_imm: Simmediate = immediate_s(instruction);
            match funct3(instruction) {
                0b000 => Ok(Instruction::SB(rs1, rs2, s_imm)),
                0b001 => Ok(Instruction::SH(rs1, rs2, s_imm)),
                0b010 => Ok(Instruction::SW(rs1, rs2, s_imm)),
                _ => Err("Invalid funct3 S-Type"),
            }
        }
        OpCode::STOREFP => todo!(),
        OpCode::CUSTOM1 => todo!(),
        OpCode::AMO => todo!(),
        OpCode::OP => {
            /* All OP are R-Type instructions */
            let rd_index: RDindex = rd(instruction);
            let rs1: RS1index = rs1(instruction);
            let rs2: RS2index = rs2(instruction);

            let is_m_extension = funct7(instruction) & 0b1 == 1;
            match funct3(instruction) {
                0b000 => {
                    if is_m_extension {
                        return Ok(Instruction::MUL(rd_index, rs1, rs2));
                    }
                    if funct7(instruction) == 0 {
                        Ok(Instruction::ADD(rd_index, rs1, rs2))
                    } else {
                        Ok(Instruction::SUB(rd_index, rs1, rs2))
                    }
                }
                0b001 => {
                    if is_m_extension {
                        return Ok(Instruction::MULH(rd_index, rs1, rs2));
                    }
                    Ok(Instruction::SLL(rd_index, rs1, rs2))
                }
                0b010 => {
                    if is_m_extension {
                        return Ok(Instruction::MULHSU(rd_index, rs1, rs2));
                    }
                    Ok(Instruction::SLT(rd_index, rs1, rs2))
                }
                0b011 => {
                    if is_m_extension {
                        return Ok(Instruction::MULHU(rd_index, rs1, rs2));
                    }
                    Ok(Instruction::SLTU(rd_index, rs1, rs2))
                }
                0b100 => {
                    if is_m_extension {
                        return Ok(Instruction::DIV(rd_index, rs1, rs2));
                    }
                    Ok(Instruction::XOR(rd_index, rs1, rs2))
                }
                0b101 => {
                    if is_m_extension {
                        return Ok(Instruction::DIVU(rd_index, rs1, rs2));
                    }
                    if funct7(instruction) == 0 {
                        Ok(Instruction::SRL(rd_index, rs1, rs2))
                    } else {
                        Ok(Instruction::SRA(rd_index, rs1, rs2))
                    }
                }
                0b110 => {
                    if is_m_extension {
                        return Ok(Instruction::REM(rd_index, rs1, rs2));
                    }
                    Ok(Instruction::OR(rd_index, rs1, rs2))
                }
                0b111 => {
                    if is_m_extension {
                        return Ok(Instruction::REMU(rd_index, rs1, rs2));
                    }
                    Ok(Instruction::AND(rd_index, rs1, rs2))
                }
                _ => Err("Invalid funct3 R-Type"),
            }
        }
        OpCode::LUI => {
            /* U Type */
            let rd_index: RDindex = rd(instruction);
            let u_imm: Uimmediate = immediate_u(instruction);
            Ok(Instruction::LUI(rd_index, u_imm))
        }
        OpCode::OP32 => todo!(),
        OpCode::LEN64 => todo!(),
        OpCode::MADD => todo!(),
        OpCode::MSUB => todo!(),
        OpCode::NMSUB => todo!(),
        OpCode::NMADD => todo!(),
        OpCode::OPFP => todo!(),
        OpCode::RESERVED1 => todo!(),
        OpCode::CUSTOM2 => todo!(),
        OpCode::LEN482 => todo!(),
        OpCode::BRANCH => {
            /* B-Type instructions */
            let rs1: RS1index = rs1(instruction);
            let rs2: RS2index = rs2(instruction);
            let b_imm: Bimmediate = immediate_b(instruction);
            match funct3(instruction) {
                0b000 => Ok(Instruction::BEQ(rs1, rs2, b_imm)),
                0b001 => Ok(Instruction::BNE(rs1, rs2, b_imm)),
                0b100 => Ok(Instruction::BLT(rs1, rs2, b_imm)),
                0b101 => Ok(Instruction::BGE(rs1, rs2, b_imm)),
                0b110 => Ok(Instruction::BLTU(rs1, rs2, b_imm)),
                0b111 => Ok(Instruction::BGEU(rs1, rs2, b_imm)),
                _ => Err("Invalid funct3 B-Type"),
            }
        }
        OpCode::JALR => {
            /* I-Type instruction */
            let rd_index: RDindex = rd(instruction);
            let rs1: RS1index = rs1(instruction);
            let i_imm: Iimmediate = immediate_i(instruction);
            Ok(Instruction::JALR(rd_index, rs1, i_imm))
        }
        OpCode::RESERVED2 => todo!(),
        OpCode::JAL => {
            let rd_index: RDindex = rd(instruction);
            let j_imm: Jimmediate = immediate_j(instruction);
            Ok(Instruction::JAL(rd_index, j_imm))
        }
        OpCode::SYSTEM => {
            /* I-Type instruction */
            let rd_index: RDindex = rd(instruction);
            let rs1: RS1index = rs1(instruction);
            let i_imm: Iimmediate = immediate_i(instruction);
            match funct3(instruction) {
                0b000 => match i_imm {
                    0b0000_0000_0000 => Ok(Instruction::ECALL()),
                    0b0000_0000_0001 => Ok(Instruction::EBREAK()),
                    0b0011_0000_0010 => Ok(Instruction::MRET()),
                    _ => Err("Invalid SYSTEM instruction immediate"),
                },
                0b001 => Ok(Instruction::CSRRW(rd_index, rs1, i_imm)),
                0b010 => Ok(Instruction::CSRRS(rd_index, rs1, i_imm)),
                0b011 => Ok(Instruction::CSRRC(rd_index, rs1, i_imm)),
                0b101 => {
                    /* This instruction repurposes rs1 as immediate */
                    Ok(Instruction::CSRRWI(rd_index, rs1, i_imm))
                }
                0b110 => {
                    /* This instruction repurposes rs1 as immediate */
                    Ok(Instruction::CSRRSI(rd_index, rs1, i_imm))
                }
                0b111 => {
                    /* This instruction repurposes rs1 as immediate */
                    Ok(Instruction::CSRRCI(rd_index, rs1, i_imm))
                }
                _ => Err("Invalid funct3 I-Type"),
            }
        }
        OpCode::RESERVED3 => todo!(),
        OpCode::CUSTOM3 => todo!(),
        OpCode::LEN80 => todo!(),
    }
}
