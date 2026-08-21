#!/usr/bin/env python3
"""
Asset Generator for Parking Escape
Renders high-quality 2.5D top-down / high-angle vehicle sprites and environment textures.
Uses 4x supersampling for ultra-crisp antialiasing, soft drop shadows, metallic highlights, and glossy finishes.
"""

import math
import os
from PIL import Image, ImageDraw, ImageFilter

CELL_SIZE = 128
SCALE = 4  # 4x supersampling

def create_shadow_layer(width, height, shape_drawer, blur_radius=12, offset=(6, 12), opacity=110):
    """Creates a realistic soft drop shadow."""
    shadow_img = Image.new("RGBA", (width, height), (0, 0, 0, 0))
    draw = ImageDraw.Draw(shadow_img)
    shape_drawer(draw, offset)
    shadow_img = shadow_img.filter(ImageFilter.GaussianBlur(blur_radius))
    
    # Adjust opacity
    r, g, b, a = shadow_img.split()
    a = a.point(lambda p: int(p * (opacity / 255.0)))
    shadow_img.putalpha(a)
    return shadow_img

def hex_to_rgb(hex_str):
    hex_str = hex_str.lstrip('#')
    return tuple(int(hex_str[i:i+2], 16) for i in (0, 2, 4))

def blend_color(c1, c2, factor):
    """Linear blend between two RGB colors."""
    return tuple(int(c1[i] + (c2[i] - c1[i]) * factor) for i in range(3))

# --- Environment Asset Generation ---

def generate_asphalt(out_path):
    """Generates a rich seamless dark asphalt texture with subtle noise and gravel specks."""
    w, h = 512, 512
    img = Image.new("RGBA", (w, h), hex_to_rgb("#1e222a") + (255,))
    draw = ImageDraw.Draw(img)
    
    import random
    random.seed(42)
    
    # Fine gravel noise
    for _ in range(12000):
        x = random.randint(0, w - 1)
        y = random.randint(0, h - 1)
        shade = random.randint(22, 48)
        alpha = random.randint(30, 90)
        draw.point((x, y), fill=(shade, shade + random.randint(0, 3), shade + random.randint(2, 6), alpha))
        
    for _ in range(800):
        x = random.randint(0, w - 1)
        y = random.randint(0, h - 1)
        size = random.randint(1, 3)
        shade = random.randint(50, 75)
        draw.ellipse([x, y, x + size, y + size], fill=(shade, shade + 2, shade + 4, 120))
        
    img.save(out_path, "PNG")
    print(f"Generated asphalt: {out_path}")

def generate_curbs(out_dir):
    """Generates horizontal, vertical, and corner modular concrete curbs."""
    # Horizontal curb
    w, h = CELL_SIZE * SCALE, 24 * SCALE
    img = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    # Concrete base
    draw.rectangle([0, 4*SCALE, w, h - 2*SCALE], fill="#4b5563")
    # Top highlight
    draw.rectangle([0, 0, w, 6*SCALE], fill="#9ca3af")
    # Hazard yellow/black stripes on curb
    stripe_w = 16 * SCALE
    for i in range(-stripe_w, w + stripe_w, stripe_w * 2):
        draw.polygon([
            (i, 6*SCALE), (i + stripe_w, 6*SCALE),
            (i + stripe_w + 10*SCALE, h - 2*SCALE), (i + 10*SCALE, h - 2*SCALE)
        ], fill="#f59e0b")
    # Bottom shadow
    draw.rectangle([0, h - 4*SCALE, w, h], fill="#1f2937")
    img = img.resize((CELL_SIZE, 24), Image.Resampling.LANCZOS)
    img.save(os.path.join(out_dir, "curb_horizontal.png"))

    # Vertical curb
    w, h = 24 * SCALE, CELL_SIZE * SCALE
    img = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    draw.rectangle([4*SCALE, 0, w - 2*SCALE, h], fill="#4b5563")
    draw.rectangle([0, 0, 6*SCALE, h], fill="#9ca3af")
    draw.rectangle([w - 4*SCALE, 0, w, h], fill="#1f2937")
    img = img.resize((24, CELL_SIZE), Image.Resampling.LANCZOS)
    img.save(os.path.join(out_dir, "curb_vertical.png"))

    # Corner curb
    w, h = 32 * SCALE, 32 * SCALE
    img = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    draw.rounded_rectangle([0, 0, w, h], radius=8*SCALE, fill="#6b7280", outline="#9ca3af", width=3*SCALE)
    img = img.resize((32, 32), Image.Resampling.LANCZOS)
    img.save(os.path.join(out_dir, "curb_corner.png"))
    print("Generated curbs")

def generate_stall_marker(out_path):
    """Generates clean white/yellow parking stall markings with round ends."""
    size = CELL_SIZE * SCALE
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # White dashed/corner stall bounds
    lw = 4 * SCALE
    col = (255, 255, 255, 180)
    # Corner lines
    draw.line([(8*SCALE, 8*SCALE), (32*SCALE, 8*SCALE)], fill=col, width=lw)
    draw.line([(8*SCALE, 8*SCALE), (8*SCALE, 32*SCALE)], fill=col, width=lw)
    draw.line([(size - 8*SCALE, 8*SCALE), (size - 32*SCALE, 8*SCALE)], fill=col, width=lw)
    draw.line([(size - 8*SCALE, 8*SCALE), (size - 8*SCALE, 32*SCALE)], fill=col, width=lw)
    draw.line([(8*SCALE, size - 8*SCALE), (32*SCALE, size - 8*SCALE)], fill=col, width=lw)
    draw.line([(8*SCALE, size - 8*SCALE), (8*SCALE, size - 32*SCALE)], fill=col, width=lw)
    draw.line([(size - 8*SCALE, size - 8*SCALE), (size - 32*SCALE, size - 8*SCALE)], fill=col, width=lw)
    draw.line([(size - 8*SCALE, size - 8*SCALE), (size - 8*SCALE, size - 32*SCALE)], fill=col, width=lw)

    img = img.resize((CELL_SIZE, CELL_SIZE), Image.Resampling.LANCZOS)
    img.save(out_path, "PNG")
    print(f"Generated stall marker: {out_path}")

def generate_exit_gate(out_path):
    """Generates glowing Exit Gate with neon green chevron arrows and barrier lights."""
    w, h = (CELL_SIZE + 24) * SCALE, 48 * SCALE
    img = Image.new("RGBA", (w, h), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    
    # Base asphalt glow
    draw.rounded_rectangle([4*SCALE, 4*SCALE, w - 4*SCALE, h - 4*SCALE], radius=10*SCALE, fill="#064e3b")
    # Neon border
    draw.rounded_rectangle([4*SCALE, 4*SCALE, w - 4*SCALE, h - 4*SCALE], radius=10*SCALE, outline="#10b981", width=4*SCALE)
    
    # Glowing Chevrons (>>>)
    cx = w // 2
    for offset in [-40*SCALE, -10*SCALE, 20*SCALE, 50*SCALE]:
        x = cx + offset
        pts = [
            (x - 12*SCALE, 12*SCALE),
            (x + 4*SCALE, h // 2),
            (x - 12*SCALE, h - 12*SCALE),
            (x - 4*SCALE, h - 12*SCALE),
            (x + 12*SCALE, h // 2),
            (x - 4*SCALE, 12*SCALE)
        ]
        draw.polygon(pts, fill="#34d399")
        
    img = img.resize((CELL_SIZE + 24, 48), Image.Resampling.LANCZOS)
    img.save(out_path, "PNG")
    print(f"Generated exit gate: {out_path}")

# --- Vehicle Sprite Renderer ---

class VehicleSpec:
    def __init__(self, name, length, primary_color, secondary_color, roof_color, window_color="#1e293b", style="car"):
        self.name = name
        self.length = length  # 2, 3, or 4 tiles
        self.primary = hex_to_rgb(primary_color)
        self.secondary = hex_to_rgb(secondary_color)
        self.roof = hex_to_rgb(roof_color)
        self.window = hex_to_rgb(window_color)
        self.style = style  # 'sports', 'sedan', 'taxi', 'police', 'van', 'limo', 'ambulance', 'semi', 'bus'

def draw_vehicle(spec, orientation, out_path):
    """Renders a single vehicle sprite in high-angle 2.5D with realistic lighting and details."""
    is_horiz = (orientation == 'h')
    
    # Dimensions in supersampled pixels
    tile_px = CELL_SIZE * SCALE
    if is_horiz:
        vw = spec.length * tile_px
        vh = tile_px
    else:
        vw = tile_px
        vh = spec.length * tile_px

    # Margins inside tile for nice spacing
    mx = 8 * SCALE
    my = 8 * SCALE

    # Main canvas
    canvas = Image.new("RGBA", (vw, vh), (0, 0, 0, 0))

    # 1. Soft Drop Shadow
    def draw_shadow(draw_ctx, offset):
        ox, oy = offset[0] * SCALE, offset[1] * SCALE
        if is_horiz:
            bx1, by1 = mx + ox, my + 6*SCALE + oy
            bx2, by2 = vw - mx + ox, vh - my + oy
            draw_ctx.rounded_rectangle([bx1, by1, bx2, by2], radius=16*SCALE, fill=(0, 0, 0, 255))
        else:
            bx1, by1 = mx + 4*SCALE + ox, my + oy
            bx2, by2 = vw - mx + ox, vh - my + oy
            draw_ctx.rounded_rectangle([bx1, by1, bx2, by2], radius=16*SCALE, fill=(0, 0, 0, 255))

    shadow = create_shadow_layer(vw, vh, draw_shadow, blur_radius=16*SCALE, offset=(4, 8), opacity=140)
    canvas.paste(shadow, (0, 0), shadow)

    # 2. Wheels / Tires (visible beneath chassis)
    wheel_img = Image.new("RGBA", (vw, vh), (0, 0, 0, 0))
    w_draw = ImageDraw.Draw(wheel_img)
    tire_color = (24, 24, 27, 255)
    rim_color = (161, 161, 170, 255)

    if is_horiz:
        wheel_w, wheel_h = 24 * SCALE, 8 * SCALE
        # Positions along X
        wheel_xs = [mx + 20*SCALE, vw - mx - 44*SCALE]
        if spec.length >= 3:
            wheel_xs.append(mx + int(vw * 0.45))
        if spec.length == 4:
            wheel_xs.append(vw - mx - 72*SCALE)

        for wx in wheel_xs:
            # Top tire
            w_draw.rounded_rectangle([wx, my + 2*SCALE, wx + wheel_w, my + 2*SCALE + wheel_h], radius=3*SCALE, fill=tire_color)
            w_draw.line([(wx + 6*SCALE, my + 6*SCALE), (wx + wheel_w - 6*SCALE, my + 6*SCALE)], fill=rim_color, width=2*SCALE)
            # Bottom tire
            w_draw.rounded_rectangle([wx, vh - my - 10*SCALE, wx + wheel_w, vh - my - 2*SCALE], radius=3*SCALE, fill=tire_color)
            w_draw.line([(wx + 6*SCALE, vh - my - 6*SCALE), (wx + wheel_w - 6*SCALE, vh - my - 6*SCALE)], fill=rim_color, width=2*SCALE)
    else:
        wheel_w, wheel_h = 8 * SCALE, 24 * SCALE
        wheel_ys = [my + 20*SCALE, vh - my - 44*SCALE]
        if spec.length >= 3:
            wheel_ys.append(my + int(vh * 0.45))
        if spec.length == 4:
            wheel_ys.append(vh - my - 72*SCALE)

        for wy in wheel_ys:
            # Left tire
            w_draw.rounded_rectangle([mx + 2*SCALE, wy, mx + 2*SCALE + wheel_w, wy + wheel_h], radius=3*SCALE, fill=tire_color)
            w_draw.line([(mx + 6*SCALE, wy + 6*SCALE), (mx + 6*SCALE, wy + wheel_h - 6*SCALE)], fill=rim_color, width=2*SCALE)
            # Right tire
            w_draw.rounded_rectangle([vw - mx - 10*SCALE, wy, vw - mx - 2*SCALE, wy + wheel_h], radius=3*SCALE, fill=tire_color)
            w_draw.line([(vw - mx - 6*SCALE, wy + 6*SCALE), (vw - mx - 6*SCALE, wy + wheel_h - 6*SCALE)], fill=rim_color, width=2*SCALE)

    canvas.paste(wheel_img, (0, 0), wheel_img)

    # 3. Main Vehicle Body
    body_img = Image.new("RGBA", (vw, vh), (0, 0, 0, 0))
    b_draw = ImageDraw.Draw(body_img)

    # Main chassis bounds
    if is_horiz:
        cx1, cy1 = mx + 4*SCALE, my + 6*SCALE
        cx2, cy2 = vw - mx - 4*SCALE, vh - my - 6*SCALE
        corner_r = 14 * SCALE
    else:
        cx1, cy1 = mx + 6*SCALE, my + 4*SCALE
        cx2, cy2 = vw - mx - 6*SCALE, vh - my - 4*SCALE
        corner_r = 14 * SCALE

    # Gradient/Lighting base for body (High-angle lighting from top-left)
    base_col = spec.primary
    highlight_col = blend_color(base_col, (255, 255, 255), 0.35)
    shadow_col = blend_color(base_col, (0, 0, 0), 0.4)

    # Base chassis with dark outline
    b_draw.rounded_rectangle([cx1, cy1, cx2, cy2], radius=corner_r, fill=base_col, outline=shadow_col, width=3*SCALE)

    # Top-edge body highlight
    if is_horiz:
        b_draw.rounded_rectangle([cx1 + 2*SCALE, cy1 + 2*SCALE, cx2 - 2*SCALE, cy1 + 8*SCALE], radius=4*SCALE, fill=highlight_col)
    else:
        b_draw.rounded_rectangle([cx1 + 2*SCALE, cy1 + 2*SCALE, cx1 + 8*SCALE, cy2 - 2*SCALE], radius=4*SCALE, fill=highlight_col)

    # 4. Cab / Greenhouse / Windows / Roof
    # Calculate cabin area
    if is_horiz:
        # Cabin bounds along length
        if spec.style == "semi":
            # Semi-truck: Cab on right (facing right) + Trailer on left
            cab_w = 48 * SCALE
            trailer_w = vw - mx*2 - cab_w - 16*SCALE
            
            # Trailer Body (corrugated cargo container)
            tx1, ty1 = cx1 + 2*SCALE, cy1 + 2*SCALE
            tx2, ty2 = tx1 + trailer_w, cy2 - 2*SCALE
            trailer_col = spec.secondary
            b_draw.rounded_rectangle([tx1, ty1, tx2, ty2], radius=8*SCALE, fill=trailer_col, outline=shadow_col, width=2*SCALE)
            # Corrugation stripes
            for sx in range(int(tx1 + 10*SCALE), int(tx2 - 10*SCALE), 14*SCALE):
                b_draw.line([(sx, ty1 + 4*SCALE), (sx, ty2 - 4*SCALE)], fill=blend_color(trailer_col, (0, 0, 0), 0.2), width=3*SCALE)
                b_draw.line([(sx + 3*SCALE, ty1 + 4*SCALE), (sx + 3*SCALE, ty2 - 4*SCALE)], fill=blend_color(trailer_col, (255, 255, 255), 0.2), width=2*SCALE)

            # Truck Cab
            cab_x1 = tx2 + 10*SCALE
            cab_x2 = cx2 - 2*SCALE
            b_draw.rounded_rectangle([cab_x1, cy1 + 2*SCALE, cab_x2, cy2 - 2*SCALE], radius=8*SCALE, fill=base_col)
            
            # Cab windshield & windows
            b_draw.rounded_rectangle([cab_x1 + 14*SCALE, cy1 + 8*SCALE, cab_x2 - 8*SCALE, cy2 - 8*SCALE], radius=4*SCALE, fill=spec.window)
            b_draw.line([(cab_x2 - 12*SCALE, cy1 + 10*SCALE), (cab_x2 - 12*SCALE, cy2 - 10*SCALE)], fill=(180, 220, 255, 200), width=3*SCALE)

        elif spec.style == "bus":
            # Transit Bus: panoramic roof and side window bands
            rx1, ry1 = cx1 + 12*SCALE, cy1 + 8*SCALE
            rx2, ry2 = cx2 - 12*SCALE, cy2 - 8*SCALE
            b_draw.rounded_rectangle([rx1, ry1, rx2, ry2], radius=8*SCALE, fill=spec.roof)
            # Windows band
            b_draw.rounded_rectangle([cx1 + 8*SCALE, cy1 + 4*SCALE, cx2 - 8*SCALE, cy1 + 10*SCALE], radius=3*SCALE, fill=spec.window)
            b_draw.rounded_rectangle([cx1 + 8*SCALE, cy2 - 10*SCALE, cx2 - 8*SCALE, cy2 - 4*SCALE], radius=3*SCALE, fill=spec.window)
            # Front windshield
            b_draw.rectangle([cx2 - 16*SCALE, cy1 + 6*SCALE, cx2 - 6*SCALE, cy2 - 6*SCALE], fill=spec.window)
            # AC units on roof
            for ac_x in [cx1 + 60*SCALE, cx1 + 140*SCALE, cx1 + 220*SCALE]:
                b_draw.rounded_rectangle([ac_x, cy1 + 14*SCALE, ac_x + 32*SCALE, cy2 - 14*SCALE], radius=4*SCALE, fill="#e2e8f0", outline="#94a3b8", width=2*SCALE)

        else:
            # Standard cars, sports, taxi, police, van, limo
            cab_margin_x = int((cx2 - cx1) * 0.22) if spec.style != "limo" else int((cx2 - cx1) * 0.12)
            cab_margin_y = 10 * SCALE
            rx1, ry1 = cx1 + cab_margin_x, cy1 + cab_margin_y
            rx2, ry2 = cx2 - int((cx2 - cx1) * 0.18), cy2 - cab_margin_y
            
            # Glass Area
            b_draw.rounded_rectangle([rx1, ry1, rx2, ry2], radius=10*SCALE, fill=spec.window)
            
            # Roof
            roof_mx = 14 * SCALE
            roof_my = 6 * SCALE
            roof_x1, roof_y1 = rx1 + roof_mx, ry1 + roof_my
            roof_x2, roof_y2 = rx2 - roof_mx, ry2 - roof_my
            b_draw.rounded_rectangle([roof_x1, roof_y1, roof_x2, roof_y2], radius=6*SCALE, fill=spec.roof)
            
            # Glass reflection streak
            b_draw.line([(rx2 - 10*SCALE, ry1 + 4*SCALE), (rx2 - 10*SCALE, ry2 - 4*SCALE)], fill=(200, 235, 255, 180), width=4*SCALE)
            b_draw.line([(rx1 + 8*SCALE, ry1 + 4*SCALE), (rx1 + 8*SCALE, ry2 - 4*SCALE)], fill=(200, 235, 255, 120), width=3*SCALE)

            # Special roof props
            if spec.style == "taxi":
                # Taxi Roof Sign (Yellow/Black)
                tsx1, tsy1 = (roof_x1 + roof_x2)//2 - 12*SCALE, (roof_y1 + roof_y2)//2 - 5*SCALE
                b_draw.rounded_rectangle([tsx1, tsy1, tsx1 + 24*SCALE, tsy1 + 10*SCALE], radius=3*SCALE, fill="#facc15", outline="#713f12", width=2*SCALE)
            elif spec.style == "police":
                # Police Lightbar (Red & Blue)
                lx, ly = (roof_x1 + roof_x2)//2 - 14*SCALE, (roof_y1 + roof_y2)//2 - 4*SCALE
                b_draw.rounded_rectangle([lx, ly, lx + 12*SCALE, ly + 8*SCALE], radius=2*SCALE, fill="#ef4444")
                b_draw.rounded_rectangle([lx + 16*SCALE, ly, lx + 28*SCALE, ly + 8*SCALE], radius=2*SCALE, fill="#3b82f6")
                b_draw.rectangle([lx + 12*SCALE, ly + 2*SCALE, lx + 16*SCALE, ly + 6*SCALE], fill="#ffffff")
            elif spec.style == "ambulance":
                # Ambulance Cross on roof & lightbar
                cx, cy = (roof_x1 + roof_x2)//2, (roof_y1 + roof_y2)//2
                b_draw.rectangle([cx - 3*SCALE, cy - 12*SCALE, cx + 3*SCALE, cy + 12*SCALE], fill="#ef4444")
                b_draw.rectangle([cx - 12*SCALE, cy - 3*SCALE, cx + 12*SCALE, cy + 3*SCALE], fill="#ef4444")
                # Lightbar at front
                b_draw.rounded_rectangle([roof_x2 - 6*SCALE, roof_y1 + 4*SCALE, roof_x2 - 2*SCALE, roof_y2 - 4*SCALE], radius=2*SCALE, fill="#ef4444")
            elif spec.style == "sports":
                # Sports Racing Stripes & Rear Spoiler
                b_draw.rectangle([cx1 + 4*SCALE, (cy1+cy2)//2 - 4*SCALE, cx2 - 4*SCALE, (cy1+cy2)//2 + 4*SCALE], fill="#ffffff")
                # Rear Spoiler on left
                b_draw.rounded_rectangle([cx1 + 2*SCALE, cy1 + 6*SCALE, cx1 + 8*SCALE, cy2 - 6*SCALE], radius=2*SCALE, fill="#18181b")

        # Headlights & Taillights (Facing Right: Headlights on right, Taillights on left)
        # Headlights (Right)
        hl_y1, hl_y2 = cy1 + 8*SCALE, cy2 - 8*SCALE
        b_draw.rounded_rectangle([cx2 - 4*SCALE, hl_y1, cx2, hl_y1 + 10*SCALE], radius=3*SCALE, fill="#fef08a")
        b_draw.rounded_rectangle([cx2 - 4*SCALE, hl_y2 - 10*SCALE, cx2, hl_y2], radius=3*SCALE, fill="#fef08a")
        # Taillights (Left)
        b_draw.rounded_rectangle([cx1, hl_y1, cx1 + 4*SCALE, hl_y1 + 8*SCALE], radius=2*SCALE, fill="#dc2626")
        b_draw.rounded_rectangle([cx1, hl_y2 - 8*SCALE, cx1 + 4*SCALE, hl_y2], radius=2*SCALE, fill="#dc2626")

    else:
        # VERTICAL (Facing Down: Headlights at bottom, Taillights at top)
        if spec.style == "semi":
            cab_h = 48 * SCALE
            trailer_h = vh - my*2 - cab_h - 16*SCALE
            
            # Trailer Body
            tx1, ty1 = cx1 + 2*SCALE, cy1 + 2*SCALE
            tx2, ty2 = cx2 - 2*SCALE, ty1 + trailer_h
            trailer_col = spec.secondary
            b_draw.rounded_rectangle([tx1, ty1, tx2, ty2], radius=8*SCALE, fill=trailer_col, outline=shadow_col, width=2*SCALE)
            # Corrugation stripes
            for sy in range(int(ty1 + 10*SCALE), int(ty2 - 10*SCALE), 14*SCALE):
                b_draw.line([(tx1 + 4*SCALE, sy), (tx2 - 4*SCALE, sy)], fill=blend_color(trailer_col, (0, 0, 0), 0.2), width=3*SCALE)
                b_draw.line([(tx1 + 4*SCALE, sy + 3*SCALE), (tx2 - 4*SCALE, sy + 3*SCALE)], fill=blend_color(trailer_col, (255, 255, 255), 0.2), width=2*SCALE)

            # Cab at bottom
            cab_y1 = ty2 + 10*SCALE
            cab_y2 = cy2 - 2*SCALE
            b_draw.rounded_rectangle([cx1 + 2*SCALE, cab_y1, cx2 - 2*SCALE, cab_y2], radius=8*SCALE, fill=base_col)
            
            # Cab Windshield
            b_draw.rounded_rectangle([cx1 + 8*SCALE, cab_y1 + 14*SCALE, cx2 - 8*SCALE, cab_y2 - 8*SCALE], radius=4*SCALE, fill=spec.window)
            b_draw.line([(cx1 + 10*SCALE, cab_y2 - 12*SCALE), (cx2 - 10*SCALE, cab_y2 - 12*SCALE)], fill=(180, 220, 255, 200), width=3*SCALE)

        elif spec.style == "bus":
            rx1, ry1 = cx1 + 8*SCALE, cy1 + 12*SCALE
            rx2, ry2 = cx2 - 8*SCALE, cy2 - 12*SCALE
            b_draw.rounded_rectangle([rx1, ry1, rx2, ry2], radius=8*SCALE, fill=spec.roof)
            # Windows band
            b_draw.rounded_rectangle([cx1 + 4*SCALE, cy1 + 8*SCALE, cx1 + 10*SCALE, cy2 - 8*SCALE], radius=3*SCALE, fill=spec.window)
            b_draw.rounded_rectangle([cx2 - 10*SCALE, cy1 + 8*SCALE, cx2 - 4*SCALE, cy2 - 8*SCALE], radius=3*SCALE, fill=spec.window)
            # Windshield (bottom)
            b_draw.rectangle([cx1 + 6*SCALE, cy2 - 16*SCALE, cx2 - 6*SCALE, cy2 - 6*SCALE], fill=spec.window)
            # AC units
            for ac_y in [cy1 + 60*SCALE, cy1 + 140*SCALE, cy1 + 220*SCALE]:
                b_draw.rounded_rectangle([cx1 + 14*SCALE, ac_y, cx2 - 14*SCALE, ac_y + 32*SCALE], radius=4*SCALE, fill="#e2e8f0", outline="#94a3b8", width=2*SCALE)

        else:
            cab_margin_y = int((cy2 - cy1) * 0.22) if spec.style != "limo" else int((cy2 - cy1) * 0.12)
            cab_margin_x = 10 * SCALE
            rx1, ry1 = cx1 + cab_margin_x, cy1 + cab_margin_y
            rx2, ry2 = cx2 - cab_margin_x, cy2 - int((cy2 - cy1) * 0.18)
            
            # Glass Area
            b_draw.rounded_rectangle([rx1, ry1, rx2, ry2], radius=10*SCALE, fill=spec.window)
            
            # Roof
            roof_mx = 6 * SCALE
            roof_my = 14 * SCALE
            roof_x1, roof_y1 = rx1 + roof_mx, ry1 + roof_my
            roof_x2, roof_y2 = rx2 - roof_mx, ry2 - roof_my
            b_draw.rounded_rectangle([roof_x1, roof_y1, roof_x2, roof_y2], radius=6*SCALE, fill=spec.roof)
            
            # Glass reflection streak
            b_draw.line([(rx1 + 4*SCALE, ry2 - 10*SCALE), (rx2 - 4*SCALE, ry2 - 10*SCALE)], fill=(200, 235, 255, 180), width=4*SCALE)
            b_draw.line([(rx1 + 4*SCALE, ry1 + 8*SCALE), (rx2 - 4*SCALE, ry1 + 8*SCALE)], fill=(200, 235, 255, 120), width=3*SCALE)

            if spec.style == "taxi":
                tsx1, tsy1 = (roof_x1 + roof_x2)//2 - 10*SCALE, (roof_y1 + roof_y2)//2 - 12*SCALE
                b_draw.rounded_rectangle([tsx1, tsy1, tsx1 + 20*SCALE, tsy1 + 24*SCALE], radius=3*SCALE, fill="#facc15", outline="#713f12", width=2*SCALE)
            elif spec.style == "police":
                lx, ly = (roof_x1 + roof_x2)//2 - 4*SCALE, (roof_y1 + roof_y2)//2 - 14*SCALE
                b_draw.rounded_rectangle([lx, ly, lx + 8*SCALE, ly + 12*SCALE], radius=2*SCALE, fill="#ef4444")
                b_draw.rounded_rectangle([lx, ly + 16*SCALE, lx + 8*SCALE, ly + 28*SCALE], radius=2*SCALE, fill="#3b82f6")
                b_draw.rectangle([lx + 2*SCALE, ly + 12*SCALE, lx + 6*SCALE, ly + 16*SCALE], fill="#ffffff")
            elif spec.style == "ambulance":
                cx, cy = (roof_x1 + roof_x2)//2, (roof_y1 + roof_y2)//2
                b_draw.rectangle([cx - 3*SCALE, cy - 12*SCALE, cx + 3*SCALE, cy + 12*SCALE], fill="#ef4444")
                b_draw.rectangle([cx - 12*SCALE, cy - 3*SCALE, cx + 12*SCALE, cy + 3*SCALE], fill="#ef4444")
                b_draw.rounded_rectangle([roof_x1 + 4*SCALE, roof_y2 - 6*SCALE, roof_x2 - 4*SCALE, roof_y2 - 2*SCALE], radius=2*SCALE, fill="#ef4444")
            elif spec.style == "sports":
                b_draw.rectangle([(cx1+cx2)//2 - 4*SCALE, cy1 + 4*SCALE, (cx1+cx2)//2 + 4*SCALE, cy2 - 4*SCALE], fill="#ffffff")
                b_draw.rounded_rectangle([cx1 + 6*SCALE, cy1 + 2*SCALE, cx2 - 6*SCALE, cy1 + 8*SCALE], radius=2*SCALE, fill="#18181b")

        # Headlights & Taillights (Facing Down: Headlights at bottom, Taillights at top)
        hl_x1, hl_x2 = cx1 + 8*SCALE, cx2 - 8*SCALE
        # Headlights (Bottom)
        b_draw.rounded_rectangle([hl_x1, cy2 - 4*SCALE, hl_x1 + 10*SCALE, cy2], radius=3*SCALE, fill="#fef08a")
        b_draw.rounded_rectangle([hl_x2 - 10*SCALE, cy2 - 4*SCALE, hl_x2, cy2], radius=3*SCALE, fill="#fef08a")
        # Taillights (Top)
        b_draw.rounded_rectangle([hl_x1, cy1, hl_x1 + 8*SCALE, cy1 + 4*SCALE], radius=2*SCALE, fill="#dc2626")
        b_draw.rounded_rectangle([hl_x2 - 8*SCALE, cy1, hl_x2, cy1 + 4*SCALE], radius=2*SCALE, fill="#dc2626")

    canvas.paste(body_img, (0, 0), body_img)

    # Downsample with Lanczos for smooth antialiasing
    final_w = (spec.length * CELL_SIZE) if is_horiz else CELL_SIZE
    final_h = CELL_SIZE if is_horiz else (spec.length * CELL_SIZE)
    final_img = canvas.resize((final_w, final_h), Image.Resampling.LANCZOS)
    final_img.save(out_path, "PNG")
    print(f"Rendered: {out_path} ({final_w}x{final_h})")

def main():
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    env_dir = os.path.join(base_dir, "assets", "environment")
    veh_dir = os.path.join(base_dir, "assets", "vehicles")
    os.makedirs(env_dir, exist_ok=True)
    os.makedirs(veh_dir, exist_ok=True)

    print("--- Generating Environment Textures ---")
    generate_asphalt(os.path.join(env_dir, "asphalt.png"))
    generate_curbs(env_dir)
    generate_stall_marker(os.path.join(env_dir, "stall_marker.png"))
    generate_exit_gate(os.path.join(env_dir, "exit_gate.png"))

    print("\n--- Generating Vehicle Sprites ---")
    vehicles = [
        # Player (2 tiles)
        VehicleSpec("player_red", 2, "#dc2626", "#991b1b", "#b91c1c", "#0f172a", "sports"),
        
        # 2-tile Obstacle Cars
        VehicleSpec("car_sedan_blue", 2, "#2563eb", "#1d4ed8", "#1e40af", "#0f172a", "sedan"),
        VehicleSpec("car_taxi_yellow", 2, "#eab308", "#ca8a04", "#a16207", "#0f172a", "taxi"),
        VehicleSpec("car_hatchback_green", 2, "#16a34a", "#15803d", "#166534", "#0f172a", "sedan"),
        VehicleSpec("car_police", 2, "#18181b", "#ffffff", "#27272a", "#0f172a", "police"),
        
        # 3-tile Obstacle Vehicles
        VehicleSpec("truck_delivery", 3, "#f97316", "#c2410c", "#ea580c", "#0f172a", "van"),
        VehicleSpec("limo_white", 3, "#f8fafc", "#e2e8f0", "#cbd5e1", "#020617", "limo"),
        VehicleSpec("ambulance", 3, "#ffffff", "#ef4444", "#f1f5f9", "#0f172a", "ambulance"),
        
        # 4-tile Obstacle Vehicles
        VehicleSpec("semi_truck", 4, "#0284c7", "#3b82f6", "#0369a1", "#0f172a", "semi"),
        VehicleSpec("bus_transit", 4, "#8b5cf6", "#7c3aed", "#6d28d9", "#0f172a", "bus"),
    ]

    for v in vehicles:
        draw_vehicle(v, 'h', os.path.join(veh_dir, f"{v.name}_h.png"))
        draw_vehicle(v, 'v', os.path.join(veh_dir, f"{v.name}_v.png"))

    print("\nAll assets generated successfully!")

if __name__ == "__main__":
    main()
