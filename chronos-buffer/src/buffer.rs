use uuid::Uuid;

use crate::{
    network::{FromNetwork, ToNetwork},
    types::{Identifier, Position, VarInt, VarLong},
};
use std::io::{Cursor, Read, Write};

macro_rules! register_buffer {
    {
        $name:ident,
        $(
            $buffer_type:ty => ($buffer_read:ident, $buffer_write:ident)
        ),*
    } => {
        #[derive(Debug, Clone)]
        pub struct $name {
            pub(crate) buf: Cursor<Vec<u8>>
        }

        impl $name {
            $(
                pub fn $buffer_read(&mut self) -> $buffer_type {
                    self.read::<$buffer_type>()
                }

                pub fn $buffer_write(&mut self, value: $buffer_type) {
                    self.write::<$buffer_type>(&value);
                }
            )*

        }
    }
}

impl ByteBuf {
    pub fn new(data: &[u8]) -> Self {
        Self {
            buf: Cursor::new(data.to_vec()),
        }
    }

    pub fn new_empty() -> Self {
        Self {
            buf: Cursor::new(Vec::new()),
        }
    }

    pub fn get_ref(&self) -> &Vec<u8> {
        self.buf.get_ref()
    }

    pub fn get_mut(&mut self) -> &mut Vec<u8> {
        self.buf.get_mut()
    }

    pub fn size(&self) -> usize {
        self.get_ref().len()
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.get_ref().clone()
    }

    pub fn read<T: FromNetwork<T>>(&mut self) -> T {
        T::from_network(self)
    }

    pub fn write<T: ToNetwork<T>>(&mut self, value: &T) {
        value.to_network(self);
    }

    pub fn read_array(&mut self, length: usize) -> Vec<u8> {
        let mut buffer = vec![0_u8; length];
        self.buf.read_exact(&mut buffer).unwrap();
        buffer
    }

    pub fn write_array(&mut self, buffer: &[u8]) {
        self.buf.write_all(buffer).unwrap();
    }
}

register_buffer! {
    ByteBuf,

    bool => (read_bool, write_bool),
    u8 => (read_byte, write_byte),
    u16 => (read_short, write_short),
    u32 => (read_int, write_int),
    u64 => (read_long, write_long),
    Uuid => (read_uuid, write_uuid),
    String => (read_string, write_string),
    Position => (read_position, write_position),
    Identifier => (read_identifier, write_identifier),
    VarInt => (read_varint, write_varint),
    VarLong => (read_varlong, write_varlong)
}
