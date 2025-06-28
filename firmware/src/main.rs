#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed, Input, Pull};
use embassy_time::Timer;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    info!("Hello World!");

    let mut led = Output::new(p.PB0, Level::High, Speed::Low);
    let mq = Input::new(p.PB2, Pull::Up);
    led.set_high();

    loop {
        if mq.is_low() {
            info!("pin low");
            led.set_low();
        } else {
            info!("pin high");
            led.set_high();
        }

        //led.set_high();
        Timer::after_millis(200).await;
        //led.set_low();
        Timer::after_millis(200).await;
    }
}

