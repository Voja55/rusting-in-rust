import time
import grid
from rules import compute_next_state

def clear():
    print('\033[2J\033[1;1H', end='')

def run(rust, hum, oxy, steps, delay_s=0.1, output_dir='output/python_seq'):
    for i in range(steps):
        clear()
        print(f'Step {i+1}/{steps} [python sequential]')
        grid.display(rust)

        grid.save_step(rust, hum, oxy, i, output_dir)
        rust = compute_next_state(rust, hum, oxy)

        time.sleep(delay_s)

    grid.save_step(rust, hum, oxy, steps, output_dir)
    grid.save_summary(steps, rust.shape[1], rust.shape[0], output_dir)

    clear()
    print(f'Step {steps}/{steps} — sequential complete')
    print(f'Output saved to: {output_dir}\n')
    grid.display(rust)

    return rust

def step_n(rust, hum, oxy, steps):
    for _ in range(steps):
        rust = compute_next_state(rust, hum, oxy)
    return rust