#![feature(alloc)]
#![feature(alloc_error_handler)]
#![no_std]
#![no_main]

mod router;

extern crate alloc;
extern crate stm32f1;
extern crate stm32f1xx_hal;
extern crate panic_semihosting; // Log panic errors to stderr
extern crate nb;

use core::alloc::Layout;
use nb::block;

use self::alloc::vec;
use self::alloc::vec::Vec;

use alloc_cortex_m::CortexMHeap;
use cortex_m::asm;
use cortex_m::Peripherals as core_peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32f1::stm32f103::{Peripherals, interrupt};
use stm32f1xx_hal::{
    prelude::*,
    rcc::Clocks,
    time::MonoTimer,
    timer::Timer,
    serial::{Serial, Tx, Rx},
    delay::Delay
};

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 10240;
const MSG_DELAY: u16 = 1000;

#[entry]
fn main() -> ! {
    unsafe { ALLOCATOR.init(cortex_m_rt::heap_start() as usize, HEAP_SIZE) }
    // Initialization
    let p = Peripherals::take().unwrap();
    let cp = core_peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(72.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain(&mut rcc.apb2);
    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    let baudrate = 105_200;

    // USART Initialization

    // USART1
    let tx1 = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx1 = gpioa.pa10;

    // USART2
    let tx2 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx2 = gpioa.pa3;

    // USART3
    let tx3 = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx3 = gpiob.pb11;

    // USART 1
    let mut serial1 = Serial::usart1(
        p.USART1,
        (tx1, rx1),
        &mut afio.mapr,
        baudrate.bps(),
        clocks,
        &mut rcc.apb2,
    );

    let serial2 = Serial::usart2(
        p.USART2,
        (tx2, rx2),
        &mut afio.mapr,
        baudrate.bps(),
        clocks,
        &mut rcc.apb1,
    );

    let serial3 = Serial::usart3(
        p.USART3,
        (tx3, rx3),
        &mut afio.mapr,
        baudrate.bps(),
        clocks,
        &mut rcc.apb1,
    );

    let (mut tx1, mut rx1) = serial1.split();
    let (mut tx2, mut rx2) = serial2.split();
    let (mut tx3, mut rx3) = serial3.split();

    let mut delay = Delay::new(cp.SYST, clocks);
    delay.delay_ms(MSG_DELAY);

    hprintln!("Going...");

    loop {
        if false {
            let data_to_send = vec![0x1, 0x2, 0x3, 0x4];
            for byte in data_to_send {
                block!(tx1.write(byte)).ok();
                delay.delay_ms(MSG_DELAY);
            }
        } else {
            match rx1.read() {
                Ok(first_byte) => hprintln!("Got message: {:?}", first_byte).unwrap(),
                Err(_) => hprintln!("No messages...").unwrap()
            }
        }
    }
}

fn send_message(tx: &mut Tx<stm32f1xx_hal::stm32::USART1>, delay: &mut stm32f1xx_hal::delay::Delay, data: Vec<u8>) {
    for byte in data {
        block!(tx.write(byte)).ok();
        delay.delay_ms(MSG_DELAY);
    }
}

fn receive_message(rx: &mut Rx<stm32f1xx_hal::stm32::USART1>, delay: &mut stm32f1xx_hal::delay::Delay, first_byte: u8) -> Vec<u8> {
    let mut data = Vec::new();
    let mut receiving = true;
    data.push(first_byte);
    while receiving {
        match rx.read() {
            Ok(byte) => {
                receiving = byte != 0xFF;
                data.push(byte);
            },
            Err(_) => {
                hprintln!("Waiting on next message").unwrap();
            }
        } 
    }
    hprintln!("Got end byte 0xFF").unwrap();
    data
}

#[interrupt]
fn USART1() {
    hprintln!("Interrupted!").unwrap();
}

// When an Out Of Memory condition occurs, trigger a breakpoint (for debugging)
// and loop indefinitely.  #[alloc_error_handler]
#[alloc_error_handler]
fn alloc_error(_layout: Layout) -> ! {
    asm::bkpt();
    loop {
        hprintln!("Memory error").unwrap();
    }
}
