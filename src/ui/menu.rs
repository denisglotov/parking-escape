use super::{draw_ui_button, ButtonStyle, TextureStore, THEME};
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuAction {
    None,
    Play,
    SelectLevels,
    ToggleSound,
}

pub fn render_main_menu(
    sound_enabled: bool,
    textures: &TextureStore,
    screen_w: f32,
    screen_h: f32,
) -> MenuAction {
    let mut action = MenuAction::None;
    let mouse_pos = mouse_position();
    let is_mouse_down = is_mouse_button_pressed(MouseButton::Left);

    draw_rectangle(0.0, 0.0, screen_w, screen_h, THEME.bg_dark);

    // 1. Parking Badge Logo
    let badge_sz = 84.0;
    let badge_y = screen_h * 0.16;
    if let Some(badge_tex) = textures.get("badge_parking") {
        draw_texture_ex(
            badge_tex,
            screen_w / 2.0 - badge_sz / 2.0,
            badge_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(badge_sz, badge_sz)),
                ..Default::default()
            },
        );
    }

    // 2. Title Typography
    let title = "PARKING ESCAPE";
    let title_font_size = 36.0;
    let title_dim = measure_text(title, None, title_font_size as u16, 1.0);
    draw_text(
        title,
        screen_w / 2.0 - title_dim.width / 2.0,
        badge_y + badge_sz + 44.0,
        title_font_size,
        THEME.text_primary,
    );

    let subtitle = "Dynamic Sliding Logic Puzzle";
    let sub_dim = measure_text(subtitle, None, 18, 1.0);
    draw_text(
        subtitle,
        screen_w / 2.0 - sub_dim.width / 2.0,
        badge_y + badge_sz + 72.0,
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

    // Sound Icon inside button
    let sound_tex_key = if sound_enabled {
        "icon_sound_on"
    } else {
        "icon_sound_off"
    };
    if let Some(snd_tex) = textures.get(sound_tex_key) {
        draw_texture_ex(
            snd_tex,
            btn_x + 20.0,
            btn_y + (btn_h - 26.0) / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(26.0, 26.0)),
                ..Default::default()
            },
        );
    }

    action
}
