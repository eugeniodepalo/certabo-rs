#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![allow(dead_code)]

extern crate cortex_m_rt;
use core::panic::PanicInfo;

use certabo::buzzer::Buzzer;
use certabo::display::Display;
use certabo::i2c::I2C;
use certabo::logger::Logger;
use certabo::peripherals::Peripherals;
use certabo::serial::Serial;
use certabo::usb::USB;
use core::fmt::Write;
use cortex_m_rt::entry;
use embedded_hal::blocking::delay::DelayMs;

#[entry]
unsafe fn main() -> ! {
  Peripherals::init();
  Serial::init(57600);
  Logger::init();
  I2C::init();
  Display::init();
  USB::init();
  Buzzer::init();

  let usb = USB::get();
  let lcd = Display::get();
  let _buzzer = Buzzer::get();

  lcd.write_str("Started!").unwrap();

  let _p = Peripherals::get();

  loop {
    // if p.white_button.try_is_low().unwrap() {
    //   buzzer.beep()
    // };

    usb.poll()
  }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  let location = info.location().unwrap();

  loop {
    Serial::get()
      .write_fmt(format_args!(
        "Panic at {} ({}, {}): {}\n\r",
        location.file(),
        location.line(),
        location.column(),
        info.message().unwrap()
      ))
      .unwrap();

    unsafe { Peripherals::get().delay.try_delay_ms(1000_u32).unwrap() }
  }
}
