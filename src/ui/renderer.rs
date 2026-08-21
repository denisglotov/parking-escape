use super::background::render_nature_background;
use super::{BoardLayout, TextureStore, THEME};
use crate::game::board::{Board, ExitSide};
use macroquad::prelude::*;

pub fn render_board(board: &Board, layout: &BoardLayout, textures: &TextureStore) {
    let ox = layout.origin_x;
    let oy = layout.origin_y;
    let cs = layout.cell_size;
    let bw = layout.total_width;
    let bh = layout.total_height;

    // 1. Draw Nature Background (Park lawn, footpaths, pond with animated ripples, flora)
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

        let (mut px, mut py, mut pw, mut ph) = veh.pixel_bounds(ox, oy, cs);

        if veh.is_player && board.is_won {
            let exit_offset = board.exit_animation_progress * cs * 3.5;
            match board.exit.side {
                ExitSide::Right => px += exit_offset,
                ExitSide::Left => px -= exit_offset,
                ExitSide::Bottom => py += exit_offset,
                ExitSide::Top => py -= exit_offset,
            }
        }

        // Apply squash & stretch on impact contact
        if let Some(bump) = &veh.bump_state {
            let (scale_len, scale_wid) = bump.squash_factors();
            let (scalex, scaley) = match veh.orientation {
                crate::game::vehicle::Orientation::Horizontal => (scale_len, scale_wid),
                crate::game::vehicle::Orientation::Vertical => (scale_wid, scale_len),
            };
            let orig_pw = pw;
            let orig_ph = ph;
            pw *= scalex;
            ph *= scaley;
            match veh.orientation {
                crate::game::vehicle::Orientation::Horizontal => {
                    py += (orig_ph - ph) * 0.5;
                    if bump.impact_direction > 0.0 {
                        px += orig_pw - pw;
                    }
                }
                crate::game::vehicle::Orientation::Vertical => {
                    px += (orig_pw - pw) * 0.5;
                    if bump.impact_direction > 0.0 {
                        py += orig_ph - ph;
                    }
                }
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

        // Render dynamic lighting, hazard flashing & emergency strobes
        if let Some(bump) = &veh.bump_state {
            render_vehicle_effects(veh, bump, px, py, pw, ph, cs);
        }
    }
}

fn render_vehicle_effects(
    veh: &crate::game::vehicle::Vehicle,
    bump: &crate::game::vehicle::BumpState,
    px: f32,
    py: f32,
    pw: f32,
    ph: f32,
    cs: f32,
) {
    let orient = veh.orientation;
    let is_emergency = veh.kind.is_emergency();

    // 1. Ground pavement lighting reflection for emergency vehicles
    if is_emergency {
        let phase = bump.emergency_strobe_phase();
        let micro_pulse = ((phase * 12.0) % 1.0 * std::f32::consts::PI).sin().max(0.0);
        let center_x = px + pw * 0.5;
        let center_y = py + ph * 0.5;
        let ground_col = if phase < 0.5 {
            Color::new(1.0, 0.1, 0.15, 0.18 * micro_pulse * bump.intensity)
        } else {
            Color::new(0.1, 0.4, 1.0, 0.18 * micro_pulse * bump.intensity)
        };
        draw_circle(center_x, center_y, cs * 1.6, ground_col);
    }

    // 2. Headlights & taillights hazard flashing
    if bump.is_hazard_on() {
        let head_halo_col = Color::new(1.0, 0.96, 0.65, 0.55 * bump.intensity);
        let head_core_col = Color::new(1.0, 1.0, 0.9, 0.95);
        let tail_halo_col = Color::new(1.0, 0.25, 0.1, 0.60 * bump.intensity);
        let tail_core_col = Color::new(1.0, 0.45, 0.2, 0.95);
        let beam_col = Color::new(1.0, 0.95, 0.6, 0.22 * bump.intensity);

        match orient {
            crate::game::vehicle::Orientation::Horizontal => {
                // Front headlights (Right edge)
                let fx = px + pw - cs * 0.08;
                let h1_y = py + ph * 0.18;
                let h2_y = py + ph * 0.82;

                // Light beams projecting forward to the right
                let beam_len = cs * 0.85;
                draw_triangle(
                    vec2(fx, h1_y),
                    vec2(fx + beam_len, h1_y - cs * 0.28),
                    vec2(fx + beam_len, h1_y + cs * 0.28),
                    beam_col,
                );
                draw_triangle(
                    vec2(fx, h2_y),
                    vec2(fx + beam_len, h2_y - cs * 0.28),
                    vec2(fx + beam_len, h2_y + cs * 0.28),
                    beam_col,
                );

                // Lamp halos & bright cores
                draw_circle(fx, h1_y, cs * 0.15, head_halo_col);
                draw_circle(fx, h1_y, cs * 0.07, head_core_col);
                draw_circle(fx, h2_y, cs * 0.15, head_halo_col);
                draw_circle(fx, h2_y, cs * 0.07, head_core_col);

                // Rear taillights (Left edge)
                let rx = px + cs * 0.08;
                let t1_y = py + ph * 0.16;
                let t2_y = py + ph * 0.84;
                draw_circle(rx, t1_y, cs * 0.13, tail_halo_col);
                draw_circle(rx, t1_y, cs * 0.06, tail_core_col);
                draw_circle(rx, t2_y, cs * 0.13, tail_halo_col);
                draw_circle(rx, t2_y, cs * 0.06, tail_core_col);
            }
            crate::game::vehicle::Orientation::Vertical => {
                // Front headlights (Bottom edge)
                let fy = py + ph - cs * 0.08;
                let h1_x = px + pw * 0.18;
                let h2_x = px + pw * 0.82;

                // Light beams projecting downwards
                let beam_len = cs * 0.85;
                draw_triangle(
                    vec2(h1_x, fy),
                    vec2(h1_x - cs * 0.28, fy + beam_len),
                    vec2(h1_x + cs * 0.28, fy + beam_len),
                    beam_col,
                );
                draw_triangle(
                    vec2(h2_x, fy),
                    vec2(h2_x - cs * 0.28, fy + beam_len),
                    vec2(h2_x + cs * 0.28, fy + beam_len),
                    beam_col,
                );

                // Lamp halos & cores
                draw_circle(h1_x, fy, cs * 0.15, head_halo_col);
                draw_circle(h1_x, fy, cs * 0.07, head_core_col);
                draw_circle(h2_x, fy, cs * 0.15, head_halo_col);
                draw_circle(h2_x, fy, cs * 0.07, head_core_col);

                // Rear taillights (Top edge)
                let ry = py + cs * 0.08;
                let t1_x = px + pw * 0.16;
                let t2_x = px + pw * 0.84;
                draw_circle(t1_x, ry, cs * 0.13, tail_halo_col);
                draw_circle(t1_x, ry, cs * 0.06, tail_core_col);
                draw_circle(t2_x, ry, cs * 0.13, tail_halo_col);
                draw_circle(t2_x, ry, cs * 0.06, tail_core_col);
            }
        }
    }

    // 3. Emergency rooftop strobe beacons (Police & Ambulance)
    if is_emergency {
        let phase = bump.emergency_strobe_phase();
        let pulse = ((phase * 12.0) % 1.0 * std::f32::consts::PI).sin().max(0.0);

        let red_strobe = Color::new(1.0, 0.12, 0.15, 0.95 * pulse);
        let blue_strobe = Color::new(0.12, 0.55, 1.0, 0.95 * pulse);
        let white_strobe = Color::new(1.0, 1.0, 1.0, 0.90 * pulse);
        let beacon_sz = cs * 0.18;

        match veh.kind {
            crate::game::vehicle::VehicleKind::CarPolice => {
                let (cx, cy) = (px + pw * 0.5, py + ph * 0.5);
                match orient {
                    crate::game::vehicle::Orientation::Horizontal => {
                        let top_y = cy - ph * 0.26;
                        let bot_y = cy + ph * 0.26;
                        if phase < 0.5 {
                            draw_circle(
                                cx,
                                top_y,
                                beacon_sz * 1.5,
                                Color::new(1.0, 0.1, 0.15, 0.45),
                            );
                            draw_circle(cx, top_y, beacon_sz * 0.8, red_strobe);
                            draw_circle(cx, cy, beacon_sz * 0.5, white_strobe);
                        } else {
                            draw_circle(
                                cx,
                                bot_y,
                                beacon_sz * 1.5,
                                Color::new(0.1, 0.5, 1.0, 0.45),
                            );
                            draw_circle(cx, bot_y, beacon_sz * 0.8, blue_strobe);
                            draw_circle(cx, cy, beacon_sz * 0.5, white_strobe);
                        }
                    }
                    crate::game::vehicle::Orientation::Vertical => {
                        let left_x = cx - pw * 0.26;
                        let right_x = cx + pw * 0.26;
                        if phase < 0.5 {
                            draw_circle(
                                left_x,
                                cy,
                                beacon_sz * 1.5,
                                Color::new(0.1, 0.5, 1.0, 0.45),
                            );
                            draw_circle(left_x, cy, beacon_sz * 0.8, blue_strobe);
                            draw_circle(cx, cy, beacon_sz * 0.5, white_strobe);
                        } else {
                            draw_circle(
                                right_x,
                                cy,
                                beacon_sz * 1.5,
                                Color::new(1.0, 0.1, 0.15, 0.45),
                            );
                            draw_circle(right_x, cy, beacon_sz * 0.8, red_strobe);
                            draw_circle(cx, cy, beacon_sz * 0.5, white_strobe);
                        }
                    }
                }
            }
            crate::game::vehicle::VehicleKind::Ambulance => match orient {
                crate::game::vehicle::Orientation::Horizontal => {
                    let bar_x = px + pw * 0.74;
                    let top_y = py + ph * 0.28;
                    let bot_y = py + ph * 0.72;
                    if phase < 0.5 {
                        draw_circle(
                            bar_x,
                            top_y,
                            beacon_sz * 1.4,
                            Color::new(1.0, 0.1, 0.15, 0.45),
                        );
                        draw_circle(bar_x, top_y, beacon_sz * 0.8, red_strobe);
                    } else {
                        draw_circle(
                            bar_x,
                            bot_y,
                            beacon_sz * 1.4,
                            Color::new(0.1, 0.5, 1.0, 0.45),
                        );
                        draw_circle(bar_x, bot_y, beacon_sz * 0.8, blue_strobe);
                    }
                    let rear_x = px + cs * 0.15;
                    draw_circle(
                        rear_x,
                        top_y,
                        beacon_sz * 0.6,
                        if phase < 0.5 { red_strobe } else { blue_strobe },
                    );
                    draw_circle(
                        rear_x,
                        bot_y,
                        beacon_sz * 0.6,
                        if phase >= 0.5 {
                            red_strobe
                        } else {
                            blue_strobe
                        },
                    );
                }
                crate::game::vehicle::Orientation::Vertical => {
                    let bar_y = py + ph * 0.74;
                    let left_x = px + pw * 0.28;
                    let right_x = px + pw * 0.72;
                    if phase < 0.5 {
                        draw_circle(
                            left_x,
                            bar_y,
                            beacon_sz * 1.4,
                            Color::new(0.1, 0.5, 1.0, 0.45),
                        );
                        draw_circle(left_x, bar_y, beacon_sz * 0.8, blue_strobe);
                    } else {
                        draw_circle(
                            right_x,
                            bar_y,
                            beacon_sz * 1.4,
                            Color::new(1.0, 0.1, 0.15, 0.45),
                        );
                        draw_circle(right_x, bar_y, beacon_sz * 0.8, red_strobe);
                    }
                    let rear_y = py + cs * 0.15;
                    draw_circle(
                        left_x,
                        rear_y,
                        beacon_sz * 0.6,
                        if phase < 0.5 { blue_strobe } else { red_strobe },
                    );
                    draw_circle(
                        right_x,
                        rear_y,
                        beacon_sz * 0.6,
                        if phase >= 0.5 {
                            blue_strobe
                        } else {
                            red_strobe
                        },
                    );
                }
            },
            _ => {}
        }
    }

    // 4. Contact spark starburst on obstacle collision
    const SPARK_DURATION: f32 = 0.18;
    if bump.timer < SPARK_DURATION {
        let st = bump.timer / SPARK_DURATION;
        let s_alpha = (1.0 - st) * bump.intensity;
        let s_rad = cs * (0.12 + st * 0.35);

        let (cx, cy) = match orient {
            crate::game::vehicle::Orientation::Horizontal => {
                let x = if bump.impact_direction > 0.0 {
                    px + pw
                } else {
                    px
                };
                (x, py + ph * 0.5)
            }
            crate::game::vehicle::Orientation::Vertical => {
                let y = if bump.impact_direction > 0.0 {
                    py + ph
                } else {
                    py
                };
                (px + pw * 0.5, y)
            }
        };

        let spark_glow = Color::new(1.0, 0.92, 0.4, s_alpha * 0.6);
        let spark_core = Color::new(1.0, 1.0, 0.9, s_alpha);

        draw_circle(cx, cy, s_rad * 0.7, spark_glow);
        draw_circle_lines(cx, cy, s_rad, 2.5, spark_core);
        draw_line(cx - s_rad * 1.3, cy, cx + s_rad * 1.3, cy, 2.0, spark_core);
        draw_line(cx, cy - s_rad * 1.3, cx, cy + s_rad * 1.3, 2.0, spark_core);
    }
}
