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

const CSGO: u32 = 730;
const RE: u32 = 967460;

fn main() -> io::Result<()> {
    let mut client = Steam3Client::connect()?;
    let process_id = unsafe { libc::getpid() };
    let app_id = CSGO;
    let hello = Hello::new(process_id).expect("valid process id");
    let identify = Identify::new(app_id);

    println!(" <- write hello with process ID: {process_id}");
    println!(" <- raw hello data: {:02X?}", hello.to_bytes());
    client.write_hello(hello)?;

    // handshake response?
    println!(" -- expecting 0c after handshake...");
    let packet = client.read_packet()?;
    println!(" -> got: {packet:02X?}");

    client.do_0E([0x73, 0xD2, 0x0F, 0xF9, 0xBA, 0x4F, 0x9C, 0xFA])?;
    client.do_0E([0xF3, 0x47, 0x5B, 0x7D, 0x39, 0x86, 0xE7, 0x7E])?;
    client.do_0E([0x2C, 0x56, 0x57, 0x8C, 0x74, 0x12, 0xE4, 0x8D])?;
    client.do_02()?;

    println!(" < write identify with app ID: {app_id}");
    println!(" < raw identify data: {:02X?}", identify.to_bytes());
    client.write_identify(identify)?;

    // 01 and 05 packets after identify
    let packet1 = client.read_packet()?;
    let packet2 = client.read_packet()?;

    println!(" -> identify response: {packet1:02x?}");
    println!(" -> identify response: {packet2:02x?}");

    client.do_0E([0x31, 0x7E, 0x60, 0x09, 0x94, 0xDF, 0xF3, 0x0A])?;

    client.write_u32(_1F)?;
    println!(" <- write 31 bytes...");
    client.stream.write(&[
        0x01, 0x04, 0x00, 0x00, 0x00, 0x00, 0x27, 0x47, 0x8a, 0x10, 0x0e, 0x53, 0x74, 0x65, 0x61,
        0x6d, 0x55, 0x74, 0x69, 0x6c, 0x73, 0x30, 0x31, 0x30, 0x00, 0x01, 0x00, 0x2a, 0x09, 0x45,
        0x12,
    ])?;

    let packet = client.read_packet()?;
    println!(" -> {packet:02x?}");

    println!(" -- sleep for 5 seconds");
    thread::sleep(Duration::from_secs(5));

    Ok(())
}
