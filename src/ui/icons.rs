use macroquad::prelude::*;

/// Draws a 5-pointed star centered at (cx, cy).
pub fn draw_star(cx: f32, cy: f32, radius: f32, filled: bool, color: Color) {
    let points = 5;
    let inner_radius = radius * 0.42;
    let angle_step = std::f32::consts::PI / points as f32;
    let start_angle = -std::f32::consts::FRAC_PI_2;

    let mut vertices = Vec::with_capacity(points * 2);
    for i in 0..(points * 2) {
        let r = if i % 2 == 0 { radius } else { inner_radius };
        let angle = start_angle + i as f32 * angle_step;
        vertices.push(vec2(cx + angle.cos() * r, cy + angle.sin() * r));
    }

    if filled {
        // Draw triangles from center
        for i in 0..vertices.len() {
            let next = (i + 1) % vertices.len();
            draw_triangle(vec2(cx, cy), vertices[i], vertices[next], color);
        }
        // Subtle outline
        for i in 0..vertices.len() {
            let next = (i + 1) % vertices.len();
            draw_line(
                vertices[i].x,
                vertices[i].y,
                vertices[next].x,
                vertices[next].y,
                1.5,
                Color::new(
                    (color.r * 1.2).min(1.0),
                    (color.g * 1.2).min(1.0),
                    (color.b * 1.2).min(1.0),
                    1.0,
                ),
            );
        }
    } else {
        // Empty / outline star
        for i in 0..vertices.len() {
            let next = (i + 1) % vertices.len();
            draw_line(
                vertices[i].x,
                vertices[i].y,
                vertices[next].x,
                vertices[next].y,
                2.0,
                Color::new(color.r * 0.4, color.g * 0.4, color.b * 0.4, 0.6),
            );
        }
    }
}

/// Draws a row of stars (e.g. 3 stars) centered horizontally.
pub fn draw_star_rating_row(
    cx: f32,
    cy: f32,
    earned: u8,
    total: u8,
    star_size: f32,
    spacing: f32,
    gold_col: Color,
) {
    let total_w = total as f32 * star_size * 2.0 + (total - 1) as f32 * spacing;
    let start_x = cx - total_w / 2.0 + star_size;

    for i in 0..total {
        let sx = start_x + i as f32 * (star_size * 2.0 + spacing);
        let is_earned = i < earned;
        draw_star(
            sx,
            cy,
            star_size,
            is_earned,
            if is_earned {
                gold_col
            } else {
                Color::new(0.4, 0.4, 0.5, 0.4)
            },
        );
    }
}

/// Draws an international Parking "P" emblem badge.
pub fn draw_parking_badge(cx: f32, cy: f32, size: f32) {
    let half = size / 2.0;
    let x = cx - half;
    let y = cy - half;

    // Drop shadow
    draw_rectangle(x + 2.0, y + 4.0, size, size, Color::new(0.0, 0.0, 0.0, 0.4));

    // Blue badge background (#2563eb)
    draw_rectangle(x, y, size, size, Color::new(0.14, 0.38, 0.92, 1.0));
    draw_rectangle_lines(x, y, size, size, 2.5, Color::new(0.4, 0.6, 1.0, 0.9));

    // Glossy top reflection
    draw_rectangle(
        x + 4.0,
        y + 4.0,
        size - 8.0,
        size * 0.35,
        Color::new(1.0, 1.0, 1.0, 0.15),
    );

    // White "P" letter
    let p_font_size = size * 0.72;
    let p_dim = measure_text("P", None, p_font_size as u16, 1.0);
    draw_text(
        "P",
        cx - p_dim.width / 2.0,
        cy + p_dim.height * 0.36,
        p_font_size,
        WHITE,
    );
}

/// Draws a vector Back arrow (chevron pointing left).
pub fn draw_icon_back(cx: f32, cy: f32, size: f32, color: Color) {
    let hw = size * 0.35;
    let hh = size * 0.35;
    let lw = (size * 0.15).max(2.0);

    draw_line(cx + hw * 0.5, cy - hh, cx - hw * 0.5, cy, lw, color);
    draw_line(cx - hw * 0.5, cy, cx + hw * 0.5, cy + hh, lw, color);
}

/// Draws a vector Undo circular arrow.
pub fn draw_icon_undo(cx: f32, cy: f32, size: f32, color: Color) {
    let r = size * 0.32;
    let lw = (size * 0.12).max(2.0);

    let start_angle = std::f32::consts::PI * 0.2;
    let end_angle = std::f32::consts::PI * 1.8;
    let segments = 16;
    let step = (end_angle - start_angle) / segments as f32;

    for i in 0..segments {
        let a1 = start_angle + i as f32 * step;
        let a2 = start_angle + (i + 1) as f32 * step;
        draw_line(
            cx + a1.cos() * r,
            cy + a1.sin() * r,
            cx + a2.cos() * r,
            cy + a2.sin() * r,
            lw,
            color,
        );
    }

    let arrow_x = cx + start_angle.cos() * r;
    let arrow_y = cy + start_angle.sin() * r;
    let ah = size * 0.22;
    draw_triangle(
        vec2(arrow_x - ah * 0.8, arrow_y + ah * 0.4),
        vec2(arrow_x + ah * 0.4, arrow_y + ah * 0.8),
        vec2(arrow_x + ah * 0.2, arrow_y - ah * 0.8),
        color,
    );
}

/// Draws a vector Reset circular arrow.
pub fn draw_icon_reset(cx: f32, cy: f32, size: f32, color: Color) {
    let r = size * 0.32;
    let lw = (size * 0.12).max(2.0);

    let start_angle = -std::f32::consts::PI * 0.7;
    let end_angle = std::f32::consts::PI * 0.9;
    let segments = 16;
    let step = (end_angle - start_angle) / segments as f32;

    for i in 0..segments {
        let a1 = start_angle + i as f32 * step;
        let a2 = start_angle + (i + 1) as f32 * step;
        draw_line(
            cx + a1.cos() * r,
            cy + a1.sin() * r,
            cx + a2.cos() * r,
            cy + a2.sin() * r,
            lw,
            color,
        );
    }

    let arrow_x = cx + end_angle.cos() * r;
    let arrow_y = cy + end_angle.sin() * r;
    let ah = size * 0.22;
    draw_triangle(
        vec2(arrow_x + ah * 0.8, arrow_y - ah * 0.4),
        vec2(arrow_x - ah * 0.4, arrow_y - ah * 0.8),
        vec2(arrow_x - ah * 0.2, arrow_y + ah * 0.8),
        color,
    );
}

/// Draws a vector Sound / Audio icon.
pub fn draw_icon_sound(cx: f32, cy: f32, size: f32, enabled: bool, color: Color) {
    let sx = cx - size * 0.15;
    let sy = cy;
    let sw = size * 0.2;
    let sh = size * 0.35;

    draw_rectangle(sx - sw * 0.6, sy - sh * 0.35, sw * 0.6, sh * 0.7, color);
    draw_triangle(
        vec2(sx, sy - sh * 0.6),
        vec2(sx, sy + sh * 0.6),
        vec2(sx - sw * 0.3, sy),
        color,
    );

    let lw = (size * 0.1).max(2.0);

    if enabled {
        let wave_r1 = size * 0.22;
        let wave_r2 = size * 0.36;
        for i in -2..=2 {
            let a = i as f32 * 0.25;
            let na = (i + 1) as f32 * 0.25;
            draw_line(
                sx + a.cos() * wave_r1,
                sy + a.sin() * wave_r1,
                sx + na.cos() * wave_r1,
                sy + na.sin() * wave_r1,
                lw,
                color,
            );
            draw_line(
                sx + a.cos() * wave_r2,
                sy + a.sin() * wave_r2,
                sx + na.cos() * wave_r2,
                sy + na.sin() * wave_r2,
                lw,
                color,
            );
        }
    } else {
        let cross_size = size * 0.25;
        let cross_x = cx + size * 0.2;
        draw_line(
            cross_x - cross_size,
            sy - cross_size,
            cross_x + cross_size,
            sy + cross_size,
            lw + 0.5,
            Color::new(0.9, 0.2, 0.2, 1.0),
        );
        draw_line(
            cross_x - cross_size,
            sy + cross_size,
            cross_x + cross_size,
            sy - cross_size,
            lw + 0.5,
            Color::new(0.9, 0.2, 0.2, 1.0),
        );
    }
}
