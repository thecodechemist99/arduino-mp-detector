#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use panic_halt as _;

mod Button;
mod Heater;
mod Display;

enum State {
  Idle,
  Running,
  TSet,
}

#[arduino_hal::entry]
fn main() -> ! {
  let dp = arduino_hal::Peripherals::take().unwrap();
  let pins = arduino_hal::pins!(dp);
  let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

  let mut led = pins.d13.into_output();

  let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
  ufmt::uwriteln!(&mut serial, "Hello World!").unwrap();

  let state = State::Idle;
  let mps: (i16, i16, i16);

  // configure buttons
  let lp_delay: u16 = 250;
  let buttons = [
    Button::init(pins.d2, lp_delay),
    Button::init(pins.d4, lp_delay),
    Button::init(pins.d5, lp_delay),
    Button::init(pins.d6, lp_delay),
  ];

  // configure heater
  let heater = Heater::init(pins.d3, pins.a0, adc);

  loop {
    // led.toggle();
    // arduino_hal::delay_ms(1000);

    // button actions
    for (index, button) in buttons.iter().enumerate() {
      let action = button.pressed();
      if action > 0 {
        match index {
          0 => {
            if action == 2 { // long press
              state = State::TSet;
            } else {
              match state {
                State::Idle => state = State::Running,
                State::Running => state = State::Idle,
                State::TSet => state = State::Idle,
              }
            }
          },
          1 => {
            match state {
              State::Idle => (),
              State::Running => mps.0 = heater.readTemp(),
              State::TSet => {
                if action == 2 {
                  heater.setTemp(10);
                } else {
                  heater.setTemp(1);
                }
              },
            }
          },
          2 => {
            match state {
              State::Idle => (),
              State::Running => mps.1 = heater.readTemp(),
              State::TSet => (),
            }
          },
          3 => {
            match state {
              State::Idle => (),
              State::Running => mps.2 = heater.readTemp(),
              State::TSet => {
                if action == 2 {
                  heater.setTemp(-10);
                } else {
                  heater.setTemp(-1);
                }
              },
            }
          },
        }
      }
    }
  }
}
