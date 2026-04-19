import argparse
import multiprocessing as mp
import grid as g
import seq_sim
import par_sim
import benchmark
import scaling
import atexit

def parse_args():
    parser = argparse.ArgumentParser(
        description='Rusting in Rust — Python simulation'
    )
    parser.add_argument('-W', '--width',          type=int,   default=40)
    parser.add_argument('-H', '--height',         type=int,   default=20)
    parser.add_argument('-s', '--steps',          type=int,   default=50)
    parser.add_argument('-d', '--delay',          type=float, default=0.1)
    parser.add_argument('-o', '--output',         type=str,   default='output/python_seq')
    parser.add_argument('--humidity',             type=float, default=0.5)
    parser.add_argument('--oxygen',               type=float, default=0.5)
    parser.add_argument('-r', '--random',         action='store_true')
    parser.add_argument('-p', '--parallel',       action='store_true')
    parser.add_argument('-w', '--workers',        type=int,   default=None)
    parser.add_argument('-b', '--benchmark',      action='store_true')
    parser.add_argument('--bench-runs',           type=int,   default=10)
    parser.add_argument('--scaling',              action='store_true')
    parser.add_argument('--max-workers',          type=int,   default=mp.cpu_count())
    parser.add_argument('--scaling-runs',         type=int,   default=10)
    return parser.parse_args()

def main():
    args = parse_args()

    atexit.register(par_sim.shutdown)

    if args.scaling:
        print('Python scaling experiments\n')
        scaling.run_strong_scaling(
            400, 200,
            args.steps,
            args.max_workers,
            args.scaling_runs,
            'output/scaling/strong_scaling_python.csv',
        )
        scaling.run_weak_scaling(
            100, 50,
            args.steps,
            args.max_workers,
            args.scaling_runs,
            'output/scaling/weak_scaling_python.csv',
        )
        return

    if args.benchmark:
        print('Python benchmark\n')
        sizes = [(50, 25), (100, 50), (200, 100), (400, 200), (600, 300)]
        benchmark.run_benchmark(
            sizes,
            args.steps,
            args.bench_runs,
            'output/benchmark/results_python.csv',
        )
        return

    if args.random:
        rust, hum, oxy = g.new_grid_random(args.width, args.height)
    else:
        rust, hum, oxy = g.new_grid(args.width, args.height, args.humidity, args.oxygen)

    if args.parallel:
        workers = args.workers or mp.cpu_count()
        output  = args.output if args.output != 'output/python_seq' else 'output/python_par'
        print(f'Python parallel ({workers} workers)')
        print(f'Grid: {args.width}x{args.height}  Steps: {args.steps}\n')
        par_sim.run(rust, hum, oxy, args.steps, workers, args.delay, output)
    else:
        print(f'Python sequential')
        print(f'Grid: {args.width}x{args.height}  Steps: {args.steps}\n')
        seq_sim.run(rust, hum, oxy, args.steps, args.delay, args.output)

if __name__ == '__main__':
    main()