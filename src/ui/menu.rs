use super::icons::{draw_icon_sound, draw_parking_badge};
use super::{draw_ui_button, ButtonStyle, THEME};
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    None,
    Play,
    SelectLevels,
    ToggleSound,
}

pub fn render_main_menu(sound_enabled: bool, screen_w: f32, screen_h: f32) -> MenuAction {
    let mut action = MenuAction::None;
    let mouse_pos = mouse_position();
    let is_mouse_down = is_mouse_button_pressed(MouseButton::Left);

    // Background gradient effect
    draw_rectangle(0.0, 0.0, screen_w, screen_h, THEME.bg_dark);

    // 1. Game Logo Emblem (Parking "P" Shield)
    let badge_size = 72.0;
    let logo_y = screen_h * 0.20;
    draw_parking_badge(screen_w / 2.0, logo_y, badge_size);

    // 2. Title Typography
    let title = "PARKING ESCAPE";
    let title_font_size = 36.0;
    let title_dim = measure_text(title, None, title_font_size as u16, 1.0);
    draw_text(
        title,
        screen_w / 2.0 - title_dim.width / 2.0,
        logo_y + badge_size / 2.0 + 44.0,
        title_font_size,
        THEME.text_primary,
    );

    let subtitle = "Dynamic Sliding Logic Puzzle";
    let sub_dim = measure_text(subtitle, None, 18, 1.0);
    draw_text(
        subtitle,
        screen_w / 2.0 - sub_dim.width / 2.0,
        logo_y + badge_size / 2.0 + 72.0,
        18.0,
        THEME.accent_blue,
    );

    // 3. Menu Buttons
    let btn_w = (screen_w * 0.7).min(280.0);
    let btn_h = 52.0;
    let btn_x = (screen_w - btn_w) / 2.0;
    let mut btn_y = screen_h * 0.48;

    if draw_ui_button(
        Rect::new(btn_x, btn_y, btn_w, btn_h),
        "PLAY GAME",
        ButtonStyle {
            bg_color: THEME.accent_green,
            font_size: 20.0,
            border_width: 2.0,
            ..Default::default()
        },
        mouse_pos,
        is_mouse_down,
    ) {
        action = MenuAction::Play;
    }
    btn_y += btn_h + 16.0;

    if draw_ui_button(
        Rect::new(btn_x, btn_y, btn_w, btn_h),
        "LEVEL SELECT",
        ButtonStyle {
            bg_color: THEME.card_bg,
            font_size: 20.0,
            border_width: 2.0,
            ..Default::default()
        },
        mouse_pos,
        is_mouse_down,
    ) {
        action = MenuAction::SelectLevels;
    }
    btn_y += btn_h + 16.0;

    let sound_label = if sound_enabled {
        "SOUND: ON"
    } else {
        "SOUND: OFF"
    };
    if draw_ui_button(
        Rect::new(btn_x, btn_y, btn_w, btn_h),
        sound_label,
        ButtonStyle {
            bg_color: THEME.surface,
            font_size: 20.0,
            border_width: 2.0,
            ..Default::default()
        },
        mouse_pos,
        is_mouse_down,
    ) {
        action = MenuAction::ToggleSound;
    }

    // Draw sound icon inside button
    let icon_sz = 20.0;
    draw_icon_sound(
        btn_x + 36.0,
        btn_y + btn_h / 2.0,
        icon_sz,
        sound_enabled,
        if sound_enabled {
            THEME.accent_gold
        } else {
            THEME.text_muted
        },
    );

    action
}
