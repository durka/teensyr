#![feature(start, core_intrinsics)]
#![no_std]

extern crate zinc;

use core::intrinsics::volatile_load;

use core::option::Option::Some;
use zinc::hal::k20::{pin, watchdog};
use zinc::hal::pin::Gpio;
use zinc::hal::cortex_m4::systick;
use zinc::util::support::wfi;

static mut i: u32 = 0;
static mut global_on: u32 = 0;

#[allow(dead_code)]
#[no_mangle]
pub unsafe extern fn isr_systick() {
  i += 1;
  if i > 100 {
    i = 0;
    global_on = (global_on + 1) % 5;
  }
}

pub fn main() {
  zinc::hal::mem_init::init_stack();
  zinc::hal::mem_init::init_data();
  watchdog::init(watchdog::State::Disabled);

  // Pins for Teensy 3.1 (http://www.pjrc.com/)
  let leds = [
      pin::Pin::new(pin::Port::PortC,  5, pin::Function::Gpio, Some(zinc::hal::pin::Out)),
      pin::Pin::new(pin::Port::PortA, 13, pin::Function::Gpio, Some(zinc::hal::pin::Out)),
      pin::Pin::new(pin::Port::PortA, 12, pin::Function::Gpio, Some(zinc::hal::pin::Out)),
      pin::Pin::new(pin::Port::PortD,  4, pin::Function::Gpio, Some(zinc::hal::pin::Out)),
      pin::Pin::new(pin::Port::PortD,  7, pin::Function::Gpio, Some(zinc::hal::pin::Out)),
  ];

  systick::setup(systick::ten_ms().unwrap_or(480000)/10);
  systick::enable();
  systick::enable_irq();

  loop {
    let on = unsafe { volatile_load(&global_on as *const u32) } as usize;
    leds[on.checked_sub(1).unwrap_or(leds.len()-1)].set_low();
    leds[on].set_high();
    wfi();
  }
}

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
  main();
  0
}
