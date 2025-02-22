#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// PL011 UART base address for Raspberry Pi 4; may vary depending on your board/revision.
const UART0_BASE: usize = 0xFE201000;
const UART0_FR: *mut u32 = (UART0_BASE + 0x18) as *mut u32;
const UART0_DR: *mut u32 = UART0_BASE as *mut u32;

fn uart_send(c: u8) {
    // Wait until UART is ready to transmit.
    while unsafe { core::ptr::read_volatile(UART0_FR) } & (1 << 5) != 0 {}
    unsafe {
        core::ptr::write_volatile(UART0_DR, c as u32);
    }
}

fn uart_send_str(s: &str) {
    for byte in s.bytes() {
        uart_send(byte);
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Optionally, add UART initialization here.
    uart_send_str("Hello world!\n");

    loop {}
}