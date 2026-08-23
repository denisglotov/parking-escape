use super::background::render_nature_background;
use super::water_fx::{compute_vessel_buoyancy, WaterRippleManager};
use super::{BoardLayout, TextureStore, THEME};
use crate::game::board::{Board, ExitSide};
use crate::game::Theme;
use macroquad::prelude::*;

pub fn render_board(
    board: &Board,
    layout: &BoardLayout,
    textures: &TextureStore,
    water_ripples: &WaterRippleManager,
) {
    let ox = layout.origin_x;
    let oy = layout.origin_y;
    let cs = layout.cell_size;
    let bw = layout.total_width;
    let bh = layout.total_height;

    // 1. Draw Nature or Marine Archipelago Background
    render_nature_background(board, layout, textures);

    // 2. Draw outer drop shadow for the board
    let shadow_col = match board.theme {
        Theme::Marine => Color::new(0.01, 0.12, 0.22, 0.45),
        Theme::City => Color::new(0.0, 0.0, 0.0, 0.45),
    };
    draw_rectangle(ox - 6.0, oy - 4.0, bw + 12.0, bh + 14.0, shadow_col);

    // 3. Draw Tiled Ground (Marine Water or Asphalt)
    match board.theme {
        Theme::Marine => {
            if let Some(water_tex) = textures.get(board.theme.ground_texture_key()) {
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
                draw_rectangle(ox, oy, bw, bh, Color::new(0.15, 0.46, 0.70, 1.0));
            }

            // Draw Subtle Lighter Marine Water Grid Lines
            for gx in 0..=board.width {
                let x = ox + gx as f32 * cs;
                draw_line(x, oy, x, oy + bh, 1.0, Color::new(0.50, 0.75, 1.0, 0.22));
            }
            for gy in 0..=board.height {
                let y = oy + gy as f32 * cs;
                draw_line(ox, y, ox + bw, y, 1.0, Color::new(0.50, 0.75, 1.0, 0.22));
            }

            // Mooring dots at intersections
            for gx in 1..board.width {
                for gy in 1..board.height {
                    let mx = ox + gx as f32 * cs;
                    let my = oy + gy as f32 * cs;
                    draw_circle(mx, my, 2.0, Color::new(0.65, 0.85, 1.0, 0.35));
                }
            }

            // Interactive Ripples
            water_ripples.render();
        }
        Theme::City => {
            if let Some(ground_tex) = textures.get(board.theme.ground_texture_key()) {
                draw_texture_ex(
                    ground_tex,
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
    }

    // 4. Draw Concrete Curbs or Wooden Pier Docks Border Perimeter
    let curb_thick = (cs * 0.14).max(6.0);
    let (curb_top, curb_bot, curb_side) = match board.theme {
        Theme::Marine => (
            Color::new(0.48, 0.34, 0.22, 1.0),
            Color::new(0.36, 0.24, 0.15, 1.0),
            Color::new(0.42, 0.28, 0.18, 1.0),
        ),
        Theme::City => (
            Color::new(0.32, 0.36, 0.42, 1.0),
            Color::new(0.24, 0.28, 0.34, 1.0),
            Color::new(0.28, 0.32, 0.38, 1.0),
        ),
    };

    draw_rectangle(
        ox - curb_thick,
        oy - curb_thick,
        bw + curb_thick * 2.0,
        curb_thick,
        curb_top,
    );
    draw_rectangle(
        ox - curb_thick,
        oy + bh,
        bw + curb_thick * 2.0,
        curb_thick,
        curb_bot,
    );
    draw_rectangle(ox - curb_thick, oy, curb_thick, bh, curb_side);
    draw_rectangle(ox + bw, oy, curb_thick, bh, curb_side);

    // Mooring bollards along piers for Marine theme
    if board.theme == Theme::Marine {
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
    }

    // 5. Draw Exit Gate (Harbor Channel Beacons or Parking Barrier Gate)
    render_exit_gate(board, layout, textures);

    // 6. Draw Static Obstacles (Buoys, Rocks, Pillars, Barriers)
    render_obstacles(board, layout, textures, water_ripples);

    // 7. Draw Vessels / Vehicles
    render_vehicles(board, layout, textures);
}

fn render_obstacles(
    board: &Board,
    layout: &BoardLayout,
    textures: &TextureStore,
    _water_ripples: &WaterRippleManager,
) {
    let ox = layout.origin_x;
    let oy = layout.origin_y;
    let cs = layout.cell_size;
    let time = get_time() as f32;

    for obs in &board.obstacles {
        let (px, mut py, pw, ph) = obs.pixel_bounds(ox, oy, cs);
        let cx = px + pw * 0.5;
        let mut cy = py + ph * 0.5;

        let base_sprite = obs.sprite_name(board.theme);

        // 1. Theme-specific dynamic wave & shadow effects
        match board.theme {
            Theme::Marine => {
                let is_buoy = base_sprite.contains("buoy");
                if is_buoy {
                    // Subtle marine wave heave & bobbing
                    let bob_y =
                        (time * 2.6 + obs.x as f32 * 1.5 + obs.y as f32 * 2.3).sin() * (cs * 0.04);
                    py += bob_y;
                    cy += bob_y;

                    // Underwater Drop Shadow / Moor Ring
                    let shadow_col = Color::new(0.01, 0.08, 0.16, 0.45);
                    draw_circle(
                        cx + 2.0,
                        cy + (pw * 0.36) * 0.35,
                        (pw * 0.36) * 0.95,
                        shadow_col,
                    );

                    // Water Foam / Ripple when wobbling
                    if obs.wobble_timer > 0.15 {
                        let foam_pulse = (time * 8.0).sin() * 2.0;
                        draw_circle_lines(
                            cx,
                            cy,
                            (pw * 0.42) + foam_pulse,
                            2.0,
                            Color::new(0.8, 0.95, 1.0, 0.5),
                        );
                    }
                } else {
                    // Marine rock underwater shadow
                    let shadow_col = Color::new(0.01, 0.08, 0.18, 0.45);
                    draw_ellipse(cx + 2.0, cy + 3.0, pw * 0.42, ph * 0.38, 0.0, shadow_col);

                    // Shoreline Foam Wash
                    let wave_pulse = (time * 3.2 + obs.x as f32 * 2.1).sin() * 1.5;
                    draw_ellipse_lines(
                        cx,
                        cy,
                        pw * 0.46 + wave_pulse,
                        ph * 0.42 + wave_pulse,
                        0.0,
                        2.0,
                        Color::new(0.75, 0.92, 1.0, 0.55),
                    );
                }
            }
            Theme::City => {
                // Cast asphalt drop shadow
                let shadow_col = Color::new(0.0, 0.0, 0.0, 0.40);
                if base_sprite == "city_barrier" {
                    draw_rectangle(px + 4.0, py + 4.0, pw - 8.0, ph - 8.0, shadow_col);
                } else if base_sprite == "city_pillar" {
                    draw_circle(cx + 2.0, cy + 3.0, pw * 0.42, shadow_col);
                } else {
                    draw_ellipse(cx + 2.0, cy + 3.0, pw * 0.42, ph * 0.38, 0.0, shadow_col);
                }
            }
        }

        // 2. Resolve final sprite key (handling buoy channel color variation)
        let is_channel_green = (obs.x + obs.y) % 2 == 1;
        let sprite_key = if base_sprite == "marine_buoy" && is_channel_green {
            "marine_buoy_green"
        } else {
            base_sprite
        };

        // 3. Draw Sprite Texture with fallback
        if let Some(tex) = textures.get(sprite_key) {
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
            let col = match board.theme {
                Theme::Marine => {
                    if is_channel_green {
                        Color::new(0.08, 0.68, 0.38, 1.0)
                    } else {
                        Color::new(0.92, 0.28, 0.22, 1.0)
                    }
                }
                Theme::City => Color::new(0.35, 0.38, 0.45, 1.0),
            };
            draw_ellipse(cx, cy, pw * 0.40, ph * 0.38, 0.0, col);
        }

        // 4. Flashing Hazard Beacon Glow Overlay on marine buoys
        if board.theme == Theme::Marine && base_sprite.contains("buoy") {
            let beacon_y = cy - pw * 0.22;
            let blink_phase = (time * 4.5 + (obs.x * 3 + obs.y * 7) as f32).sin();
            if blink_phase > -0.2 {
                let glow_alpha = ((blink_phase + 0.2) / 1.2).clamp(0.0, 1.0);
                let glow_col = if is_channel_green {
                    Color::new(0.2, 1.0, 0.5, 0.55 * glow_alpha)
                } else {
                    Color::new(1.0, 0.85, 0.25, 0.55 * glow_alpha)
                };
                draw_circle(cx, beacon_y, pw * 0.18, glow_col);
                draw_circle(cx, beacon_y, pw * 0.08, WHITE);
            }
        }
    }
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

    if board.theme == Theme::City {
        draw_rectangle(
            gx,
            gy,
            gw,
            gh,
            Color::new(0.02, 0.25, 0.15, 0.85 * glow_pulse),
        );
    }

    let gate_key = board.theme.exit_gate_texture_key();
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
        match board.theme {
            Theme::Marine => {
                draw_rectangle(gx, gy, gw, gh, Color::new(0.0, 0.6, 0.8, 0.8 * glow_pulse));
            }
            Theme::City => {
                draw_rectangle_lines(gx, gy, gw, gh, 2.0, THEME.accent_green);
            }
        }
    }

    if board.theme == Theme::Marine {
        // Harbor channel beacon glow aura
        let beacon_glow = Color::new(0.1, 0.9, 0.7, 0.35 * glow_pulse);
        draw_circle(gx + gw * 0.5, gy + gh * 0.5, cs * 0.38, beacon_glow);
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

        // Apply subtle squash & stretch on impact contact centered on vehicle (City theme only)
        if board.theme == Theme::City {
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
        }

        if is_being_dragged {
            py -= 2.0;
        }

        // Apply marine buoyancy idle heave and roll, plus any impact drift rocking
        let (heave_x, heave_y, roll) = if board.theme == Theme::Marine {
            let (hx, hy, base_roll) = compute_vessel_buoyancy(idx, px, py, cs, is_being_dragged);
            let drift_roll = veh.drift_state.as_ref().map_or(0.0, |d| d.roll());
            (hx, hy, base_roll + drift_roll)
        } else {
            (0.0, 0.0, 0.0)
        };
        px += heave_x;
        py += heave_y;

        // 1. Draw Under-Vehicle Effects (Ground Reflection)
        if let Some(bump) = &veh.bump_state {
            render_ground_effects(board.theme, veh, bump, Rect::new(px, py, pw, ph), cs);
        }

        // 2. Draw Vehicle / Ship Body
        let sprite_name = veh.kind.sprite_for_theme(veh.orientation, board.theme);
        if let Some(tex) = textures.get(sprite_name) {
            draw_texture_ex(
                tex,
                px,
                py,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(pw, ph)),
                    rotation: roll,
                    pivot: Some(vec2(px + pw * 0.5, py + ph * 0.5)),
                    ..Default::default()
                },
            );
        } else {
            let col = if veh.is_player {
                RED
            } else {
                match board.theme {
                    Theme::Marine => Color::new(0.1, 0.65, 0.85, 1.0),
                    Theme::City => THEME.accent_blue,
                }
            };
            draw_rectangle(px + 2.0, py + 2.0, pw - 4.0, ph - 4.0, col);
            draw_rectangle_lines(px + 2.0, py + 2.0, pw - 4.0, ph - 4.0, 2.0, WHITE);
        }

        // 3. Draw Drag Selection Highlight
        if is_being_dragged {
            let select_col = match board.theme {
                Theme::Marine => Color::new(0.3, 0.95, 1.0, 0.85),
                Theme::City => Color::new(1.0, 0.9, 0.3, 0.8),
            };
            draw_rectangle_lines(px + 1.0, py + 1.0, pw - 2.0, ph - 2.0, 2.0, select_col);
        }

        // 4. Draw Over-Vehicle Effects (Rooftop Strobes, Navigation Lights, Contact Sparks)
        if let Some(bump) = &veh.bump_state {
            render_vehicle_effects(board.theme, veh, bump, Rect::new(px, py, pw, ph), cs);
        }
    }
}

/// Renders under-vehicle ground reflections beneath the vehicle.
fn render_ground_effects(
    theme: Theme,
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
        let reflection_col = match theme {
            Theme::Marine => {
                if phase < 0.5 {
                    Color::new(0.05, 0.75, 1.0, 0.20 * micro_pulse * bump.intensity)
                } else {
                    Color::new(1.0, 0.25, 0.15, 0.20 * micro_pulse * bump.intensity)
                }
            }
            Theme::City => {
                if phase < 0.5 {
                    Color::new(1.0, 0.1, 0.15, 0.18 * micro_pulse * bump.intensity)
                } else {
                    Color::new(0.1, 0.4, 1.0, 0.18 * micro_pulse * bump.intensity)
                }
            }
        };
        draw_circle(center_x, center_y, cs * 1.5, reflection_col);
    }
}

fn render_vehicle_effects(
    theme: Theme,
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

    // 2. Headlights for City vehicles on bump / hazard
    if bump.is_hazard_on() && theme == Theme::City {
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

        match theme {
            Theme::Marine => {
                // Water splash concentric ripple rings & foam spray
                let splash_ring = Color::new(0.85, 0.95, 1.0, s_alpha * 0.8);
                let splash_core = Color::new(0.4, 0.85, 1.0, s_alpha * 0.5);
                draw_circle_lines(cx, cy, s_rad, 2.5, splash_ring);
                draw_circle_lines(cx, cy, s_rad * 0.55, 1.8, splash_core);
                draw_circle(cx, cy, s_rad * 0.25, splash_ring);
            }
            Theme::City => {
                let spark_glow = Color::new(1.0, 0.92, 0.4, s_alpha * 0.6);
                let spark_core = Color::new(1.0, 1.0, 0.9, s_alpha);

                draw_circle(cx, cy, s_rad * 0.7, spark_glow);
                draw_circle_lines(cx, cy, s_rad, 2.5, spark_core);
                draw_line(cx - s_rad * 1.3, cy, cx + s_rad * 1.3, cy, 2.0, spark_core);
                draw_line(cx, cy - s_rad * 1.3, cx, cy + s_rad * 1.3, 2.0, spark_core);
            }
        }
    }
}
