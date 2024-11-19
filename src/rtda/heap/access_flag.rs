use bitflags::bitflags;

bitflags! {
    pub struct AccessFlags: u16 {
        const ACC_PUBLIC       = 0x0001; // class field method
        const ACC_PRIVATE      = 0x0002; // field method
        const ACC_PROTECTED    = 0x0004; // field method
        const ACC_STATIC       = 0x0008; // field method
        const ACC_FINAL        = 0x0010; // class field method
        const ACC_SUPER        = 0x0020; // class
        const ACC_SYNCHRONIZED = 0x0020; // method
        const ACC_VOLATILE     = 0x0040; // field
        const ACC_BRIDGE       = 0x0040; // method
        const ACC_TRANSIENT    = 0x0080; // field
        const ACC_VARARGS      = 0x0080; // method
        const ACC_NATIVE       = 0x0100; // method
        const ACC_INTERFACE    = 0x0200; // class
        const ACC_ABSTRACT     = 0x0400; // class method
        const ACC_STRICT       = 0x0800; // method
        const ACC_SYNTHETIC    = 0x1000; // class field method
        const ACC_ANNOTATION   = 0x2000; // class
        const ACC_ENUM         = 0x4000; // class field
    }
}
