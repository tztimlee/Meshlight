#![no_std]
#![no_main]

// pick a panicking behavior
// extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// extern crate panic_itm; // logs messages over ITM; requires ITM support
extern crate nb;
extern crate panic_semihosting; // logs messages to the host stderr; requires a debugger
extern crate rtfm;
extern crate stm32f1;
extern crate stm32f1xx_hal;

use cortex_m::{asm, Peripherals as core_peripherals};
use cortex_m_rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hprintln;
use nb::block;
use stm32f1::stm32f103;
use stm32f1::stm32f103::Peripherals;
use stm32f1xx_hal::{
    delay::Delay,
    prelude::*,
    serial::{Rx, Serial, Tx},
};

// static mut TX = ();
// static mut RX = ();

macro_rules! send_one {
    // `()` indicates that the macro takes no argument.
    ($pin: expr, $syst: expr, $clocks: expr) => (
        $pin.set_high();
        delay_us2(&mut $syst, &mut $clocks, 12);
        $pin.set_low();
        delay_us2(&mut $syst, &mut $clocks, 13);
    )
}

macro_rules! send_zero {
    // `()` indicates that the macro takes no argument.
    ($pin: expr, $syst: expr, $clocks: expr) => (
        $pin.set_high();
        delay_us2(&mut $syst, &mut $clocks, 5);
        $pin.set_low();
        delay_us2(&mut $syst, &mut $clocks, 20);
    )
}

macro_rules! reset_led {
    // `()` indicates that the macro takes no argument.
    ($pin: expr, $syst: expr, $clocks: expr) => (
        $pin.set_low();
        delay_us2(&mut $syst, &mut $clocks, 500);
    )
}

#[entry]
fn main() -> ! {
    hprintln!("Here we go :D").unwrap();
    let p = Peripherals::take().unwrap();
    let cp = core_peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut clocks = rcc
        .cfgr
        .sysclk(72.mhz())
        .pclk1(32.mhz())
        .freeze(&mut flash.acr);
    let mut syst = cp.SYST;

    let mut afio = p.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = p.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = p.GPIOB.split(&mut rcc.apb2);

    let mut led = gpioa.pa1.into_push_pull_output(&mut gpioa.crl);
    hprintln!("Ready").unwrap();

    led.set_low();

    loop {
        // hprintln!("Sendbyte").unwrap();
        reset_led!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_zero!(&mut led, &mut syst, &mut clocks);
        send_zero!(&mut led, &mut syst, &mut clocks);
        send_zero!(&mut led, &mut syst, &mut clocks);
        send_zero!(&mut led, &mut syst, &mut clocks);
        send_zero!(&mut led, &mut syst, &mut clocks);
        send_zero!(&mut led, &mut syst, &mut clocks);
        send_zero!(&mut led, &mut syst, &mut clocks);
        send_zero!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        send_one!(&mut led, &mut syst, &mut clocks);
        
        // delay_us2(&mut syst, &mut clocks, 100000000);
        // delay_us2(&mut syst, &mut clocks, 100000000);
        // delay_us2(&mut syst, &mut clocks, 100000000);
        // delay_us2(&mut syst, &mut clocks, 100000000);
        // delay_us2(&mut syst, &mut clocks, 100000000);
        // delay_us2(&mut syst, &mut clocks, 100000000);
        // delay_us2(&mut syst, &mut clocks, 100000000);
        // delay_us2(&mut syst, &mut clocks, 100000000);
        // delay_us2(&mut syst, &mut clocks, 100000000);
    }

    fn send_color(color : u8){

    }

    fn delay_us2(syst : &mut cortex_m::peripheral::SYST, clocks :  &mut stm32f1xx_hal::rcc::Clocks, us: u32) {
        // The SysTick Reload Value register supports values between 1 and 0x00FFFFFF.<stm32f1::stm32f103::Peripherals>
        const MAX_RVR: u32 = 0x00FF_FFFF;

        // hprintln!("la multiplica").unwrap();

        let mut total_rvr = us * (clocks.sysclk().0 / 10_000_000);

        // hprintln!("la multiplica2").unwrap();

        while total_rvr != 0 {
            let current_rvr = if total_rvr <= MAX_RVR {
                total_rvr
            } else {
                MAX_RVR
            };

            syst.set_reload(current_rvr);
            syst.clear_current();
            syst.enable_counter();

            // Update the tracking variable while we are waiting...
            total_rvr -= current_rvr;

            while !syst.has_wrapped() {}

            syst.disable_counter();
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
