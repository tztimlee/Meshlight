
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

    #[init(spawn = [parent_task])]
    fn init() {
        unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }
        hprintln!("Spawning parent task...").unwrap();
        match spawn.parent_task() {
            Ok(ok) => {
                hprintln!("{:?} Spawn succeeded.", ok).unwrap();
            },
            Err(err) => {
                hprintln!("{:?}", err).unwrap();
            }
        }
        match spawn.parent_task() {
            Ok(ok) => {
                hprintln!("{:?} Spawn succeeded.", ok).unwrap();
            },
            Err(err) => {
                hprintln!("{:?}", err).unwrap();
            }
        }
    }

    #[task(priority=3, spawn = [task_one, task_two])]
    fn parent_task() {
        hprintln!("Inside parent task...").unwrap();

        hprintln!("Spawning task_one").unwrap();
        spawn.task_one().unwrap();
        hprintln!("\n... delay ...").unwrap();
        hprintln!("... delay ...").unwrap();
        hprintln!("... delay ...").unwrap();
        hprintln!("\nSpawning task_two").unwrap();
        spawn.task_two().unwrap();
    }

    #[task(priority=2)]
    fn task_one() {
        let mut i = 5;
        while i != 0 {
            hprintln!("{:?} task_one", i).unwrap();
            i = i - 1;
        }
        hprintln!("task_one finished").unwrap();
    }

    #[task(priority=2)]
    fn task_two() {
        let mut i = 7;
        while i != 0 {
            hprintln!("{:?} task_two", i).unwrap();
            i = i - 1;
        }
        hprintln!("task_two finished").unwrap();
    }

    extern "C" {
        fn USART1();
        fn USART2();
        fn USART3();
    }

};

// When an Out Of Memory condition occurs, trigger a breakpoint (for debugging)
// and loop indefinitely.  #[alloc_error_handler]
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop {}
}
