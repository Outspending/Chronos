use std::{io::{Read, Write}, ops::Deref};
use lazy_static::lazy_static;
use regex::Regex;

use crate::{buffer::ByteBuf, network::{FromNetwork, ToNetwork}};
use std::convert::From;

lazy_static! {
    static ref NAMESPACE_REGEX: Regex = Regex::new(r"^[a-z0-9.-_]+$").unwrap();
    static ref PATH_REGEX: Regex = Regex::new(r"^[a-z0-9.-_/]+$").unwrap();
}

macro_rules! register_varnum {
    ( $name:ident, $type:ty, $working_type:ty, $max_size:literal ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(pub $type);

        impl Deref for $name {
            type Target = $type;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                $name(value)
            }
        }

        impl FromNetwork<$name> for $name {
            fn from_network(buffer: &mut ByteBuf) -> Self {
                let mut value = 0;
                let mut size = 0;
                loop {
                    let byte = buffer.read_byte();
                    value |= ((byte & 0b01111111) as $working_type) << (size * 7);
                    size += 1;
                    if size > $max_size {
                        panic!("VarNum too big");
                    }
                    if byte & 0b10000000 == 0 {
                        break;
                    }
                }

                $name(value as $type)
            }
        }

        impl ToNetwork<$name> for $name {
            fn to_network(&self, buffer: &mut ByteBuf) {
                let mut value = self.0 as $working_type;
                loop {
                    let mut byte = (value & 0b01111111) as u8;
                    value >>= 7;
                    if value != 0 {
                        byte |= 0b10000000;
                    }
                    buffer.write_byte(byte);
                    if value == 0 {
                        break;
                    }
                }
            }
        }
    };
}

register_varnum!(VarInt, i32, u32, 5);
register_varnum!(VarLong, i64, u64, 10);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl FromNetwork<Position> for Position {
    fn from_network(buf: &mut ByteBuf) -> Position {
        let val = buf.read_long();
        Position::new(
            (val >> 38) as i32, 
            (val << 52 >> 52) as i32, 
            (val << 26 >> 38) as i32
        )
    }
}

impl ToNetwork<Position> for Position {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write_long((((self.x & 0x3FFFFFF) as u64) << 38) | (((self.z & 0x3FFFFFF) as u64) << 12) | (self.y & 0xFFF) as u64)
    }
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Position { x, y, z }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Angle(pub u32);

impl FromNetwork<Angle> for Angle {
    fn from_network(buf: &mut ByteBuf) -> Angle {
        let byte = buf.read_byte() as u32;
        Angle(byte * 256 / 360)
    }
}

impl ToNetwork<Angle> for Angle {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write_byte((self.0 * 360 / 256) as u8)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier {
    pub namespace: String,
    pub path: String,
}

impl FromNetwork<Identifier> for Identifier {
    fn from_network(buf: &mut ByteBuf) -> Identifier {
        let namespace = buf.read_string();
        let path = buf.read_string();
        Identifier::new(namespace, path).unwrap()
    }
}

impl ToNetwork<Identifier> for Identifier {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write_string(self.namespace.clone());
        buf.write_string(self.path.clone());
    }
}

impl Identifier {
    pub fn new(namespace: String, path: String) -> Option<Self> {
        if !NAMESPACE_REGEX.is_match(&namespace) || !PATH_REGEX.is_match(&path) {
            None
        } else {
            Some(Identifier { namespace, path })
        }
    }
}

impl FromNetwork<bool> for bool {
    fn from_network(buf: &mut ByteBuf) -> bool {
        buf.read_byte() != 0
    }
}

impl ToNetwork<bool> for bool {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.write_byte(if *self { 1 } else { 0 })
    }
}

impl FromNetwork<u8> for u8 {
    fn from_network(buf: &mut ByteBuf) -> u8 {
        let mut buffer = [0; 1];
        buf.buf.read_exact(&mut buffer).unwrap();
        buffer[0]
    }
}

impl ToNetwork<u8> for u8 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.buf.write_all(&[*self]).unwrap();
    }
}

impl FromNetwork<u16> for u16 {
    fn from_network(buf: &mut ByteBuf) -> u16 {
        let mut buffer = [0; 2];
        buf.buf.read_exact(&mut buffer).unwrap();
        u16::from_be_bytes(buffer)
    }
}

impl ToNetwork<u16> for u16 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.buf.write_all(&self.to_be_bytes()).unwrap();
    }
}

impl FromNetwork<u32> for u32 {
    fn from_network(buf: &mut ByteBuf) -> u32 {
        let mut buffer = [0; 4];
        buf.buf.read_exact(&mut buffer).unwrap();
        u32::from_be_bytes(buffer)
    }
}

impl ToNetwork<u32> for u32 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.buf.write_all(&self.to_be_bytes()).unwrap();
    }
}

impl FromNetwork<u64> for u64 {
    fn from_network(buf: &mut ByteBuf) -> u64 {
        let mut buffer = [0; 8];
        buf.buf.read_exact(&mut buffer).unwrap();
        u64::from_be_bytes(buffer)
    }
}

impl ToNetwork<u64> for u64 {
    fn to_network(&self, buf: &mut ByteBuf) {
        buf.buf.write_all(&self.to_be_bytes()).unwrap();
    }
}

impl FromNetwork<String> for String {
    fn from_network(buf: &mut ByteBuf) -> String {
        let len = *buf.read_varint() as usize;
        let bytes = buf.read_array(len);

        String::from_utf8(bytes).unwrap()
    }
}

impl ToNetwork<String> for String {
    fn to_network(&self, buf: &mut ByteBuf) {
        let bytes = self.as_bytes();
        buf.write_varint(VarInt::from(bytes.len() as i32));
        buf.write_array(bytes);
    }
}