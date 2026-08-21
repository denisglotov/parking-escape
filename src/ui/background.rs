use super::{BoardLayout, TextureStore};
use crate::game::board::{Board, ExitSide};
use macroquad::prelude::*;

// Color Palette Constants
const GRASS_BASE: Color = Color::new(0.12, 0.22, 0.14, 1.0);
const GRASS_STRIPE: Color = Color::new(0.14, 0.25, 0.16, 1.0);
const GRASS_TUFT: Color = Color::new(0.20, 0.35, 0.22, 0.7);

const PATH_STONE: Color = Color::new(0.40, 0.38, 0.34, 1.0);
const PATH_BORDER: Color = Color::new(0.28, 0.26, 0.23, 1.0);
const PATH_PEBBLE: Color = Color::new(0.50, 0.48, 0.44, 0.85);

const ROAD_ASPHALT: Color = Color::new(0.16, 0.18, 0.22, 1.0);
const ROAD_CURB: Color = Color::new(0.30, 0.34, 0.40, 1.0);
const ROAD_MARKING: Color = Color::new(0.95, 0.82, 0.20, 0.85);

const POND_SAND: Color = Color::new(0.48, 0.44, 0.35, 1.0);
const POND_SHALLOW: Color = Color::new(0.14, 0.46, 0.52, 1.0);
const POND_MID: Color = Color::new(0.10, 0.36, 0.44, 1.0);
const POND_DEEP: Color = Color::new(0.06, 0.24, 0.36, 1.0);

const LILY_PAD: Color = Color::new(0.16, 0.54, 0.24, 1.0);
const LILY_PAD_DARK: Color = Color::new(0.10, 0.36, 0.16, 1.0);
const LILY_FLOWER: Color = Color::new(0.98, 0.92, 0.96, 1.0);
const LILY_CENTER: Color = Color::new(0.98, 0.82, 0.15, 1.0);

const REED_STALK: Color = Color::new(0.25, 0.45, 0.22, 1.0);
const REED_HEAD: Color = Color::new(0.38, 0.24, 0.14, 1.0);

const TREE_SHADOW: Color = Color::new(0.04, 0.08, 0.05, 0.45);
const TREE_DARK: Color = Color::new(0.11, 0.28, 0.14, 1.0);
const TREE_MID: Color = Color::new(0.17, 0.44, 0.20, 1.0);
const TREE_LIGHT: Color = Color::new(0.25, 0.58, 0.26, 1.0);
const TREE_HIGHLIGHT: Color = Color::new(0.35, 0.70, 0.32, 1.0);
const TREE_TRUNK: Color = Color::new(0.32, 0.22, 0.14, 1.0);

const BUSH_SHADOW: Color = Color::new(0.05, 0.09, 0.06, 0.35);
const BUSH_DARK: Color = Color::new(0.13, 0.32, 0.16, 1.0);
const BUSH_LIGHT: Color = Color::new(0.22, 0.50, 0.24, 1.0);

/// Renders the entire park nature background using the AI-generated park texture (or procedural fallback),
/// plus exit roadway connection.
pub fn render_nature_background(board: &Board, layout: &BoardLayout, textures: &TextureStore) {
    let time = get_time() as f32;
    let sw = layout.screen_width;
    let sh = layout.screen_height;
    let start_y = layout.hud_height;
    let area_h = (sh - start_y).max(0.0);

    // 1. Draw AI generated park background texture
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
        // Procedural fallback
        render_park_lawn(layout);
        render_footpaths(layout, board.exit.side);
        render_pond(layout, board.exit.side, time);
        render_park_benches(layout, board.exit.side);
        render_flora(board, layout);
    }

    // 2. Draw asphalt exit road connecting exit gate to edge of screen
    render_exit_road(board, layout, textures);
}

/// Renders the park lawn with soft stripes and grass tufts.
fn render_park_lawn(layout: &BoardLayout) {
    let sw = layout.screen_width;
    let sh = layout.screen_height;
    let start_y = layout.hud_height;
    let lawn_h = (sh - start_y).max(0.0);

    // Base lawn fill
    draw_rectangle(0.0, start_y, sw, lawn_h, GRASS_BASE);

    // Subtle lawn mowing stripes
    let stripe_h = (layout.cell_size * 0.65).clamp(24.0, 60.0);
    let num_stripes = ((lawn_h / stripe_h).ceil() as usize) + 1;

    for i in 0..num_stripes {
        if i % 2 == 1 {
            let sy = start_y + i as f32 * stripe_h;
            let h = stripe_h.min(sh - sy);
            if h > 0.0 {
                draw_rectangle(0.0, sy, sw, h, GRASS_STRIPE);
            }
        }
    }

    // Organic grass tufts scattered in corners
    draw_grass_tuft(sw * 0.08, start_y + 24.0);
    draw_grass_tuft(sw * 0.92, start_y + 32.0);
    draw_grass_tuft(sw * 0.15, sh - 45.0);
    draw_grass_tuft(sw * 0.88, sh - 38.0);
    draw_grass_tuft(sw * 0.50, sh - 28.0);
}

fn draw_grass_tuft(cx: f32, cy: f32) {
    let thick = 1.6;
    draw_line(cx - 3.0, cy, cx - 5.0, cy - 6.0, thick, GRASS_TUFT);
    draw_line(cx, cy, cx, cy - 8.0, thick, GRASS_TUFT);
    draw_line(cx + 3.0, cy, cx + 5.0, cy - 6.0, thick, GRASS_TUFT);
}

/// Renders cobblestone / gravel footpaths winding through the park.
fn render_footpaths(layout: &BoardLayout, exit_side: ExitSide) {
    let sw = layout.screen_width;
    let sh = layout.screen_height;
    let oy = layout.origin_y;
    let ox = layout.origin_x;
    let bw = layout.total_width;
    let bh = layout.total_height;

    // Top footpath running horizontally above the parking lot
    let top_path_y = (layout.hud_height + oy) / 2.0;
    if exit_side != ExitSide::Top {
        draw_stepping_stone_path(18.0, top_path_y, sw - 18.0, top_path_y, 8);
    }

    // Bottom footpath connecting park sides below parking lot
    let bottom_path_y = oy + bh + (sh - (oy + bh)) * 0.28;
    if exit_side != ExitSide::Bottom {
        draw_stepping_stone_path(24.0, bottom_path_y, sw - 24.0, bottom_path_y + 12.0, 9);
    } else {
        // Path around the bottom exit road
        draw_stepping_stone_path(20.0, bottom_path_y, ox - 12.0, bottom_path_y, 4);
        draw_stepping_stone_path(ox + bw + 12.0, bottom_path_y, sw - 20.0, bottom_path_y, 4);
    }
}

fn draw_stepping_stone_path(x1: f32, y1: f32, x2: f32, y2: f32, steps: usize) {
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let x = x1 + (x2 - x1) * t;
        let y = y1 + (y2 - y1) * t + (t * std::f32::consts::PI).sin() * 6.0;
        let rx = 10.0 + (i % 3) as f32 * 1.5;
        let ry = 7.5 + ((i + 1) % 2) as f32 * 1.2;

        // Shadow & Stone
        draw_ellipse(x + 1.5, y + 2.0, rx, ry, 0.0, PATH_BORDER);
        draw_ellipse(x, y, rx, ry, 0.0, PATH_STONE);
        draw_ellipse(x - 2.0, y - 1.5, rx * 0.6, ry * 0.55, 0.0, PATH_PEBBLE);
    }
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

/// Renders a natural pond with shoreline, depth gradient, animated ripples, lily pads, and reeds.
fn render_pond(layout: &BoardLayout, exit_side: ExitSide, time: f32) {
    let sw = layout.screen_width;
    let sh = layout.screen_height;
    let oy = layout.origin_y;
    let bh = layout.total_height;

    // Determine pond position based on available space and exit orientation
    let space_bottom = sh - (oy + bh);
    let (cx, cy, rx, ry) = if space_bottom >= 80.0 {
        let cy = oy + bh + space_bottom * 0.62;
        let cx = match exit_side {
            ExitSide::Right => sw * 0.28,
            ExitSide::Left => sw * 0.72,
            ExitSide::Bottom => sw * 0.20,
            ExitSide::Top => sw * 0.32,
        };
        let rx = (sw * 0.22).clamp(42.0, 95.0);
        let ry = (space_bottom * 0.32).clamp(24.0, 52.0);
        (cx, cy, rx, ry)
    } else {
        // If bottom space is compact, place near bottom right/left corner
        let cy = sh - 45.0;
        let cx = if exit_side == ExitSide::Left {
            sw * 0.80
        } else {
            sw * 0.20
        };
        (cx, cy, 50.0, 30.0)
    };

    // 1. Organic Sandy Shoreline (layered offset ellipses)
    draw_ellipse(
        cx + 2.0,
        cy + 2.0,
        rx + 10.0,
        ry + 8.0,
        0.0,
        Color::new(0.04, 0.08, 0.05, 0.35),
    );
    draw_ellipse(cx, cy, rx + 8.0, ry + 6.0, 0.0, POND_SAND);
    draw_ellipse(
        cx - 1.5,
        cy - 1.0,
        rx + 6.0,
        ry + 4.5,
        0.0,
        Color::new(0.40, 0.36, 0.28, 1.0),
    );

    // Smooth shoreline pebbles
    draw_circle(cx - rx * 0.85, cy - ry * 0.4, 3.5, PATH_PEBBLE);
    draw_circle(cx + rx * 0.90, cy + ry * 0.3, 3.0, PATH_PEBBLE);
    draw_circle(cx - rx * 0.45, cy + ry * 0.9, 2.5, PATH_PEBBLE);
    draw_circle(cx + rx * 0.55, cy - ry * 0.85, 3.0, PATH_PEBBLE);

    // 2. Water Surface Depth Gradient (outer shallow -> mid -> deep center)
    draw_ellipse(cx, cy, rx, ry, 0.0, POND_SHALLOW);
    draw_ellipse(cx + 1.0, cy + 1.0, rx * 0.80, ry * 0.78, 0.0, POND_MID);
    draw_ellipse(cx + 2.0, cy + 2.0, rx * 0.55, ry * 0.52, 0.0, POND_DEEP);

    // 3. Gentle Water Ripples (animated concentric wave pulses)
    let max_ripple_r = rx * 0.65;
    let wave1_phase = (time * 1.8) % 3.0;
    let r1 = (wave1_phase / 3.0) * max_ripple_r;
    let a1 = (1.0 - (wave1_phase / 3.0)) * 0.45;
    draw_ellipse_lines(
        cx - rx * 0.15,
        cy - ry * 0.1,
        r1,
        r1 * (ry / rx),
        0.0,
        1.4,
        Color::new(0.7, 0.9, 1.0, a1),
    );

    let wave2_phase = ((time * 1.8) + 1.5) % 3.0;
    let r2 = (wave2_phase / 3.0) * max_ripple_r;
    let a2 = (1.0 - (wave2_phase / 3.0)) * 0.45;
    draw_ellipse_lines(
        cx + rx * 0.20,
        cy + ry * 0.15,
        r2,
        r2 * (ry / rx),
        0.0,
        1.4,
        Color::new(0.7, 0.9, 1.0, a2),
    );

    // 4. Floating Lily Pads & Lotus Flowers
    draw_lily_pad(cx - rx * 0.40, cy - ry * 0.20, 8.0, 0.4);
    draw_lily_pad(cx - rx * 0.22, cy + ry * 0.35, 10.0, 2.1);
    draw_water_lily_flower(cx - rx * 0.22, cy + ry * 0.35, 4.5);

    draw_lily_pad(cx + rx * 0.42, cy - ry * 0.10, 9.0, 4.2);
    draw_water_lily_flower(cx + rx * 0.42, cy - ry * 0.10, 4.0);
    draw_lily_pad(cx + rx * 0.25, cy + ry * 0.40, 7.5, 1.2);

    // 5. Shoreline Reeds & Cattails
    draw_cattail(cx - rx * 0.78, cy + ry * 0.35, time);
    draw_cattail(cx - rx * 0.70, cy + ry * 0.55, time + 0.5);
    draw_cattail(cx + rx * 0.75, cy - ry * 0.35, time + 1.2);
    draw_cattail(cx + rx * 0.82, cy - ry * 0.15, time + 1.8);
}

fn draw_lily_pad(x: f32, y: f32, radius: f32, angle: f32) {
    // Drop shadow in water
    draw_circle(x + 1.0, y + 1.0, radius, Color::new(0.04, 0.15, 0.22, 0.6));
    // Pad disc
    draw_circle(x, y, radius, LILY_PAD_DARK);
    draw_circle(x, y, radius - 1.0, LILY_PAD);

    // Small wedge notch to give classic lily pad appearance
    let notch_x = x + angle.cos() * radius * 0.8;
    let notch_y = y + angle.sin() * radius * 0.8;
    draw_line(x, y, notch_x, notch_y, 1.5, POND_MID);
}

fn draw_water_lily_flower(x: f32, y: f32, size: f32) {
    let petals = 6;
    for i in 0..petals {
        let a = i as f32 * (std::f32::consts::TAU / petals as f32);
        let px = x + a.cos() * (size * 0.75);
        let py = y + a.sin() * (size * 0.75);
        draw_circle(px, py, size * 0.5, LILY_FLOWER);
    }
    draw_circle(x, y, size * 0.45, LILY_CENTER);
}

fn draw_cattail(x: f32, y: f32, time: f32) {
    let sway = (time * 2.0).sin() * 2.0;
    let h = 18.0;
    let tip_x = x + sway;
    let tip_y = y - h;

    // Green stalk
    draw_line(x, y, tip_x, tip_y, 1.8, REED_STALK);
    // Brown cattail cylinder head
    let mid_x = x + (tip_x - x) * 0.65;
    let mid_y = y - h * 0.65;
    draw_line(mid_x, mid_y, tip_x, tip_y, 3.2, REED_HEAD);
}

/// Renders decorative wooden park benches.
fn render_park_benches(layout: &BoardLayout, exit_side: ExitSide) {
    let sw = layout.screen_width;
    let sh = layout.screen_height;
    let oy = layout.origin_y;
    let bh = layout.total_height;

    // Bench in the top area if exit is not top
    if exit_side != ExitSide::Top {
        let bench_x = sw * 0.82;
        let bench_y = (layout.hud_height + oy) / 2.0 - 4.0;
        draw_bench(bench_x, bench_y, 22.0, 8.0);
    }

    // Bench in the bottom area opposite the pond
    if exit_side != ExitSide::Bottom && sh - (oy + bh) >= 70.0 {
        let bench_x = sw * 0.76;
        let bench_y = oy + bh + (sh - (oy + bh)) * 0.35;
        draw_bench(bench_x, bench_y, 24.0, 9.0);
    }
}

fn draw_bench(x: f32, y: f32, w: f32, h: f32) {
    // Shadow
    draw_rectangle(
        x - w / 2.0 + 1.5,
        y - h / 2.0 + 2.0,
        w,
        h,
        Color::new(0.04, 0.08, 0.05, 0.4),
    );
    // Metal cast-iron frame
    draw_rectangle(
        x - w / 2.0,
        y - h / 2.0,
        w,
        h,
        Color::new(0.18, 0.20, 0.22, 1.0),
    );
    // Wooden slats
    let slat_h = (h - 3.0) / 2.0;
    draw_rectangle(
        x - w / 2.0 + 1.5,
        y - h / 2.0 + 1.0,
        w - 3.0,
        slat_h,
        Color::new(0.55, 0.34, 0.18, 1.0),
    );
    draw_rectangle(
        x - w / 2.0 + 1.5,
        y + 0.5,
        w - 3.0,
        slat_h,
        Color::new(0.48, 0.28, 0.14, 1.0),
    );
}

/// Renders lush trees, hedges, and wildflowers.
fn render_flora(board: &Board, layout: &BoardLayout) {
    let sw = layout.screen_width;
    let sh = layout.screen_height;
    let ox = layout.origin_x;
    let oy = layout.origin_y;
    let bw = layout.total_width;
    let bh = layout.total_height;
    let hud_h = layout.hud_height;

    // Top park area trees
    let top_y = (hud_h + oy) / 2.0;
    if board.exit.side != ExitSide::Top {
        draw_lush_tree(sw * 0.18, top_y, 22.0);
        draw_lush_tree(sw * 0.36, top_y - 2.0, 18.0);
        if board.exit.side != ExitSide::Right {
            draw_lush_tree(sw * 0.84, top_y + 4.0, 24.0);
        }
    } else {
        // Space around top exit road
        draw_lush_tree(sw * 0.15, top_y, 20.0);
        draw_lush_tree(sw * 0.85, top_y, 20.0);
    }

    // Side margins (if wide screen or tablet)
    if ox >= 40.0 {
        draw_lush_tree(ox / 2.0, oy + bh * 0.35, 20.0);
        draw_lush_tree(ox / 2.0, oy + bh * 0.70, 18.0);
    }
    if sw - (ox + bw) >= 40.0 && board.exit.side != ExitSide::Right {
        let right_x = ox + bw + (sw - (ox + bw)) / 2.0;
        draw_lush_tree(right_x, oy + bh * 0.35, 20.0);
        draw_lush_tree(right_x, oy + bh * 0.70, 18.0);
    }

    // Bottom area trees
    let bot_space = sh - (oy + bh);
    if bot_space >= 80.0 && board.exit.side != ExitSide::Bottom {
        let bot_tree_y = oy + bh + bot_space * 0.70;
        draw_lush_tree(sw * 0.88, bot_tree_y, 24.0);
    }

    // Shrub hedges along parking lot curbs
    render_curb_hedges(board, layout);

    // Scattered wildflower clusters
    render_wildflowers(sw, sh, hud_h);
}

fn draw_lush_tree(cx: f32, cy: f32, r: f32) {
    // 1. Directional soft shadow
    draw_ellipse(
        cx + r * 0.25,
        cy + r * 0.30,
        r * 1.15,
        r * 0.90,
        0.0,
        TREE_SHADOW,
    );

    // 2. Canopy Base Layers (multi-lobed top-down foliage cloud)
    let lobes = 5;
    for i in 0..lobes {
        let a = i as f32 * (std::f32::consts::TAU / lobes as f32);
        let lx = cx + a.cos() * (r * 0.50);
        let ly = cy + a.sin() * (r * 0.50);
        draw_circle(lx, ly, r * 0.65, TREE_DARK);
    }
    draw_circle(cx, cy, r * 0.85, TREE_DARK);

    // 3. Mid-tone foliage clumps
    for i in 0..lobes {
        let a = i as f32 * (std::f32::consts::TAU / lobes as f32);
        let lx = cx + a.cos() * (r * 0.42);
        let ly = cy + a.sin() * (r * 0.42);
        draw_circle(lx, ly, r * 0.55, TREE_MID);
    }
    draw_circle(cx, cy, r * 0.72, TREE_MID);

    // 4. Sunlit highlights (top-left illumination)
    for i in 0..3 {
        let a = (i as f32 * 0.7) - 2.2;
        let lx = cx + a.cos() * (r * 0.35);
        let ly = cy + a.sin() * (r * 0.35);
        draw_circle(lx, ly, r * 0.42, TREE_LIGHT);
        draw_circle(lx - 1.5, ly - 1.5, r * 0.25, TREE_HIGHLIGHT);
    }

    // 5. Central tree crown center & trunk detail
    draw_circle(cx, cy, 2.0, TREE_TRUNK);
    draw_circle(cx - 2.0, cy - 2.0, r * 0.35, TREE_LIGHT);
    draw_circle(cx - 3.0, cy - 3.0, r * 0.18, TREE_HIGHLIGHT);
}

fn render_curb_hedges(board: &Board, layout: &BoardLayout) {
    let ox = layout.origin_x;
    let oy = layout.origin_y;
    let bw = layout.total_width;
    let bh = layout.total_height;
    let cs = layout.cell_size;

    // Small rounded bushes hugging the corners of the parking lot
    let bush_r = (cs * 0.12).clamp(5.0, 10.0);

    // Top-Left corner bushes
    draw_hedge_cluster(ox - bush_r * 1.5, oy + 4.0, bush_r);
    draw_hedge_cluster(ox + 4.0, oy - bush_r * 1.5, bush_r);

    // Top-Right corner bushes (if not exit top/right)
    if board.exit.side != ExitSide::Top
        && (board.exit.side != ExitSide::Right || board.exit.row != 0)
    {
        draw_hedge_cluster(ox + bw - 4.0, oy - bush_r * 1.5, bush_r);
    }

    // Bottom-Left corner bushes
    if board.exit.side != ExitSide::Bottom || board.exit.col != 0 {
        draw_hedge_cluster(ox - bush_r * 1.5, oy + bh - 4.0, bush_r);
        draw_hedge_cluster(ox + 4.0, oy + bh + bush_r * 1.5, bush_r);
    }
}

fn draw_hedge_cluster(x: f32, y: f32, r: f32) {
    draw_circle(x + 1.0, y + 1.5, r * 1.1, BUSH_SHADOW);
    draw_circle(x, y, r, BUSH_DARK);
    draw_circle(x - 1.0, y - 1.0, r * 0.7, BUSH_LIGHT);
}

fn render_wildflowers(sw: f32, sh: f32, hud_h: f32) {
    let yellow = Color::new(0.98, 0.85, 0.20, 0.9);
    let white = Color::new(0.95, 0.95, 0.98, 0.9);
    let lavender = Color::new(0.75, 0.55, 0.90, 0.9);
    let poppy = Color::new(0.92, 0.35, 0.35, 0.9);

    // Flower patches in open grass zones
    draw_flower_patch(sw * 0.08, hud_h + 35.0, yellow, white);
    draw_flower_patch(sw * 0.90, hud_h + 40.0, lavender, poppy);
    draw_flower_patch(sw * 0.12, sh - 35.0, white, yellow);
    draw_flower_patch(sw * 0.84, sh - 45.0, poppy, lavender);
    draw_flower_patch(sw * 0.52, sh - 22.0, yellow, white);
}

fn draw_flower_patch(cx: f32, cy: f32, c1: Color, c2: Color) {
    draw_circle(cx - 4.0, cy - 3.0, 1.8, c1);
    draw_circle(cx + 3.0, cy - 2.0, 1.6, c2);
    draw_circle(cx, cy + 3.0, 1.7, c1);
    draw_circle(cx + 5.0, cy + 4.0, 1.5, c2);
}
