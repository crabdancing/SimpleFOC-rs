#![no_std]
#![no_main]

use defmt::info;
use esp_hal::delay::Delay;
use esp_hal::prelude::*;
use {defmt_rtt as _, esp_backtrace as _};

// extern crate alloc;

#[entry]
fn main() -> ! {
    let _peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_alloc::heap_allocator!(72 * 1024);

    let delay = Delay::new();
    loop {
        info!("Hello world!");
        delay.delay(500.millis());
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.22.0/examples/src/bin
}
