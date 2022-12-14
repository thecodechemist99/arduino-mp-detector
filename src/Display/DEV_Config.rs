/*****************************************************************************
* | File        :   DEV_Config.rs
* | Author      :   Florian Beck, Waveshare team
* | Function    :   Hardware underlying interface
* | Info        :
*                Used to shield the underlying layers of each master 
*                and enhance portability
*----------------
* | This version:   V1.0
* | Date        :   2022-10-06
* | Info        :

# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documnetation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to  whom the Software is
# furished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in
# all copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS OR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
# THE SOFTWARE.
#
******************************************************************************/

use arduino_hal::{spi, Peripherals, Spi};
// use embedded_hal::spi::FullDuplex;
use arduino_hal::port::{mode::*, *};

mod Debug;

pub type UBYTE = u8;
pub type UWORD = u16;
pub type UDOUBLE = u32;

/**
 * GPIO config
**/
const DP: Peripherals = arduino_hal::Peripherals::take().unwrap();
const PINS: Pins = arduino_hal::pins!(DP);

pub const DEV_CS_PIN: i32 = 10;
pub const DEV_DC_PIN: i32 = 7;
pub const DEV_RST_PIN: i32 = 8;
pub const DEV_BL_PIN: i32 = 9;

/**
 * GPIO read and write
**/
pub fn dev_digital_write (_pin: Pin<Input<Floating>>, _value: bool) {
  if _value {
    _pin.set_high();
  } else {
    _pin.set_low();
  }
} 
pub fn dev_digital_read (_pin: Pin<Input<Floating>>) -> bool {
  if _pin.is_low() {
    false
  } else {
    true
  }
}

/**
 * SPI
**/
pub fn dev_spi_write (spi: Spi, _dat: u8) {
  spi.transfer(_dat); // what's the correct function?
}

/**
 * delay x ms
**/
pub fn dev_delay_ms (__xms: u16) {
  arduino_hal::delay_ms(__xms);
}

/**
 * PWM_BL
**/
fn dev_set_pwm (_value: u8) {
  PINS[DEV_BL_PIN].analogWrite(_value);// what's the correct function?
}

fn gpio_init () {
  PINS[DEV_CS_PIN].into_output();
  PINS[DEV_RST_PIN].into_output();
  PINS[DEV_DC_PIN].into_output();
  PINS[DEV_BL_PIN].into_output();
  PINS[DEV_BL_PIN].analogWrite(140); // what's the right method?
}

pub fn config_init () {
  gpio_init();
  
  // serial
  let mut serial = arduino_hal::default_serial!(DP, PINS, 115200);

  // SPI
  let (mut spi, _) = arduino_hal::Spi::new(
    DP.SPI,
    PINS.d13.into_output(),
    PINS.d11.into_output(),
    PINS.d12.into_pull_up_input(),
    PINS.d10.into_output(),
    spi::Settings::default(),
  );

  // SPI.setDataMode(SPI_MODE3);
  // SPI.setBitOrder(MSBFIRST);
  // SPI.setClockDivider(SPI_CLOCK_DIV2);
  // SPI.begin();

  spi
}