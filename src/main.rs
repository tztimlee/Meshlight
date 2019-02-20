#![no_std]
#![no_main]

// pick a panicking behavior
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_itm; // logs messages over ITM; requires ITM support
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate stm32f1;
extern crate stm32f1xx_hal;
extern crate rtfm;
extern crate nb;

use cortex_m::{asm};
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f1::stm32f103::Peripherals;
use stm32f1xx_hal::{
    prelude::*,
    serial::Serial,
};
use nb::block;
// use rtfm::app; 




#[entry]
fn main() -> ! {

    hprintln!("Here we go :D").unwrap();
    let p = Peripherals::take().unwrap();
    hprintln!("test").unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    // let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    hprintln!("test").unwrap();

    // USART1
    // let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    // let rx = gpioa.pa10;

    // USART1
    // let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    // let rx = gpiob.pb7;

    // USART2
    // let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // let rx = gpioa.pa3;

    // USART3
    let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx = gpiob.pb11;

    let serial = Serial::usart3(
        p.USART3,
        (tx, rx),
        &mut afio.mapr,
        9_600.bps(),
        clocks,
        &mut rcc.apb1,
    );

    hprintln!("test").unwrap();

    let (mut tx, mut rx) = serial.split();

    let sent = b'X';

    hprintln!("test").unwrap();

    loop{
        tx.write(0b10101010);
        hprintln!("send").unwrap();
    }

    block!(tx.write(sent)).ok();

    let received = block!(rx.read()).unwrap();

    hprintln!("drumroll please").unwrap();

    if sent == received {
        hprintln!("FCKING FINALLY").unwrap();
    } else {
        hprintln!("FAK").unwrap();
    }

        

    loop {
        hprintln!("Idling").unwrap();
    }

    
}
