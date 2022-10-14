mod LCD_Driver;
mod GUI_Paint;

use LCD_Driver::*;
use GUI_Paint::*;

const PAINT: PAINT = paint_new_image(LCD_WIDTH, LCD_HEIGHT, 0, WHITE);

fn init_display () {
  config_init();
  lcd_init();
  lcd_clear(0xffff);
}

fn show_display (t: i16, t_set: i16) {
  PAINT.clear(WHITE);

  // temperature headings
  PAINT.draw_string_en(20, 20, "Set Temp", Font20, WHITE, BLACK);
  PAINT.draw_string_en(LCD_WIDTH / 2 + 20, 20, "Temp", Font20, WHITE, BLACK);

  // temperature display
  PAINT.draw_num(20, 52, t_set, Font24, WHITE, BLACK);
  PAINT.draw_num(LCD_WIDTH / 2 + 20, 52, t, Font24, WHITE, BLACK);

  // unit display
  PAINT.draw_circle(107, 80, 2, BLACK, DotPixel1x1, DrawFillEmpty);
  PAINT.draw_string_en(110, 76, "C", Font20, WHITE, BLACK);  
  PAINT.draw_circle(LCD_WIDTH / 2 + 107, 80, 2, BLACK, DotPixel1x1, DrawFillEmpty);
  PAINT.draw_string_en(LCD_WIDTH / 2 + 110, 76, "C", Font20, WHITE, BLACK);
}