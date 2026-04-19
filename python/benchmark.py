import time
import csv
import os
import multiprocessing as mp
import numpy as np
import grid
import seq_sim
import par_sim

def run_benchmark(sizes, steps, runs_per_config, output_path):
    os.makedirs(os.path.dirname(output_path), exist_ok=True)

    with open(output_path, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['mode', 'width', 'height', 'steps', 'run', 'duration_ms'])

        for width, height in sizes:
            for mode in ['sequential', 'parallel']:
                n_workers = mp.cpu_count() if mode == 'parallel' else 1
                print(f'Benchmarking {mode} on {width}x{height} ({runs_per_config} runs)...')

                for run in range(runs_per_config):
                    rust, hum, oxy = grid.new_grid(width, height, 0.7, 0.7)

                    start = time.perf_counter()
                    if mode == 'sequential':
                        seq_sim.step_n(rust, hum, oxy, steps)
                    else:
                        par_sim.step_n(rust, hum, oxy, steps, n_workers)
                    elapsed_ms = (time.perf_counter() - start) * 1000

                    writer.writerow([mode, width, height, steps, run, f'{elapsed_ms:.3f}'])
                    print(f'  run {run+1}/{runs_per_config}: {elapsed_ms:.1f}ms')

                f.flush()

    print(f'\nBenchmark complete. Results saved to: {output_path}')