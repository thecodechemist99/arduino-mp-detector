/*****************************************************************************
* | File      	:	  LCD_Driver.rs
* | Author      :   Florian Beck, Waveshare team
* | Function    :   LCD driver
* | Info        :
*----------------
* |	This version:   V1.0
* | Date        :   2022-10-06
* | Info        :   
#
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
#[path = "DEV_Config.rs"]
mod DEV_Config;

use DEV_Config::*;

pub const LCD_WIDTH: UWORD = 320;  // LCD width
pub const LCD_HEIGHT: UWORD = 240; // LCD height

/*******************************************************************************
function:
	Hardware reset
*******************************************************************************/
fn lcd_reset () {
	dev_delay_ms(200);
	dev_digital_write(DEV_RST_PIN, 0);
	dev_delay_ms(200);
	dev_digital_write(DEV_RST_PIN, 1);
	dev_delay_ms(200);
}

/*******************************************************************************
function:
		Write data and commands
*******************************************************************************/
fn lcd_write_command (data: UBYTE) {	
	dev_digital_write(DEV_CS_PIN, 0);
	dev_digital_write(DEV_DC_PIN, 0);
	dev_spi_write(data);
}

fn lcd_write_data_byte (data: UWORD) {	
	dev_digital_write(DEV_CS_PIN, 0);
	dev_digital_write(DEV_DC_PIN, 1);
	dev_spi_write(data);  
	dev_digital_write(DEV_CS_PIN,1);
}  

pub fn lcd_write_data_word (data: UWORD) {
	dev_digital_write(DEV_CS_PIN, 0);
	dev_digital_write(DEV_DC_PIN, 1);
	dev_spi_write((data>>8) & 0xff);
	dev_spi_write(data);
	dev_digital_write(DEV_CS_PIN, 1);
}	  

/******************************************************************************
function:	
		Common register initialization
******************************************************************************/
fn lcd_init () {
	lcd_reset();

	lcd_write_command(0x36);
	lcd_write_data_byte(0xA0); 

	lcd_write_command(0x3A); 
	lcd_write_data_byte(0x05);

	lcd_write_command(0x21); 

	lcd_write_command(0x2A);
	lcd_write_data_byte(0x00);
	lcd_write_data_byte(0x01);
	lcd_write_data_byte(0x00);
	lcd_write_data_byte(0x3F);

	lcd_write_command(0x2B);
	lcd_write_data_byte(0x00);
	lcd_write_data_byte(0x00);
	lcd_write_data_byte(0x00);
	lcd_write_data_byte(0xEF);

	lcd_write_command(0xB2);
	lcd_write_data_byte(0x0C);
	lcd_write_data_byte(0x0C);
	lcd_write_data_byte(0x00);
	lcd_write_data_byte(0x33);
	lcd_write_data_byte(0x33);

	lcd_write_command(0xB7);
	lcd_write_data_byte(0x35); 

	lcd_write_command(0xBB);
	lcd_write_data_byte(0x1F);

	lcd_write_command(0xC0);
	lcd_write_data_byte(0x2C);

	lcd_write_command(0xC2);
	lcd_write_data_byte(0x01);

	lcd_write_command(0xC3);
	lcd_write_data_byte(0x12);   

	lcd_write_command(0xC4);
	lcd_write_data_byte(0x20);

	lcd_write_command(0xC6);
	lcd_write_data_byte(0x0F); 

	lcd_write_command(0xD0);
	lcd_write_data_byte(0xA4);
	lcd_write_data_byte(0xA1);

	lcd_write_command(0xE0);
	lcd_write_data_byte(0xD0);
	lcd_write_data_byte(0x08);
	lcd_write_data_byte(0x11);
	lcd_write_data_byte(0x08);
	lcd_write_data_byte(0x0C);
	lcd_write_data_byte(0x15);
	lcd_write_data_byte(0x39);
	lcd_write_data_byte(0x33);
	lcd_write_data_byte(0x50);
	lcd_write_data_byte(0x36);
	lcd_write_data_byte(0x13);
	lcd_write_data_byte(0x14);
	lcd_write_data_byte(0x29);
	lcd_write_data_byte(0x2D);

	lcd_write_command(0xE1);
	lcd_write_data_byte(0xD0);
	lcd_write_data_byte(0x08);
	lcd_write_data_byte(0x10);
	lcd_write_data_byte(0x08);
	lcd_write_data_byte(0x06);
	lcd_write_data_byte(0x06);
	lcd_write_data_byte(0x39);
	lcd_write_data_byte(0x44);
	lcd_write_data_byte(0x51);
	lcd_write_data_byte(0x0B);
	lcd_write_data_byte(0x16);
	lcd_write_data_byte(0x14);
	lcd_write_data_byte(0x2F);
	lcd_write_data_byte(0x31);
	lcd_write_command(0x21);

	lcd_write_command(0x11);

	lcd_write_command(0x29);
}

/******************************************************************************
function:	Set the cursor position
parameter	:
	  Xstart: Start UWORD x coordinate
	  Ystart:	Start UWORD y coordinate
	  Xend  :	End UWORD coordinates
	  Yend  :	End UWORD coordinatesen
******************************************************************************/
pub fn lcd_set_window (xstart: UWORD, ystart: UWORD, xend: UWORD, yend: UWORD) {
	lcd_write_command(0x2a);
	lcd_write_data_byte(0x00);
	lcd_write_data_byte(xstart & 0xff);
	lcd_write_data_byte((xend - 1) >> 8);
	lcd_write_data_byte((xend - 1) & 0xff);

	lcd_write_command(0x2b);
	lcd_write_data_byte(0x00);
	lcd_write_data_byte(ystart & 0xff);
	lcd_write_data_byte((yend - 1) >> 8);
	lcd_write_data_byte((yend - 1) & 0xff);

	lcd_write_command(0x2C);
}

/******************************************************************************
function:	Settings window
parameter	:
	  Xstart: Start UWORD x coordinate
	  Ystart:	Start UWORD y coordinate

******************************************************************************/
fn lcd_set_cursor (x: UWORD, y: UWORD) { 
	lcd_write_command(0x2a);
	lcd_write_data_byte(x >> 8);
	lcd_write_data_byte(x);
	lcd_write_data_byte(x >> 8);
	lcd_write_data_byte(x);

	lcd_write_command(0x2b);
	lcd_write_data_byte(y >> 8);
	lcd_write_data_byte(y);
	lcd_write_data_byte(y >> 8);
	lcd_write_data_byte(y);

	lcd_write_command(0x2C);
}

/******************************************************************************
function:	Clear screen function, refresh the screen to a certain color
parameter	:
	  Color :	The color you want to clear all the screen
******************************************************************************/
fn lcd_clear (color: UWORD) {
  let i: u32;
  let j: u32;
	lcd_set_window(0, 0, LCD_WIDTH, LCD_HEIGHT);
	dev_digital_write(DEV_DC_PIN, 1);
	for i in 0..LCD_WIDTH {
		for j in 0..LCD_HEIGHT {
			dev_spi_write((color >> 8) & 0xff);
			dev_spi_write(color);
		}
	 }
}

/******************************************************************************
function:	Refresh a certain area to the same color
parameter	:
	  Xstart: Start UWORD x coordinate
	  Ystart:	Start UWORD y coordinate
	  Xend  :	End UWORD coordinates
	  Yend  :	End UWORD coordinates
	  color :	Set the color
******************************************************************************/
fn lcd_clear_window (xstart: UWORD, ystart: UWORD, xend: UWORD, yend: UWORD, color: UWORD) {          
	let i: UWORD;
  let j: UWORD;
	lcd_set_window(xstart, ystart, xend - 1,yend - 1);
	for i in ystart..yend {
		for j in xstart..xend {
			lcd_write_data_word(color);
		}
	} 					  	    
}

/******************************************************************************
function: Draw a point
parameter	:
  	    X	: Set the X coordinate
	      Y	:	Set the Y coordinate
	  Color :	Set the color
******************************************************************************/
pub fn lcd_draw_point (x: UWORD, y: UWORD, color: UWORD) {
	lcd_set_cursor(x, y);
	lcd_write_data_word(color); 	    
}
