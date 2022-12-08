#![no_std]
#![no_main]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use stm32f4::stm32f446;

use panic_halt as _;

const DELAY_CYCLES: u32 = 5000_000;

#[entry]
fn main() -> ! {
    let peripherals = stm32f446::Peripherals::take().unwrap();
    let gpioa = peripherals.GPIOA;
    let rcc = peripherals.RCC;

    // PA5 enabled and configured as slow output without pull up or pull down
    rcc.ahb1enr.modify(|_, w| w.gpioaen().enabled());
    gpioa.moder.modify(|_, w| w.moder5().output());
    gpioa.ospeedr.modify(|_, w| w.ospeedr5().low_speed());
    gpioa.pupdr.modify(|_, w| w.pupdr5().floating());

    loop {
        gpioa.bsrr.write(|w| w.bs5().set()); // led on
        delay(DELAY_CYCLES);
        gpioa.bsrr.write(|w| w.br5().reset()); // led off
        delay(DELAY_CYCLES);
    }
}
