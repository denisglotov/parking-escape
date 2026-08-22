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

    // 1. Draw Nature or Marine Archipelago Background
    render_nature_background(board, layout, textures);

    // 2. Draw outer drop shadow for the board
    let shadow_col = if board.is_marine {
        Color::new(0.01, 0.12, 0.22, 0.45)
    } else {
        Color::new(0.0, 0.0, 0.0, 0.45)
    };
    draw_rectangle(ox - 6.0, oy - 4.0, bw + 12.0, bh + 14.0, shadow_col);

    // 3. Draw Tiled Ground (Marine Water or Asphalt)
    if board.is_marine {
        if let Some(water_tex) = textures.get("marine_water") {
            draw_texture_ex(
                water_tex,
                ox,
                oy,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(bw, bh)),
                    ..Default::default()
                },
            );
        } else {
            draw_rectangle(ox, oy, bw, bh, Color::new(0.08, 0.26, 0.42, 1.0));
        }

        // Draw Subtle Deep Marina Water Grid Lines
        for gx in 0..=board.width {
            let x = ox + gx as f32 * cs;
            draw_line(x, oy, x, oy + bh, 1.0, Color::new(0.35, 0.60, 0.85, 0.18));
        }
        for gy in 0..=board.height {
            let y = oy + gy as f32 * cs;
            draw_line(ox, y, ox + bw, y, 1.0, Color::new(0.35, 0.60, 0.85, 0.18));
        }

        // Mooring dots at intersections
        for gx in 1..board.width {
            for gy in 1..board.height {
                let mx = ox + gx as f32 * cs;
                let my = oy + gy as f32 * cs;
                draw_circle(mx, my, 2.0, Color::new(0.45, 0.70, 0.90, 0.30));
            }
        }
    } else {
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

        // Draw Stall Grid Lines & Markers
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
    }

    // 4. Draw Concrete Curbs or Wooden Pier Docks Border Perimeter
    let curb_thick = (cs * 0.14).max(6.0);
    if board.is_marine {
        let wood_top = Color::new(0.48, 0.34, 0.22, 1.0);
        let wood_bot = Color::new(0.36, 0.24, 0.15, 1.0);
        let wood_side = Color::new(0.42, 0.28, 0.18, 1.0);

        draw_rectangle(
            ox - curb_thick,
            oy - curb_thick,
            bw + curb_thick * 2.0,
            curb_thick,
            wood_top,
        );
        draw_rectangle(
            ox - curb_thick,
            oy + bh,
            bw + curb_thick * 2.0,
            curb_thick,
            wood_bot,
        );
        draw_rectangle(ox - curb_thick, oy, curb_thick, bh, wood_side);
        draw_rectangle(ox + bw, oy, curb_thick, bh, wood_side);

        // Mooring bollards along piers
        let bollard_rad = curb_thick * 0.32;
        let bollard_col = Color::new(0.20, 0.15, 0.12, 1.0);
        for i in 0..=board.width {
            let bx = ox + i as f32 * cs;
            draw_circle(bx, oy - curb_thick * 0.5, bollard_rad, bollard_col);
            draw_circle(bx, oy + bh + curb_thick * 0.5, bollard_rad, bollard_col);
        }
        for j in 0..=board.height {
            let by = oy + j as f32 * cs;
            draw_circle(ox - curb_thick * 0.5, by, bollard_rad, bollard_col);
            draw_circle(ox + bw + curb_thick * 0.5, by, bollard_rad, bollard_col);
        }
    } else {
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
    }

    // 5. Draw Exit Gate (Harbor Channel Beacons or Parking Barrier Gate)
    render_exit_gate(board, layout, textures);

    // 6. Draw Vessels / Vehicles
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

    if board.is_marine {
        let gate_key = "marine_exit_gate";
        if let Some(gate_tex) = textures.get(gate_key) {
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
            draw_rectangle(gx, gy, gw, gh, Color::new(0.0, 0.6, 0.8, 0.8 * glow_pulse));
        }

        // Harbor channel beacon glow aura
        let beacon_glow = Color::new(0.1, 0.9, 0.7, 0.35 * glow_pulse);
        draw_circle(gx + gw * 0.5, gy + gh * 0.5, cs * 0.38, beacon_glow);
    } else {
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
        let is_coasting = board
            .active_coast
            .as_ref()
            .is_some_and(|c| c.vehicle_index == idx);
        let is_moving = is_being_dragged || is_coasting || veh.drag_offset.abs() > 0.01;

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

        // Apply subtle squash & stretch on impact contact centered on vehicle
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
            px += (orig_pw - pw) * 0.5;
            py += (orig_ph - ph) * 0.5;
        }

        if is_being_dragged {
            py -= 2.0;
        }

        // 1. Draw Under-Vehicle Effects (Hydrodynamic Wakes & Ground Reflection)
        if board.is_marine && is_moving {
            render_ship_wake(veh, Rect::new(px, py, pw, ph), cs);
        }
        if let Some(bump) = &veh.bump_state {
            render_ground_effects(board.is_marine, veh, bump, Rect::new(px, py, pw, ph), cs);
        }

        // 2. Draw Vehicle / Ship Body
        let sprite_name = veh.kind.sprite_for_theme(veh.orientation, board.is_marine);
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
            } else if board.is_marine {
                Color::new(0.1, 0.65, 0.85, 1.0)
            } else {
                THEME.accent_blue
            };
            draw_rectangle(px + 2.0, py + 2.0, pw - 4.0, ph - 4.0, col);
            draw_rectangle_lines(px + 2.0, py + 2.0, pw - 4.0, ph - 4.0, 2.0, WHITE);
        }

        // 3. Draw Drag Selection Highlight
        if is_being_dragged {
            let select_col = if board.is_marine {
                Color::new(0.3, 0.95, 1.0, 0.85)
            } else {
                Color::new(1.0, 0.9, 0.3, 0.8)
            };
            draw_rectangle_lines(px + 1.0, py + 1.0, pw - 2.0, ph - 2.0, 2.0, select_col);
        }

        // 4. Draw Over-Vehicle Effects (Rooftop Strobes, Navigation Lights, Contact Sparks)
        if let Some(bump) = &veh.bump_state {
            render_vehicle_effects(board.is_marine, veh, bump, Rect::new(px, py, pw, ph), cs);
        }
    }
}

/// Renders under-vehicle ground reflections beneath the vehicle.
fn render_ground_effects(
    is_marine: bool,
    veh: &crate::game::vehicle::Vehicle,
    bump: &crate::game::vehicle::BumpState,
    bounds: Rect,
    cs: f32,
) {
    if veh.kind.is_emergency() {
        let phase = bump.emergency_strobe_phase();
        let micro_pulse = ((phase * 12.0) % 1.0 * std::f32::consts::PI).sin().max(0.0);
        let center_x = bounds.x + bounds.w * 0.5;
        let center_y = bounds.y + bounds.h * 0.5;
        let reflection_col = if is_marine {
            if phase < 0.5 {
                Color::new(0.05, 0.75, 1.0, 0.20 * micro_pulse * bump.intensity)
            } else {
                Color::new(1.0, 0.25, 0.15, 0.20 * micro_pulse * bump.intensity)
            }
        } else if phase < 0.5 {
            Color::new(1.0, 0.1, 0.15, 0.18 * micro_pulse * bump.intensity)
        } else {
            Color::new(0.1, 0.4, 1.0, 0.18 * micro_pulse * bump.intensity)
        };
        draw_circle(center_x, center_y, cs * 1.5, reflection_col);
    }
}

/// Renders dynamic trailing hydrodynamic wakes and white foam ripples behind moving ships.
fn render_ship_wake(veh: &crate::game::vehicle::Vehicle, bounds: Rect, cs: f32) {
    let px = bounds.x;
    let py = bounds.y;
    let pw = bounds.w;
    let ph = bounds.h;

    let time = get_time() as f32;
    let wave_pulse = (time * 8.0).sin() * 0.15 + 0.85;
    let foam_col = Color::new(0.88, 0.96, 1.0, 0.55 * wave_pulse);
    let wake_wave_col = Color::new(0.25, 0.75, 0.95, 0.35 * wave_pulse);

    let orient = veh.orientation;
    let offset = veh.drag_offset;

    match orient {
        crate::game::vehicle::Orientation::Horizontal => {
            let is_moving_right = offset >= 0.0;
            let (stern_x, stern_y1, stern_y2, wake_dir) = if is_moving_right {
                (px + cs * 0.05, py + ph * 0.25, py + ph * 0.75, -1.0)
            } else {
                (px + pw - cs * 0.05, py + ph * 0.25, py + ph * 0.75, 1.0)
            };

            let wake_len = cs * 0.55;
            // V-shaped wake lines
            draw_line(
                stern_x,
                stern_y1,
                stern_x + wake_dir * wake_len,
                stern_y1 - cs * 0.22,
                2.5,
                wake_wave_col,
            );
            draw_line(
                stern_x,
                stern_y2,
                stern_x + wake_dir * wake_len,
                stern_y2 + cs * 0.22,
                2.5,
                wake_wave_col,
            );
            // Foam bubbles at stern
            draw_circle(
                stern_x + wake_dir * cs * 0.15,
                py + ph * 0.5,
                cs * 0.14,
                foam_col,
            );
            draw_circle(
                stern_x + wake_dir * cs * 0.35,
                stern_y1,
                cs * 0.09,
                foam_col,
            );
            draw_circle(
                stern_x + wake_dir * cs * 0.35,
                stern_y2,
                cs * 0.09,
                foam_col,
            );
        }
        crate::game::vehicle::Orientation::Vertical => {
            let is_moving_down = offset >= 0.0;
            let (stern_y, stern_x1, stern_x2, wake_dir) = if is_moving_down {
                (py + cs * 0.05, px + pw * 0.25, px + pw * 0.75, -1.0)
            } else {
                (py + ph - cs * 0.05, px + pw * 0.25, px + pw * 0.75, 1.0)
            };

            let wake_len = cs * 0.55;
            // V-shaped wake lines
            draw_line(
                stern_x1,
                stern_y,
                stern_x1 - cs * 0.22,
                stern_y + wake_dir * wake_len,
                2.5,
                wake_wave_col,
            );
            draw_line(
                stern_x2,
                stern_y,
                stern_x2 + cs * 0.22,
                stern_y + wake_dir * wake_len,
                2.5,
                wake_wave_col,
            );
            // Foam bubbles at stern
            draw_circle(
                px + pw * 0.5,
                stern_y + wake_dir * cs * 0.15,
                cs * 0.14,
                foam_col,
            );
            draw_circle(
                stern_x1,
                stern_y + wake_dir * cs * 0.35,
                cs * 0.09,
                foam_col,
            );
            draw_circle(
                stern_x2,
                stern_y + wake_dir * cs * 0.35,
                cs * 0.09,
                foam_col,
            );
        }
    }
}

fn render_vehicle_effects(
    is_marine: bool,
    veh: &crate::game::vehicle::Vehicle,
    bump: &crate::game::vehicle::BumpState,
    bounds: Rect,
    cs: f32,
) {
    let px = bounds.x;
    let py = bounds.y;
    let pw = bounds.w;
    let ph = bounds.h;

    let orient = veh.orientation;
    let is_emergency = veh.kind.is_emergency();

    // 2. Headlights / Nautical Port & Starboard Navigation Lanterns
    if bump.is_hazard_on() {
        if is_marine {
            // Marine navigation lights: Port = Red (left), Starboard = Green (right)
            let port_col = Color::new(1.0, 0.15, 0.2, 0.85 * bump.intensity);
            let stbd_col = Color::new(0.1, 0.95, 0.35, 0.85 * bump.intensity);
            let glow_sz = cs * 0.16;

            match orient {
                crate::game::vehicle::Orientation::Horizontal => {
                    let bow_x = px + pw - cs * 0.12;
                    let port_y = py + ph * 0.20;
                    let stbd_y = py + ph * 0.80;
                    draw_circle(bow_x, port_y, glow_sz, port_col);
                    draw_circle(bow_x, stbd_y, glow_sz, stbd_col);
                }
                crate::game::vehicle::Orientation::Vertical => {
                    let bow_y = py + ph - cs * 0.12;
                    let port_x = px + pw * 0.80;
                    let stbd_x = px + pw * 0.20;
                    draw_circle(port_x, bow_y, glow_sz, port_col);
                    draw_circle(stbd_x, bow_y, glow_sz, stbd_col);
                }
            }
        } else {
            let head_halo_col = Color::new(1.0, 0.96, 0.65, 0.55 * bump.intensity);
            let head_core_col = Color::new(1.0, 1.0, 0.9, 0.95);
            let tail_halo_col = Color::new(1.0, 0.25, 0.1, 0.60 * bump.intensity);
            let tail_core_col = Color::new(1.0, 0.45, 0.2, 0.95);
            let beam_col = Color::new(1.0, 0.95, 0.6, 0.22 * bump.intensity);

            match orient {
                crate::game::vehicle::Orientation::Horizontal => {
                    let fx = px + pw - cs * 0.08;
                    let h1_y = py + ph * 0.18;
                    let h2_y = py + ph * 0.82;

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

                    draw_circle(fx, h1_y, cs * 0.15, head_halo_col);
                    draw_circle(fx, h1_y, cs * 0.07, head_core_col);
                    draw_circle(fx, h2_y, cs * 0.15, head_halo_col);
                    draw_circle(fx, h2_y, cs * 0.07, head_core_col);

                    let rx = px + cs * 0.08;
                    let t1_y = py + ph * 0.16;
                    let t2_y = py + ph * 0.84;
                    draw_circle(rx, t1_y, cs * 0.13, tail_halo_col);
                    draw_circle(rx, t1_y, cs * 0.06, tail_core_col);
                    draw_circle(rx, t2_y, cs * 0.13, tail_halo_col);
                    draw_circle(rx, t2_y, cs * 0.06, tail_core_col);
                }
                crate::game::vehicle::Orientation::Vertical => {
                    let fy = py + ph - cs * 0.08;
                    let h1_x = px + pw * 0.18;
                    let h2_x = px + pw * 0.82;

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

                    draw_circle(h1_x, fy, cs * 0.15, head_halo_col);
                    draw_circle(h1_x, fy, cs * 0.07, head_core_col);
                    draw_circle(h2_x, fy, cs * 0.15, head_halo_col);
                    draw_circle(h2_x, fy, cs * 0.07, head_core_col);

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
    }

    // 3. Emergency rooftop strobe beacons (Coast Guard / Patrol & SAR Ambulance)
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

    // 4. Contact spark or Water splash starburst on obstacle collision
    const SPARK_DURATION: f32 = 0.22;
    if bump.timer < SPARK_DURATION {
        let st = bump.timer / SPARK_DURATION;
        let s_alpha = (1.0 - st) * bump.intensity;
        let s_rad = cs * (0.15 + st * 0.45);

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

        if is_marine {
            // Water splash concentric ripple rings & foam spray
            let splash_ring = Color::new(0.85, 0.95, 1.0, s_alpha * 0.8);
            let splash_core = Color::new(0.4, 0.85, 1.0, s_alpha * 0.5);
            draw_circle_lines(cx, cy, s_rad, 2.5, splash_ring);
            draw_circle_lines(cx, cy, s_rad * 0.55, 1.8, splash_core);
            draw_circle(cx, cy, s_rad * 0.25, splash_ring);
        } else {
            let spark_glow = Color::new(1.0, 0.92, 0.4, s_alpha * 0.6);
            let spark_core = Color::new(1.0, 1.0, 0.9, s_alpha);

            draw_circle(cx, cy, s_rad * 0.7, spark_glow);
            draw_circle_lines(cx, cy, s_rad, 2.5, spark_core);
            draw_line(cx - s_rad * 1.3, cy, cx + s_rad * 1.3, cy, 2.0, spark_core);
            draw_line(cx, cy - s_rad * 1.3, cx, cy + s_rad * 1.3, 2.0, spark_core);
        }
    }
}
