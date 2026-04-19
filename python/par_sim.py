import time
import numpy as np
import multiprocessing as mp
import grid

_pool = None

def get_pool(n_workers):
    global _pool
    if _pool is None:
        _pool = mp.Pool(processes=n_workers)
    return _pool

def _process_chunk(args):
    row_start, row_end, rust_chunk, hum_chunk, oxy_chunk = args
    from rules import compute_next_state
    return compute_next_state(rust_chunk, hum_chunk, oxy_chunk)

def step_parallel(rust, hum, oxy, n_workers):
    height = rust.shape[0]
    chunk_size = max(1, height // n_workers)

    chunks = []
    for i in range(n_workers):
        row_start = i * chunk_size
        row_end   = height if i == n_workers - 1 else (i + 1) * chunk_size
        chunks.append((
            row_start, row_end,
            rust[row_start:row_end].copy(),
            hum[row_start:row_end].copy(),
            oxy[row_start:row_end].copy(),
        ))

    pool = get_pool(n_workers)
    results = pool.map(_process_chunk, chunks)
    # with mp.Pool(processes=n_workers) as pool:
    #     results = pool.map(_process_chunk, chunks)

    return np.vstack(results)

def run(rust, hum, oxy, steps, n_workers=None, delay_s=0.1, output_dir='output/python_par'):
    if n_workers is None:
        n_workers = mp.cpu_count()

    def clear():
        print('\033[2J\033[1;1H', end='')

    for i in range(steps):
        clear()
        print(f'Step {i+1}/{steps} [python parallel — {n_workers} workers]')
        print('  . = Clean  # = Surface rust  @ = Heavy rust  X = Rotten\n')
        grid.display(rust)

        grid.save_step(rust, hum, oxy, i, output_dir)
        rust = step_parallel(rust, hum, oxy, n_workers)

        time.sleep(delay_s)

    grid.save_step(rust, hum, oxy, steps, output_dir)
    grid.save_summary(steps, rust.shape[1], rust.shape[0], output_dir)

    clear()
    print(f'Step {steps}/{steps} — parallel complete')
    print(f'Output saved to: {output_dir}\n')
    grid.display(rust)

    return rust

def step_n(rust, hum, oxy, steps, n_workers=None):
    if n_workers is None:
        n_workers = mp.cpu_count()
    for _ in range(steps):
        rust = step_parallel(rust, hum, oxy, n_workers)
    return rust

def shutdown():
    global _pool
    if _pool is not None:
        _pool.close()
        _pool.join()
        _pool = None