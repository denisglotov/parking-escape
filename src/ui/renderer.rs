use super::background::{render_lamppost_fixtures, render_nature_background};
use super::{BoardLayout, TextureStore, THEME};
use crate::game::board::{Board, ExitSide};
use macroquad::prelude::*;

pub fn render_board(board: &Board, layout: &BoardLayout, textures: &TextureStore) {
    let ox = layout.origin_x;
    let oy = layout.origin_y;
    let cs = layout.cell_size;
    let bw = layout.total_width;
    let bh = layout.total_height;

    // 1. Draw Nature Background (Park lawn, footpaths, pond with animated ripples, flora, lamppost light halos)
    render_nature_background(board, layout, textures);

    // 2. Draw outer drop shadow for the parking lot
    draw_rectangle(
        ox - 6.0,
        oy - 4.0,
        bw + 12.0,
        bh + 14.0,
        Color::new(0.0, 0.0, 0.0, 0.45),
    );

    // 2. Draw Tiled Asphalt Ground
    if let Some(asphalt_tex) = textures.get("asphalt") {
        draw_texture_ex(
            asphalt_tex,
            ox,
            oy,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(bw, bh)),
                ..Default::default()
            },
        );
    } else {
        draw_rectangle(ox, oy, bw, bh, Color::new(0.12, 0.13, 0.16, 1.0));
    }

    // 3. Draw Stall Grid Lines & Markers
    if let Some(marker_tex) = textures.get("stall_marker") {
        for gx in 0..board.width {
            for gy in 0..board.height {
                draw_texture_ex(
                    marker_tex,
                    ox + gx as f32 * cs,
                    oy + gy as f32 * cs,
                    Color::new(1.0, 1.0, 1.0, 0.35),
                    DrawTextureParams {
                        dest_size: Some(vec2(cs, cs)),
                        ..Default::default()
                    },
                );
            }
        }
    }

    // 4. Draw Concrete Curbs & Border Perimeter
    let curb_thick = (cs * 0.14).max(6.0);
    draw_rectangle(
        ox - curb_thick,
        oy - curb_thick,
        bw + curb_thick * 2.0,
        curb_thick,
        Color::new(0.32, 0.36, 0.42, 1.0),
    );
    draw_rectangle(
        ox - curb_thick,
        oy + bh,
        bw + curb_thick * 2.0,
        curb_thick,
        Color::new(0.24, 0.28, 0.34, 1.0),
    );
    draw_rectangle(
        ox - curb_thick,
        oy,
        curb_thick,
        bh,
        Color::new(0.28, 0.32, 0.38, 1.0),
    );
    draw_rectangle(
        ox + bw,
        oy,
        curb_thick,
        bh,
        Color::new(0.28, 0.32, 0.38, 1.0),
    );

    // 5. Draw Exit Gate
    render_exit_gate(board, layout, textures);

    // 6. Draw Vehicles
    render_vehicles(board, layout, textures);

    // 7. Draw Lamppost Fixtures
    render_lamppost_fixtures(board, layout);
}

fn render_exit_gate(board: &Board, layout: &BoardLayout, textures: &TextureStore) {
    let ox = layout.origin_x;
    let oy = layout.origin_y;
    let cs = layout.cell_size;
    let bw = layout.total_width;
    let bh = layout.total_height;

    let time = get_time() as f32;
    let glow_pulse = (time * 4.0).sin() * 0.2 + 0.8;

    let (gx, gy, gw, gh, rot): (f32, f32, f32, f32, f32) = match board.exit.side {
        ExitSide::Right => {
            let row_y = oy + board.exit.row as f32 * cs;
            (ox + bw - 4.0, row_y, cs * 0.35, cs, 0.0)
        }
        ExitSide::Left => {
            let row_y = oy + board.exit.row as f32 * cs;
            (ox - cs * 0.35 + 4.0, row_y, cs * 0.35, cs, 180.0)
        }
        ExitSide::Bottom => {
            let col_x = ox + board.exit.col as f32 * cs;
            (col_x, oy + bh - 4.0, cs, cs * 0.35, 90.0)
        }
        ExitSide::Top => {
            let col_x = ox + board.exit.col as f32 * cs;
            (col_x, oy - cs * 0.35 + 4.0, cs, cs * 0.35, 270.0)
        }
    };

    draw_rectangle(
        gx,
        gy,
        gw,
        gh,
        Color::new(0.02, 0.25, 0.15, 0.85 * glow_pulse),
    );

    if let Some(gate_tex) = textures.get("exit_gate") {
        draw_texture_ex(
            gate_tex,
            gx,
            gy,
            Color::new(1.0, 1.0, 1.0, glow_pulse),
            DrawTextureParams {
                dest_size: Some(vec2(gw, gh)),
                rotation: rot.to_radians(),
                ..Default::default()
            },
        );
    } else {
        draw_rectangle_lines(gx, gy, gw, gh, 2.0, THEME.accent_green);
    }
}

fn render_vehicles(board: &Board, layout: &BoardLayout, textures: &TextureStore) {
    let ox = layout.origin_x;
    let oy = layout.origin_y;
    let cs = layout.cell_size;

    for (idx, veh) in board.vehicles.iter().enumerate() {
        let is_being_dragged = board
            .active_drag
            .as_ref()
            .is_some_and(|d| d.vehicle_index == idx);

        let (mut px, mut py, pw, ph) = veh.pixel_bounds(ox, oy, cs);

        if veh.is_player && board.is_won {
            let exit_offset = board.exit_animation_progress * cs * 3.5;
            match board.exit.side {
                ExitSide::Right => px += exit_offset,
                ExitSide::Left => px -= exit_offset,
                ExitSide::Bottom => py += exit_offset,
                ExitSide::Top => py -= exit_offset,
            }
        }

        if is_being_dragged {
            py -= 2.0;
        }

        let sprite_name = veh.kind.sprite_name(veh.orientation);
        if let Some(tex) = textures.get(sprite_name) {
            draw_texture_ex(
                tex,
                px,
                py,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(pw, ph)),
                    ..Default::default()
                },
            );
        } else {
            let col = if veh.is_player {
                RED
            } else {
                THEME.accent_blue
            };
            draw_rectangle(px + 2.0, py + 2.0, pw - 4.0, ph - 4.0, col);
            draw_rectangle_lines(px + 2.0, py + 2.0, pw - 4.0, ph - 4.0, 2.0, WHITE);
        }

        if is_being_dragged {
            draw_rectangle_lines(
                px + 1.0,
                py + 1.0,
                pw - 2.0,
                ph - 2.0,
                2.0,
                Color::new(1.0, 0.9, 0.3, 0.8),
            );
        }
    }
}
