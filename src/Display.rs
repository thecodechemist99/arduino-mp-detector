mod LCD_Driver;
mod GUI_Paint;

use crate::Display::LCD_Driver::*;
use crate::Display::GUI_Paint::*;

fn init_display () {
  Config_Init();
  LCD_Init();
  LCD_Clear(0xffff);
  Paint_NewImage(LCD_WIDTH, LCD_HEIGHT, 0, WHITE);
}

fn show_display () {
  Paint_Clear(WHITE);

  // temperature headings
  Paint_DrawString_EN(20, 20, "Set Temp", &Font20, WHITE, BLACK);
  Paint_DrawString_EN(LCD_WIDTH / 2 + 20, 20, "Temp", &Font20, WHITE, BLACK);

  // temperature display
  Paint_DrawNum(20, 52, getSetTemp(), &Font24, WHITE, BLACK);
  Paint_DrawNum(LCD_WIDTH / 2 + 20, 52, readTemp(), &Font24, WHITE, BLACK);

  // unit display
  Paint_DrawCircle(107, 80, 2, BLACK, DOT_PIXEL_1X1, DRAW_FILL_EMPTY);
  Paint_DrawString_EN(110, 76, "C", &Font20, WHITE, BLACK);  
  Paint_DrawCircle(LCD_WIDTH / 2 + 107, 80, 2, BLACK, DOT_PIXEL_1X1, DRAW_FILL_EMPTY);
  Paint_DrawString_EN(LCD_WIDTH / 2 + 110, 76, "C", &Font20, WHITE, BLACK);
}