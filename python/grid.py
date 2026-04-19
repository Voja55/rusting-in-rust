import numpy as np
import json
import os

CLEAN        = 0
SURFACE_RUST = 1
HEAVY_RUST   = 2
ROTTEN       = 3


STATE_CHAR = {
    CLEAN:        '0',
    SURFACE_RUST: '1',
    HEAVY_RUST:   '2',
    ROTTEN:       '3'
}

STATE_STR = {
    CLEAN:        'clean',
    SURFACE_RUST: 'surface',
    HEAVY_RUST:   'heavy',
    ROTTEN:       'rotten'
}

def new_grid(width, height, humidity=0.5, oxygen=0.5, rust_seed=0.1):
    rust = np.zeros((height, width), dtype=np.int32)
    hum  = np.full((height, width), humidity, dtype=np.float32)
    oxy  = np.full((height, width), oxygen,   dtype=np.float32)

    mask = np.random.random((height, width)) < rust_seed
    rust[mask] = SURFACE_RUST

    return rust, hum, oxy

def new_grid_random(width, height, rust_seed=0.1):
    rust = np.zeros((height, width), dtype=np.int32)
    hum  = np.random.random((height, width)).astype(np.float32)
    oxy  = np.random.random((height, width)).astype(np.float32)

    mask = np.random.random((height, width)) < rust_seed
    rust[mask] = SURFACE_RUST

    return rust, hum, oxy

def display(rust):
    for row in rust:
        print(' '.join(STATE_CHAR[c] for c in row))


def save_step(rust, hum, oxy, step, output_dir):
    os.makedirs(output_dir, exist_ok=True)
    data = {
        'step':     step,
        'width':    rust.shape[1],
        'height':   rust.shape[0],
        'grid':     [[STATE_STR[c] for c in row] for row in rust.tolist()],
        'humidity': hum.tolist(),
        'oxygen':   oxy.tolist(),
    }
    path = os.path.join(output_dir, f'step_{step:04d}.json')
    with open(path, 'w') as f:
        json.dump(data, f, indent=2)

def save_summary(total_steps, width, height, output_dir):
    data = {
        'total_steps': total_steps,
        'width':       width,
        'height':      height,
        'output_dir':  output_dir,
    }
    path = os.path.join(output_dir, 'summary.json')
    with open(path, 'w') as f:
        json.dump(data, f, indent=2)