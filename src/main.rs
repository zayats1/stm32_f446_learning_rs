#![no_std]
#![no_main]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use panic_halt as _;
use cortex_m_semihosting::hprintln;

const DELAY_CYCLES: u32 = 500_000;

#[entry]
fn main() -> ! {
    the_adc::init();
    loop {
        hprintln!("{}","Hello, world!");
        delay(DELAY_CYCLES);
    }
}

mod the_adc {
    use stm32f4::stm32f446;
    use stm32f4::stm32f446::Peripherals;

    pub fn init() {
        let p: Peripherals = stm32f446::Peripherals::take().unwrap();
        let rcc = p.RCC;
        rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
        rcc.apb2enr.modify(|_, w| w.adc1en().enabled());


        let adc = p.ADC_COMMON;
        adc.ccr.modify(|_, w| w.adcpre().div4());

        let adc1 = p.ADC1;
        adc1.cr2.write(|w| w.cont().continuous());
        adc1.cr2.write(|w|w.align().set_bit());
        adc1.cr2.modify(|_, w| w.adon().enabled());

        let gpioa = p.GPIOA;
        gpioa.moder.modify(|_,w|w.moder5().output());
        gpioa.pupdr.modify(|_,w|w.pupdr1().floating());
    }

    pub fn read() -> u16 {
        let p: Peripherals = stm32f446::Peripherals::take().unwrap();
        let adc1 = p.ADC1;
        adc1.cr2.write(|w|w.swstart().set_bit());
        let value = adc1.dr.read().data().bits();
        return value;
    }
}




