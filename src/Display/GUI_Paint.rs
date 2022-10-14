/******************************************************************************
* | File      	:   GUI_Paint.rs
* | Author      :   Florian Beck, Waveshare electronics
* | Function    :	  Achieve drawing: draw points, lines, boxes, circles and
*                   their size, solid dotted line, solid rectangle hollow
*                   rectangle, solid circle hollow circle.
* | Info        :
*   Achieve display characters: Display a single character, string, number
*   Achieve time display: adaptive size display time minutes and seconds
*----------------
* |	This version:   V3.1
* | Date        :   2022-10-06
* | Info        :
* -----------------------------------------------------------------------------
* V3.1(2020-08-15):
* 1.Fix: 
*       Paint_DrawNum
*         Fixed a BUG where the Paint_DrawNum function failed to display 0
* 2.Add： Paint_DrawFloatNum
*     Can display FloatNum   
*
* -----------------------------------------------------------------------------
* V3.0(2019-04-18):
* 1.Change: 
*    Paint_DrawPoint(..., DOT_STYLE DOT_STYLE)
* => Paint_DrawPoint(..., DOT_STYLE Dot_Style)
*    Paint_DrawLine(..., LINE_STYLE Line_Style, DOT_PIXEL Dot_Pixel)
* => Paint_DrawLine(..., DOT_PIXEL Line_width, LINE_STYLE Line_Style)
*    Paint_DrawRectangle(..., DRAW_FILL Filled, DOT_PIXEL Dot_Pixel)
* => Paint_DrawRectangle(..., DOT_PIXEL Line_width, DRAW_FILL Draw_Fill)
*    Paint_DrawCircle(..., DRAW_FILL Draw_Fill, DOT_PIXEL Dot_Pixel)
* => Paint_DrawCircle(..., DOT_PIXEL Line_width, DRAW_FILL Draw_Filll)
*
* -----------------------------------------------------------------------------
* V2.0(2018-11-15):
* 1.add: Paint_NewImage()
*    Create an image's properties
* 2.add: Paint_SelectImage()
*    Select the picture to be drawn
* 3.add: Paint_SetRotate()
*    Set the direction of the cache    
* 4.add: Paint_RotateImage() 
*    Can flip the picture, Support 0-360 degrees, 
*    but only 90.180.270 rotation is better
* 4.add: Paint_SetMirroring() 
*    Can Mirroring the picture, horizontal, vertical, origin
* 5.add: Paint_DrawString_CN() 
*    Can display Chinese(GB1312)   
*
* ----------------------------------------------------------------------------- 
* V1.0(2018-07-17):
*   Create library
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documnetation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to  whom the Software is
* furished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in
* all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS OR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
* THE SOFTWARE.
*
******************************************************************************/
#[path = "DEV_Config.rs"]
mod DEV_Config;
#[path = "LCD_Driver.rs"]
mod LCD_Driver;
#[path = "fonts.rs"]
mod fonts;
#[path = "Debug.rs"]
#[macro_use]
mod Debug;

use DEV_Config::*;
use LCD_Driver::*;
use MirrorImage::*;
use DotPixel::*;
use DotStyle::*;
use LineStyle::*;
use DrawFill::*;

/**
 * Image attributes
**/
pub struct PAINT {
  image: UBYTE,
  width: UWORD,
  height: UWORD,
  width_memory: UWORD,
  height_memory: UWORD,
  color: UWORD,
  rotate: UWORD,
  mirror: MirrorImage,
  width_byte: UWORD,
  height_byte: UWORD,
}

/**
 * Display rotate
**/
const ROTATE_0: UWORD = 0;
const ROTATE_90: UWORD = 90;
const ROTATE_180: UWORD = 180;
const ROTATE_270: UWORD = 270;

/**
 * Display Flip
**/
enum MirrorImage {
  MirrorNone = 0x00,
  MirrorHorizontal = 0x01,
  MirrorVertical = 0x02,
  MirrorOrigin = 0x03,
}

const MIRROR_IMAGE_DFT: MirrorImage = MirrorNone;

/**
 * image color
**/

pub const WHITE: UWORD = 0xFFFF;
pub const BLACK: UWORD = 0x0000;
pub const BLUE: UWORD = 0x001F;
pub const BRED: UWORD = 0xF81F;
pub const GRED: UWORD = 0xFFE0;
pub const GBLUE: UWORD = 0x07FF;
pub const RED: UWORD = 0xF800;
pub const MAGENTA: UWORD = 0xF81F;
pub const GREEN: UWORD = 0x07E0;
pub const CYAN: UWORD = 0x7FFF;
pub const YELLOW: UWORD = 0xFFE0;
pub const BROWN: UWORD = 0xBC40;
pub const BRRED: UWORD = 0xFC07;
pub const GRAY: UWORD = 0x8430;
pub const DARKBLUE: UWORD = 0x01CF;
pub const LIGHTBLUE: UWORD = 0x7D7C;
pub const GRAYBLUE: UWORD = 0x5458;
pub const LIGHTGREEN: UWORD = 0x841F;
pub const LGRAY: UWORD = 0xC618;
pub const LGRAYBLUE: UWORD = 0xA651;
pub const LBBLUE: UWORD = 0x2B12;

const IMAGE_BACKGROUND: UWORD = WHITE;
const FONT_FOREGROUND: UWORD = BLACK;
const FONT_BACKGROUND: UWORD = WHITE;

/**
 * The size of the point
**/
enum DotPixel {
    DotPixel1x1 = 1,  // 1 x 1
    DotPixel2x2,      // 2 x 2
    DotPixel3x3,      // 3 x 3
    DotPixel4x4,      // 4 x 4
    DotPixel5x5,      // 5 x 5
    DotPixel6x6,      // 6 x 6
    DotPixel7x7,      // 7 x 7
    DotPixel8x8,      // 8 x 8
}
const DOT_PIXEL_DFT: DotPixel = DotPixel1x1; // Default dot pilex

/**
 * Point size fill style
**/
enum DotStyle {
    DotFillAround = 1,  // dot pixel 1 x 1
    DotFillRightup,     // dot pixel 2 x 2
}
const DOT_STYLE_DFT: DotStyle = DotFillAround; // Default dot pilex

/**
 * Line style, solid or dashed
**/
enum LineStyle {
    LineStyleSolid = 0,
    LineStyleDotted,
}

/**
 * Whether the graphic is filled
**/
enum DrawFill {
    DrawFillEmpty = 0,
    DrawFillFull,
}

/**
 * Custom structure of a time attribute
**/
pub struct SPaintTime {
    year: UWORD,  // 0000
    month: UBYTE, // 1-12
    day: UBYTE,   // 1-30
    hour: UBYTE,  // 0-23
    min: UBYTE,   // 0-59
    sec: UBYTE,   // 0-59
}

const ARRAY_LEN: i32 = 50;

impl PAINT {
  /******************************************************************************
    function: Create Image
    parameter :
      image   :   Pointer to the image cache
      width   :   The width of the picture
      height  :   The height of the picture
      color   :   Whether the picture is inverted
  ******************************************************************************/
  fn new_image (&self, width: UWORD, height: UWORD, rotate: UWORD, color: UWORD) {
    self.width_memory = width;
    self.height_memory = height;
    self.color = color;
    self.width_byte = width;
    self.height_byte = height;
    self.rotate = rotate;
    self.mirror = MirrorNone;

    if rotate == ROTATE_0 || rotate == ROTATE_180 {
      self.width = width;
      self.height = height;
    } else {
      self.width = height;
      self.height = width;
    }
  }

  /******************************************************************************
    function: Select Image Rotate
      parameter:
      rotate   :   0,90,180,270
  ******************************************************************************/
  fn set_rotate (&self, rotate: UWORD) {
    if rotate == ROTATE_0 || rotate == ROTATE_90 || rotate == ROTATE_180 || rotate == ROTATE_270 {
      // debug!("Set image Rotate", Rotate);
      self.rotate;
    } else {
      // debug!("rotate = 0, 90, 180, 270");
      // Ok(())
    }
  }

  /******************************************************************************
    function: Select Image mirror
      parameter:
      mirror   :       Not mirror,Horizontal mirror,Vertical mirror,Origin mirror
  ******************************************************************************/
  fn set_mirroring (&self, mirror: MirrorImage) {
    match mirror {
      MirrorNone => {
        self.mirror;
        // debug!("mirror image x:{}, y:{}", (mirror & 0x01) ? "mirror" : "none", ((mirror >> 1) & 0x01) ? "mirror" : "none");
      },
      MirrorHorizontal => {
        self.mirror;
        // debug!("mirror image x:{}, y:{}", (mirror & 0x01) ? "mirror" : "none", ((mirror >> 1) & 0x01) ? "mirror" : "none");
      },
      MirrorVertical => {
        self.mirror;
        // debug!("mirror image x:{}, y:{}", (mirror & 0x01) ? "mirror" : "none", ((mirror >> 1) & 0x01) ? "mirror" : "none");
      },
      MirrorOrigin => {
        self.mirror;
        // debug!("mirror image x:{}, y:{}", (mirror & 0x01) ? "mirror" : "none", ((mirror >> 1) & 0x01) ? "mirror" : "none");
      },
      _ => {
        // debug!("mirror should be MIRROR_NONE, MIRROR_HORIZONTAL, MIRROR_VERTICAL or MIRROR_ORIGIN");
        // Ok(())
      }
    }
  }

  /******************************************************************************
    function: Draw Pixels
    parameter :
      xpoint  :   At point X
      ypoint  :   At point Y
      color   :   Painted colors
  ******************************************************************************/
  fn set_pixel (&self, xpoint: UWORD, ypoint: UWORD, color: UWORD) {
    if xpoint > self.width || ypoint > self.height {
      // debug!("Exceeding display boundaries");
      return;
    }
    let x: UWORD;
    let y: UWORD;
  
    match self.rotate {
      0 => {
        x = xpoint;
        y = ypoint;
      },
      90 => {
        x = self.width_memory - ypoint - 1;
        y = xpoint;
      },
      180 => {
        x = self.width_memory - xpoint - 1;
        y = self.height_memory - ypoint - 1;
      },
      270 => {
        x = ypoint;
        y = self.height_memory - xpoint - 1;
      },
      _ => return,
    }
  
    match self.mirror {
      MirrorNone => (),
      MirrorHorizontal => x = self.width_memory - x - 1,
      MirrorVertical => y = self.height_memory - y - 1,
      MirrorOrigin => {
        x = self.width_memory - x - 1;
        y = self.height_memory - y - 1;
      },
      _ => return,
    }
  
    // println!("x = {}, y = {}", x, y);
    if x > self.width_memory || y > self.height_memory {
      // debug!("Exceeding display boundaries");
      return;
    }
  
    // let addr: UDOUBLE = x / 8 + y * self.width_byte;
    lcd_draw_point(x, y, color);
  }

  /******************************************************************************
    function: Clear the color of the picture
    parameter :
      color   :   Painted colors
  ******************************************************************************/
  fn clear (&self, color: UWORD) {
    lcd_set_window(0, 0, self.width_byte , self.height_byte);
    for y in 0..self.height_byte {
      for x in 0..self.width_byte { // 8 pixel = 1 byte
        lcd_write_data_word(color);
      }
    }    
  }

  /******************************************************************************
    function: Clear the color of a window
    parameter:
      xstart :   x starting point
      ystart :   y starting point
      xend   :   x end point
      yend   :   y end point
  ******************************************************************************/
  fn clear_windows (&self, xstart: UWORD, ystart: UWORD, xend: UWORD, yend: UWORD, color: UWORD) {
    let x: UWORD;
    let y: UWORD;
    for y in ystart..yend {
      for x in xstart..xend { // 8 pixel = 1 byte
        self.set_pixel(x, y, color);
      }
    }
  }

  /******************************************************************************
    function:	Draw Point(Xpoint, Ypoint) Fill the color
    parameter :
      xpoint  :   The xpoint coordinate of the point
      ypoint	:   The ypoint coordinate of the point
      color		:   Set color
    dot_Pixel	:	  point size
  ******************************************************************************/
  fn draw_point (&self, xpoint: UWORD, ypoint: UWORD, color: UWORD,
      dot_pixel: DotPixel, dot_fill_way: DotStyle) {
    if xpoint > self.width || ypoint > self.height {
      debug!("Paint_DrawPoint Input exceeds the normal display range");
      return;
    }

    if dot_fill_way == DotFillAround {
      for xdir_num in 0..(2 * dot_pixel - 1) {
        for ydir_num in 0..(2 * dot_pixel - 1) {
          if xpoint + xdir_num - dot_pixel < 0 || ypoint + ydir_num - dot_pixel < 0 {
            break;
          }
          // println!("x = {}, y = {}", xpoint + XDir_Num - Dot_Pixel, Ypoint + YDir_Num - Dot_Pixel);
          self.set_pixel(xpoint + xdir_num - dot_pixel, ypoint + ydir_num - dot_pixel, color);
        }
      }
    } else {
      for xdir_num in 0..dot_pixel {
        for ydir_num in 0..dot_pixel {
          self.set_pixel(xpoint + xdir_num - 1, ypoint + ydir_num - 1, color);
        }
      }
    }
  }

  /******************************************************************************
  function:	Draw a line of arbitrary slope
    parameter:
      xstart : Starting Xpoint point coordinates
      ystart : Starting Xpoint point coordinates
      xend   : End point Xpoint coordinate
      yend   : End point Ypoint coordinate
      color  : The color of the line segment
  ******************************************************************************/
  fn draw_line(&self, xstart: UWORD, ystart: UWORD, xend: UWORD, yend: UWORD,
      color: UWORD, line_width: DotPixel, line_style: LineStyle) {
    if xstart > self.width || ystart > self.height ||
        xend > self.width || yend > self.height {
      debug!("Paint_DrawLine Input exceeds the normal display range");
      return;
    }

    let xpoint: UWORD = xstart;
    let ypoint: UWORD = ystart;
    let dx: UWORD;
    let dy: UWORD;
    if xend - xstart >= 0 {
      dx = xend - xstart;
    } else {
      dx = xstart - xend;
    }
    if yend - ystart <= 0 {
      dy = yend - ystart;
    } else {
      dy = ystart - yend;
    }

    // Increment direction, 1 is positive, -1 is counter;
    let xaddway: UWORD;
    let yaddway: UWORD;
    if xstart < xend {
      xaddway = 1;
    } else {
      xaddway = -1;
    }
    if ystart < yend {
      yaddway = 1;
    } else {
      yaddway = -1;
    }

    // Cumulative error
    let esp: UWORD = dx + dy;
    let dotted_len: u8 = 0;

    loop {
      dotted_len += 1;
      // Painted dotted line, 2 point is really virtual
      if line_style == LineStyleDotted && dotted_len % 3 == 0 {
        // debug!("LINE_DOTTED");
        self.draw_point(xpoint, ypoint, IMAGE_BACKGROUND, line_width, DOT_STYLE_DFT);
        dotted_len = 0;
      } else {
        self.draw_point(xpoint, ypoint, color, line_width, DOT_STYLE_DFT);
      }
      if 2 * esp >= dy {
        if xpoint == xend {
          break;
        }
        esp += dy;
        xpoint += xaddway;
      }
      if 2 * esp <= dx {
        if ypoint == yend {
          break;
        }
        esp += dx;
        ypoint += yaddway;
      }
    }
  }

  /******************************************************************************
  function:	Draw a rectangle
    parameter:
      xstart : Rectangular  Starting Xpoint point coordinates
      ystart : Rectangular  Starting Xpoint point coordinates
      xend   : Rectangular  End point Xpoint coordinate
      yend   : Rectangular  End point Ypoint coordinate
      color  : The color of the Rectangular segment
      filled : Whether it is filled--- 1 solid 0：empty
  ******************************************************************************/
  fn draw_rectangle (&self, xstart: UWORD, ystart: UWORD, xend: UWORD, yend: UWORD,
      color: UWORD, line_width: DotPixel, filled: DrawFill) {
    if xstart > self.width || ystart > self.height ||
        xend > self.width || yend > self.height {
      debug!("Input exceeds the normal display range");
      return;
    }

    if filled {
      let ypoint: UWORD;
      for ypoint in ystart..yend {
        self.draw_line(xstart, ypoint, xend, ypoint, color ,line_width, LineStyleSolid);
      }
    } else {
      self.draw_line(xstart, ystart, xend, ystart, color ,line_width, LineStyleSolid);
      self.draw_line(xstart, ystart, xstart, yend, color ,line_width, LineStyleSolid);
      self.draw_line(xend, yend, xend, ystart, color ,line_width, LineStyleSolid);
      self.draw_line(xend, yend, xstart, yend, color ,line_width, LineStyleSolid);
    }
  }

  /******************************************************************************
    function:	Use the 8-point method to draw a circle of the
              specified size at the specified position->
      parameter :
      x_Center  : Center X coordinate
      y_Center  : Center Y coordinate
      radius    : circle Radius
      color     : The color of the ：circle segment
      filled    : Whether it is filled: 1 filling 0：Do not
  ******************************************************************************/
  fn draw_circle (&self, x_center: UWORD, y_center: UWORD, radius: UWORD,
      color: UWORD, line_width: DotPixel, draw_fill: DrawFill) {
    if x_center > self.width || y_center >= self.height {
      debug!("Paint_DrawCircle Input exceeds the normal display range");
      return;
    }

    // Draw a circle from(0, R) as a starting point
    let xcurrent: UWORD = 0;
    let ycurrent: UWORD = radius;

    // Cumulative error,judge the next point of the logo
    let esp: UWORD = 3 - (radius << 1);
    let s_count_y: i16;
    if draw_fill == DrawFillFull {
      while xcurrent <= ycurrent { // Realistic circles
        for sCountY in xcurrent..(ycurrent + 1) {
          self.draw_point(x_center + xcurrent, y_center + sCountY, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 1
          self.draw_point(x_center - xcurrent, y_center + sCountY, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 2
          self.draw_point(x_center - sCountY, y_center + xcurrent, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 3
          self.draw_point(x_center - sCountY, y_center - xcurrent, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 4
          self.draw_point(x_center - xcurrent, y_center - sCountY, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 5
          self.draw_point(x_center + xcurrent, y_center - sCountY, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 6
          self.draw_point(x_center + sCountY, y_center - xcurrent, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 7
          self.draw_point(x_center + sCountY, y_center + xcurrent, color, DOT_PIXEL_DFT, DOT_STYLE_DFT);
        }
        if esp < 0 {
          esp += 4 * xcurrent + 6;
        } else {
          esp += 10 + 4 * (xcurrent - ycurrent );
          ycurrent -= 1;
        }
        xcurrent += 1;
      }
    } else { // Draw a hollow circle
      while xcurrent <= ycurrent {
        self.draw_point(x_center + xcurrent, y_center + ycurrent, color, line_width, DOT_STYLE_DFT); // 1
        self.draw_point(x_center - xcurrent, y_center + ycurrent, color, line_width, DOT_STYLE_DFT); // 2
        self.draw_point(x_center - ycurrent, y_center + xcurrent, color, line_width, DOT_STYLE_DFT); // 3
        self.draw_point(x_center - ycurrent, y_center - xcurrent, color, line_width, DOT_STYLE_DFT); // 4
        self.draw_point(x_center - xcurrent, y_center - ycurrent, color, line_width, DOT_STYLE_DFT); // 5
        self.draw_point(x_center + xcurrent, y_center - ycurrent, color, line_width, DOT_STYLE_DFT); // 6
        self.draw_point(x_center + ycurrent, y_center - xcurrent, color, line_width, DOT_STYLE_DFT); // 7
        self.draw_point(x_center + ycurrent, y_center + xcurrent, color, line_width, DOT_STYLE_DFT); // 0

        if esp < 0 {
          esp += 4 * xcurrent + 6;
        } else {
          esp += 10 + 4 * (xcurrent - ycurrent );
          ycurrent -= 1;
        }
        xcurrent += 1;
      }
    }
  }

  /******************************************************************************
  function: Show English characters
  parameter:
    xpoint           : X coordinate
    ypoint           : Y coordinate
    acsii_char       : To display the English characters
    font             : A structure pointer that displays a character size
    color_background : Select the background color of the English character
    color_foreground : Select the foreground color of the English character
  ******************************************************************************/
  fn draw_char(&self, xpoint: UWORD, ypoint: UWORD, acsii_char: char,
      font: SFONT, color_background: UWORD, color_foreground: UWORD) {
    let page: UWORD;
    let column: UWORD;

    if xpoint > self.width || ypoint > self.height {
      // debug!("Paint_DrawChar Input exceeds the normal display range");
      return;
    }
    let char_offset: u32;
    if font.width % 8 {
      char_offset = (acsii_char - ' ') * font.height * (font.width / 8 + 1);
    } else {
      char_offset = (acsii_char - ' ') * font.height * (font.width / 8);
    }

    let ptr: char = font.table[char_offset];

    for Page in 0..font.height {
      for Column in 0..font.width {
        // To determine whether the font background color and screen background color is consistent
        if FONT_BACKGROUND == color_background { // this process is to speed up the scan
          if pgm_read_byte(ptr) & (0x80 >> (column % 8)) {
            self.set_pixel (xpoint + column, ypoint + Page, color_foreground );
          }
        } else {
          if pgm_read_byte(ptr) & (0x80 >> (column % 8)) {
            self.set_pixel (xpoint + column, ypoint + Page, color_foreground );
          } else {
            self.set_pixel (xpoint + column, ypoint + Page, color_background );
          }
        }
        // One pixel is 8 bits
        if column % 8 == 7 {
          ptr += 1;
        }
      } /* Write a line */
      if font.width % 8 != 0 {
        ptr += 1;
      }
    }/* Write all */
  }

  /******************************************************************************
  function: Display the string
  parameter:
    xstart           : X coordinate
    ystart           : Y coordinate
    p_string          : The first address of the English string to be displayed
    font             : A structure pointer that displays a character size
    color_background : Select the background color of the English character
    color_foreground : Select the foreground color of the English character
  ******************************************************************************/
  fn draw_string_en(&self, xstart: UWORD, ystart: UWORD, p_string: char,
      font: SFONT, color_background: UWORD, color_foreground: UWORD) {
    let xpoint: UWORD = xstart;
    let ypoint: UWORD = ystart;

    if xstart > self.width || ystart > self.height {
      // debug!("Paint_DrawString_EN Input exceeds the normal display range");
      return;
    }

    while p_string != '\0' {
      // if x direction filled , reposition to (xstart,ypoint), ypoint is y direction plus the height of the character
      if xpoint + font.width > self.width {
        xpoint = xstart;
        ypoint += font.height;
      }

      // If the Y direction is full, reposition to(Xstart, Ystart)
      if ypoint  + font.height > self.height {
        xpoint = xstart;
        ypoint = ystart;
      }
      self.draw_char(xpoint, ypoint, p_string, font, color_background, color_foreground);

      //The next character of the address
      p_string += 1;

      //The next word of the abscissa increases the font of the broadband
      xpoint += font.width;
    }
  }

  /******************************************************************************
  function: Display the string
  parameter:
    xstart           : X coordinate
    ystart           : Y coordinate
    p_string         : The first address of the Chinese string and English
                       string to be displayed
    font             : A structure pointer that displays a character size
    color_background : Select the background color of the English character
    color_foreground : Select the foreground color of the English character
  ******************************************************************************/
  fn draw_string_cn (&self, xstart: UWORD, ystart: UWORD, p_string: &char, font: cFONT,
      color_background: UWORD, color_foreground: UWORD) {
    let p_text: char = p_string;

    let refcolumn: UWORD = xstart;

    /* Send the string character by character on EPD */
    while p_text != 0 {
      if p_text < 0x7F { //ASCII
        for num in 0..font.size {
          if p_text == pgm_read_byte(font.table[num].index[0]) {
            let ptr: char = font.table[num].matrix[0];

            for j in 0..font.height {
              for i in 0..font.width {
                if pgm_read_byte(ptr) & (0x80 >> (i % 8)) {
                  paint_set_pixel(refcolumn + i,ystart + j, color_foreground);
                }
                if i % 8 == 7 {
                  ptr += 1;
                }
              }
              if font.width % 8 != 0 {
                ptr += 1;
              }
            }
            break;
          }
        }
        /* Point on the next character */
        p_text += 1;
        /* Decrement the column position by 16 */
        refcolumn += font.ascii_width;
      } else { // 中文
        for num in 0..font.size {
          if p_text == pgm_read_byte(font.table[num].index[0]) && ((p_text + 1) == pgm_read_byte(font.table[num].index[1])) && ((p_text + 2) == pgm_read_byte(font.table[num].index[2])) {
            let ptr: char = font.table[num].matrix[0];
            for j in 0..font.height {
              for i in 0..font.width {
                if pgm_read_byte(ptr) & (0x80 >> (i % 8)) {
                  paint_set_pixel(refcolumn + i,ystart + j, color_foreground);
                }
                if i % 8 == 7 {
                  ptr += 1;
                }
              }
              if font.width % 8 != 0 {
                ptr += 1;
              }
            }
            break;
          }
        }
        /* Point on the next character */
        p_text += 3;
        /* Decrement the column position by 16 */
        refcolumn += font.width;
      }
    }
  }

  /******************************************************************************
  function: Display nummber
  parameter:
    xstart           : X coordinate
    ystart           : Y coordinate
    number           : The number displayed
    font             : A structure pointer that displays a character size
    color_background : Select the background color of the English character
    color_foreground : Select the foreground color of the English character
  ******************************************************************************/

  fn draw_num (&self, xpoint: UWORD, ypoint: UWORD, number: i32,
      font: SFONT, color_background: UWORD, color_foreground: UWORD) {

    let num_bit: i16 = 0;
    let str_bit: i16 = 0;
    let str_array: [u8; ARRAY_LEN] = {0};
    let num_array: [u8; ARRAY_LEN] = {0};
    let &pStr: u8 = str_array;

    if xpoint > self.width || ypoint > self.height {
      // debug!("Paint_DisNum Input exceeds the normal display range");
      return;
    }

    // Converts a number to a string
    loop {
      num_array[num_bit] = number % 10 + '0';
      num_bit += 1;
      number /= 10;

      if !number {
        break;
      }
    }

    // The string is inverted
    while num_bit > 0 {
      str_array[str_bit] = num_array[num_bit - 1];
      str_bit += 1;
      num_bit -= 1;
    }

    // show
    self.draw_string_en(xpoint, ypoint, pStr, font, color_background, color_foreground);
  }

  /******************************************************************************
  function:	Display float number
  parameter:
    xstart           : X coordinate
    ystart           : Y coordinate
    number           : The float data that you want to display
    decimal_point    : Show decimal places
    font             : A structure pointer that displays a character size
    color            : Select the background color of the English character
  ******************************************************************************/
  fn draw_float_num (&self, xpoint: UWORD, ypoint: UWORD, number: f64, decimal_point: UBYTE,
      font: SFONT, color_background: UWORD, color_foreground: UWORD) {
    let str: [char; ARRAY_LEN] = {0};
    dtostrf(number, 0, decimal_point + 2, str);
    let p_str: char= (char *)malloc((strlen(str)) * sizeof(char));
    memcpy(p_str, str, (strlen(str) - 2));
    * (p_str + strlen(str) - 1) = '\0';
    if (*(p_str + strlen(str) - 3)) == '.' {
      *(p_str + strlen(str) - 3) = '\0';
    }
    // show
    self.draw_string_en(xpoint, ypoint, p_str, font, color_foreground, color_background);
    free(p_str);
    p_str = NULL;
  }

  /******************************************************************************
  function: Display time
  parameter:
    xstart           : X coordinate
    ystart           : Y coordinate
    p_time           : Time-related structures
    font             : A structure pointer that displays a character size
    color            : Select the background color of the English character
  ******************************************************************************/
  fn draw_time (&self, xstart: UWORD, ystart: UWORD, p_time: &SPaintTime, font: SFONT,
      color_background: UWORD, color_foreground: UWORD) {
    let value: [u8] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let dx: UWORD = font.width;

    // Write data into the cache
    self.draw_char(xstart                           , ystart, value[p_time.hour / 10], font, color_background, color_foreground);
    self.draw_char(xstart + dx                      , ystart, value[p_time.hour % 10], font, color_background, color_foreground);
    self.draw_char(xstart + dx  + dx / 4 + dx / 2   , ystart, ':'                    , font, color_background, color_foreground);
    self.draw_char(xstart + dx * 2 + dx / 2         , ystart, value[p_time.min / 10] , font, color_background, color_foreground);
    self.draw_char(xstart + dx * 3 + dx / 2         , ystart, value[p_time.min % 10] , font, color_background, color_foreground);
    self.draw_char(xstart + dx * 4 + dx / 2 - dx / 4, ystart, ':'                    , font, color_background, color_foreground);
    self.draw_char(xstart + dx * 5                  , ystart, value[p_time.sec / 10] , font, color_background, color_foreground);
    self.draw_char(xstart + dx * 6                  , ystart, value[p_time.sec % 10] , font, color_background, color_foreground);
  }

  /******************************************************************************
  function: Display image
  parameter:
    image            : Image start address
    xstart           : X starting coordinates
    ystart           : Y starting coordinates
    xend             : Image width
    yend             : Image height
  ******************************************************************************/
  fn draw_image (&self, image: char, x_start: UWORD, y_start: UWORD, w_image: UWORD, h_image: UWORD) {
    for j in 0..h_image {
      for i in 0..w_image {
        if x_start + i < LCD_WIDTH  &&  y_start + j < LCD_HEIGHT { // Exceeded part does not display
          self.set_pixel(x_start + i, y_start + j, (pgm_read_byte(image + j * w_image * 2 + i * 2 + 1)) << 8 | (pgm_read_byte(image + j * w_image * 2 + i * 2)));
        }
        // Using arrays is a property of sequential storage, accessing the original array by algorithm
        // j*w_image*2          Y offset
        // i*2                  X offset
        // pgm_read_byte()
      }
    }
  }
}
