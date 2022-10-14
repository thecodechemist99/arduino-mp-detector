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

#[macro_export]
macro_rules! debug {
  ($($args:expr),*) => {{
    let mut string: String = "".to_owned();
    $(
      string = format!("{} {}", string, $args);
    )*
    println!("Debug :{}", string);
  }};
}
