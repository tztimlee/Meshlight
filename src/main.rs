
#![feature(alloc)]
#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

// alloc
extern crate alloc;
extern crate stm32f1;
extern crate stm32f1xx_hal;
extern crate rtfm;
extern crate panic_semihosting; // Log panic errors to stderr

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm;
use core::alloc::Layout;
use cortex_m_rt::entry;

use stm32f1::stm32f103;
use stm32f1::stm32f103::Interrupt;

mod router;

use cortex_m_semihosting::hprintln;
use stm32f1xx_hal::stm32;
use rtfm::app;

use stm32f1xx_hal::prelude::*;

use self::alloc::vec;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 10240;

#[app(device = stm32f1xx_hal::stm32)]
const APP: () = {

    static mut DEVICE: stm32f1xx_hal::stm32::Peripherals = ();
 
    #[init]
    fn init() -> init::LateResources {
        let device: stm32f1xx_hal::stm32::Peripherals = device;
        init::LateResources {
            DEVICE: device,
        }
    }

    #[idle(resources = [DEVICE])]
    fn idle() -> ! {
        // Get handles on the hardware
        let gpiob = &resources.DEVICE.GPIOB;
        let rcc = &resources.DEVICE.RCC;

        // Enable GPIO clock
        rcc.apb2enr.write(|w| w.iopben().set_bit());
        gpiob.crh.write(|w| unsafe{
            w.mode12().bits(0b11);
            w.cnf12().bits(0b00)
        });
        gpiob.brr.write(|w| w.br12().set_bit());

        loop {
            hprintln!("looping");

            // Blink the LEDs!
            gpiob.bsrr.write(|w| w.bs12().set_bit());
            cortex_m::asm::delay(2000000);
            gpiob.brr.write(|w| w.br12().set_bit());
            cortex_m::asm::delay(2000000);
        }
    }
};

// When an Out Of Memory condition occurs, trigger a breakpoint (for debugging)
// and loop indefinitely.  #[alloc_error_handler]
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop {}
}
