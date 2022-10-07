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
mod Debug;

use DEV_Config::*;
use LCD_Driver::*;
use MirrorImage::*;
use DotPixel::*;
use DotStyle::*;
use LineStyle::*;
use Debug::*;
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
pub const Paint: PAINT = PAINT {};

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

/******************************************************************************
  function: Create Image
  parameter :
    image   :   Pointer to the image cache
    width   :   The width of the picture
    Height  :   The height of the picture
    Color   :   Whether the picture is inverted
******************************************************************************/
fn paint_new_image(width: UWORD, height: UWORD, rotate: UWORD, color: UWORD) {
  Paint.width_memory = width;
  Paint.height_memory = height;
  Paint.color = color;
  Paint.width_byte = width;
  Paint.height_byte = height;
  
  Paint.rotate = rotate;
  Paint.mirror = MirrorNone;

  if rotate == ROTATE_0 || rotate == ROTATE_180 {
    Paint.width = width;
    Paint.height = height;
  } else {
    Paint.width = height;
    Paint.height = width;
  }
}

/******************************************************************************
  function: Select Image Rotate
    parameter:
    Rotate   :   0,90,180,270
******************************************************************************/
fn paint_set_rotate(rotate: UWORD) -> () {
  if rotate == ROTATE_0 || rotate == ROTATE_90 || rotate == ROTATE_180 || rotate == ROTATE_270 {
    // Debug("Set image Rotate %d\r\n", Rotate);
    Paint.rotate = rotate;
  } else {
    // Debug("rotate = 0, 90, 180, 270\r\n");
    // exit(0);
  }
}

/******************************************************************************
  function: Select Image mirror
    parameter:
    mirror   :       Not mirror,Horizontal mirror,Vertical mirror,Origin mirror
******************************************************************************/
fn paint_set_mirroring(mirror: MirrorImage) -> () {
  match mirror {
    MirrorNone => {
      Paint.mirror = mirror;
      // Debug("mirror image x:%s, y:%s\r\n", (mirror & 0x01) ? "mirror" : "none", ((mirror >> 1) & 0x01) ? "mirror" : "none");
    },
    MirrorHorizontal => {
      Paint.mirror = mirror;
      // Debug("mirror image x:%s, y:%s\r\n", (mirror & 0x01) ? "mirror" : "none", ((mirror >> 1) & 0x01) ? "mirror" : "none");
    },
    MirrorVertical => {
      Paint.mirror = mirror;
      // Debug("mirror image x:%s, y:%s\r\n", (mirror & 0x01) ? "mirror" : "none", ((mirror >> 1) & 0x01) ? "mirror" : "none");
    },
    MirrorOrigin => {
      Paint.mirror = mirror;
      // Debug("mirror image x:%s, y:%s\r\n", (mirror & 0x01) ? "mirror" : "none", ((mirror >> 1) & 0x01) ? "mirror" : "none");
    },
    _ => {
      // Debug("mirror should be MIRROR_NONE, MIRROR_HORIZONTAL, \
      // MIRROR_VERTICAL or MIRROR_ORIGIN\r\n");
      // exit(0);
    }
  }
}

/******************************************************************************
  function: Draw Pixels
  parameter :
    Xpoint  :   At point X
    Ypoint  :   At point Y
    Color   :   Painted colors
******************************************************************************/
fn paint_set_pixel(xpoint: UWORD, ypoint: UWORD, color: UWORD) -> () {
  if xpoint > Paint.width || ypoint > Paint.height {
    // Debug("Exceeding display boundaries\r\n");
    return;
  }
  let x: UWORD;
  let y: UWORD;

  match Paint.rotate {
    0 => {
      x = xpoint;
      y = ypoint;
    },
    90 => {
      x = Paint.width_memory - ypoint - 1;
      y = xpoint;
    },
    180 => {
      x = Paint.width_memory - xpoint - 1;
      y = Paint.height_memory - ypoint - 1;
    },
    270 => {
      x = ypoint;
      y = Paint.height_memory - xpoint - 1;
    },
    _ => return,
  }

  match Paint.mirror {
    MirrorNone => (),
    MirrorHorizontal => x = Paint.width_memory - x - 1,
    MirrorVertical => y = Paint.height_memory - y - 1,
    MirrorOrigin => {
      x = Paint.width_memory - x - 1;
      y = Paint.height_memory - y - 1;
    },
    _ => return,
  }

  // printf("x = %d, y = %d\r\n", X, Y);
  if x > Paint.width_memory || y > Paint.height_memory {
    // Debug("Exceeding display boundaries\r\n");
    return;
  }

  // UDOUBLE Addr = X / 8 + Y * Paint.WidthByte;
  lcd_draw_point(x, y, color);
}

/******************************************************************************
  function: Clear the color of the picture
  parameter :
    Color   :   Painted colors
******************************************************************************/
fn paint_clear(color: UWORD) -> () {
  lcd_set_window(0, 0, Paint.width_byte , Paint.height_byte);
  for y in 0..Paint.height_byte {
    for x in 0..Paint.width_byte { // 8 pixel = 1 byte
      lcd_write_data_word(color);
    }
  }
}

/******************************************************************************
  function: Clear the color of a window
  parameter:
    Xstart :   x starting point
    Ystart :   y starting point
    Xend   :   x end point
    Yend   :   y end point
******************************************************************************/
fn paint_clear_windows(xstart: UWORD, ystart: UWORD, xend: UWORD, yend: UWORD, color: UWORD) -> () {
  let x: UWORD;
  let y: UWORD;
  for y in ystart..yend {
    for x in xstart..xend { // 8 pixel = 1 byte
      paint_set_pixel(x, y, color);
    }
  }
}

/******************************************************************************
  function:	Draw Point(Xpoint, Ypoint) Fill the color
  parameter :
    Xpoint  :   The xpoint coordinate of the point
    Ypoint	:   The ypoint coordinate of the point
    Color		:   Set color
  Dot_Pixel	:	  point size
******************************************************************************/
fn paint_draw_point(xpoint: UWORD, ypoint: UWORD, color: UWORD,
    dot_pixel: DotPixel, dot_fill_way: DotStyle) -> () {
  if xpoint > Paint.width || ypoint > Paint.height {
    debug("Paint_DrawPoint Input exceeds the normal display range\r\n");
    return;
  }

  let xdir_num: i16;
  let ydir_num: i16;
  if dot_fill_way == DotFillAround {
    for XDir_Num in 0..(2 * dot_pixel - 1) {
      for YDir_Num in 0..(2 * dot_pixel - 1) {
        if xpoint + XDir_Num - dot_pixel < 0 || ypoint + YDir_Num - dot_pixel < 0 {
          break;
        }
        // printf("x = %d, y = %d\r\n", Xpoint + XDir_Num - Dot_Pixel, Ypoint + YDir_Num - Dot_Pixel);
        paint_set_pixel(xpoint + XDir_Num - dot_pixel, ypoint + YDir_Num - dot_pixel, color);
      }
    }
  } else {
    for XDir_Num in 0..dot_pixel {
      for YDir_Num in 0..dot_pixel {
        paint_set_pixel(xpoint + XDir_Num - 1, ypoint + YDir_Num - 1, color);
      }
    }
  }
}

/******************************************************************************
function:	Draw a line of arbitrary slope
  parameter:
    Xstart : Starting Xpoint point coordinates
    Ystart : Starting Xpoint point coordinates
    Xend   : End point Xpoint coordinate
    Yend   : End point Ypoint coordinate
    Color  : The color of the line segment
******************************************************************************/
fn paint_draw_line(xstart: UWORD, ystart: UWORD, xend: UWORD, yend: UWORD,
    color: UWORD, line_width: DotPixel, line_style: LineStyle) -> () {
  if xstart > Paint.width || ystart > Paint.height ||
      xend > Paint.width || yend > Paint.height {
    debug("Paint_DrawLine Input exceeds the normal display range\r\n");
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
      // debug("LINE_DOTTED\r\n");
      paint_draw_point(xpoint, ypoint, IMAGE_BACKGROUND, line_width, DOT_STYLE_DFT);
      dotted_len = 0;
    } else {
      paint_draw_point(xpoint, ypoint, color, line_width, DOT_STYLE_DFT);
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
    Xstart : Rectangular  Starting Xpoint point coordinates
    Ystart : Rectangular  Starting Xpoint point coordinates
    Xend   : Rectangular  End point Xpoint coordinate
    Yend   : Rectangular  End point Ypoint coordinate
    Color  : The color of the Rectangular segment
    Filled : Whether it is filled--- 1 solid 0：empty
******************************************************************************/
fn paint_draw_rectangle(xstart: UWORD, ystart: UWORD, xend: UWORD, yend: UWORD,
    color: UWORD, line_width: DotPixel, filled: DrawFill) -> () {
  if xstart > Paint.width || ystart > Paint.height ||
      xend > Paint.width || yend > Paint.height {
    debug("Input exceeds the normal display range\r\n");
    return;
  }

  if filled {
    let ypoint: UWORD;
    for ypoint in ystart..yend {
      paint_draw_line(xstart, ypoint, xend, ypoint, color ,line_width, LineStyleSolid);
    }
  } else {
    paint_draw_line(xstart, ystart, xend, ystart, color ,line_width, LineStyleSolid);
    paint_draw_line(xstart, ystart, xstart, yend, color ,line_width, LineStyleSolid);
    paint_draw_line(xend, yend, xend, ystart, color ,line_width, LineStyleSolid);
    paint_draw_line(xend, yend, xstart, yend, color ,line_width, LineStyleSolid);
  }
}

/******************************************************************************
  function:	Use the 8-point method to draw a circle of the
            specified size at the specified position->
    parameter :
    X_Center  : Center X coordinate
    Y_Center  : Center Y coordinate
    Radius    : circle Radius
    Color     : The color of the ：circle segment
    Filled    : Whether it is filled: 1 filling 0：Do not
******************************************************************************/
fn paint_draw_circle(x_center: UWORD, y_center: UWORD, radius: UWORD,
    color: UWORD, line_width: DotPixel, draw_fill: DrawFill) {
  if x_center > Paint.width || y_center >= Paint.height {
    debug("Paint_DrawCircle Input exceeds the normal display range\r\n");
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
        paint_draw_point(x_center + xcurrent, y_center + sCountY, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 1
        paint_draw_point(x_center - xcurrent, y_center + sCountY, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 2
        paint_draw_point(x_center - sCountY, y_center + xcurrent, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 3
        paint_draw_point(x_center - sCountY, y_center - xcurrent, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 4
        paint_draw_point(x_center - xcurrent, y_center - sCountY, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 5
        paint_draw_point(x_center + xcurrent, y_center - sCountY, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 6
        paint_draw_point(x_center + sCountY, y_center - xcurrent, color, DOT_PIXEL_DFT, DOT_STYLE_DFT); // 7
        paint_draw_point(x_center + sCountY, y_center + xcurrent, color, DOT_PIXEL_DFT, DOT_STYLE_DFT);
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
      paint_draw_point(x_center + xcurrent, y_center + ycurrent, color, line_width, DOT_STYLE_DFT); // 1
      paint_draw_point(x_center - xcurrent, y_center + ycurrent, color, line_width, DOT_STYLE_DFT); // 2
      paint_draw_point(x_center - ycurrent, y_center + xcurrent, color, line_width, DOT_STYLE_DFT); // 3
      paint_draw_point(x_center - ycurrent, y_center - xcurrent, color, line_width, DOT_STYLE_DFT); // 4
      paint_draw_point(x_center - xcurrent, y_center - ycurrent, color, line_width, DOT_STYLE_DFT); // 5
      paint_draw_point(x_center + xcurrent, y_center - ycurrent, color, line_width, DOT_STYLE_DFT); // 6
      paint_draw_point(x_center + ycurrent, y_center - xcurrent, color, line_width, DOT_STYLE_DFT); // 7
      paint_draw_point(x_center + ycurrent, y_center + xcurrent, color, line_width, DOT_STYLE_DFT); // 0

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
    Xpoint           : X coordinate
    Ypoint           : Y coordinate
    Acsii_Char       : To display the English characters
    Font             : A structure pointer that displays a character size
    Color_Background : Select the background color of the English character
    Color_Foreground : Select the foreground color of the English character
******************************************************************************/
fn paint_draw_char(xpoint: UWORD, ypoint: UWORD, acsii_char: char,
    font: sFONT, color_background: UWORD, color_foreground: UWORD) {
  let page: UWORD;
  let column: UWORD;

  if xpoint > Paint.width || ypoint > Paint.height {
    // Debug("Paint_DrawChar Input exceeds the normal display range\r\n");
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
          paint_set_pixel (xpoint + column, ypoint + Page, color_foreground );
        }
      } else {
        if pgm_read_byte(ptr) & (0x80 >> (column % 8)) {
          paint_set_pixel (xpoint + column, ypoint + Page, color_foreground );
        } else {
          paint_set_pixel (xpoint + column, ypoint + Page, color_background );
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
    Xstart           : X coordinate
    Ystart           : Y coordinate
    pString          : The first address of the English string to be displayed
    Font             : A structure pointer that displays a character size
    Color_Background : Select the background color of the English character
    Color_Foreground : Select the foreground color of the English character
******************************************************************************/
fn paint_draw_string_en(xstart: UWORD, ystart: UWORD, p_string: char,
    font: sFONT, color_background: UWORD, color_foreground: UWORD) {
  let xpoint: UWORD = xstart;
  let ypoint: UWORD = ystart;

  if xstart > Paint.width || ystart > Paint.height {
    // debug("Paint_DrawString_EN Input exceeds the normal display range\r\n");
    return;
  }

  while p_string != '\0' {
    // if X direction filled , reposition to(Xstart,Ypoint),Ypoint is Y direction plus the Height of the character
    if xpoint + font.width > Paint.width {
      xpoint = xstart;
      ypoint += font.height;
    }

    // If the Y direction is full, reposition to(Xstart, Ystart)
    if ypoint  + font.height > Paint.height {
      xpoint = xstart;
      ypoint = ystart;
    }
    paint_draw_char(xpoint, ypoint, p_string, font, color_background, color_foreground);

    //The next character of the address
    p_string += 1;

    //The next word of the abscissa increases the font of the broadband
    xpoint += font.width;
  }
}

/******************************************************************************
  function: Display the string
  parameter:
    Xstart           : X coordinate
    Ystart           : Y coordinate
    pString          : The first address of the Chinese string and English
                       string to be displayed
    Font             : A structure pointer that displays a character size
    Color_Background : Select the background color of the English character
    Color_Foreground : Select the foreground color of the English character
******************************************************************************/
fn paint_draw_string_cn(xstart: UWORD, ystart: UWORD, p_string: &char, font: cFONT,
    color_background: UWORD, color_foreground: UWORD) {
  const p_text: uchar = pString;

  let refcolumn: UWORD = xstart;
  let i: i32;
  let j: i32;
  let num: i32;

  /* Send the string character by character on EPD */
  while p_text != 0 {
    if p_text < 0x7F { //ASCII
      for Num in 0..font.size {
        if p_text == pgm_read_byte(font.table[Num].index[0]) {
          const ptr: char = font.table[Num].matrix[0];

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
      for Num in 0..font.size {
        if p_text == pgm_read_byte(font.table[Num].index[0]) && ((p_text + 1) == pgm_read_byte(font.table[Num].index[1])) && ((p_text + 2) == pgm_read_byte(font.table[Num].index[2])) {
          const ptr: char = font.table[Num].matrix[0];
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
    Xstart           : X coordinate
    Ystart           : Y coordinate
    Nummber          : The number displayed
    Font             : A structure pointer that displays a character size
    Color_Background : Select the background color of the English character
    Color_Foreground : Select the foreground color of the English character
******************************************************************************/
const ARRAY_LEN: i32 = 50;

fn paint_draw_num(xpoint: UWORD, ypoint: UWORD, number: i32,
    font: sFONT, color_background: UWORD, color_foreground: UWORD) {

  let num_bit: i16 = 0;
  let str_bit: i16 = 0;
  let str_array[ARRAY_LEN]: u8 = {0}, Num_Array[ARRAY_LEN] = {0};
  let &pStr: u8 = str_array;

  if xpoint > Paint.width || ypoint > Paint.height {
    // debug("Paint_DisNum Input exceeds the normal display range\r\n");
    return;
  }

  // Converts a number to a string
  do {
    Num_Array[Num_Bit] = Number % 10 + '0';
    Num_Bit += 1;
    Number /= 10;
  } while (number);

  // The string is inverted
  while (num_bit > 0) {
    str_array[str_bit] = Num_Array[num_bit - 1];
    str_bit += 1;
    num_bit -= 1;
  }

  // show
  paint_draw_string_en(xpoint, ypoint, pStr, font, color_background, color_foreground);
}

/******************************************************************************
function:	Display float number
parameter:
    Xstart           : X coordinate
    Ystart           : Y coordinate
    Nummber          : The float data that you want to display
	Decimal_Point	     : Show decimal places
    Font             : A structure pointer that displays a character size
    Color            : Select the background color of the English character
******************************************************************************/
fn paint_draw_float_num(xpoint: UWORD, ypoint: UWORD, number: f64, decimal_point: UBYTE,
    font: sFONT, color_background: UWORD, color_foreground: UWORD) {
  let str[ARRAY_LEN]: char = {0};
  dtostrf(number, 0, decimal_point + 2, str);
  let p_str: &char= (char *)malloc((strlen(str)) * sizeof(char));
  memcpy(p_str, str, (strlen(str) - 2));
  * (p_str + strlen(str) - 1) = '\0';
  if (*(p_str + strlen(str) - 3)) == '.' {
	  *(p_str + strlen(str) - 3) = '\0';
  }
  // show
  paint_draw_string_en(xpoint, ypoint, p_str, font, color_foreground, color_background);
  free(p_str);
  p_str = NULL;
}

/******************************************************************************
  function: Display time
  parameter:
    Xstart           : X coordinate
    Ystart           : Y coordinate
    pTime            : Time-related structures
    Font             : A structure pointer that displays a character size
    Color            : Select the background color of the English character
******************************************************************************/
fn paint_draw_time(xstart: UWORD, ystart: UWORD, p_time: &SPaintTime, font: sFONT,
    color_background: UWORD, color_foreground: UWORD) {
  value: [u8] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

  let dx: UWORD = (*font).Width;

  // Write data into the cache
  paint_draw_char(xstart                           , ystart, value[p_time.hour / 10], font, color_background, color_foreground);
  paint_draw_char(xstart + dx                      , ystart, value[p_time.hour % 10], font, color_background, color_foreground);
  paint_draw_char(xstart + dx  + dx / 4 + dx / 2   , ystart, ':'                    , font, color_background, color_foreground);
  paint_draw_char(xstart + dx * 2 + dx / 2         , ystart, value[p_time.min / 10] , font, color_background, color_foreground);
  paint_draw_char(xstart + dx * 3 + dx / 2         , ystart, value[p_time.min % 10] , font, color_background, color_foreground);
  paint_draw_char(xstart + dx * 4 + dx / 2 - dx / 4, ystart, ':'                    , font, color_background, color_foreground);
  paint_draw_char(xstart + dx * 5                  , ystart, value[p_time.sec / 10] , font, color_background, color_foreground);
  paint_draw_char(xstart + dx * 6                  , ystart, value[p_time.sec % 10] , font, color_background, color_foreground);
}

/******************************************************************************
  function: Display image
  parameter:
    image            : Image start address
    xStart           : X starting coordinates
    yStart           : Y starting coordinates
    xEnd             : Image width
    yEnd             : Image height
******************************************************************************/
fn paint_draw_image(image: char, x_start: UWORD, y_start: UWORD, w_image: UWORD, h_image: UWORD) {
  let i: i32;
  let j: i32;
  for j in 0..h_image {
    for i in 0..w_image {
      if x_start + i < LCD_WIDTH  &&  y_start + j < LCD_HEIGHT { // Exceeded part does not display
        paint_set_pixel(x_start + i, y_start + j, (pgm_read_byte(image + j * w_image * 2 + i * 2 + 1)) << 8 | (pgm_read_byte(image + j * w_image * 2 + i * 2)));
      }
      // Using arrays is a property of sequential storage, accessing the original array by algorithm
      // j*W_Image*2          Y offset
      // i*2                  X offset
      // pgm_read_byte()
    }
  }
}
