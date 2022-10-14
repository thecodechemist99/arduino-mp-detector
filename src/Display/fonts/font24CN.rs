/**
  ******************************************************************************
  * @file    Font12.rs
  * @author  MCD Application Team
  * @version V1.0.0
  * @date    14-October-2022
  * @brief   This file provides text Font12 for STM32xx-EVAL's LCD driver. 
  ******************************************************************************
  * @attention
  *
  * <h2><center>&copy; COPYRIGHT(c) 2014 STMicroelectronics</center></h2>
  *
  * Redistribution and use in source and binary forms, with or without modification,
  * are permitted provided that the following conditions are met:
  *   1. Redistributions of source code must retain the above copyright notice,
  *      this list of conditions and the following disclaimer.
  *   2. Redistributions in binary form must reproduce the above copyright notice,
  *      this list of conditions and the following disclaimer in the documentation
  *      and/or other materials provided with the distribution.
  *   3. Neither the name of STMicroelectronics nor the names of its contributors
  *      may be used to endorse or promote products derived from this software
  *      without specific prior written permission.
  *
  * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
  * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
  * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
  * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
  * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
  * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
  * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
  * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
  * OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
  * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
  *
  ******************************************************************************
  */

#[path = "../fonts.rs"]
mod fonts;
use fonts::*;

// 
//  Font data for Courier New 12pt
// 

const Font24CN_Table: ChCn = [
  /*--  文字:  你  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "你",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x01,0xC1,0xC0,0x00,
    0x01,0xE3,0xE0,0x00,0x03,0xE3,0xC0,0x00,0x03,0xC7,0x80,0x00,0x03,0xC7,0xFF,0xFF,
    0x07,0x8F,0xFF,0xFF,0x07,0x8F,0x00,0x0F,0x0F,0x1E,0x00,0x1E,0x0F,0x3C,0x1E,0x1E,
    0x1F,0x3C,0x1E,0x3E,0x1F,0x18,0x1E,0x3C,0x3F,0x00,0x1E,0x1C,0x7F,0x00,0x1E,0x00,
    0x7F,0x07,0x9E,0x70,0xFF,0x07,0x9E,0xF0,0xEF,0x0F,0x9E,0x78,0x6F,0x0F,0x1E,0x78,
    0x0F,0x0F,0x1E,0x3C,0x0F,0x1E,0x1E,0x3C,0x0F,0x1E,0x1E,0x1E,0x0F,0x3C,0x1E,0x1E,
    0x0F,0x3C,0x1E,0x1F,0x0F,0x7C,0x1E,0x0F,0x0F,0x78,0x1E,0x0E,0x0F,0x00,0x1E,0x00,
    0x0F,0x00,0x1E,0x00,0x0F,0x00,0x3C,0x00,0x0F,0x07,0xFC,0x00,0x0F,0x07,0xF8,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  好  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "好",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0F,0x00,0x00,0x00,0x0F,0x00,0x00,0x00,
    0x0F,0x07,0xFF,0xFE,0x0F,0x07,0xFF,0xFE,0x0F,0x00,0x00,0x3E,0x1E,0x00,0x00,0xFC,
    0xFF,0xF8,0x01,0xF0,0xFF,0xF8,0x03,0xE0,0x1E,0x78,0x07,0xC0,0x1E,0x78,0x0F,0x80,
    0x3C,0x78,0x0F,0x00,0x3C,0x78,0x0F,0x00,0x3C,0x78,0x0F,0x00,0x3C,0x78,0x0F,0x00,
    0x3C,0x7F,0xFF,0xFF,0x78,0xFF,0xFF,0xFF,0x78,0xF0,0x0F,0x00,0x78,0xF0,0x0F,0x00,
    0x3D,0xE0,0x0F,0x00,0x1F,0xE0,0x0F,0x00,0x0F,0xE0,0x0F,0x00,0x07,0xC0,0x0F,0x00,
    0x07,0xE0,0x0F,0x00,0x07,0xF0,0x0F,0x00,0x0F,0xF8,0x0F,0x00,0x1E,0x7C,0x0F,0x00,
    0x3C,0x38,0x0F,0x00,0x78,0x00,0x0F,0x00,0xF0,0x03,0xFF,0x00,0x60,0x01,0xFE,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  微  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "微",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x03,0x07,0x01,0xE0,0x07,0x87,0x01,0xE0,
    0x07,0x07,0x01,0xC0,0x0F,0xF7,0x79,0xC0,0x1E,0xF7,0x7B,0xC0,0x1E,0xF7,0x7B,0x80,
    0x3C,0xF7,0x7B,0xFF,0x78,0xF7,0x7B,0xFF,0xF8,0xF7,0x7F,0x9E,0xF7,0xFF,0xFF,0x9E,
    0x67,0xFF,0xFF,0x9E,0x07,0x00,0x7F,0x9C,0x0F,0x00,0x0F,0x9C,0x1E,0x00,0x1F,0x9C,
    0x1E,0x7F,0xFF,0xBC,0x3E,0x7F,0xF3,0xFC,0x3E,0x00,0x03,0xFC,0x7E,0x00,0x01,0xF8,
    0xFE,0x00,0x01,0xF8,0xFE,0x7F,0xE1,0xF8,0xDE,0x7F,0xE1,0xF8,0x1E,0x78,0xE0,0xF0,
    0x1E,0x78,0xEE,0xF0,0x1E,0x78,0xFF,0xF0,0x1E,0x78,0xFD,0xF8,0x1E,0x79,0xFB,0xFC,
    0x1E,0xF1,0xF7,0xBC,0x1E,0xF0,0xEF,0x9E,0x1F,0xE0,0x0F,0x0F,0x1E,0xC0,0x1E,0x0F,
    0x1E,0x00,0x0C,0x07,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  软  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "软",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x03,0xC0,0x78,0x00,0x07,0x80,0x78,0x00,0x07,0x80,0x78,0x00,
    0x07,0x80,0xF0,0x00,0x0F,0x00,0xF0,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,
    0x1E,0x03,0xC0,0x1F,0x1E,0x03,0xC0,0x1E,0x1F,0xE7,0x8F,0x3E,0x3D,0xE7,0x8F,0x3C,
    0x3D,0xEF,0x0F,0x7C,0x3D,0xE7,0x0F,0x78,0x79,0xE0,0x0F,0x00,0x79,0xE0,0x0E,0x00,
    0x7F,0xFE,0x0E,0x00,0x7F,0xFE,0x1F,0x00,0x01,0xE0,0x1F,0x00,0x01,0xE0,0x1F,0x00,
    0x01,0xE0,0x1F,0x80,0x01,0xE0,0x1F,0x80,0x01,0xE0,0x3F,0x80,0x01,0xFF,0x3F,0xC0,
    0x0F,0xFF,0x7B,0xC0,0xFF,0xF0,0x79,0xE0,0xF9,0xE0,0xF1,0xF0,0x01,0xE1,0xF0,0xF0,
    0x01,0xE3,0xE0,0xF8,0x01,0xE7,0xC0,0x7C,0x01,0xFF,0x80,0x3F,0x01,0xFF,0x00,0x1F,
    0x01,0xEC,0x00,0x0E,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  雅  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "雅",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x77,0x00,0x00,0x00,0xFF,0x00,
    0x7F,0xFC,0xF7,0x80,0x7F,0xFD,0xE3,0xC0,0x01,0xC1,0xE3,0xC0,0x01,0xC3,0xC1,0x80,
    0x3D,0xC7,0xFF,0xFF,0x39,0xC7,0xFF,0xFF,0x39,0xCF,0x83,0x80,0x79,0xDF,0x83,0x80,
    0x79,0xFF,0x83,0x80,0x79,0xDF,0x83,0x80,0x71,0xC3,0x83,0x80,0x7F,0xFF,0xFF,0xFE,
    0x7F,0xFF,0xFF,0xFE,0x03,0xC3,0x83,0x80,0x07,0xC3,0x83,0x80,0x07,0xC3,0x83,0x80,
    0x0F,0xC3,0x83,0x80,0x0F,0xC3,0x83,0x80,0x1F,0xC3,0xFF,0xFE,0x1D,0xC3,0xFF,0xFE,
    0x3D,0xC3,0x83,0x80,0x79,0xC3,0x83,0x80,0xF1,0xC3,0x83,0x80,0xF1,0xC3,0x83,0x80,
    0x61,0xC3,0x83,0x80,0x01,0xC3,0xFF,0xFF,0x03,0xC3,0xFF,0xFF,0x1F,0xC3,0x80,0x00,
    0x1F,0x83,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  黑  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "黑",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x1F,0xFF,0xFF,0xFC,0x1F,0xFF,0xFF,0xFC,0x1E,0x03,0xC0,0x3C,0x1E,0xC3,0xC7,0x3C,
    0x1F,0xE3,0xC7,0xBC,0x1E,0xF3,0xCF,0x3C,0x1E,0xFB,0xDF,0x3C,0x1E,0x7B,0xDE,0x3C,
    0x1E,0x33,0xDC,0x3C,0x1E,0x03,0xC0,0x3C,0x1F,0xFF,0xFF,0xFC,0x1F,0xFF,0xFF,0xFC,
    0x1E,0x03,0xC0,0x3C,0x00,0x03,0xC0,0x00,0x00,0x03,0xC0,0x00,0x3F,0xFF,0xFF,0xFC,
    0x3F,0xFF,0xFF,0xFC,0x00,0x03,0xC0,0x00,0x00,0x03,0xC0,0x00,0x00,0x03,0xC0,0x00,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x00,0x00,0x00,0x00,0x1C,0x38,0x70,0x70,
    0x3E,0x78,0xF8,0xF8,0x3C,0x7C,0x78,0x7C,0x7C,0x3C,0x3C,0x3E,0xF8,0x3E,0x3C,0x1F,
    0xF0,0x1C,0x18,0x0E,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  此  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "此",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x78,0x3C,0x00,
    0x00,0x78,0x3C,0x00,0x00,0x78,0x3C,0x00,0x00,0x78,0x3C,0x00,0x00,0x78,0x3C,0x00,
    0x00,0x78,0x3C,0x0C,0x3C,0x78,0x3C,0x1E,0x3C,0x78,0x3C,0x3F,0x3C,0x78,0x3C,0xF8,
    0x3C,0x7F,0xFD,0xF0,0x3C,0x7F,0xFF,0xE0,0x3C,0x78,0x3F,0x80,0x3C,0x78,0x3E,0x00,
    0x3C,0x78,0x3C,0x00,0x3C,0x78,0x3C,0x00,0x3C,0x78,0x3C,0x00,0x3C,0x78,0x3C,0x00,
    0x3C,0x78,0x3C,0x00,0x3C,0x78,0x3C,0x00,0x3C,0x78,0x3C,0x0E,0x3C,0x78,0x3C,0x0F,
    0x3C,0x78,0x3C,0x0F,0x3C,0x79,0xFC,0x0F,0x3C,0x7F,0xFC,0x0F,0x3F,0xFF,0x3C,0x0F,
    0x3F,0xF0,0x3E,0x1E,0xFF,0x00,0x1F,0xFE,0xF0,0x00,0x0F,0xFC,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  字  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "字",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x03,0x80,0x00,0x00,0x07,0x80,0x00,0x00,0x03,0xC0,0x00,
    0x00,0x03,0xE0,0x00,0x00,0x01,0xE0,0x00,0x7F,0xFF,0xFF,0xFE,0x7F,0xFF,0xFF,0xFE,
    0x78,0x00,0x00,0x1E,0x78,0x00,0x00,0x1E,0x78,0x00,0x00,0x1E,0x78,0x00,0x00,0x1E,
    0x7B,0xFF,0xFF,0xDE,0x03,0xFF,0xFF,0xC0,0x00,0x00,0x0F,0xC0,0x00,0x00,0x3F,0x00,
    0x00,0x00,0x7E,0x00,0x00,0x01,0xF8,0x00,0x00,0x01,0xE0,0x00,0x00,0x01,0xE0,0x00,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x00,0x01,0xE0,0x00,0x00,0x01,0xE0,0x00,
    0x00,0x01,0xE0,0x00,0x00,0x01,0xE0,0x00,0x00,0x01,0xE0,0x00,0x00,0x01,0xE0,0x00,
    0x00,0x03,0xE0,0x00,0x00,0x03,0xC0,0x00,0x00,0xFF,0xC0,0x00,0x00,0xFF,0x80,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  体  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "体",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x03,0xC0,0x3C,0x00,
    0x03,0xC0,0x3C,0x00,0x03,0xC0,0x3C,0x00,0x07,0x80,0x3C,0x00,0x07,0x80,0x3C,0x00,
    0x07,0x80,0x3C,0x00,0x0F,0xFF,0xFF,0xFF,0x0F,0xFF,0xFF,0xFF,0x1F,0x01,0xFE,0x00,
    0x1F,0x01,0xFF,0x00,0x3F,0x01,0xFF,0x00,0x3F,0x03,0xFF,0x00,0x7F,0x03,0xFF,0x80,
    0x7F,0x07,0xBF,0x80,0xFF,0x07,0xBF,0xC0,0xEF,0x0F,0x3D,0xC0,0xCF,0x0F,0x3D,0xE0,
    0x0F,0x1E,0x3D,0xE0,0x0F,0x1E,0x3C,0xF0,0x0F,0x3C,0x3C,0x78,0x0F,0x7C,0x3C,0x7C,
    0x0F,0xF8,0x3C,0x3E,0x0F,0xF7,0xFF,0xDF,0x0F,0xE7,0xFF,0xCF,0x0F,0xC0,0x3C,0x06,
    0x0F,0x00,0x3C,0x00,0x0F,0x00,0x3C,0x00,0x0F,0x00,0x3C,0x00,0x0F,0x00,0x3C,0x00,
    0x0F,0x00,0x3C,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  下  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "下",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x00,0x0F,0x80,0x00,0x00,0x0F,0x80,0x00,
    0x00,0x0F,0x80,0x00,0x00,0x0F,0x80,0x00,0x00,0x0F,0x80,0x00,0x00,0x0F,0x80,0x00,
    0x00,0x0F,0xE0,0x00,0x00,0x0F,0xF8,0x00,0x00,0x0F,0xFC,0x00,0x00,0x0F,0xBF,0x00,
    0x00,0x0F,0x9F,0x80,0x00,0x0F,0x87,0xE0,0x00,0x0F,0x83,0xF0,0x00,0x0F,0x80,0xF8,
    0x00,0x0F,0x80,0x7C,0x00,0x0F,0x80,0x38,0x00,0x0F,0x80,0x00,0x00,0x0F,0x80,0x00,
    0x00,0x0F,0x80,0x00,0x00,0x0F,0x80,0x00,0x00,0x0F,0x80,0x00,0x00,0x0F,0x80,0x00,
    0x00,0x0F,0x80,0x00,0x00,0x0F,0x80,0x00,0x00,0x0F,0x80,0x00,0x00,0x0F,0x80,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  对  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "对",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x78,
    0x00,0x00,0x00,0x78,0x00,0x00,0x00,0x78,0x7F,0xFC,0x00,0x78,0x7F,0xFC,0x00,0x78,
    0x00,0x3C,0x00,0x78,0x00,0x3F,0xFF,0xFF,0x30,0x3F,0xFF,0xFF,0x78,0x3C,0x00,0x78,
    0x3C,0x38,0x00,0x78,0x3E,0x78,0x00,0x78,0x1E,0x78,0xC0,0x78,0x0F,0x79,0xE0,0x78,
    0x0F,0xF0,0xF0,0x78,0x07,0xF0,0xF8,0x78,0x03,0xF0,0x78,0x78,0x01,0xE0,0x3C,0x78,
    0x03,0xF0,0x3E,0x78,0x03,0xF0,0x18,0x78,0x07,0xF8,0x00,0x78,0x07,0xFC,0x00,0x78,
    0x0F,0x3E,0x00,0x78,0x1F,0x1E,0x00,0x78,0x3E,0x1F,0x00,0x78,0x7C,0x0E,0x00,0xF8,
    0xF8,0x00,0x00,0xF0,0xF0,0x00,0x3F,0xF0,0x60,0x00,0x3F,0xE0,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  应  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "应",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x01,0xC0,0x00,0x00,0x03,0xE0,0x00,0x00,0x01,0xE0,0x00,
    0x00,0x01,0xF0,0x00,0x00,0x00,0xF0,0x00,0x1F,0xFF,0xFF,0xFF,0x1F,0xFF,0xFF,0xFF,
    0x1E,0x00,0x00,0x00,0x1E,0x00,0x00,0x00,0x1E,0x01,0xE0,0x78,0x1E,0x01,0xE0,0x78,
    0x1E,0xE1,0xE0,0x78,0x1F,0xF1,0xF0,0xF8,0x1E,0xF0,0xF0,0xF0,0x1E,0xF0,0xF0,0xF0,
    0x1E,0xF8,0xF0,0xF0,0x1E,0x78,0xF1,0xF0,0x1E,0x78,0xF9,0xE0,0x1E,0x78,0x79,0xE0,
    0x1E,0x7C,0x7B,0xE0,0x1E,0x3C,0x7B,0xC0,0x1E,0x3C,0x7B,0xC0,0x1E,0x3C,0x7B,0xC0,
    0x3C,0x3E,0x07,0x80,0x3C,0x1C,0x07,0x80,0x3C,0x00,0x07,0x80,0x3C,0x00,0x0F,0x00,
    0x78,0x00,0x0F,0x00,0x7B,0xFF,0xFF,0xFF,0xF3,0xFF,0xFF,0xFF,0xF0,0x00,0x00,0x00,
    0x60,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  的  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "的",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x03,0x80,0x3C,0x00,0x07,0xC0,0x3E,0x00,
    0x07,0x80,0x3C,0x00,0x07,0x80,0x7C,0x00,0x0F,0x00,0x78,0x00,0x7F,0xFE,0x7F,0xFE,
    0x7F,0xFE,0xFF,0xFE,0x78,0x1E,0xF0,0x1E,0x78,0x1F,0xE0,0x1E,0x78,0x1F,0xE0,0x1E,
    0x78,0x1F,0xC0,0x1E,0x78,0x1F,0xC0,0x1E,0x78,0x1F,0xF0,0x1E,0x78,0x1E,0xF8,0x1E,
    0x78,0x1E,0x7C,0x1E,0x7F,0xFE,0x3C,0x1E,0x7F,0xFE,0x1E,0x1E,0x78,0x1E,0x1F,0x1E,
    0x78,0x1E,0x0F,0x9E,0x78,0x1E,0x07,0x9E,0x78,0x1E,0x07,0x1E,0x78,0x1E,0x00,0x1E,
    0x78,0x1E,0x00,0x1E,0x78,0x1E,0x00,0x3E,0x78,0x1E,0x00,0x3C,0x78,0x1E,0x00,0x3C,
    0x7F,0xFE,0x00,0x3C,0x7F,0xFE,0x00,0x7C,0x78,0x1E,0x3F,0xF8,0x78,0x1E,0x3F,0xF0,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  点  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "点",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x03,0xC0,0x00,0x00,0x03,0xC0,0x00,
    0x00,0x03,0xC0,0x00,0x00,0x03,0xC0,0x00,0x00,0x03,0xFF,0xFF,0x00,0x03,0xFF,0xFF,
    0x00,0x03,0xC0,0x00,0x00,0x03,0xC0,0x00,0x00,0x03,0xC0,0x00,0x00,0x03,0xC0,0x00,
    0x0F,0xFF,0xFF,0xF8,0x0F,0xFF,0xFF,0xF8,0x0F,0x00,0x00,0x78,0x0F,0x00,0x00,0x78,
    0x0F,0x00,0x00,0x78,0x0F,0x00,0x00,0x78,0x0F,0x00,0x00,0x78,0x0F,0x00,0x00,0x78,
    0x0F,0xFF,0xFF,0xF8,0x0F,0xFF,0xFF,0xF8,0x0F,0x00,0x00,0x78,0x00,0x00,0x00,0x00,
    0x0C,0x38,0x38,0x30,0x1E,0x7C,0x78,0x78,0x3E,0x3C,0x78,0x78,0x3C,0x3C,0x3C,0x3C,
    0x7C,0x3E,0x3C,0x3E,0xF8,0x1E,0x3C,0x1E,0xF0,0x1E,0x1E,0x1F,0x70,0x1E,0x1C,0x0E,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  阵  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "阵",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x3C,0x00,0x00,0x00,0x78,0x00,
    0x7F,0xF0,0x78,0x00,0x7F,0xF0,0x78,0x00,0x79,0xFF,0xFF,0xFF,0x79,0xFF,0xFF,0xFF,
    0x79,0xE1,0xE0,0x00,0x79,0xE1,0xE0,0x00,0x7B,0xC1,0xEF,0x80,0x7B,0xC3,0xCF,0x80,
    0x7B,0xC3,0xCF,0x80,0x7F,0x87,0xCF,0x80,0x7F,0x87,0x8F,0x80,0x7F,0x87,0x8F,0x80,
    0x7B,0xCF,0x0F,0x80,0x7B,0xCF,0xFF,0xFE,0x79,0xEF,0xFF,0xFE,0x79,0xE0,0x0F,0x80,
    0x78,0xE0,0x0F,0x80,0x78,0xF0,0x0F,0x80,0x78,0xF0,0x0F,0x80,0x78,0xF0,0x0F,0x80,
    0x78,0xFF,0xFF,0xFF,0x79,0xFF,0xFF,0xFF,0x7F,0xE0,0x0F,0x80,0x7F,0xC0,0x0F,0x80,
    0x78,0x00,0x0F,0x80,0x78,0x00,0x0F,0x80,0x78,0x00,0x0F,0x80,0x78,0x00,0x0F,0x80,
    0x78,0x00,0x0F,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  为  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "为",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x07,0x80,0x00,
    0x0E,0x07,0x80,0x00,0x1F,0x07,0x80,0x00,0x0F,0x87,0x80,0x00,0x07,0xC7,0x80,0x00,
    0x01,0xE7,0x80,0x00,0x00,0xC7,0x80,0x00,0x00,0x07,0x80,0x00,0x7F,0xFF,0xFF,0xFC,
    0x7F,0xFF,0xFF,0xFC,0x00,0x07,0x80,0x3C,0x00,0x0F,0x80,0x3C,0x00,0x0F,0x00,0x3C,
    0x00,0x0F,0x00,0x3C,0x00,0x0F,0x60,0x3C,0x00,0x1F,0xF0,0x3C,0x00,0x1E,0x78,0x3C,
    0x00,0x3E,0x3C,0x3C,0x00,0x3C,0x3E,0x3C,0x00,0x7C,0x1F,0x3C,0x00,0x78,0x0F,0x3C,
    0x00,0xF8,0x06,0x3C,0x01,0xF0,0x00,0x3C,0x03,0xE0,0x00,0x7C,0x07,0xC0,0x00,0x7C,
    0x0F,0x80,0x00,0x78,0x1F,0x00,0x00,0xF8,0x3E,0x00,0xFF,0xF0,0x7C,0x00,0xFF,0xE0,
    0x38,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  树  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "树",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x0F,0x00,0x00,0x38,
    0x0F,0x00,0x00,0x38,0x0F,0x00,0x00,0x38,0x0F,0x3F,0xF8,0x38,0x0F,0x3F,0xF8,0x38,
    0x0F,0x00,0x78,0x38,0xFF,0xE0,0x7F,0xFF,0xFF,0xE0,0x7F,0xFF,0x0F,0x00,0x70,0x38,
    0x0F,0x18,0xF0,0x38,0x1F,0x3C,0xF0,0x38,0x1F,0x1C,0xFE,0x38,0x1F,0xDE,0xFE,0x38,
    0x3F,0xEF,0xEF,0x38,0x3F,0xFF,0xEF,0x38,0x3F,0xF7,0xE7,0xB8,0x7F,0x67,0xC7,0xB8,
    0x7F,0x03,0xC3,0xB8,0xFF,0x07,0xE0,0x38,0xEF,0x07,0xE0,0x38,0xEF,0x0F,0xF0,0x38,
    0xCF,0x1F,0xF0,0x38,0x0F,0x1E,0x78,0x38,0x0F,0x3C,0x7C,0x38,0x0F,0x78,0x3C,0x38,
    0x0F,0xF8,0x38,0x38,0x0F,0x60,0x00,0x78,0x0F,0x00,0x0F,0xF8,0x0F,0x00,0x07,0xF0,
    0x0F,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  莓  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "莓",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x3C,0x1E,0x00,0x00,0x3C,0x1E,0x00,
    0x00,0x3C,0x1E,0x00,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x00,0x3C,0x1E,0x00,
    0x07,0xBC,0x1E,0x00,0x07,0x80,0x00,0x00,0x0F,0xFF,0xFF,0xFC,0x0F,0xFF,0xFF,0xFC,
    0x1E,0x00,0x00,0x00,0x3C,0x00,0x00,0x00,0x3C,0x00,0x00,0x00,0x7F,0xFF,0xFF,0xF0,
    0xF7,0xFF,0xFF,0xF0,0x37,0x83,0x80,0xF0,0x07,0x87,0xC0,0xF0,0x07,0x83,0xF0,0xF0,
    0x07,0x00,0xE0,0xF0,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x0F,0x0F,0x00,0xE0,
    0x0F,0x0F,0x81,0xE0,0x0E,0x03,0xE1,0xE0,0x1E,0x01,0xC1,0xE0,0x1F,0xFF,0xFF,0xFE,
    0x1F,0xFF,0xFF,0xFE,0x00,0x00,0x01,0xE0,0x00,0x00,0x03,0xC0,0x00,0x00,0xFF,0xC0,
    0x00,0x00,0xFF,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  派  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "派",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x38,0x00,0x00,0x3E,
    0x7C,0x00,0x3F,0xFE,0x3F,0x3F,0xFF,0xF0,0x1F,0xBF,0xE0,0x00,0x07,0xBC,0x00,0x00,
    0x03,0x3C,0x00,0x00,0x00,0x3C,0x00,0x3C,0x00,0x3C,0x0F,0xFE,0x70,0x3D,0xFF,0xF8,
    0xF8,0x3D,0xFF,0x00,0x7C,0x3D,0xE7,0x80,0x3F,0x3D,0xE7,0x80,0x1F,0x3D,0xE7,0x8E,
    0x0E,0x3D,0xE7,0x9F,0x00,0x3D,0xE7,0xFE,0x00,0x39,0xE7,0xF8,0x00,0x39,0xE3,0xF0,
    0x1C,0x39,0xE3,0xC0,0x1E,0x79,0xE3,0xC0,0x1E,0x79,0xE1,0xE0,0x1E,0x79,0xE1,0xE0,
    0x3C,0x79,0xE0,0xF0,0x3C,0x79,0xE0,0xF8,0x3C,0xF1,0xE0,0x7C,0x3C,0xF1,0xE3,0x7C,
    0x7D,0xF1,0xEF,0x3F,0x79,0xE1,0xFE,0x1F,0x7B,0xE1,0xF8,0x0E,0x7B,0xC3,0xE0,0x00,
    0x79,0x81,0xC0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  A  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "A",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x7C,0x00,0x00,0x00,0xFC,0x00,0x00,0x00,0xFE,0x00,0x00,0x00,0xFE,0x00,0x00,
    0x01,0xFF,0x00,0x00,0x01,0xFF,0x00,0x00,0x01,0xEF,0x00,0x00,0x03,0xEF,0x80,0x00,
    0x03,0xCF,0x80,0x00,0x07,0xC7,0x80,0x00,0x07,0xC7,0xC0,0x00,0x07,0x87,0xC0,0x00,
    0x0F,0x83,0xE0,0x00,0x0F,0x83,0xE0,0x00,0x0F,0x01,0xE0,0x00,0x1F,0xFF,0xF0,0x00,
    0x1F,0xFF,0xF0,0x00,0x3F,0xFF,0xF8,0x00,0x3E,0x00,0xF8,0x00,0x3C,0x00,0xF8,0x00,
    0x7C,0x00,0x7C,0x00,0x7C,0x00,0x7C,0x00,0x78,0x00,0x3C,0x00,0xF8,0x00,0x3E,0x00,
    0xF8,0x00,0x3E,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  a  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "a",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x07,0xF8,0x00,0x00,
    0x1F,0xFE,0x00,0x00,0x3F,0xFE,0x00,0x00,0x3E,0x3F,0x00,0x00,0x38,0x1F,0x00,0x00,
    0x00,0x0F,0x00,0x00,0x00,0x0F,0x00,0x00,0x03,0xFF,0x00,0x00,0x1F,0xFF,0x00,0x00,
    0x3F,0x8F,0x00,0x00,0x7C,0x0F,0x00,0x00,0x7C,0x0F,0x00,0x00,0x78,0x1F,0x00,0x00,
    0x7C,0x1F,0x00,0x00,0x7E,0x7F,0x00,0x00,0x7F,0xFF,0x00,0x00,0x3F,0xFF,0x00,0x00,
    0x0F,0xCF,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  b  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "b",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x3C,0x00,0x00,0x00,
    0x3C,0x00,0x00,0x00,0x3C,0x00,0x00,0x00,0x3C,0x00,0x00,0x00,0x3C,0x00,0x00,0x00,
    0x3C,0x00,0x00,0x00,0x3C,0x00,0x00,0x00,0x3C,0x00,0x00,0x00,0x3C,0xFE,0x00,0x00,
    0x3D,0xFF,0x80,0x00,0x3F,0xFF,0xC0,0x00,0x3F,0x8F,0xC0,0x00,0x3F,0x07,0xE0,0x00,
    0x3E,0x03,0xE0,0x00,0x3E,0x03,0xE0,0x00,0x3C,0x01,0xE0,0x00,0x3C,0x01,0xE0,0x00,
    0x3C,0x01,0xE0,0x00,0x3C,0x03,0xE0,0x00,0x3E,0x03,0xE0,0x00,0x3E,0x03,0xE0,0x00,
    0x3F,0x07,0xC0,0x00,0x3F,0x8F,0xC0,0x00,0x3F,0xFF,0x80,0x00,0x3F,0xFF,0x00,0x00,
    0x3C,0xFC,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  c  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "c",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x01,0xFC,0x00,0x00,
    0x07,0xFE,0x00,0x00,0x1F,0xFE,0x00,0x00,0x3F,0x86,0x00,0x00,0x3E,0x00,0x00,0x00,
    0x7C,0x00,0x00,0x00,0x7C,0x00,0x00,0x00,0x7C,0x00,0x00,0x00,0x78,0x00,0x00,0x00,
    0x78,0x00,0x00,0x00,0x7C,0x00,0x00,0x00,0x7C,0x00,0x00,0x00,0x7C,0x00,0x00,0x00,
    0x3E,0x00,0x00,0x00,0x3F,0x86,0x00,0x00,0x1F,0xFE,0x00,0x00,0x0F,0xFE,0x00,0x00,
    0x03,0xFC,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  微  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "微",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x03,0x07,0x01,0xE0,0x07,0x87,0x01,0xE0,
    0x07,0x07,0x01,0xC0,0x0F,0xF7,0x79,0xC0,0x1E,0xF7,0x7B,0xC0,0x1E,0xF7,0x7B,0x80,
    0x3C,0xF7,0x7B,0xFF,0x78,0xF7,0x7B,0xFF,0xF8,0xF7,0x7F,0x9E,0xF7,0xFF,0xFF,0x9E,
    0x67,0xFF,0xFF,0x9E,0x07,0x00,0x7F,0x9C,0x0F,0x00,0x0F,0x9C,0x1E,0x00,0x1F,0x9C,
    0x1E,0x7F,0xFF,0xBC,0x3E,0x7F,0xF3,0xFC,0x3E,0x00,0x03,0xFC,0x7E,0x00,0x01,0xF8,
    0xFE,0x00,0x01,0xF8,0xFE,0x7F,0xE1,0xF8,0xDE,0x7F,0xE1,0xF8,0x1E,0x78,0xE0,0xF0,
    0x1E,0x78,0xEE,0xF0,0x1E,0x78,0xFF,0xF0,0x1E,0x78,0xFD,0xF8,0x1E,0x79,0xFB,0xFC,
    0x1E,0xF1,0xF7,0xBC,0x1E,0xF0,0xEF,0x9E,0x1F,0xE0,0x0F,0x0F,0x1E,0xC0,0x1E,0x0F,
    0x1E,0x00,0x0C,0x07,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  雪  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "雪",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x1F,0xFF,0xFF,0xF8,0x1F,0xFF,0xFF,0xF8,0x00,0x03,0xC0,0x00,0x00,0x03,0xC0,0x00,
    0x7F,0xFF,0xFF,0xFE,0x7F,0xFF,0xFF,0xFE,0x78,0x03,0xC0,0x1E,0x78,0x03,0xC0,0x1E,
    0x7F,0xFF,0xFF,0xFE,0x7F,0xFF,0xFF,0xFE,0x00,0x03,0xC0,0x00,0x00,0x03,0xC0,0x00,
    0x07,0xFF,0xFF,0xE0,0x07,0xFF,0xFF,0xE0,0x00,0x03,0xC0,0x00,0x00,0x00,0x00,0x00,
    0x1F,0xFF,0xFF,0xF8,0x1F,0xFF,0xFF,0xF8,0x00,0x00,0x00,0x78,0x00,0x00,0x00,0x78,
    0x1F,0xFF,0xFF,0xF8,0x1F,0xFF,0xFF,0xF8,0x00,0x00,0x00,0x78,0x00,0x00,0x00,0x78,
    0x00,0x00,0x00,0x78,0x3F,0xFF,0xFF,0xF8,0x3F,0xFF,0xFF,0xF8,0x00,0x00,0x00,0x78,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  电  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "电",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x07,0x80,0x00,0x00,0x07,0x80,0x00,
    0x00,0x07,0x80,0x00,0x00,0x07,0x80,0x00,0x7F,0xFF,0xFF,0xF8,0x7F,0xFF,0xFF,0xF8,
    0x78,0x07,0x80,0xF8,0x78,0x07,0x80,0xF8,0x78,0x07,0x80,0xF8,0x78,0x07,0x80,0xF8,
    0x78,0x07,0x80,0xF8,0x78,0x07,0x80,0xF8,0x7F,0xFF,0xFF,0xF8,0x7F,0xFF,0xFF,0xF8,
    0x78,0x07,0x80,0xF8,0x78,0x07,0x80,0xF8,0x78,0x07,0x80,0xF8,0x78,0x07,0x80,0xF8,
    0x78,0x07,0x80,0xF8,0x78,0x07,0x80,0xF8,0x7F,0xFF,0xFF,0xF8,0x7F,0xFF,0xFF,0xF8,
    0x78,0x07,0x80,0x0E,0x78,0x07,0x80,0x0F,0x00,0x07,0x80,0x0F,0x00,0x07,0x80,0x0F,
    0x00,0x07,0x80,0x1F,0x00,0x07,0x80,0x1E,0x00,0x03,0xFF,0xFE,0x00,0x01,0xFF,0xFC,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],

  /*--  文字:  子  --*/
  /*--  微软雅黑24;  此字体下对应的点阵为：宽x高=32x41   --*/
  [
    "子",
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x1F,0xFF,0xFF,0xF8,0x1F,0xFF,0xFF,0xF8,0x00,0x00,0x01,0xF8,0x00,0x00,0x07,0xE0,
    0x00,0x00,0x0F,0xC0,0x00,0x00,0x1F,0x80,0x00,0x00,0x3E,0x00,0x00,0x00,0xFC,0x00,
    0x00,0x01,0xF8,0x00,0x00,0x03,0xE0,0x00,0x00,0x03,0xE0,0x00,0x00,0x03,0xE0,0x00,
    0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0xFF,0x00,0x03,0xE0,0x00,0x00,0x03,0xE0,0x00,
    0x00,0x03,0xE0,0x00,0x00,0x03,0xE0,0x00,0x00,0x03,0xE0,0x00,0x00,0x03,0xE0,0x00,
    0x00,0x03,0xE0,0x00,0x00,0x03,0xE0,0x00,0x00,0x03,0xE0,0x00,0x00,0x03,0xE0,0x00,
    0x00,0x03,0xE0,0x00,0x00,0x03,0xC0,0x00,0x01,0xFF,0xC0,0x00,0x00,0xFF,0x80,0x00,
    0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,
    0x00,0x00,0x00,0x00
  ],
];

pub const Font24CN: CFont = {
  Font24CN_Table;
  sizeof(Font24CN_Table) / sizeof(ChCn);  /* size of table */
  24; /* ASCII Width */
  32; /* Width */
  41; /* Height */
};

/************************ (C) COPYRIGHT Florian Beck, STMicroelectronics *****END OF FILE****/
