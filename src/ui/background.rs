use super::{BoardLayout, TextureStore};
use crate::game::board::Board;
use crate::game::Theme;
use macroquad::prelude::*;

/// Computes the source crop rectangle to cover the destination area while preserving aspect ratio (aspect-fill / center-crop).
pub fn compute_aspect_cover_crop(tex_w: f32, tex_h: f32, target_w: f32, target_h: f32) -> Rect {
    if tex_w <= 0.0 || tex_h <= 0.0 || target_w <= 0.0 || target_h <= 0.0 {
        return Rect::new(0.0, 0.0, tex_w.max(0.0), tex_h.max(0.0));
    }

    let target_aspect = target_w / target_h;
    let tex_aspect = tex_w / tex_h;

    if target_aspect > tex_aspect {
        // Target is wider than texture (e.g. horizontal / landscape mode)
        let crop_h = (tex_w / target_aspect).min(tex_h);
        let crop_y = ((tex_h - crop_h) / 2.0).max(0.0);
        Rect::new(0.0, crop_y, tex_w, crop_h)
    } else {
        // Target is taller than texture (or identical aspect ratio)
        let crop_w = (tex_h * target_aspect).min(tex_w);
        let crop_x = ((tex_w - crop_w) / 2.0).max(0.0);
        Rect::new(crop_x, 0.0, crop_w, tex_h)
    }
}

/// Renders the background texture for the active theme without stretching.
pub fn render_nature_background(board: &Board, layout: &BoardLayout, textures: &TextureStore) {
    let sw = layout.screen_width;
    let sh = layout.screen_height;
    let start_y = layout.hud_height;
    let area_h = (sh - start_y).max(0.0);

    let bg_tex_key = board.theme.background_texture_key();

    // Draw background texture (or fallback solid fill)
    if let Some(bg_tex) = textures.get(bg_tex_key) {
        if sw > 0.0 && area_h > 0.0 && bg_tex.width() > 0.0 && bg_tex.height() > 0.0 {
            let src_rect = compute_aspect_cover_crop(bg_tex.width(), bg_tex.height(), sw, area_h);
            draw_texture_ex(
                &bg_tex,
                0.0,
                start_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(sw, area_h)),
                    source: Some(src_rect),
                    ..Default::default()
                },
            );
        }
    } else {
        let fallback_color = match board.theme {
            Theme::Marine => Color::new(0.12, 0.38, 0.58, 1.0),
            Theme::City => Color::new(0.12, 0.22, 0.14, 1.0),
            Theme::Railroad => Color::new(0.20, 0.38, 0.22, 1.0),
        };
        draw_rectangle(0.0, start_y, sw, area_h, fallback_color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aspect_cover_crop_horizontal_landscape() {
        // Texture is 768 x 1376 (portrait), Target is 1920 x 960 (horizontal 2:1)
        let rect = compute_aspect_cover_crop(768.0, 1376.0, 1920.0, 960.0);
        assert_eq!(rect.x, 0.0);
        assert_eq!(rect.w, 768.0);
        // Crop height should match the 2:1 target aspect: 768 / 2 = 384
        assert!((rect.h - 384.0).abs() < 1e-3);
        // Crop should be centered vertically: (1376 - 384) / 2 = 496
        assert!((rect.y - 496.0).abs() < 1e-3);

        // Aspect ratio of source crop should match target aspect ratio
        let crop_aspect = rect.w / rect.h;
        let target_aspect = 1920.0 / 960.0;
        assert!((crop_aspect - target_aspect).abs() < 1e-3);
    }

    #[test]
    fn test_aspect_cover_crop_vertical_portrait() {
        // Texture is 768 x 1376 (~0.558 aspect), Target is 1080 x 2400 (0.45 aspect - taller portrait)
        let rect = compute_aspect_cover_crop(768.0, 1376.0, 1080.0, 2400.0);
        assert_eq!(rect.y, 0.0);
        assert_eq!(rect.h, 1376.0);
        // Crop width should match target aspect: 1376 * (1080 / 2400) = 619.2
        assert!((rect.w - 619.2).abs() < 1e-3);
        // Crop should be centered horizontally: (768 - 619.2) / 2 = 74.4
        assert!((rect.x - 74.4).abs() < 1e-3);

        let crop_aspect = rect.w / rect.h;
        let target_aspect = 1080.0 / 2400.0;
        assert!((crop_aspect - target_aspect).abs() < 1e-3);
    }

    #[test]
    fn test_aspect_cover_crop_exact_aspect() {
        let rect = compute_aspect_cover_crop(768.0, 1376.0, 768.0, 1376.0);
        assert!((rect.x - 0.0).abs() < 1e-3);
        assert!((rect.y - 0.0).abs() < 1e-3);
        assert!((rect.w - 768.0).abs() < 1e-3);
        assert!((rect.h - 1376.0).abs() < 1e-3);
    }
}
