#include <stdio.h>
#include "../../src/microui.h"

static int text_width(mu_Font font, const char *str, int len) {
  (void) font;
  if (len < 0) {
    len = 0;
    while (str[len]) { len++; }
  }
  return len;
}

static int text_height(mu_Font font) {
  (void) font;
  return 10;
}

int main(void) {
  mu_Context ctx;
  mu_Command *cmd = NULL;

  mu_init(&ctx);
  ctx.text_width = text_width;
  ctx.text_height = text_height;

  mu_begin(&ctx);
  if (mu_begin_window(&ctx, "Test", mu_rect(0, 0, 100, 100))) {
    int widths[] = { -1 };
    mu_layout_row(&ctx, 1, widths, 0);
    mu_label(&ctx, "Hello");
    mu_end_window(&ctx);
  }
  mu_end(&ctx);

  while (mu_next_command(&ctx, &cmd)) {
    switch (cmd->type) {
      case MU_COMMAND_CLIP:
        printf("clip %d %d %d %d\n", cmd->clip.rect.x, cmd->clip.rect.y, cmd->clip.rect.w, cmd->clip.rect.h);
        break;
      case MU_COMMAND_RECT:
        printf(
          "rect %d %d %d %d %u %u %u %u\n",
          cmd->rect.rect.x, cmd->rect.rect.y, cmd->rect.rect.w, cmd->rect.rect.h,
          cmd->rect.color.r, cmd->rect.color.g, cmd->rect.color.b, cmd->rect.color.a);
        break;
      case MU_COMMAND_TEXT:
        printf("text %d %d %s\n", cmd->text.pos.x, cmd->text.pos.y, cmd->text.str);
        break;
      case MU_COMMAND_ICON:
        printf("icon %d %d %d %d %d\n", cmd->icon.id, cmd->icon.rect.x, cmd->icon.rect.y, cmd->icon.rect.w, cmd->icon.rect.h);
        break;
    }
  }

  return 0;
}
