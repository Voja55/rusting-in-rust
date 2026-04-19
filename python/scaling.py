import time
import csv
import os
import multiprocessing as mp
import grid
import seq_sim
import par_sim

def run_strong_scaling(width, height, steps, max_workers, runs, output_path):
    os.makedirs(os.path.dirname(output_path), exist_ok=True)

    print(f'\nStrong scaling — fixed grid {width}x{height}, {steps} steps')
    print('(same problem size, more workers)\n')

    with open(output_path, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['experiment', 'workers', 'width', 'height', 'steps', 'run', 'duration_ms'])

        #sequential baseline
        for run in range(runs):
            rust, hum, oxy = grid.new_grid(width, height, 0.7, 0.7)
            start = time.perf_counter()
            seq_sim.step_n(rust, hum, oxy, steps)
            ms = (time.perf_counter() - start) * 1000
            writer.writerow(['strong_sequential', 1, width, height, steps, run, f'{ms:.3f}'])
        print(f'  sequential baseline: done ({runs} runs)')

        #parallel with increasing workers
        for workers in range(2, max_workers + 1):
            for run in range(runs):
                rust, hum, oxy = grid.new_grid(width, height, 0.7, 0.7)
                start = time.perf_counter()
                par_sim.step_n(rust, hum, oxy, steps, workers)
                ms = (time.perf_counter() - start) * 1000
                writer.writerow(['strong_parallel', workers, width, height, steps, run, f'{ms:.3f}'])
            print(f'  {workers} workers: done ({runs} runs)')
            f.flush()

    print(f'Strong scaling results saved to: {output_path}')

def run_weak_scaling(base_width, base_height, steps, max_workers, runs, output_path):
    os.makedirs(os.path.dirname(output_path), exist_ok=True)

    print(f'\nWeak scaling — base grid {base_width}x{base_height}, {steps} steps')
    print('(problem grows with workers, constant work per worker)\n')

    with open(output_path, 'w', newline='') as f:
        writer = csv.writer(f)
        writer.writerow(['experiment', 'workers', 'width', 'height', 'steps', 'run', 'duration_ms'])

        #sequential baseline at base size
        for run in range(runs):
            rust, hum, oxy = grid.new_grid(base_width, base_height, 0.7, 0.7)
            start = time.perf_counter()
            seq_sim.step_n(rust, hum, oxy, steps)
            ms = (time.perf_counter() - start) * 1000
            writer.writerow(['weak_sequential', 1, base_width, base_height, steps, run, f'{ms:.3f}'])
        print(f'  1 worker ({base_width}x{base_height}): done')

        for workers in range(2, max_workers + 1):
            width  = base_width
            height = base_height * workers
            for run in range(runs):
                rust, hum, oxy = grid.new_grid(width, height, 0.7, 0.7)
                start = time.perf_counter()
                par_sim.step_n(rust, hum, oxy, steps, workers)
                ms = (time.perf_counter() - start) * 1000
                writer.writerow(['weak_parallel', workers, width, height, steps, run, f'{ms:.3f}'])
            print(f'  {workers} workers ({width}x{height}): done')
            f.flush()

    print(f'Weak scaling results saved to: {output_path}')