use super::{draw_ui_button, ButtonStyle, ShadowTextStyle, TextureStore, UiMetrics, THEME};
use crate::game::i18n::LocaleStrings;
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
    locales: &LocaleStrings,
    textures: &TextureStore,
    screen_w: f32,
    screen_h: f32,
) -> MenuAction {
    let mut action = MenuAction::None;
    let mouse_pos = mouse_position();
    let is_mouse_down = is_mouse_button_pressed(MouseButton::Left);
    let metrics = UiMetrics::new(screen_w, screen_h);

    draw_rectangle(0.0, 0.0, screen_w, screen_h, THEME.bg_dark);

    // 1. Parking Badge Logo
    let badge_sz = metrics.s(96.0);
    let badge_y = (screen_h * 0.14).max(metrics.s(32.0));
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

    // 2. Title Typography (Large, Punchy with Shadow)
    let title = &locales.menu.title;
    let title_font_size = metrics.s(48.0);
    let title_y = badge_y + badge_sz + metrics.s(44.0);
    textures.draw_text_with_shadow(
        title,
        screen_w / 2.0,
        title_y,
        ShadowTextStyle::new(
            title_font_size,
            THEME.text_primary,
            Color::new(0.0, 0.0, 0.0, 0.6),
            metrics.s(3.0),
        ),
    );

    let subtitle = &locales.menu.subtitle;
    let sub_font_size = metrics.s(22.0);
    let sub_y = title_y + metrics.s(34.0);
    textures.draw_text_centered(
        subtitle,
        screen_w / 2.0,
        sub_y,
        sub_font_size,
        THEME.accent_blue,
    );

    // 3. Menu Buttons (Bigger, Generous Touch Targets)
    let btn_w = (screen_w * 0.82).min(metrics.s(380.0));
    let btn_h = metrics.s(64.0);
    let btn_x = (screen_w - btn_w) / 2.0;
    let mut btn_y = (sub_y + metrics.s(64.0)).max(screen_h * 0.48);
    let btn_font_size = metrics.s(24.0);
    let btn_border = metrics.s(2.0).max(1.5);
    let spacing = metrics.s(18.0);

    let btn_style = |bg_color| ButtonStyle {
        bg_color,
        font_size: btn_font_size,
        border_width: btn_border,
        ..Default::default()
    };

    if draw_ui_button(
        textures,
        Rect::new(btn_x, btn_y, btn_w, btn_h),
        &locales.menu.play_game,
        btn_style(THEME.accent_green),
        mouse_pos,
        is_mouse_down,
    ) {
        action = MenuAction::Play;
    }
    btn_y += btn_h + spacing;

    if draw_ui_button(
        textures,
        Rect::new(btn_x, btn_y, btn_w, btn_h),
        &locales.menu.level_select,
        btn_style(THEME.card_bg),
        mouse_pos,
        is_mouse_down,
    ) {
        action = MenuAction::SelectLevels;
    }
    btn_y += btn_h + spacing;

    let sound_label = if sound_enabled {
        &locales.menu.sound_on
    } else {
        &locales.menu.sound_off
    };
    if draw_ui_button(
        textures,
        Rect::new(btn_x, btn_y, btn_w, btn_h),
        sound_label,
        btn_style(THEME.surface),
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
        let icon_sz = metrics.s(32.0);
        draw_texture_ex(
            snd_tex,
            btn_x + metrics.s(20.0),
            btn_y + (btn_h - icon_sz) / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(icon_sz, icon_sz)),
                ..Default::default()
            },
        );
    }

    action
}
