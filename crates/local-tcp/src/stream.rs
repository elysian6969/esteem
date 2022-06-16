use super::hello::Hello;
use super::identify::Identify;
use super::op::Op;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;
use std::{io, thread};

pub const _1F: u32 = u32::from_be_bytes([0x1F, 0x00, 0x00, 0x00]);
pub const _13: u32 = u32::from_be_bytes([0x13, 0x00, 0x00, 0x00]);
pub const _0E: u32 = u32::from_be_bytes([0x0E, 0x00, 0x00, 0x00]);
pub const _0D: u32 = u32::from_be_bytes([0x0D, 0x00, 0x00, 0x00]);
pub const _0C: u32 = u32::from_be_bytes([0x0C, 0x00, 0x00, 0x00]);
pub const _05: u32 = u32::from_be_bytes([0x05, 0x00, 0x00, 0x00]);
pub const _02: u32 = u32::from_be_bytes([0x02, 0x00, 0x00, 0x00]);
pub const _01: u32 = u32::from_be_bytes([0x10, 0x00, 0x00, 0x00]);

pub struct Steam3Client {
    pub stream: TcpStream,
}

impl Steam3Client {
    pub fn connect() -> io::Result<Self> {
        let stream = TcpStream::connect("127.0.0.1:57343")?;

        Ok(Self { stream })
    }

    pub fn write_op(&mut self, op: Op) -> io::Result<()> {
        self.stream.write(&op.to_bytes())?;

        Ok(())
    }

    pub fn write_u32(&mut self, value: u32) -> io::Result<()> {
        let bytes = value.to_be_bytes();

        self.stream.write(&bytes)?;

        Ok(())
    }

    pub fn write_02(&mut self) -> io::Result<()> {
        self.write_u32(_02)?;
        self.stream.write(&[0x03, 0x02])?;

        Ok(())
    }

    pub fn write_0E(&mut self, value: [u8; 8]) -> io::Result<()> {
        const PREFIX: [u8; 6] = [0x01, 0x04, 0x00, 0x00, 0x00, 0x00];

        let mut bytes = [0; 14];

        bytes[..6].copy_from_slice(&PREFIX);
        bytes[6..].copy_from_slice(&value);

        self.write_u32(_0E)?;
        self.stream.write(&bytes)?;

        Ok(())
    }

    // 02's have an 05 follow,
    pub fn do_02(&mut self) -> io::Result<Option<[u8; 5]>> {
        println!(" <- write 02 packet");
        self.write_02()?;

        println!(" -- expect 05 packet");

        if let Some(Packet::Unknown05(bytes)) = self.read_packet()? {
            println!(" -> read 05");
            println!(" -> 05 data: {bytes:02X?}");

            Ok(Some(bytes))
        } else {
            println!(" !! read incorrect packet");

            Ok(None)
        }
    }

    // 0E's have an 05 follow,
    pub fn do_0E(&mut self, value: [u8; 8]) -> io::Result<Option<[u8; 5]>> {
        println!(" <- write 0E packet");
        println!(" <- raw 0E data {value:02X?}");
        self.write_0E(value)?;

        println!(" -- expect 05 packet");

        if let Some(Packet::Unknown05(bytes)) = self.read_packet()? {
            println!(" -> read 05");
            println!(" -> 05 data: {bytes:02X?}");

            Ok(Some(bytes))
        } else {
            println!(" !! read incorrect packet");

            Ok(None)
        }
    }

    pub fn read_u32(&mut self) -> io::Result<u32> {
        let mut bytes = [0; 4];

        self.stream.read(&mut bytes)?;

        let value = u32::from_be_bytes(bytes);

        Ok(value)
    }

    pub fn read_op(&mut self) -> io::Result<Op> {
        let mut bytes = [0; 4];

        self.stream.read(&mut bytes)?;

        Ok(Op::from_bytes(bytes))
    }

    pub fn read_exact<const N: usize>(&mut self) -> io::Result<[u8; N]> {
        let mut bytes = [0; N];

        self.stream.read(&mut bytes)?;

        Ok(bytes)
    }

    pub fn read_packet(&mut self) -> io::Result<Option<Packet>> {
        let op = self.read_op()?;

        println!(" -> read_packet: op: {op:?}");

        let packet = match op {
            Op::UNKNOWN_01 => Some(Packet::Unknown01(self.read_exact::<1>()?)),
            Op::UNKNOWN_05 => Some(Packet::Unknown05(self.read_exact::<5>()?)),
            Op::UNKNOWN_0C => Some(Packet::Unknown0C(self.read_exact::<12>()?)),
            _ => None,
        };

        Ok(packet)
    }

    // after finally getting logs to appear from the steam client, i noticed the process id was the
    // same each time of running this (12847). after some comparison of bytes (2F, 32, 00, 00), the
    // process id is sent in the very first packet, twice, for some reason.
    pub fn write_hello(&mut self, hello: Hello) -> io::Result<()> {
        self.write_op(Op::HELLO)?;
        self.stream.write(&hello.to_bytes())?;

        Ok(())
    }

    // looking at a couple appids (730, 967476) as bytes ([DA, 02, 00, 00], [24, C3, 0E, 00]),
    // i determined this is how it is sent
    // TODO: even though the steam client accepts this, a coup[le other bytes seem to change with
    // different games, investigate the difference!
    pub fn write_identify(&mut self, identify: Identify) -> io::Result<()> {
        self.write_op(Op::IDENTIFY)?;
        self.stream.write(&identify.to_bytes())?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum Packet {
    Unknown0C([u8; 12]),
    Unknown05([u8; 5]),
    Unknown01([u8; 1]),
}
