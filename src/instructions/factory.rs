use super::{
    base::instruction::Instruction,
    comparisons::{
        dcmp::{DCMPG, DCMPL},
        fcmp::{FCMPG, FCMPL},
        if_acmp::{IF_ACMPEQ, IF_ACMPNE},
        if_icmp::{IF_ICMPEQ, IF_ICMPGE, IF_ICMPGT, IF_ICMPLE, IF_ICMPLT, IF_ICMPNE},
        ifcond::{IFEQ, IFGE, IFGT, IFLE, IFLT, IFNE},
        lcmp::LCMP,
    },
    constants::{
        consts::{
            ACONST_NULL, DCONST_0, DCONST_1, FCONST_0, FCONST_1, FCONST_2, ICONST_0, ICONST_1,
            ICONST_2, ICONST_3, ICONST_4, ICONST_5, ICONST_M1, LCONST_0, LCONST_1,
        },
        ipush::{BIPUSH, SIPUSH},
        nop::NOP,
    },
    control::{goto::GOTO, lookupswitch::LOOKUP_SWITCH, tableswitch::TABLE_SWITCH},
    conversions::{
        d2x::{D2F, D2I, D2L},
        f2x::{F2D, F2I, F2L},
        i2x::{I2B, I2C, I2D, I2F, I2L, I2S},
        l2x::{L2D, L2F, L2I},
    },
    extended::{
        goto_w::GOTO_W,
        ifnull::{IFNONNULL, IFNULL},
        wide::WIDE,
    },
    loads::{
        aload::{ALOAD, ALOAD_0, ALOAD_1, ALOAD_2, ALOAD_3},
        dload::{DLOAD, DLOAD_0, DLOAD_1, DLOAD_2, DLOAD_3},
        fload::{FLOAD, FLOAD_0, FLOAD_1, FLOAD_2, FLOAD_3},
        iload::{ILOAD, ILOAD_0, ILOAD_1, ILOAD_2, ILOAD_3},
        lload::{LLOAD, LLOAD_0, LLOAD_1, LLOAD_2, LLOAD_3},
    },
    math::{
        add::{DADD, FADD, IADD, LADD},
        and::{IAND, LAND},
        div::{DDIV, FDIV, IDIV, LDIV},
        iinc::IINC,
        mul::{DMUL, FMUL, IMUL, LMUL},
        neg::{DNEG, FNEG, INEG, LNEG},
        or::{IOR, LOR},
        rem::{DREM, FREM, IREM, LREM},
        sh::{ISHL, ISHR, IUSHR, LSHL, LSHR, LUSHR},
        sub::{DSUB, FSUB, ISUB, LSUB},
        xor::{IXOR, LXOR},
    },
    stack::{
        dup::{DUP, DUP2, DUP2_X1, DUP2_X2, DUP_X1, DUP_X2},
        pop::{POP, POP2},
        swap::SWAP,
    },
    stores::{
        astore::{ASTORE, ASTORE_0, ASTORE_1, ASTORE_2, ASTORE_3},
        dstore::{DSTORE, DSTORE_0, DSTORE_1, DSTORE_2, DSTORE_3},
        fstore::{FSTORE, FSTORE_0, FSTORE_1, FSTORE_2, FSTORE_3},
        istore::{ISTORE, ISTORE_0, ISTORE_1, ISTORE_2, ISTORE_3},
        lstore::{LSTORE, LSTORE_0, LSTORE_1, LSTORE_2, LSTORE_3},
    },
    invoke::INVOKE_VIRTUAL,
    return_inst::RETURN_INST,
};

const nop: NOP = NOP {};
const aconst_null: ACONST_NULL = ACONST_NULL {};
const iconst_m1: ICONST_M1 = ICONST_M1 {};
const iconst_0: ICONST_0 = ICONST_0 {};
const iconst_1: ICONST_1 = ICONST_1 {};
const iconst_2: ICONST_2 = ICONST_2 {};
const iconst_3: ICONST_3 = ICONST_3 {};
const iconst_4: ICONST_4 = ICONST_4 {};
const iconst_5: ICONST_5 = ICONST_5 {};
const lconst_0: LCONST_0 = LCONST_0 {};
const lconst_1: LCONST_1 = LCONST_1 {};
const fconst_0: FCONST_0 = FCONST_0 {};
const fconst_1: FCONST_1 = FCONST_1 {};
const fconst_2: FCONST_2 = FCONST_2 {};
const dconst_0: DCONST_0 = DCONST_0 {};
const dconst_1: DCONST_1 = DCONST_1 {};
const iload_0: ILOAD_0 = ILOAD_0 {};
const iload_1: ILOAD_1 = ILOAD_1 {};
const iload_2: ILOAD_2 = ILOAD_2 {};
const iload_3: ILOAD_3 = ILOAD_3 {};
const lload_0: LLOAD_0 = LLOAD_0 {};
const lload_1: LLOAD_1 = LLOAD_1 {};
const lload_2: LLOAD_2 = LLOAD_2 {};
const lload_3: LLOAD_3 = LLOAD_3 {};
const fload_0: FLOAD_0 = FLOAD_0 {};
const fload_1: FLOAD_1 = FLOAD_1 {};
const fload_2: FLOAD_2 = FLOAD_2 {};
const fload_3: FLOAD_3 = FLOAD_3 {};
const dload_0: DLOAD_0 = DLOAD_0 {};
const dload_1: DLOAD_1 = DLOAD_1 {};
const dload_2: DLOAD_2 = DLOAD_2 {};
const dload_3: DLOAD_3 = DLOAD_3 {};
const aload_0: ALOAD_0 = ALOAD_0 {};
const aload_1: ALOAD_1 = ALOAD_1 {};
const aload_2: ALOAD_2 = ALOAD_2 {};
const aload_3: ALOAD_3 = ALOAD_3 {};
const istore_0: ISTORE_0 = ISTORE_0 {};
const istore_1: ISTORE_1 = ISTORE_1 {};
const istore_2: ISTORE_2 = ISTORE_2 {};
const istore_3: ISTORE_3 = ISTORE_3 {};
const lstore_0: LSTORE_0 = LSTORE_0 {};
const lstore_1: LSTORE_1 = LSTORE_1 {};
const lstore_2: LSTORE_2 = LSTORE_2 {};
const lstore_3: LSTORE_3 = LSTORE_3 {};
const fstore_0: FSTORE_0 = FSTORE_0 {};
const fstore_1: FSTORE_1 = FSTORE_1 {};
const fstore_2: FSTORE_2 = FSTORE_2 {};
const fstore_3: FSTORE_3 = FSTORE_3 {};
const dstore_0: DSTORE_0 = DSTORE_0 {};
const dstore_1: DSTORE_1 = DSTORE_1 {};
const dstore_2: DSTORE_2 = DSTORE_2 {};
const dstore_3: DSTORE_3 = DSTORE_3 {};
const astore_0: ASTORE_0 = ASTORE_0 {};
const astore_1: ASTORE_1 = ASTORE_1 {};
const astore_2: ASTORE_2 = ASTORE_2 {};
const astore_3: ASTORE_3 = ASTORE_3 {};
const pop: POP = POP {};
const pop2: POP2 = POP2 {};
const dup: DUP = DUP {};
const dup_x1: DUP_X1 = DUP_X1 {};
const dup_x2: DUP_X2 = DUP_X2 {};
const dup2: DUP2 = DUP2 {};
const dup2_x1: DUP2_X1 = DUP2_X1 {};
const dup2_x2: DUP2_X2 = DUP2_X2 {};
const swap: SWAP = SWAP {};
const iadd: IADD = IADD {};
const ladd: LADD = LADD {};
const fadd: FADD = FADD {};
const dadd: DADD = DADD {};
const isub: ISUB = ISUB {};
const lsub: LSUB = LSUB {};
const fsub: FSUB = FSUB {};
const dsub: DSUB = DSUB {};
const imul: IMUL = IMUL {};
const lmul: LMUL = LMUL {};
const fmul: FMUL = FMUL {};
const dmul: DMUL = DMUL {};
const idiv: IDIV = IDIV {};
const ldiv: LDIV = LDIV {};
const fdiv: FDIV = FDIV {};
const ddiv: DDIV = DDIV {};
const irem: IREM = IREM {};
const lrem: LREM = LREM {};
const frem: FREM = FREM {};
const drem: DREM = DREM {};
const ineg: INEG = INEG {};
const lneg: LNEG = LNEG {};
const fneg: FNEG = FNEG {};
const dneg: DNEG = DNEG {};
const ishl: ISHL = ISHL {};
const lshl: LSHL = LSHL {};
const ishr: ISHR = ISHR {};
const lshr: LSHR = LSHR {};
const iushr: IUSHR = IUSHR {};
const lushr: LUSHR = LUSHR {};
const iand: IAND = IAND {};
const land: LAND = LAND {};
const ior: IOR = IOR {};
const lor: LOR = LOR {};
const ixor: IXOR = IXOR {};
const lxor: LXOR = LXOR {};
const i2l: I2L = I2L {};
const i2f: I2F = I2F {};
const i2d: I2D = I2D {};
const l2i: L2I = L2I {};
const l2f: L2F = L2F {};
const l2d: L2D = L2D {};
const f2i: F2I = F2I {};
const f2l: F2L = F2L {};
const f2d: F2D = F2D {};
const d2i: D2I = D2I {};
const d2l: D2L = D2L {};
const d2f: D2F = D2F {};
const i2b: I2B = I2B {};
const i2c: I2C = I2C {};
const i2s: I2S = I2S {};
const lcmp: LCMP = LCMP {};
const fcmpl: FCMPL = FCMPL {};
const fcmpg: FCMPG = FCMPG {};
const dcmpl: DCMPL = DCMPL {};
const dcmpg: DCMPG = DCMPG {};

pub(crate) fn new_instruction(opcode: u8) -> Box<dyn Instruction> {
    match opcode {
        0x00 => Box::new(nop),
        0x01 => Box::new(aconst_null),
        0x02 => Box::new(iconst_m1),
        0x03 => Box::new(iconst_0),
        0x04 => Box::new(iconst_1),
        0x05 => Box::new(iconst_2),
        0x06 => Box::new(iconst_3),
        0x07 => Box::new(iconst_4),
        0x08 => Box::new(iconst_5),
        0x09 => Box::new(lconst_0),
        0x0a => Box::new(lconst_1),
        0x0b => Box::new(fconst_0),
        0x0c => Box::new(fconst_1),
        0x0d => Box::new(fconst_2),
        0x0e => Box::new(dconst_0),
        0x0f => Box::new(dconst_1),
        0x10 => Box::new(BIPUSH::default()),
        0x11 => Box::new(SIPUSH::default()),
        //  0x12 => Box::new(return),
        //  0x13 => Box::new(return),
        //  0x14 => Box::new(return),
        0x15 => Box::new(ILOAD::default()),
        0x16 => Box::new(LLOAD::default()),
        0x17 => Box::new(FLOAD::default()),
        0x18 => Box::new(DLOAD::default()),
        0x19 => Box::new(ALOAD::default()),
        0x1a => Box::new(iload_0),
        0x1b => Box::new(iload_1),
        0x1c => Box::new(iload_2),
        0x1d => Box::new(iload_3),
        0x1e => Box::new(lload_0),
        0x1f => Box::new(lload_1),
        0x20 => Box::new(lload_2),
        0x21 => Box::new(lload_3),
        0x22 => Box::new(fload_0),
        0x23 => Box::new(fload_1),
        0x24 => Box::new(fload_2),
        0x25 => Box::new(fload_3),
        0x26 => Box::new(dload_0),
        0x27 => Box::new(dload_1),
        0x28 => Box::new(dload_2),
        0x29 => Box::new(dload_3),
        0x2a => Box::new(aload_0),
        0x2b => Box::new(aload_1),
        0x2c => Box::new(aload_2),
        0x2d => Box::new(aload_3),
        //  0x2e => Box::new(return),
        //  0x2f => Box::new(return),
        //  0x30 => Box::new(return),
        //  0x31 => Box::new(return),
        //  0x32 => Box::new(return),
        //  0x33 => Box::new(return),
        //  0x34 => Box::new(return),
        //  0x35 => Box::new(return),
        0x36 => Box::new(ISTORE::default()),
        0x37 => Box::new(LSTORE::default()),
        0x38 => Box::new(FSTORE::default()),
        0x39 => Box::new(DSTORE::default()),
        0x3a => Box::new(ASTORE::default()),
        0x3b => Box::new(istore_0),
        0x3c => Box::new(istore_1),
        0x3d => Box::new(istore_2),
        0x3e => Box::new(istore_3),
        0x3f => Box::new(lstore_0),
        0x40 => Box::new(lstore_1),
        0x41 => Box::new(lstore_2),
        0x42 => Box::new(lstore_3),
        0x43 => Box::new(fstore_0),
        0x44 => Box::new(fstore_1),
        0x45 => Box::new(fstore_2),
        0x46 => Box::new(fstore_3),
        0x47 => Box::new(dstore_0),
        0x48 => Box::new(dstore_1),
        0x49 => Box::new(dstore_2),
        0x4a => Box::new(dstore_3),
        0x4b => Box::new(astore_0),
        0x4c => Box::new(astore_1),
        0x4d => Box::new(astore_2),
        0x4e => Box::new(astore_3),
        //  0x4f => Box::new(return),
        //  0x50 => Box::new(return),
        //  0x51 => Box::new(return),
        //  0x52 => Box::new(return),
        //  0x53 => Box::new(return),
        //  0x54 => Box::new(return),
        //  0x55 => Box::new(return),
        //  0x56 => Box::new(return),
        0x57 => Box::new(pop),
        0x58 => Box::new(pop2),
        0x59 => Box::new(dup),
        0x5a => Box::new(dup_x1),
        0x5b => Box::new(dup_x2),
        0x5c => Box::new(dup2),
        0x5d => Box::new(dup2_x1),
        0x5e => Box::new(dup2_x2),
        0x5f => Box::new(swap),
        0x60 => Box::new(iadd),
        0x61 => Box::new(ladd),
        0x62 => Box::new(fadd),
        0x63 => Box::new(dadd),
        0x64 => Box::new(isub),
        0x65 => Box::new(lsub),
        0x66 => Box::new(fsub),
        0x67 => Box::new(dsub),
        0x68 => Box::new(imul),
        0x69 => Box::new(lmul),
        0x6a => Box::new(fmul),
        0x6b => Box::new(dmul),
        0x6c => Box::new(idiv),
        0x6d => Box::new(ldiv),
        0x6e => Box::new(fdiv),
        0x6f => Box::new(ddiv),
        0x70 => Box::new(irem),
        0x71 => Box::new(lrem),
        0x72 => Box::new(frem),
        0x73 => Box::new(drem),
        0x74 => Box::new(ineg),
        0x75 => Box::new(lneg),
        0x76 => Box::new(fneg),
        0x77 => Box::new(dneg),
        0x78 => Box::new(ishl),
        0x79 => Box::new(lshl),
        0x7a => Box::new(ishr),
        0x7b => Box::new(lshr),
        0x7c => Box::new(iushr),
        0x7d => Box::new(lushr),
        0x7e => Box::new(iand),
        0x7f => Box::new(land),
        0x80 => Box::new(ior),
        0x81 => Box::new(lor),
        0x82 => Box::new(ixor),
        0x83 => Box::new(lxor),
        0x84 => Box::new(IINC::default()),
        0x85 => Box::new(i2l),
        0x86 => Box::new(i2f),
        0x87 => Box::new(i2d),
        0x88 => Box::new(l2i),
        0x89 => Box::new(l2f),
        0x8a => Box::new(l2d),
        0x8b => Box::new(f2i),
        0x8c => Box::new(f2l),
        0x8d => Box::new(f2d),
        0x8e => Box::new(d2i),
        0x8f => Box::new(d2l),
        0x90 => Box::new(d2f),
        0x91 => Box::new(i2b),
        0x92 => Box::new(i2c),
        0x93 => Box::new(i2s),
        0x94 => Box::new(lcmp),
        0x95 => Box::new(fcmpl),
        0x96 => Box::new(fcmpg),
        0x97 => Box::new(dcmpl),
        0x98 => Box::new(dcmpg),
        0x99 => Box::new(IFEQ::default()),
        0x9a => Box::new(IFNE::default()),
        0x9b => Box::new(IFLT::default()),
        0x9c => Box::new(IFGE::default()),
        0x9d => Box::new(IFGT::default()),
        0x9e => Box::new(IFLE::default()),
        0x9f => Box::new(IF_ICMPEQ::default()),
        0xa0 => Box::new(IF_ICMPNE::default()),
        0xa1 => Box::new(IF_ICMPLT::default()),
        0xa2 => Box::new(IF_ICMPGE::default()),
        0xa3 => Box::new(IF_ICMPGT::default()),
        0xa4 => Box::new(IF_ICMPLE::default()),
        0xa5 => Box::new(IF_ACMPEQ::default()),
        0xa6 => Box::new(IF_ACMPNE::default()),
        0xa7 => Box::new(GOTO::default()),
        //  0xa8 => Box::new(return),
        //  0xa9 => Box::new(return),
        0xaa => Box::new(TABLE_SWITCH::default()),
        0xab => Box::new(LOOKUP_SWITCH::default()),
        //  0xac => Box::new(return),
        //  0xad => Box::new(return),
        //  0xae => Box::new(return),
        //  0xaf => Box::new(return),
        //  0xb0 => Box::new(return),
        0xb1 => Box::new(RETURN_INST::default()),
        0xb2 => Box::new(NOP::default()),
        //  0xb3 => Box::new(return),
        //  0xb4 => Box::new(return),
        //  0xb5 => Box::new(return),
        0xb6 => Box::new(INVOKE_VIRTUAL::default()),
        //  0xb7 => Box::new(return),
        //  0xb8 => Box::new(return),
        //  0xb9 => Box::new(return),
        //  0xba => Box::new(return),
        //  0xbb => Box::new(return),
        //  0xbc => Box::new(return),
        //  0xbd => Box::new(return),
        //  0xbe => Box::new(return),
        //  0xbf => Box::new(return),
        //  0xc0 => Box::new(return),
        //  0xc1 => Box::new(return),
        //  0xc2 => Box::new(return),
        //  0xc3 => Box::new(return),
        0xc4 => Box::new(WIDE::default()),
        //  0xc5 => Box::new(return),
        0xc6 => Box::new(IFNULL::default()),
        0xc7 => Box::new(IFNONNULL::default()),
        0xc8 => Box::new(GOTO_W::default()),
        //  0xc9 => Box::new(return),
        //  0xca => Box::new() breakpoint,
        // case 0xff: impdep => Box::new((fmt),
        _ => {
            panic!("Unknown opcode: {}", opcode);
        }
    }
}
