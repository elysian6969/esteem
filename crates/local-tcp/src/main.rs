#![allow(non_snake_case)]

use hello::Hello;
use identify::Identify;
use listener::Steam3Server;
use std::io::Write;
use std::time::Duration;
use std::{io, thread};
use stream::Steam3Client;

pub mod hello;
pub mod identify;
pub mod listener;
pub mod op;
pub mod stream;

const _1F: u32 = u32::from_be_bytes([0x1F, 0x00, 0x00, 0x00]);

const CSGO: u32 = 730;
const RE: u32 = 967460;

fn main() -> io::Result<()> {
    /*let mut server = Steam3Server::bind()?;

    println!("esteem | steam3server is bound");

    for client in server.listener.incoming() {
        println!("esteem | client connected");
    }*/

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
