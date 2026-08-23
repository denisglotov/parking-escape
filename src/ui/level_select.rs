use super::{draw_ui_button, ButtonStyle, TextureStore, UiMetrics, THEME};
use crate::game::level::{DifficultyTier, FieldSize, LevelRecord, LevelRepository, PackKey};
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LevelSelectAction {
    None,
    SelectLevel(PackKey, usize),
    BackToMenu,
}

pub fn render_level_select(
    repo: &LevelRepository,
    records: &HashMap<(PackKey, usize), LevelRecord>,
    active_pack: &mut PackKey,
    scroll_offset: &mut f32,
    textures: &TextureStore,
    screen_w: f32,
    screen_h: f32,
) -> LevelSelectAction {
    let mut action = LevelSelectAction::None;
    let mouse_pos = mouse_position();
    let is_mouse_down = is_mouse_button_pressed(MouseButton::Left);
    let metrics = UiMetrics::new(screen_w, screen_h);

    // Layout metrics calculation
    let header_h = metrics.hud_height;
    let tab_gap = metrics.s(8.0);
    let tab_padding = metrics.s(16.0);
    let tab_w = ((screen_w - tab_padding * 2.0 - tab_gap * 2.0) / 3.0).min(metrics.s(220.0));
    let tab_h = metrics.s(42.0);
    let tab_y = header_h + metrics.s(12.0);
    let total_tabs_w = tab_w * 3.0 + tab_gap * 2.0;
    let tab_start_x = (screen_w - total_tabs_w) / 2.0;

    let diff_h = metrics.s(38.0);
    let diff_y = tab_y + tab_h + metrics.s(8.0);

    let grid_top = diff_y + diff_h + metrics.s(14.0);
    let grid_bottom = screen_h - metrics.s(10.0);
    let viewport_h = grid_bottom - grid_top;

    // Handle mouse wheel / trackpad scrolling
    let (_, wheel_y) = mouse_wheel();
    if wheel_y.abs() > 0.001 {
        *scroll_offset -= wheel_y * metrics.s(60.0);
    }

    let levels = repo.get_pack(*active_pack);
    let cols: usize = if screen_w > metrics.s(720.0) { 4 } else { 3 };
    let spacing = metrics.s(14.0);
    let grid_side_padding = metrics.s(16.0);
    let total_spacing = spacing * (cols - 1) as f32;
    let available_grid_w = screen_w - grid_side_padding * 2.0;
    let card_w = ((available_grid_w - total_spacing) / cols as f32).min(metrics.s(180.0));
    let card_h = (card_w * 0.90).round();
    let actual_grid_w = cols as f32 * card_w + total_spacing;
    let grid_start_x = (screen_w - actual_grid_w) / 2.0;

    let total_rows = levels.len().div_ceil(cols);
    let total_content_h = if total_rows > 0 {
        total_rows as f32 * card_h + (total_rows - 1) as f32 * spacing
    } else {
        0.0
    };

    let max_scroll = (total_content_h - viewport_h + metrics.s(20.0)).max(0.0);
    *scroll_offset = scroll_offset.clamp(0.0, max_scroll);

    // ==========================================
    // 1. Render Level Grid Cards (Underneath top/bottom masks)
    // ==========================================
    if levels.is_empty() {
        textures.draw_text_centered(
            "Generating levels for this difficulty...",
            screen_w / 2.0,
            grid_top + viewport_h / 2.0,
            metrics.s(20.0),
            THEME.text_secondary,
        );
    } else {
        let card_num_font = metrics.s(32.0);
        let star_size = (card_w * 0.20).clamp(metrics.s(16.0), metrics.s(28.0));
        let star_spacing = metrics.s(4.0);

        for (idx, lvl) in levels.iter().enumerate() {
            let row = idx / cols;
            let col = idx % cols;
            let cx = grid_start_x + col as f32 * (card_w + spacing);
            let cy = grid_top + row as f32 * (card_h + spacing) - *scroll_offset;

            // Only skip cards that are completely off-screen beyond the top header or bottom edge
            if cy + card_h < 0.0 || cy > screen_h + metrics.s(50.0) {
                continue;
            }

            let record = records.get(&(*active_pack, idx));
            let is_completed = record.is_some_and(|r| r.completed);
            let stars = record.map_or(0, |r| r.stars);

            let in_viewport = mouse_pos.1 >= grid_top && mouse_pos.1 <= grid_bottom;
            let hovered = in_viewport
                && Rect::new(cx, cy, card_w, card_h).contains(vec2(mouse_pos.0, mouse_pos.1));

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
            textures.draw_text_centered(
                &num_str,
                cx + card_w / 2.0,
                cy + card_h * 0.42,
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

        // Scrollbar indicator
        if max_scroll > 0.0 {
            let scrollbar_w = metrics.s(4.0);
            let scrollbar_x = screen_w - metrics.s(8.0);
            let thumb_ratio = (viewport_h / total_content_h).clamp(0.1, 1.0);
            let thumb_h = viewport_h * thumb_ratio;
            let scroll_ratio = *scroll_offset / max_scroll;
            let thumb_y = grid_top + scroll_ratio * (viewport_h - thumb_h);

            draw_rectangle(
                scrollbar_x,
                thumb_y,
                scrollbar_w,
                thumb_h,
                Color::new(0.4, 0.45, 0.55, 0.5),
            );
        }
    }

    // ==========================================
    // 2. Render Bottom Viewport Margin Mask
    // ==========================================
    if grid_bottom < screen_h {
        draw_rectangle(
            0.0,
            grid_bottom,
            screen_w,
            screen_h - grid_bottom,
            THEME.bg_dark,
        );
        draw_line(
            0.0,
            grid_bottom,
            screen_w,
            grid_bottom,
            (1.0 * metrics.scale).max(1.0),
            Color::new(0.2, 0.24, 0.32, 0.4),
        );
    }

    // ==========================================
    // 3. Render Top Fixed Control Panel (Header & Tabs over scrolling grid)
    // ==========================================
    // Solid background mask for tab area
    draw_rectangle(0.0, 0.0, screen_w, grid_top, THEME.bg_dark);
    draw_line(
        0.0,
        grid_top,
        screen_w,
        grid_top,
        (1.0 * metrics.scale).max(1.0),
        Color::new(0.2, 0.24, 0.32, 0.6),
    );

    // Row 1: Field Size Tabs (Small 6x6, Medium 8x8, Big 10x10)
    let size_tabs = FieldSize::ALL.map(|s| (s, s.label()));
    for (i, (size, label)) in size_tabs.iter().enumerate() {
        let tx = tab_start_x + i as f32 * (tab_w + tab_gap);
        let is_selected = active_pack.size == *size;

        let bg_col = if is_selected {
            THEME.accent_blue
        } else {
            THEME.card_bg
        };

        if draw_ui_button(
            textures,
            Rect::new(tx, tab_y, tab_w, tab_h),
            label,
            ButtonStyle {
                bg_color: bg_col,
                font_size: metrics.s(17.0),
                border_width: (1.5 * metrics.scale).max(1.0),
                ..Default::default()
            },
            mouse_pos,
            is_mouse_down,
        ) && active_pack.size != *size
        {
            active_pack.size = *size;
            *scroll_offset = 0.0;
        }
    }

    // Row 2: Difficulty Tier Tabs (Relaxed, Challenging, Hard)
    let diff_tabs = DifficultyTier::ALL.map(|d| (d, d.label()));
    for (i, (diff, label)) in diff_tabs.iter().enumerate() {
        let tx = tab_start_x + i as f32 * (tab_w + tab_gap);
        let is_selected = active_pack.difficulty == *diff;

        let bg_col = if is_selected {
            Color::new(0.20, 0.45, 0.85, 1.0)
        } else {
            THEME.surface
        };

        if draw_ui_button(
            textures,
            Rect::new(tx, diff_y, tab_w, diff_h),
            label,
            ButtonStyle {
                bg_color: bg_col,
                font_size: metrics.s(16.0),
                border_width: (1.5 * metrics.scale).max(1.0),
                ..Default::default()
            },
            mouse_pos,
            is_mouse_down,
        ) && active_pack.difficulty != *diff
        {
            active_pack.difficulty = *diff;
            *scroll_offset = 0.0;
        }
    }

    // Header Bar
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

    let btn_h = metrics.s(48.0);
    let btn_w = metrics.s(104.0);
    let btn_pad_x = metrics.s(16.0);
    let btn_y = (header_h - btn_h) / 2.0;

    let btn_rect = Rect::new(btn_pad_x, btn_y, btn_w, btn_h);
    let back_hovered = btn_rect.contains(vec2(mouse_pos.0, mouse_pos.1));

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

    let icon_sz = metrics.s(24.0);
    if let Some(back_tex) = textures.get("icon_back") {
        draw_texture_ex(
            back_tex,
            btn_pad_x + metrics.s(10.0),
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
    let back_text_size = metrics.s(20.0);
    let back_color = if back_hovered {
        THEME.accent_gold
    } else {
        THEME.text_primary
    };
    textures.draw_text(
        "Back",
        btn_pad_x + icon_sz + metrics.s(14.0),
        btn_y + btn_h / 2.0 + metrics.s(6.0),
        back_text_size,
        back_color,
    );

    if back_hovered && is_mouse_down {
        action = LevelSelectAction::BackToMenu;
    }

    let header_title = "SELECT LEVEL";
    let title_font_size = metrics.s(28.0);
    textures.draw_text_centered(
        header_title,
        screen_w / 2.0,
        header_h / 2.0,
        title_font_size,
        THEME.text_primary,
    );

    action
}
