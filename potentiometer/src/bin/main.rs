#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::analog::adc::{Adc, AdcConfig, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::time::{Duration, Instant};
use esp_hal::{analog, main};
use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
};
use esp_println::println;
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut config = AdcConfig::default();
    let mut pin = config.enable_pin(peripherals.GPIO13, Attenuation::_11dB);
    let mut adc2 = Adc::new(peripherals.ADC2, config);
    let mut delay = Delay::new();
    loop {
        let pin_value: u16 = nb::block!(adc2.read_oneshot(&mut pin)).unwrap_or_default();
        println!("Pin Value: {}", pin_value);
        if pin_value > 1500 {
            println!("Exiting due to pin value being too high");
            panic!("Pin value too high");
        };
        delay.delay_millis(3000);
    }
}
