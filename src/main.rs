#![no_std]
#![no_main]

extern crate stm32f1xx_hal;
extern crate panic_semihosting; // Log panic errors to stderr
extern crate rtfm;

use cortex_m_semihosting::hprintln;
use stm32f1xx_hal::stm32;
use rtfm::app;

#[app(device = stm32f1xx_hal::stm32)]
const APP: () = {

    #[init]    
    unsafe fn init() {
        hprintln!("Starting!!").unwrap();
    }

    #[idle]
    unsafe fn idle() -> ! {
        hprintln!("Idle!").unwrap();
        loop {
            hprintln!("Hello World!").unwrap();
        }
    }

};
