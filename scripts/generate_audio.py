#!/usr/bin/env python3
"""
Procedural Audio Synthesizer for Parking Escape
Generates clean 16-bit PCM WAV sound effects:
- click.wav (UI click)
- slide.wav (car sliding friction sound)
- bump.wav (solid bumper collision)
- win.wav (triumphant victory chime)
- exit_drive.wav (car acceleration drive-off sound)
"""

import math
import os
import struct
import wave

SAMPLE_RATE = 44100

def write_wav(filename, samples):
    os.makedirs(os.path.dirname(filename), exist_ok=True)
    with wave.open(filename, 'w') as wav:
        wav.setnchannels(1)  # mono
        wav.setsampwidth(2)  # 16-bit
        wav.setframerate(SAMPLE_RATE)
        
        # Clamp and pack
        packed = bytearray()
        for s in samples:
            s_clamped = max(-1.0, min(1.0, s))
            val = int(s_clamped * 32767.0)
            packed.extend(struct.pack('<h', val))
        wav.writeframes(packed)
    print(f"Generated audio: {filename}")

def gen_click():
    # 35ms crisp woody click
    duration = 0.04
    n = int(SAMPLE_RATE * duration)
    samples = []
    for i in range(n):
        t = i / SAMPLE_RATE
        env = math.exp(-t * 90.0)
        freq = 1200.0 * math.exp(-t * 50.0)
        s = math.sin(2.0 * math.pi * freq * t) * env * 0.7
        samples.append(s)
    return samples

def gen_slide():
    # 180ms gentle tire friction glide
    duration = 0.18
    n = int(SAMPLE_RATE * duration)
    samples = []
    import random
    random.seed(123)
    for i in range(n):
        t = i / SAMPLE_RATE
        # attack-decay envelope
        env = math.sin(math.pi * (t / duration)) ** 1.5
        noise = (random.random() * 2.0 - 1.0) * 0.35
        low_hum = math.sin(2.0 * math.pi * 140.0 * t) * 0.4
        mid_hum = math.sin(2.0 * math.pi * 280.0 * t) * 0.25
        samples.append((noise + low_hum + mid_hum) * env * 0.6)
    return samples

def gen_bump():
    # 120ms thud with rapid low-frequency pitch drop
    duration = 0.12
    n = int(SAMPLE_RATE * duration)
    samples = []
    for i in range(n):
        t = i / SAMPLE_RATE
        env = math.exp(-t * 35.0)
        freq = 160.0 * math.exp(-t * 30.0) + 45.0
        s = math.sin(2.0 * math.pi * freq * t) * env * 0.85
        samples.append(s)
    return samples

def gen_win():
    # 600ms rich victory arpeggio (C5 -> E5 -> G5 -> C6)
    notes = [523.25, 659.25, 783.99, 1046.50]
    total_dur = 0.75
    n = int(SAMPLE_RATE * total_dur)
    samples = [0.0] * n
    
    note_dur = 0.15
    for idx, f in enumerate(notes):
        start_t = idx * 0.12
        start_idx = int(start_t * SAMPLE_RATE)
        dur = total_dur - start_t
        note_len = int(dur * SAMPLE_RATE)
        
        for i in range(note_len):
            if start_idx + i >= n:
                break
            t = i / SAMPLE_RATE
            env = math.exp(-t * 5.0)
            # Fund + harmonic
            s = (math.sin(2.0 * math.pi * f * t) * 0.6 + 
                 math.sin(2.0 * math.pi * f * 2.0 * t) * 0.3 +
                 math.sin(2.0 * math.pi * f * 3.0 * t) * 0.1) * env * 0.4
            samples[start_idx + i] += s
    return samples

def gen_exit_drive():
    # 800ms engine rev acceleration
    duration = 0.8
    n = int(SAMPLE_RATE * duration)
    samples = []
    for i in range(n):
        t = i / SAMPLE_RATE
        env = min(1.0, t * 5.0) * (1.0 - (t / duration) ** 2)
        # Rising pitch 100Hz -> 380Hz
        freq = 100.0 + 280.0 * (t / duration) ** 1.8
        # Engine cylinder pulses
        s1 = math.sin(2.0 * math.pi * freq * t) * 0.5
        s2 = math.sin(2.0 * math.pi * (freq * 2.0) * t) * 0.3
        s3 = math.sin(2.0 * math.pi * (freq * 0.5) * t) * 0.2
        samples.append((s1 + s2 + s3) * env * 0.75)
    return samples

def main():
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    audio_dir = os.path.join(base_dir, "assets", "audio")
    
    write_wav(os.path.join(audio_dir, "click.wav"), gen_click())
    write_wav(os.path.join(audio_dir, "slide.wav"), gen_slide())
    write_wav(os.path.join(audio_dir, "bump.wav"), gen_bump())
    write_wav(os.path.join(audio_dir, "win.wav"), gen_win())
    write_wav(os.path.join(audio_dir, "exit_drive.wav"), gen_exit_drive())
    print("All audio files generated successfully!")

if __name__ == "__main__":
    main()
