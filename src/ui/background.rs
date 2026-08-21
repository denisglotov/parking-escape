use super::{BoardLayout, TextureStore};
use crate::game::board::{Board, ExitSide};
use macroquad::prelude::*;

const ROAD_ASPHALT: Color = Color::new(0.16, 0.18, 0.22, 1.0);
const ROAD_CURB: Color = Color::new(0.30, 0.34, 0.40, 1.0);
const ROAD_MARKING: Color = Color::new(0.95, 0.82, 0.20, 0.85);

/// Renders the park nature background texture and exit roadway connection.
pub fn render_nature_background(board: &Board, layout: &BoardLayout, textures: &TextureStore) {
    let sw = layout.screen_width;
    let sh = layout.screen_height;
    let start_y = layout.hud_height;
    let area_h = (sh - start_y).max(0.0);

    // 1. Draw park background texture (or fallback solid fill)
    if let Some(park_tex) = textures.get("park_background") {
        draw_texture_ex(
            park_tex,
            0.0,
            start_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(sw, area_h)),
                ..Default::default()
            },
        );
    } else {
        draw_rectangle(0.0, start_y, sw, area_h, Color::new(0.12, 0.22, 0.14, 1.0));
    }

    // 2. Draw asphalt exit road connecting exit gate to edge of screen
    render_exit_road(board, layout, textures);
}

/// Renders the asphalt exit road extending seamlessly from the exit gate to the screen border.
fn render_exit_road(board: &Board, layout: &BoardLayout, textures: &TextureStore) {
    let ox = layout.origin_x;
    let oy = layout.origin_y;
    let bw = layout.total_width;
    let bh = layout.total_height;
    let cs = layout.cell_size;
    let sw = layout.screen_width;
    let sh = layout.screen_height;
    let curb_thick = (cs * 0.14).max(6.0);

    let (rx, ry, rw, rh, is_horizontal, dash_start, dash_end) = match board.exit.side {
        ExitSide::Right => {
            let row_y = oy + board.exit.row as f32 * cs;
            let rx = ox + bw;
            (rx, row_y, (sw - rx).max(0.0), cs, true, rx + 6.0, sw - 6.0)
        }
        ExitSide::Left => {
            let row_y = oy + board.exit.row as f32 * cs;
            (0.0, row_y, ox, cs, true, 6.0, ox - 6.0)
        }
        ExitSide::Bottom => {
            let col_x = ox + board.exit.col as f32 * cs;
            let ry = oy + bh;
            (col_x, ry, cs, (sh - ry).max(0.0), false, ry + 6.0, sh - 6.0)
        }
        ExitSide::Top => {
            let col_x = ox + board.exit.col as f32 * cs;
            let ry = layout.hud_height;
            (col_x, ry, cs, (oy - ry).max(0.0), false, ry + 6.0, oy - 6.0)
        }
    };

    if let Some(asphalt_tex) = textures.get("asphalt") {
        draw_texture_ex(
            asphalt_tex,
            rx,
            ry,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(rw, rh)),
                ..Default::default()
            },
        );
    } else {
        draw_rectangle(rx, ry, rw, rh, ROAD_ASPHALT);
    }

    if is_horizontal {
        draw_rectangle(rx, ry - curb_thick, rw, curb_thick, ROAD_CURB);
        draw_rectangle(rx, ry + rh, rw, curb_thick, ROAD_CURB);
        draw_dashed_line_h(
            dash_start,
            dash_end,
            ry + rh / 2.0,
            2.5,
            12.0,
            8.0,
            ROAD_MARKING,
        );
    } else {
        draw_rectangle(rx - curb_thick, ry, curb_thick, rh, ROAD_CURB);
        draw_rectangle(rx + rw, ry, curb_thick, rh, ROAD_CURB);
        draw_dashed_line_v(
            rx + rw / 2.0,
            dash_start,
            dash_end,
            2.5,
            12.0,
            8.0,
            ROAD_MARKING,
        );
    }
}

fn draw_dashed_line_h(
    x1: f32,
    x2: f32,
    y: f32,
    thick: f32,
    dash_len: f32,
    gap_len: f32,
    color: Color,
) {
    let mut cur_x = x1;
    while cur_x < x2 {
        let end_x = (cur_x + dash_len).min(x2);
        draw_line(cur_x, y, end_x, y, thick, color);
        cur_x += dash_len + gap_len;
    }
}

fn draw_dashed_line_v(
    x: f32,
    y1: f32,
    y2: f32,
    thick: f32,
    dash_len: f32,
    gap_len: f32,
    color: Color,
) {
    let mut cur_y = y1;
    while cur_y < y2 {
        let end_y = (cur_y + dash_len).min(y2);
        draw_line(x, cur_y, x, end_y, thick, color);
        cur_y += dash_len + gap_len;
    }
}
