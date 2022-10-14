use arduino_hal::port::*;
use arduino_hal::port::mode::*;
use arduino_hal::pac::*;
use arduino_hal::clock::*;

struct Heater {
  pwm_pin: Pin<Output>,
  probe_pin: Pin<Analog>,
  setTemp: i16,
  temp: i16,
}

impl Heater {
  fn update (&self) {
    self.temp = self.probe_pin.analog_read(adc); // read from analog pin
  }

  pub fn set_temp (&self, val: i16) -> () {
    self.setTemp += val;
  }

  pub fn get_settemp (&self) -> i16 {
    self.setTemp
  }

  pub fn read_temp (&self) -> i16 {
    self.update();
    self.temp
  }  
}

pub fn init (pwm_pin: Pin<Input<Floating>>, probe_pin: Pin<Input<Floating>>, adc: &mut Adc<Atmega, ADC, MHz16>) -> Heater {
  Heater {
    pwm_pin: pwm_pin.into_output(),
    probe_pin: probe_pin.into_analog_input(&mut adc),
    setTemp: 20,
    temp: 0,
  }
}