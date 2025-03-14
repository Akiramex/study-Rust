#![no_std]
#![no_main]

use core::cell::RefCell;

use critical_section::Mutex;
use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output, Input, Pull, Io};
use esp_hal::interrupt::InterruptConfigurable;
use esp_hal::{main, handler};
use log::info;


static BUTTON: Mutex<RefCell<Option<Input>>> = Mutex::new(RefCell::new(None));
static LEDS: Mutex<RefCell<Option<(Output, Output)>>> = Mutex::new(RefCell::new(None));

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[main]
fn main() -> ! {
    // generator version: 0.3.1
    esp_println::logger::init_logger_from_env();

    info!("Hello, world!");

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let mut io = Io::new(peripherals.IO_MUX);
    io.set_interrupt_handler(handler);

    let led1 = Output::new(peripherals.GPIO12, Level::High);
    let led2 = Output::new(peripherals.GPIO13, Level::Low);
    let mut button = Input::new(peripherals.GPIO9, Pull::Up);



    critical_section::with(|cs| {
        LEDS.borrow_ref_mut(cs).replace((led1, led2));

        button.listen(esp_hal::gpio::Event::FallingEdge);
        BUTTON.borrow_ref_mut(cs).replace(button)
    });

    let delay = Delay::new();
    loop {
        delay.delay_millis(500u32);
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.0/examples/src/bin
}

#[handler]
fn handler() {
    critical_section::with(|cs| {
        BUTTON
            .borrow_ref_mut(cs)
            .as_mut()
            .unwrap()
            .clear_interrupt();

        // 切换LED状态
        if let Some((led1, led2)) = LEDS.borrow_ref_mut(cs).as_mut() {
            led1.toggle();
            led2.toggle();
        }
    });
}

