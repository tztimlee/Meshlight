
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

use stm32f1::stm32f103::Interrupt;

use cortex_m_semihosting::hprintln;
use stm32f1xx_hal::stm32;
use rtfm::app;

use self::alloc::vec;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 10240;

#[app(device = stm32f1xx_hal::stm32)]
const APP: () = {

    #[init]    
    fn init() {
        unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }
        rtfm::pend(Interrupt::USART1);
        hprintln!("Starting!!").unwrap();
    }

    #[idle]
    unsafe fn idle() -> ! {
        // Growable array allocated on the heap!
        let xs = vec![0, 1, 2, 3, 4, 5];
        hprintln!("Vector: {:?}", xs).unwrap();
        rtfm::pend(Interrupt::USART1);

        loop {
            hprintln!("Idling...").unwrap();
        }
    }

    #[interrupt]
    fn USART1() {
        hprintln!("Inside interrupt!");
    }

};

// When an Out Of Memory condition occurs, trigger a breakpoint (for debugging)
// and loop indefinitely.  #[alloc_error_handler]
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop {}
}
