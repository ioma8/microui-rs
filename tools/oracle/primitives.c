#include <stdio.h>

#include "../../src/microui.h"

int main(void) {
  mu_Vec2 v = mu_vec2(1, 2);
  mu_Rect r = mu_rect(3, 4, 5, 6);
  mu_Color c = mu_color(7, 8, 9, 10);

  printf("version=%s\n", MU_VERSION);
  printf("vec2=Vec2 { x: %d, y: %d }\n", v.x, v.y);
  printf("rect=Rect { x: %d, y: %d, w: %d, h: %d }\n", r.x, r.y, r.w, r.h);
  printf("color=Color { r: %u, g: %u, b: %u, a: %u }\n", c.r, c.g, c.b, c.a);
  return 0;
}
