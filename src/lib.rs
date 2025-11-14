#![feature(abi_x86_interrupt)]
#![no_std]

pub mod interrupts;
mod vga_buffer;
pub mod gdt;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
}

