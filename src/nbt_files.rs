use std::io::{Cursor, Result};

use nbt::{from_reader, to_writer, Blob, Endianness};

pub fn read_nbt(value: Vec<u8>) -> nbt::Result<Blob> {
    let reader: Cursor<Vec<u8>> = Cursor::new(value);
    from_reader(reader, Endianness::LittleEndian)
}

pub fn write_nbt(nbt: &Blob) -> Result<Vec<u8>> {
    let mut value: Vec<u8> = Vec::new();
    to_writer(&mut value, &nbt, None, Endianness::LittleEndian)?;
    Ok(value)
}
