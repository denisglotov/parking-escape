use super::icons::{draw_icon_back, draw_star_rating_row};
use super::{draw_ui_button, ButtonStyle, THEME};
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
    screen_w: f32,
    screen_h: f32,
) -> LevelSelectAction {
    let mut action = LevelSelectAction::None;
    let mouse_pos = mouse_position();
    let is_mouse_down = is_mouse_button_pressed(MouseButton::Left);

    // 1. Header Bar
    let header_h = 70.0;
    draw_rectangle(0.0, 0.0, screen_w, header_h, THEME.surface);

    let btn_size = 38.0;
    let back_hovered = mouse_pos.0 >= 16.0
        && mouse_pos.0 <= 16.0 + btn_size + 40.0
        && mouse_pos.1 >= 16.0
        && mouse_pos.1 <= 16.0 + btn_size;

    draw_rectangle(
        16.0,
        16.0,
        btn_size + 40.0,
        btn_size,
        if back_hovered {
            THEME.surface_hover
        } else {
            THEME.card_bg
        },
    );
    draw_rectangle_lines(
        16.0,
        16.0,
        btn_size + 40.0,
        btn_size,
        1.5,
        Color::new(0.3, 0.35, 0.45, 0.6),
    );
    draw_icon_back(
        32.0,
        16.0 + btn_size / 2.0,
        18.0,
        if back_hovered {
            THEME.accent_gold
        } else {
            THEME.text_primary
        },
    );
    draw_text(
        "Back",
        46.0,
        16.0 + btn_size / 2.0 + 5.0,
        16.0,
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
    let title_dim = measure_text(header_title, None, 24, 1.0);
    draw_text(
        header_title,
        screen_w / 2.0 - title_dim.width / 2.0,
        42.0,
        24.0,
        THEME.text_primary,
    );

    // 2. Pack Tabs (6x6 Beginner, 8x8 Advanced, 10x10 Expert)
    let tabs = [
        (PackType::Grid6x6, "6x6 Beginner"),
        (PackType::Grid8x8, "8x8 Advanced"),
        (PackType::Grid10x10, "10x10 Expert"),
    ];

    let tab_w = ((screen_w - 48.0) / 3.0).min(180.0);
    let tab_h = 44.0;
    let tab_y = 86.0;
    let total_tabs_w = tab_w * 3.0 + 24.0;
    let tab_start_x = (screen_w - total_tabs_w) / 2.0;

    for (i, (pack, label)) in tabs.iter().enumerate() {
        let tx = tab_start_x + i as f32 * (tab_w + 12.0);
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
                font_size: 16.0,
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
    let grid_y = tab_y + tab_h + 30.0;
    let card_w = 110.0;
    let card_h = 100.0;
    let spacing = 16.0;

    let cols = ((screen_w - 32.0) / (card_w + spacing)).floor().max(1.0) as usize;
    let total_grid_w = cols as f32 * card_w + (cols - 1) as f32 * spacing;
    let grid_start_x = (screen_w - total_grid_w) / 2.0;

    for (idx, lvl) in levels.iter().enumerate() {
        let row = idx / cols;
        let col = idx % cols;
        let cx = grid_start_x + col as f32 * (card_w + spacing);
        let cy = grid_y + row as f32 * (card_h + spacing);

        if cy + card_h > screen_h - 20.0 {
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
        draw_rectangle_lines(cx, cy, card_w, card_h, 2.0, border_col);

        // Level Number
        let num_str = format!("{}", lvl.id);
        let num_dim = measure_text(&num_str, None, 28, 1.0);
        draw_text(
            &num_str,
            cx + (card_w - num_dim.width) / 2.0,
            cy + 42.0,
            28.0,
            THEME.text_primary,
        );

        // Star rating rendered as crisp vector stars
        draw_star_rating_row(
            cx + card_w / 2.0,
            cy + 74.0,
            stars,
            3,
            8.0,
            5.0,
            THEME.accent_gold,
        );

        if hovered && is_mouse_down {
            action = LevelSelectAction::SelectLevel(*active_pack, idx);
        }
    }

    action
}
