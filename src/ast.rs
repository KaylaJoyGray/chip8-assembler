pub(crate) enum NullInstr {
    CLS,
    RET,
}

pub(crate) enum UnaryInstr {
    SYS,
    JP,
    CALL,
    SKP,
    SKNP,
}

pub(crate) enum BinaryInstr {
    SE,
    SNE,
    LD,
    ADD,
    SUB,
    SUBN,
    AND,
    OR,
    XOR,
    SHR,
    SHL,
    JP,
    RND,
}

pub(crate) enum TernaryInstr {
    DRAW,
}

pub(crate) enum GeneralPurposeReg {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
}

pub(crate) enum AddressReg {
    I,
}

pub(crate) enum SpecialReg {
    DT,
    ST,
}

pub(crate) enum Register {
    GP(GeneralPurposeReg),
    Address(AddressReg),
    Special(SpecialReg),
}

/// macros not included
/// macros resolved when building the AST
pub(crate) enum Node {
    Literal(u16),
    Register(Register),
    Label(String),
    NullExpr {
        Instr: NullInstr,
    },
    UnaryExpr {
        instr: UnaryInstr,
        child: Box<Node>,
    },
    BinaryExpr {
        instr: BinaryInstr,
        arg1: Box<Node>,
        arg2: Box<Node>,
    },
    TernaryExpr {
        instr: TernaryInstr,
        arg1: Box<Node>,
        arg2: Box<Node>,
        arg3: Box<Node>,
    },
}
