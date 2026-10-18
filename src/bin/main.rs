#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::{
    analog::adc::{Adc, AdcConfig},
    delay::{self, Delay},
    main,
};
use esp_println::println;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        let mut delay = Delay::new();
        println!("Board entered panic state!!!!");
        delay.delay_millis(2000);
    }
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
    let mut gpio32 = config.enable_pin(peripherals.GPIO32, esp_hal::analog::adc::Attenuation::_6dB);
    let mut gpio34 = config.enable_pin(peripherals.GPIO34, esp_hal::analog::adc::Attenuation::_6dB);
    let mut gpio35 = config.enable_pin(peripherals.GPIO35, esp_hal::analog::adc::Attenuation::_6dB);
    let mut adc1_config = Adc::new(peripherals.ADC1, config);

    let mut delay = Delay::new();
    loop {
        let vals = [
            nb::block!(adc1_config.read_oneshot(&mut gpio32)).unwrap_or_default(),
            nb::block!(adc1_config.read_oneshot(&mut gpio34)).unwrap_or_default(),
            nb::block!(adc1_config.read_oneshot(&mut gpio35)).unwrap_or_default(),
        ];
        println!(
            "GPIO32: {:?}, GPIO34: {:?}, GPIO35: {:?}",
            vals[0], vals[1], vals[2]
        );
        delay.delay_millis(2000);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.2.2/examples
}
