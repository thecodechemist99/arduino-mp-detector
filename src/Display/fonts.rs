/**
  ******************************************************************************
  * @file    fonts.rs
  * @author  Florian Beck, MCD Application Team
  * @version V1.0.0
  * @date    06-October-2022
  * @brief   Header for fonts.c file
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

/* 最大字体微软雅黑24 (32x41) */
const MAX_HEIGHT_FONT: u8 = 41;
const MAX_WIDTH_FONT: u8 = 32;
const OFFSET_BITMAP: u8;

//ASCII
pub struct  SFont {
  table: u8,
  width: u16,
  height: u16,
}


// GB2312
struct ChCn {                                         // 汉字字模数据结构
  index: [char; 3],                                   // 汉字内码索引
  matrix: [char; MAX_HEIGHT_FONT * MAX_WIDTH_FONT / 8],    // 点阵码数据
}


struct CFont {
  table: &ChCn,
  size: u16,
  ascii_width: u16,
  width: u16,
  height: u16,
}

pub const FONT24: SFont;
pub const FONT20: SFont;
pub const FONT16: SFont;
pub const FONT12: SFont;
pub const FONT8: SFont;

pub const FONT12_CN: CFont;
pub const FONT24_CN: CFont;
pub const Font16_Table: [char];
 

/************************ (C) COPYRIGHT Florian Beck, STMicroelectronics *****END OF FILE****/