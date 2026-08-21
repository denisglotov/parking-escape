use super::{draw_ui_button, ButtonStyle, ShadowTextStyle, TextureStore, UiMetrics, THEME};
use crate::game::level::LevelData;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WinModalAction {
    None,
    NextLevel,
    Replay,
    LevelSelect,
}

pub fn render_win_modal(
    level: &LevelData,
    moves_taken: u32,
    has_next_level: bool,
    textures: &TextureStore,
    screen_w: f32,
    screen_h: f32,
) -> WinModalAction {
    let mut action = WinModalAction::None;
    let metrics = UiMetrics::new(screen_w, screen_h);

    // Semi-transparent backdrop overlay
    draw_rectangle(
        0.0,
        0.0,
        screen_w,
        screen_h,
        Color::new(0.0, 0.0, 0.0, 0.75),
    );

    let modal_w = (screen_w * 0.90).min(metrics.s(480.0));
    let modal_h = metrics.s(430.0);
    let modal_x = (screen_w - modal_w) / 2.0;
    let modal_y = (screen_h - modal_h) / 2.0;

    // Modal card background & border
    draw_rectangle(modal_x, modal_y, modal_w, modal_h, THEME.card_bg);
    let modal_border = (2.5 * metrics.scale).max(2.0);
    draw_rectangle_lines(
        modal_x,
        modal_y,
        modal_w,
        modal_h,
        modal_border,
        THEME.accent_gold,
    );

    let mouse_pos = mouse_position();
    let is_mouse_down = is_mouse_button_pressed(MouseButton::Left);

    // Title: LEVEL CLEARED! (Big Celebration)
    let title = "LEVEL CLEARED!";
    let title_font_size = metrics.s(38.0);
    textures.draw_text_with_shadow(
        title,
        modal_x + modal_w / 2.0,
        modal_y + metrics.s(50.0),
        ShadowTextStyle::new(
            title_font_size,
            THEME.accent_green,
            Color::new(0.0, 0.0, 0.0, 0.5),
            metrics.s(2.5),
        ),
    );

    // Large 3D Gold Stars
    let stars = level.calculate_stars(moves_taken);
    let star_size = metrics.s(52.0);
    let star_spacing = metrics.s(14.0);
    textures.draw_star_row(
        modal_x + modal_w / 2.0,
        modal_y + metrics.s(116.0),
        stars,
        3,
        star_size,
        star_spacing,
    );

    // Moves vs Par moves summary (Large & Clear)
    let stats_text = format!(
        "Completed in {} moves! (Par: {})",
        moves_taken, level.par_moves
    );
    let stats_font = metrics.s(24.0);
    textures.draw_text_centered(
        &stats_text,
        modal_x + modal_w / 2.0,
        modal_y + metrics.s(176.0),
        stats_font,
        THEME.text_secondary,
    );

    let rating_eval = if moves_taken <= level.par_moves {
        "Perfect Strategy!"
    } else if moves_taken <= level.par_moves + 2 {
        "Great Driving!"
    } else {
        "Good Job!"
    };
    let eval_font = metrics.s(22.0);
    textures.draw_text_centered(
        rating_eval,
        modal_x + modal_w / 2.0,
        modal_y + metrics.s(214.0),
        eval_font,
        THEME.accent_gold,
    );

    // Action Buttons (Enlarged)
    let btn_h = metrics.s(54.0);
    let btn_w = modal_w - metrics.s(48.0);
    let mut btn_y = modal_y + metrics.s(256.0);
    let btn_font_size = metrics.s(22.0);
    let btn_border = (1.5 * metrics.scale).max(1.0);
    let btn_spacing = metrics.s(14.0);

    if has_next_level {
        if draw_ui_button(
            textures,
            Rect::new(modal_x + metrics.s(24.0), btn_y, btn_w, btn_h),
            "NEXT LEVEL",
            ButtonStyle {
                bg_color: THEME.accent_green,
                font_size: btn_font_size,
                border_width: btn_border,
                ..Default::default()
            },
            mouse_pos,
            is_mouse_down,
        ) {
            action = WinModalAction::NextLevel;
        }
        btn_y += btn_h + btn_spacing;
    }

    let half_w = (btn_w - btn_spacing) / 2.0;
    let secondary_style = ButtonStyle {
        bg_color: THEME.surface,
        text_color: THEME.text_secondary,
        font_size: btn_font_size,
        border_width: btn_border,
    };

    if draw_ui_button(
        textures,
        Rect::new(modal_x + metrics.s(24.0), btn_y, half_w, btn_h),
        "REPLAY",
        secondary_style,
        mouse_pos,
        is_mouse_down,
    ) {
        action = WinModalAction::Replay;
    }

    if draw_ui_button(
        textures,
        Rect::new(
            modal_x + metrics.s(24.0) + half_w + btn_spacing,
            btn_y,
            half_w,
            btn_h,
        ),
        "MENU",
        secondary_style,
        mouse_pos,
        is_mouse_down,
    ) {
        action = WinModalAction::LevelSelect;
    }

    action
}
