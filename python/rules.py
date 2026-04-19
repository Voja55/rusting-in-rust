import numpy as np
from grid import CLEAN, SURFACE_RUST, HEAVY_RUST, ROTTEN

def compute_next_state(rust, hum, oxy):
    height, width = rust.shape
    new_rust = rust.copy()
    spread = hum * oxy

    surface_mask = rust == SURFACE_RUST
    roll = np.random.random((height, width))
    new_rust[surface_mask & (roll < spread * 0.3)] = HEAVY_RUST

    heavy_mask = rust == HEAVY_RUST
    roll = np.random.random((height, width))
    new_rust[heavy_mask & (roll < spread * 0.4)] = ROTTEN

    neigh_effect = np.zeros((height, width), dtype=np.float32)

    for dy in [-1, 0, 1]:
        for dx in [-1, 0, 1]:
            if dy == 0 and dx == 0:
                continue
            shifted = np.roll(rust, shift=dy, axis=0)
            shifted = np.roll(shifted, shift=dx, axis=1)

            pressure = np.where(shifted == SURFACE_RUST, 0.2,
                       np.where(shifted == HEAVY_RUST,   0.5,
                       np.where(shifted == ROTTEN,       0.8, 0.0)))

            neigh_effect = np.maximum(neigh_effect, pressure)

    clean_mask = rust == CLEAN
    spread     = neigh_effect * spread
    roll       = np.random.random((height, width))
    new_rust[clean_mask & (roll < spread)] = SURFACE_RUST

    return new_rust