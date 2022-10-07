/*****************************************************************************
* | File        :   Debug.rs
* | Author      :   Waveshare team
* | Function    :   debug with prntf
* | Info        :
*   Image scanning
*      Please use progressive scanning to generate images or fonts
*----------------
* | This version:   V1.0
* | Date        :   2022-10-06
* | Info        :   Basic version
*
******************************************************************************/

const DEV_DEBUG: bool = true;

pub fn debug (__info, ...) {
  if DEV_DEBUG {
    // printf("Debug : " __info,##__VA_ARGS__);
  }
}