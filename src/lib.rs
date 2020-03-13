#![no_std]
#![allow(non_camel_case_types)]

mod register;

use core::fmt::Debug;
use core::convert::TryInto;
use embedded_hal::blocking::i2c::{WriteRead, Write};

pub use register::Register;
pub use accelerometer;
use accelerometer::{I16x3, Accelerometer};

#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data.
    WrongAddress,
    WriteToReadOnly,
    InvalidDataRate,
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum DataRate {
    Hz_800         = 0b110,
    Hz_400         = 0b101,
    Hz_200         = 0b100,
    Hz_100         = 0b011,
    Hz_50          = 0b010,
    Hz_10          = 0b001,
    PowerDown      = 0b000,
    Invalid        = 0xff,
}

impl DataRate {
    pub fn bits(self) -> u8 {
        self as u8
    }

    fn from(value: u8) -> DataRate {
        match value {
            0b110 => DataRate::Hz_800,
            0b101 => DataRate::Hz_400,
            0b100 => DataRate::Hz_200,
            0b011 => DataRate::Hz_100,
            0b010 => DataRate::Hz_50,
            0b001 => DataRate::Hz_10,
            0b000 => DataRate::PowerDown,
            _ => DataRate::Invalid,
        }
    }
}

pub struct Lis2hh12<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> Lis2hh12<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>
{
    pub fn new(i2c: I2C, address: u8) -> Result<Self, Error<E>> {
        let mut lis2hh12 = Lis2hh12 { i2c, address };

        let buf = lis2hh12.read_register(Register::WHOAMI)?;

        if buf != 0x41 {
            return Err(Error::WrongAddress)
        }
        // Enable all axes, normal mode.
        lis2hh12.write_register(Register::CTRL1, 0x07)?;
        // Set 400Hz data rate.
        lis2hh12.set_datarate(DataRate::Hz_400)?;
        // Enable FIFO
        lis2hh12.write_register(Register::CTRL3, 0x82)?;
        // Set FIFO mode to stream mode
        lis2hh12.write_register(Register::FIFO_CTRL, 0x40)?;
        Ok(lis2hh12)
    }

    pub fn set_datarate(&mut self, datarate: DataRate) -> Result<(), Error<E>> {
        if datarate == DataRate::Invalid {
            return Err(Error::InvalidDataRate);
        }
        let mut ctrl1 = self.read_register(Register::CTRL1)?;
        ctrl1 &= !0xf0;
        ctrl1 |= datarate.bits() << 4;
        self.write_register(Register::CTRL1, ctrl1)
    }

    pub fn get_datarate(&mut self) -> Result<DataRate, Error<E>> {
        let ctrl1 = self.read_register(Register::CTRL1)?;
        Ok(DataRate::from((ctrl1 >> 4) & 0x0F))
    }

    pub fn read_register(&mut self, register: Register) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[register.addr()], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }

    pub fn read_accel_bytes(&mut self) -> Result<[u8;6], Error<E>> {
        let mut data = [0u8;6];
        self.i2c
            .write_read(self.address, &[Register::OUT_X_L.addr() | 0x80], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data))
    }

    pub fn write_register(&mut self, register: Register, value: u8) -> Result<(), Error<E>> {
        if register.read_only() {
            return Err(Error::WriteToReadOnly);
        }
        self.i2c.write(self.address, &[register.addr(), value]).map_err(Error::I2C)
    }

    pub fn get_acceleration(&mut self) -> Result<I16x3, Error<E>> {
       let accel_bytes = self.read_accel_bytes()?;
       let x = i16::from_le_bytes(accel_bytes[0..2].try_into().unwrap());
       let y = i16::from_le_bytes(accel_bytes[2..4].try_into().unwrap());
       let z = i16::from_le_bytes(accel_bytes[4..6].try_into().unwrap());
       Ok(I16x3::new(x, y, z))
    }
}

impl<I2C, E> Accelerometer<I16x3> for Lis2hh12<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>,
    E: Debug,
{
    type Error = Error<E>;

    /// Get acceleration reading from the accelerometer
    fn acceleration(&mut self) -> Result<I16x3, Error<E>> {
       let accel_bytes = self.read_accel_bytes()?;
       let x = i16::from_le_bytes(accel_bytes[0..2].try_into().unwrap());
       let y = i16::from_le_bytes(accel_bytes[2..4].try_into().unwrap());
       let z = i16::from_le_bytes(accel_bytes[4..6].try_into().unwrap());
       Ok(I16x3::new(x, y, z))
    }
}
