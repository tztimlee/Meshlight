
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
extern crate nb;

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm;
use core::alloc::Layout;
use stm32f1::stm32f103::{Interrupt, Peripherals};
use cortex_m_semihosting::hprintln;
use stm32f1xx_hal::{
    prelude::*,
    serial::Serial,
};
use rtfm::app;
use nb::block;

use self::alloc::vec;

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 10240;

#[app(device = stm32f1xx_hal::stm32)]
const APP: () = {

    #[init]    
    fn init() {
        unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }
        hprintln!("Starting!!").unwrap();
    }

    #[idle]
    unsafe fn idle() -> ! {

        let p = Peripherals::take().unwrap();
        asm::bkpt();
        let mut flash = p.FLASH.constrain();
        let mut rcc = p.RCC.constrain();

        let clocks = rcc.cfgr.freeze(&mut flash.acr);

        let mut afio = p.AFIO.constrain(&mut rcc.apb2);

        let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
        let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

        asm::bkpt();

        let tx1 = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
        let rx1 = gpioa.pa10;
        
        let tx2 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
        let rx2 = gpioa.pa3;

        asm::bkpt();

        let serial1 = Serial::usart1(
            p.USART1,
            (tx1, rx1),
            &mut afio.mapr,
            115_200.bps(),
            clocks,
            &mut rcc.apb2,
        );

        let serial2 = Serial::usart2(
            p.USART2,
            (tx2, rx2),
            &mut afio.mapr,
            115_200.bps(),
            clocks,
            &mut rcc.apb1,
        );

        asm::bkpt();

        let (mut tx1, mut rx1) = serial1.split();
        let (mut tx2, mut rx2) = serial2.split();

        let data = 4;

        block!(tx1.write(data)).ok();

        let received = block!(rx2.read()).unwrap();

        

        loop {
            hprintln!("Idling...").unwrap();
            if received == data {
                hprintln!("fucking finally").unwrap();
            }
            
        }
    }

};

// When an Out Of Memory condition occurs, trigger a breakpoint (for debugging)
// and loop indefinitely.  #[alloc_error_handler]
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    hprintln!("fak...").unwrap();
    asm::bkpt();
    loop {}
}
