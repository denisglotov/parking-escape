#!/usr/bin/env python3
"""
UI Asset Generator for Parking Escape
Renders high-quality, glossy 2.5D UI icons with soft shadows and metallic highlights.
Coordinates are strictly based on 512x512 canvas downscaled to 128x128.
"""

import math
import os
from PIL import Image, ImageDraw, ImageFilter

SIZE = 128
CANVAS_SIZE = 512

def create_shadow(shape_fn, blur_r=24, opacity=130, offset=(0, 12)):
    shadow = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(shadow)
    shape_fn(draw, offset)
    shadow = shadow.filter(ImageFilter.GaussianBlur(blur_r))
    r, g, b, a = shadow.split()
    a = a.point(lambda p: int(p * (opacity / 255.0)))
    shadow.putalpha(a)
    return shadow

def generate_parking_badge(out_path):
    canvas = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    x1, y1 = 48, 48
    x2, y2 = 464, 464
    radius = 72

    def draw_sh(d, off):
        d.rounded_rectangle([x1 + off[0], y1 + off[1], x2 + off[0], y2 + off[1]], radius=radius, fill=(0, 0, 0, 255))
    sh = create_shadow(draw_sh)
    canvas.paste(sh, (0, 0), sh)

    img = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    # Blue shield background
    draw.rounded_rectangle([x1, y1, x2, y2], radius=radius, fill="#2563eb", outline="#1d4ed8", width=12)

    # Bevel highlight on top
    draw.rounded_rectangle([x1 + 10, y1 + 10, x2 - 10, y1 + 160], radius=radius-10, fill=(255, 255, 255, 45))

    # White "P" symbol
    stem_x1, stem_y1 = 150, 120
    stem_w, stem_h = 60, 272
    # Stem
    draw.rounded_rectangle([stem_x1, stem_y1, stem_x1 + stem_w, stem_y1 + stem_h], radius=16, fill="#ffffff")
    # Loop
    loop_w = 170
    loop_h = 160
    draw.rounded_rectangle([stem_x1, stem_y1, stem_x1 + loop_w, stem_y1 + loop_h], radius=48, fill="#ffffff")
    draw.rounded_rectangle([stem_x1 + stem_w, stem_y1 + 36, stem_x1 + loop_w - 36, stem_y1 + loop_h - 36], radius=24, fill="#2563eb")

    canvas.paste(img, (0, 0), img)
    final = canvas.resize((SIZE, SIZE), Image.Resampling.LANCZOS)
    final.save(out_path, "PNG")
    print(f"Rendered: {out_path}")

def generate_star(out_path, is_gold=True):
    canvas = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    cx, cy = 256, 256
    r_outer = 190
    r_inner = 82
    points = 5
    start_angle = -math.pi / 2
    step = math.pi / points

    vertices = []
    for i in range(points * 2):
        r = r_outer if i % 2 == 0 else r_inner
        ang = start_angle + i * step
        vertices.append((cx + math.cos(ang) * r, cy + math.sin(ang) * r))

    def draw_sh(d, off):
        pts = [(x + off[0], y + off[1]) for x, y in vertices]
        d.polygon(pts, fill=(0, 0, 0, 255))
    sh = create_shadow(draw_sh, blur_r=20, offset=(0, 12), opacity=150)
    canvas.paste(sh, (0, 0), sh)

    star_img = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(star_img)

    if is_gold:
        dark_col = "#d97706"
        light_col = "#fde047"
        for i in range(points * 2):
            next_idx = (i + 1) % (points * 2)
            shade = light_col if i % 2 == 0 else dark_col
            draw.polygon([(cx, cy), vertices[i], vertices[next_idx]], fill=shade)
        draw.polygon(vertices, outline="#b45309", width=6)
    else:
        draw.polygon(vertices, fill="#1e293b", outline="#64748b", width=10)

    canvas.paste(star_img, (0, 0), star_img)
    final = canvas.resize((SIZE, SIZE), Image.Resampling.LANCZOS)
    final.save(out_path, "PNG")
    print(f"Rendered: {out_path}")

def generate_undo_icon(out_path):
    canvas = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    cx, cy = 256, 256
    r = 135
    lw = 36

    img = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    start_a = math.pi * 0.2
    end_a = math.pi * 1.8
    segs = 32
    step = (end_a - start_a) / segs
    for i in range(segs):
        a1 = start_a + i * step
        a2 = start_a + (i + 1) * step
        draw.line([
            (cx + math.cos(a1) * r, cy + math.sin(a1) * r),
            (cx + math.cos(a2) * r, cy + math.sin(a2) * r)
        ], fill="#38bdf8", width=lw)

    ax = cx + math.cos(start_a) * r
    ay = cy + math.sin(start_a) * r
    ah = 60
    draw.polygon([
        (ax - ah * 0.9, ay + ah * 0.4),
        (ax + ah * 0.4, ay + ah * 0.9),
        (ax + ah * 0.2, ay - ah * 0.9)
    ], fill="#38bdf8")

    canvas.paste(img, (0, 0), img)
    final = canvas.resize((SIZE, SIZE), Image.Resampling.LANCZOS)
    final.save(out_path, "PNG")
    print(f"Rendered: {out_path}")

def generate_reset_icon(out_path):
    canvas = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    cx, cy = 256, 256
    r = 135
    lw = 36

    img = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    start_a = -math.pi * 0.7
    end_a = math.pi * 0.9
    segs = 32
    step = (end_a - start_a) / segs
    for i in range(segs):
        a1 = start_a + i * step
        a2 = start_a + (i + 1) * step
        draw.line([
            (cx + math.cos(a1) * r, cy + math.sin(a1) * r),
            (cx + math.cos(a2) * r, cy + math.sin(a2) * r)
        ], fill="#f59e0b", width=lw)

    ax = cx + math.cos(end_a) * r
    ay = cy + math.sin(end_a) * r
    ah = 60
    draw.polygon([
        (ax + ah * 0.9, ay - ah * 0.4),
        (ax - ah * 0.4, ay - ah * 0.9),
        (ax - ah * 0.2, ay + ah * 0.9)
    ], fill="#f59e0b")

    canvas.paste(img, (0, 0), img)
    final = canvas.resize((SIZE, SIZE), Image.Resampling.LANCZOS)
    final.save(out_path, "PNG")
    print(f"Rendered: {out_path}")

def generate_back_icon(out_path):
    canvas = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    cx, cy = 256, 256
    hw = 90
    hh = 120
    lw = 40

    img = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    draw.line([(cx + hw * 0.35, cy - hh), (cx - hw * 0.65, cy)], fill="#f1f5f9", width=lw)
    draw.line([(cx - hw * 0.65, cy), (cx + hw * 0.35, cy + hh)], fill="#f1f5f9", width=lw)

    canvas.paste(img, (0, 0), img)
    final = canvas.resize((SIZE, SIZE), Image.Resampling.LANCZOS)
    final.save(out_path, "PNG")
    print(f"Rendered: {out_path}")

def generate_sound_icons(out_on_path, out_off_path):
    for is_on, path in [(True, out_on_path), (False, out_off_path)]:
        canvas = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
        cx, cy = 256, 256
        
        img = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
        draw = ImageDraw.Draw(img)

        sx = cx - 50
        sw = 65
        sh = 95
        draw.rounded_rectangle([sx - sw, cy - sh*0.5, sx, cy + sh*0.5], radius=12, fill="#e2e8f0")
        draw.polygon([(sx, cy - sh*0.65), (sx + 85, cy - sh*1.15), (sx + 85, cy + sh*1.15), (sx, cy + sh*0.65)], fill="#e2e8f0")

        if is_on:
            lw = 24
            for wr in [120, 180]:
                for a_idx in range(-3, 4):
                    a1 = a_idx * 0.18
                    a2 = (a_idx + 1) * 0.18
                    draw.line([(sx + 40 + math.cos(a1)*wr, cy + math.sin(a1)*wr), (sx + 40 + math.cos(a2)*wr, cy + math.sin(a2)*wr)], fill="#f59e0b", width=lw)
        else:
            cross_x = cx + 85
            csz = 60
            lw = 28
            draw.line([(cross_x - csz, cy - csz), (cross_x + csz, cy + csz)], fill="#ef4444", width=lw)
            draw.line([(cross_x - csz, cy + csz), (cross_x + csz, cy - csz)], fill="#ef4444", width=lw)

        canvas.paste(img, (0, 0), img)
        final = canvas.resize((SIZE, SIZE), Image.Resampling.LANCZOS)
        final.save(path, "PNG")
        print(f"Rendered: {path}")

def generate_trophy_icon(out_path):
    canvas = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    cx, cy = 256, 256
    
    img = Image.new("RGBA", (CANVAS_SIZE, CANVAS_SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    gold = "#fbbf24"
    gold_dark = "#d97706"
    gold_light = "#fef08a"

    draw.rounded_rectangle([cx - 95, cy - 140, cx + 95, cy + 10], radius=32, fill=gold, outline=gold_dark, width=8)
    draw.polygon([(cx - 95, cy + 10), (cx + 95, cy + 10), (cx, cy + 80)], fill=gold)

    draw.rounded_rectangle([cx - 150, cy - 120, cx - 75, cy - 20], radius=24, outline=gold, width=20)
    draw.rounded_rectangle([cx + 75, cy - 120, cx + 150, cy - 20], radius=24, outline=gold, width=20)

    draw.rectangle([cx - 20, cy + 70, cx + 20, cy + 120], fill=gold_dark)
    draw.rounded_rectangle([cx - 90, cy + 120, cx + 90, cy + 160], radius=12, fill=gold_dark, outline=gold, width=6)

    draw.line([(cx - 60, cy - 110), (cx - 60, cy - 10)], fill=gold_light, width=12)

    canvas.paste(img, (0, 0), img)
    final = canvas.resize((SIZE, SIZE), Image.Resampling.LANCZOS)
    final.save(out_path, "PNG")
    print(f"Rendered: {out_path}")

def main():
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    ui_dir = os.path.join(base_dir, "assets", "ui")
    os.makedirs(ui_dir, exist_ok=True)

    print("--- Generating UI Assets ---")
    generate_parking_badge(os.path.join(ui_dir, "badge_parking.png"))
    generate_star(os.path.join(ui_dir, "star_gold.png"), is_gold=True)
    generate_star(os.path.join(ui_dir, "star_empty.png"), is_gold=False)
    generate_undo_icon(os.path.join(ui_dir, "icon_undo.png"))
    generate_reset_icon(os.path.join(ui_dir, "icon_reset.png"))
    generate_back_icon(os.path.join(ui_dir, "icon_back.png"))
    generate_sound_icons(os.path.join(ui_dir, "icon_sound_on.png"), os.path.join(ui_dir, "icon_sound_off.png"))
    generate_trophy_icon(os.path.join(ui_dir, "trophy_gold.png"))

    print("All UI assets generated successfully!")

if __name__ == "__main__":
    main()
