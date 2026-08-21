use super::{draw_ui_button, ButtonStyle, TextureStore, UiMetrics, THEME};
use crate::game::level::{LevelRecord, LevelRepository, PackType};
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelSelectAction {
    None,
    SelectLevel(PackType, usize),
    BackToMenu,
}

pub fn render_level_select(
    repo: &LevelRepository,
    records: &HashMap<(PackType, usize), LevelRecord>,
    active_pack: &mut PackType,
    textures: &TextureStore,
    screen_w: f32,
    screen_h: f32,
) -> LevelSelectAction {
    let mut action = LevelSelectAction::None;
    let mouse_pos = mouse_position();
    let is_mouse_down = is_mouse_button_pressed(MouseButton::Left);
    let metrics = UiMetrics::new(screen_w, screen_h);

    // 1. Header Bar
    let header_h = metrics.hud_height;
    draw_rectangle(0.0, 0.0, screen_w, header_h, THEME.surface);
    let border_thick = (1.5 * metrics.scale).max(1.0);
    draw_line(
        0.0,
        header_h,
        screen_w,
        header_h,
        border_thick,
        Color::new(0.2, 0.24, 0.32, 0.6),
    );

    let btn_h = metrics.s(44.0);
    let btn_w = metrics.s(96.0);
    let btn_pad_x = metrics.s(16.0);
    let btn_y = (header_h - btn_h) / 2.0;

    let back_hovered = mouse_pos.0 >= btn_pad_x
        && mouse_pos.0 <= btn_pad_x + btn_w
        && mouse_pos.1 >= btn_y
        && mouse_pos.1 <= btn_y + btn_h;

    draw_rectangle(
        btn_pad_x,
        btn_y,
        btn_w,
        btn_h,
        if back_hovered {
            THEME.surface_hover
        } else {
            THEME.card_bg
        },
    );
    draw_rectangle_lines(
        btn_pad_x,
        btn_y,
        btn_w,
        btn_h,
        (1.5 * metrics.scale).max(1.0),
        Color::new(0.3, 0.35, 0.45, 0.6),
    );

    let icon_sz = metrics.s(22.0);
    if let Some(back_tex) = textures.get("icon_back") {
        draw_texture_ex(
            back_tex,
            btn_pad_x + metrics.s(8.0),
            btn_y + (btn_h - icon_sz) / 2.0,
            if back_hovered {
                THEME.accent_gold
            } else {
                WHITE
            },
            DrawTextureParams {
                dest_size: Some(vec2(icon_sz, icon_sz)),
                ..Default::default()
            },
        );
    }
    let back_text_size = metrics.s(18.0);
    let back_dim = measure_text("Back", None, back_text_size as u16, 1.0);
    draw_text(
        "Back",
        btn_pad_x + icon_sz + metrics.s(14.0),
        btn_y + (btn_h + back_dim.height) / 2.0 - metrics.s(2.0),
        back_text_size,
        if back_hovered {
            THEME.accent_gold
        } else {
            THEME.text_primary
        },
    );

    if back_hovered && is_mouse_down {
        action = LevelSelectAction::BackToMenu;
    }

    let header_title = "SELECT LEVEL";
    let title_font_size = metrics.s(24.0);
    let title_dim = measure_text(header_title, None, title_font_size as u16, 1.0);
    draw_text(
        header_title,
        screen_w / 2.0 - title_dim.width / 2.0,
        header_h / 2.0 + title_dim.height / 2.0 - metrics.s(2.0),
        title_font_size,
        THEME.text_primary,
    );

    // 2. Pack Tabs (6x6 Beginner, 8x8 Advanced, 10x10 Expert)
    let tabs = [
        (PackType::Grid6x6, "6x6 Beginner"),
        (PackType::Grid8x8, "8x8 Advanced"),
        (PackType::Grid10x10, "10x10 Expert"),
    ];

    let tab_gap = metrics.s(8.0);
    let tab_padding = metrics.s(16.0);
    let tab_w = ((screen_w - tab_padding * 2.0 - tab_gap * 2.0) / 3.0).min(metrics.s(220.0));
    let tab_h = metrics.s(44.0);
    let tab_y = header_h + metrics.s(16.0);
    let total_tabs_w = tab_w * 3.0 + tab_gap * 2.0;
    let tab_start_x = (screen_w - total_tabs_w) / 2.0;

    for (i, (pack, label)) in tabs.iter().enumerate() {
        let tx = tab_start_x + i as f32 * (tab_w + tab_gap);
        let is_selected = *active_pack == *pack;

        let bg_col = if is_selected {
            THEME.accent_blue
        } else {
            THEME.card_bg
        };

        if draw_ui_button(
            Rect::new(tx, tab_y, tab_w, tab_h),
            label,
            ButtonStyle {
                bg_color: bg_col,
                font_size: metrics.s(15.0),
                border_width: (1.5 * metrics.scale).max(1.0),
                ..Default::default()
            },
            mouse_pos,
            is_mouse_down,
        ) {
            *active_pack = *pack;
        }
    }

    // 3. Level Grid Cards
    let levels = repo.get_pack(*active_pack);
    let grid_y = tab_y + tab_h + metrics.s(24.0);
    let cols: usize = if screen_w > metrics.s(720.0) { 4 } else { 3 };
    let spacing = metrics.s(16.0);
    let grid_side_padding = metrics.s(20.0);
    let total_spacing = spacing * (cols - 1) as f32;
    let available_grid_w = screen_w - grid_side_padding * 2.0;
    let card_w = ((available_grid_w - total_spacing) / cols as f32).min(metrics.s(180.0));
    let card_h = (card_w * 0.90).round();
    let actual_grid_w = cols as f32 * card_w + total_spacing;
    let grid_start_x = (screen_w - actual_grid_w) / 2.0;

    let card_num_font = metrics.s(28.0);
    let star_size = (card_w * 0.18).clamp(metrics.s(16.0), metrics.s(28.0));
    let star_spacing = metrics.s(4.0);

    for (idx, lvl) in levels.iter().enumerate() {
        let row = idx / cols;
        let col = idx % cols;
        let cx = grid_start_x + col as f32 * (card_w + spacing);
        let cy = grid_y + row as f32 * (card_h + spacing);

        if cy + card_h > screen_h - metrics.s(16.0) {
            break;
        }

        let record = records.get(&(*active_pack, idx));
        let is_completed = record.is_some_and(|r| r.completed);
        let stars = record.map_or(0, |r| r.stars);

        let hovered = mouse_pos.0 >= cx
            && mouse_pos.0 <= cx + card_w
            && mouse_pos.1 >= cy
            && mouse_pos.1 <= cy + card_h;

        let bg = if hovered {
            THEME.surface_hover
        } else if is_completed {
            THEME.surface
        } else {
            THEME.card_bg
        };

        draw_rectangle(cx, cy, card_w, card_h, bg);
        let border_col = if hovered {
            THEME.accent_gold
        } else if is_completed {
            THEME.accent_green
        } else {
            Color::new(0.25, 0.3, 0.4, 0.5)
        };
        let card_border = (2.0 * metrics.scale).max(1.5);
        draw_rectangle_lines(cx, cy, card_w, card_h, card_border, border_col);

        // Level Number
        let num_str = format!("{}", lvl.id);
        let num_dim = measure_text(&num_str, None, card_num_font as u16, 1.0);
        draw_text(
            &num_str,
            cx + (card_w - num_dim.width) / 2.0,
            cy + card_h * 0.44,
            card_num_font,
            THEME.text_primary,
        );

        // Star rating
        textures.draw_star_row(
            cx + card_w / 2.0,
            cy + card_h * 0.74,
            stars,
            3,
            star_size,
            star_spacing,
        );

        if hovered && is_mouse_down {
            action = LevelSelectAction::SelectLevel(*active_pack, idx);
        }
    }

    action
}
