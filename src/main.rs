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

use cortex_m::{asm, Peripherals as core_peripherals};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hprintln;
use stm32f1::stm32f103::{
    Peripherals,
    interrupt,
};
use stm32f1::stm32f103;
use stm32f1xx_hal::{
    prelude::*,
    serial::{Serial, Tx, Rx},
    delay::Delay,    
};
use nb::block;
// use rtfm::app; 

// CONNECTIONS
// serial tx and rx
type TX = Tx<stm32f103::USART1>;
type RX = Rx<stm32f103::USART1>;

// static mut TX = ();
// static mut RX = ();





#[entry]
fn main() -> ! {

    hprintln!("Here we go :D").unwrap();
    let p = Peripherals::take().unwrap();
    let cp = core_peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let clocks = rcc.cfgr.sysclk(72.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    //clock config
    // rcc.crr.write(|w| w.hseon()).set_bit());
    // rcc.cfgr.write(|w| unsafe {
    //     w.usart3en().set_bit();
    //     w.usart2en().set_bit()
    // });
    
    
    // static mut TX: TX = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    // static mut RX: RX = gpioa.pa10;

    // let baudrate = 9_600;
    let baudrate = 105_200;



    // USART1
    let tx1 = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx1 = gpioa.pa10;

    // USART2
    let tx2 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    let rx2 = gpioa.pa3;

    // USART3
    let tx3 = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    let rx3 = gpiob.pb11;

    let serial1 = Serial::usart1(
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

    let sent = b'X';
    let received= b'D';

    // hprintln!("USART1 Test").unwrap();

    // hprintln!("send block1").unwrap();
    block!(tx1.write(sent)).ok();
    // tx1.write(sent).unwrap();

    // hprintln!("receive block1").unwrap();
    let received = block!(rx1.read()).unwrap();
    // let received = rx1.read().unwrap();

    // hprintln!("message check, message sent = {}, message recieved is {}", sent, received).unwrap();
    // write!("message is {}!",  str::from_u8(received));
    if sent == received {
        // hprintln!("USART1 works").unwrap();
    }

    // hprintln!("USART2 Test").unwrap();

    let sent = b'E';

    // hprintln!("send block2").unwrap();
    block!(tx2.write(sent)).ok();
    // tx2.write(sent).unwrap();

    // hprintln!("receive block2").unwrap();
    let received = block!(rx2.read()).unwrap();
    // let received = rx2.read().unwrap();

    // hprintln!("message check, message sent = {}, message recieved is {}", sent, received).unwrap();
    // write!("message is {}!",  str::from_u8(received));
    if sent == received {
        // hprintln!("USART2 works").unwrap();
    }

    // hprintln!("USART3 Test").unwrap();

    let sent = b'C';

    // hprintln!("send block3").unwrap();
    block!(tx3.write(sent)).ok();
    // tx3.write(sent).unwrap();

    // hprintln!("receive block3").unwrap();
    let received = block!(rx3.read()).unwrap();
    // let received = rx3.read().unwrap();

    // hprintln!("message check, message sent = {}, message recieved is {}", sent, received).unwrap();
    // write!("message is {}!",  str::from_u8(received));
    if sent == received {
        // hprintln!("USART3 works").unwrap();
    }

    // hprintln!("end").unwrap();

    // asm::bkpt();

    let mut led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);
    let mut outPin = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    let mut delay = Delay::new(cp.SYST, clocks);

    let mut timera : u32 = 0;

    led.set_low();

    hprintln!("sysclock test {}, clock2 {}", clocks.sysclk().0, clocks.pclk1().0);

    // asm::bkpt();


loop {

    timera = 0;

    hprintln!("LED on").unwrap();

    led.set_high();

    while timera <= (400000 as u32){
        timera = timera + 1;
    }

    

    hprintln!("LED off").unwrap();
    led.set_low();    

    timera = 0;
    while timera <= (400000 as u32){
        timera = timera + 1;
    }

}
    
    
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

fn send_one() {
    hprintln!("Another function.");
}

fn another_function() {
    hprintln!("Another function.");
}

    // #[interrupt]
    // fn USART1() {
    //     // Read each character from serial as it comes in
    //     match rx1.read() {
    //         Ok(c) => {
    //             // TODO: handle buffer being full
    //             if resources.RX_BUF.push(c).is_ok() {}
    //         }
    //         Err(e) => {
    //             match e {
    //                 nb::Error::WouldBlock => {
    //                     for c in b"blocking\r\n" {
    //                         block!(resources.TX.write(*c)).ok();
    //                     }
    //                 }
    //                 // currently no way to easily clear the overrun flag, if you hit this
    //                 // it'll be stuck here
    //                 nb::Error::Other(stm32f103xx_hal::serial::Error::Overrun) => {
    //                     for c in b"overrun error\r\n" {
    //                         block!(resources.TX.write(*c)).ok();
    //                     }
    //                 }
    //                 _ => {
    //                     for c in b"other error\r\n" {
    //                         block!(resources.TX.write(*c)).ok();
    //                     }
    //                 }
    //             }
    //         }
    //     }
    // }
