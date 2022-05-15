#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use uefi::prelude::*;
use log::info;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();


    use uefi::proto::console::text::Color;

    system_table.stdout().reset(true)?;

    let mode = system_table.stdout().modes().last().unwrap();
    system_table.stdout().set_mode(mode)?;

    system_table.stdout().set_color(Color::Black, Color::Magenta)?;
    system_table.stdout().clear()?;

    info!("Hello World");


    #[allow(clippy::empty_loop)]
    loop {}
}
