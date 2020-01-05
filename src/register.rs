#![allow(non_camel_case_types)]

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    TEMP_L          = 0x0B,
    TEMP_H          = 0x0C,
    WHOAMI          = 0x0F,
    ACT_THS         = 0x1E,
    ACT_DUR         = 0x1F,
    CTRL1           = 0x20,
    CTRL2           = 0x21,
    CTRL3           = 0x22,
    CTRL4           = 0x23,
    CTRL5           = 0x24,
    CTRL6           = 0x25,
    CTRL7           = 0x26,
    STATUS          = 0x27,
    OUT_X_L         = 0x28,
    OUT_X_H         = 0x29,
    OUT_Y_L         = 0x2A,
    OUT_Y_H         = 0x2B,
    OUT_Z_L         = 0x2C,
    OUT_Z_H         = 0x2D,
    FIFO_CTRL       = 0x2E,
    FIFO_SRC        = 0x2F,
    INT1_CFG        = 0x30,
    INT1_SRC        = 0x31,
    INT1_THS_X      = 0x32,
    INT1_THS_Y      = 0x33,
    INT2_THS_Z      = 0x34,
    INT1_DUR        = 0x35,
    INT2_CFG        = 0x36,
    INT2_SRC        = 0x37,
    INT2_THS        = 0x38,
    INT2_DUR        = 0x39,
    XL_REF          = 0x3A,
    XH_REF          = 0x3B,
    YL_REF          = 0x3C,
    YH_REF          = 0x3D,
    ZL_REF          = 0x3E,
    ZH_REF          = 0x3F,
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }

    pub fn read_only(self) -> bool {
        match self {
            Register::TEMP_L |
            Register::TEMP_H |
            Register::WHOAMI |
            Register::STATUS |
            Register::OUT_X_L |
            Register::OUT_X_H |
            Register::OUT_Y_L |
            Register::OUT_Y_H |
            Register::OUT_Z_L |
            Register::OUT_Z_H |
            Register::FIFO_SRC |
            Register::INT1_SRC |
            Register::INT2_SRC => true,
            _ => false,
        }
    }
}
