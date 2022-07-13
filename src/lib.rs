#![feature(asm_sym)]
#![feature(used_with_arg)]

pub mod arg;
pub mod bootstrap;
pub mod interface;
pub mod hooks;
pub mod init;
pub mod key;
pub mod options;
pub mod shared;
pub mod ui;
pub mod webhelper;

const NAME: &str = "esteem";

fn main() {
    if bootstrap::ready() {
        interface::open();
    } else {
        bootstrap::setup();
    }
}
