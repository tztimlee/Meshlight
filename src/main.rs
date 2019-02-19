#![no_std]
#![no_main]

// pick a panicking behavior
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_itm; // logs messages over ITM; requires ITM support
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate stm32f1;
extern crate stm32f1xx_hal;
extern crate rtfm;

use cortex_m::{asm, Peripherals as core_peripherals};
use cortex_m_rt::entry;
// use cortex_m_semihosting::{hprintln};
use stm32f1::stm32f103::{Peripherals as board_peripherals};
// use stm32f1xx_hal::prelude::*;
// use stm32f1xx_hal::delay::Delay;
// use rtfm::app; 


#[entry]
fn main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code

    loop {
        // your code goes here
    }
}
