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
use stm32f1::stm32f103::{Peripherals as board_peripherals;
// use stm32f1xx_hal::prelude::*;
use stm32f1xx_hal::{delay, };
// use rtfm::app; 


#[entry]
fn main() -> ! {
    let peripherals = board_peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    let gpiob = &peripherals.GPIOB;
    let rcc = &peripherals.RCC;

    let usart1 = &peripherals.USART1;
    let usart2 = &peripherals.USART2;
    let usart3 = &peripherals.USART3;

    //115384
    let mantissa : u16 = 4;
    let fraction : u8 = 5;

    //19200
    // let mantissa : u16 = 26;
    // let fraction : u8 = 1;

    

    // rcc.apb2enr.write(|w| w.iopben().set_bit()); //gpiob enable
    // rcc.apb2enr.write(|w| w.iopaen().set_bit()); //gpioa enable
    // rcc.apb2enr.write(|w| w.usart1en().set_bit());
    rcc.apb1enr.write(|w| w.usart2en().set_bit());
    

    rcc.apb1enr.write(|w| unsafe {
        w.usart3en().set_bit();
        w.usart2en().set_bit()
    }); 

    // rcc.apb2enr.write(|w| unsafe {w.bits(0b100000000001100)}); 
    rcc.apb2enr.write(|w| unsafe {
        w.iopaen().set_bit();
        w.iopben().set_bit();
        w.usart1en().set_bit()
    }); 


    // asm::bkpt();

    gpioa.crh.write(|w| unsafe { //usart 1 gpio config
        w.mode9().bits(0b11); //output mode
        w.cnf9().bits(0b10); //alternate function push-pull

        w.mode10().bits(0b00); //input mode 
        w.cnf10().bits(0b01) //input pull-up
    });

    gpioa.crl.write(|w| unsafe {
        w.mode2().bits(0b11);
        w.cnf2().bits(0b10);

        w.mode3().bits(0b00);
        w.cnf3().bits(0b01)
    });

    gpiob.crh.write(|w| unsafe{
        w.mode12().bits(0b11);
        w.cnf12().bits(0b00);

        w.mode10().bits(0b11);
        w.cnf10().bits(0b10);

        w.mode11().bits(0b11);
        w.cnf11().bits(0b10)

    });

    // asm::bkpt();

    //usart 1 config block
    // usart1.cr1.write(|w| w.ue().set_bit()); //enable usart
    // usart1.cr1.write(|w| w.m().clear_bit()); //set word length (1 for 9 bits 0 for 8)
    // usart1.cr1.write(|w| w.te().set_bit()); //Transmission enable set to high

    usart1.cr1.write(|w| unsafe {
        w.ue().set_bit();
        w.m().clear_bit();
        w.te().set_bit()
    });

    usart1.cr2.write(|w| unsafe {w.stop().bits(0b00)}); //setting stop bits
    usart1.brr.write(|w| unsafe {
        w.div_mantissa().bits(mantissa);
        w.div_fraction().bits(fraction)
    }); //setting baud rate

    // asm::bkpt();
    
    //usart 1 config block end

    // usart 2 config block
    // usart2.cr1.write(|w| w.ue().set_bit()); //enable usart
    // usart2.cr1.write(|w| w.m().clear_bit()); //set word length (1 for 9 bits 0 for 8)
    // usart2.cr1.write(|w| w.re().set_bit()); //Reciever enable set to high

    usart2.cr1.write(|w| unsafe {
        w.ue().set_bit();
        w.m().clear_bit();
        w.te().set_bit();
        w.re().set_bit()
    });

    usart2.cr2.write(|w| unsafe {w.stop().bits(0b00)}); //setting stop bits
    usart2.brr.write(|w| unsafe {
        w.div_mantissa().bits(mantissa);
        w.div_fraction().bits(fraction)
    }); //setting baud rate

    
    usart3.cr1.write(|w| unsafe {
        w.ue().set_bit();
        w.m().clear_bit();
        w.te().set_bit()
    });

    usart3.cr2.write(|w| unsafe {w.stop().bits(0b00)}); //setting stop bits
    usart3.brr.write(|w| unsafe {
        w.div_mantissa().bits(mantissa);
        w.div_fraction().bits(fraction)
    }); //setting baud rate
    
    // asm::bkpt();
    // usart 2 config block end

    if usart1.sr.read().tc().bit() {
        // usart1.dr.write(|w| unsafe { w.bits(4) });
        usart1.dr.write(|w| unsafe { w.bits(0b10101010) });
    }

    
   

    gpiob.bsrr.write(|w| w.bs12().set_bit()); //led off

    let mut x : u8 = 0;

    asm::bkpt();

    loop {

    //     if usart2.sr.read().tc().bit() {
    //     // usart1.dr.write(|w| unsafe { w.bits(4) });
    //     usart2.dr.write(|w| unsafe { w.bits(0b10101010) });
    // }
        
        // asm::bkpt();

        if usart1.sr.read().tc().bit() {
        // usart1.dr.write(|w| unsafe { w.bits(4) });
        usart1.dr.write(|w| unsafe { w.bits(0b10101010) });
        }

    
        usart1.dr.write(|w| unsafe { w.bits(0b10101010) });
        usart2.dr.write(|w| unsafe { w.bits(0b10101010) });
        usart3.dr.write(|w| unsafe { w.bits(0b10101010) });
    //    usart1.dr.write(|w| unsafe { w.bits(0b10101010) });
        
        // if usart2.sr.read().rxne().bit() {
        //     x = usart2.dr.read().bits() as u8;
        // }

        // if x == 0b10101010 {
        //     gpiob.brr.write(|w| w.br12().set_bit()); //led on
        // }
    	
        // hprintln!("test").unwrap();
        
        // gpiob.bsrr.write(|w| w.bs12().set_bit()); //led off

        // cortex_m::asm::delay(8000000);

        // cortex_m::asm::delay(8000000);
        
    }
}
