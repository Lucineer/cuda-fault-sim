# cuda-fault-sim

GPU-accelerated fault simulation for digital design verification.

## Fault Types
- Stuck-at-0 / Stuck-at-1
- Bridging (adjacent net shorts)
- Delay faults (timing violations)
- Open circuits

## CUDA Parallelism
Each fault injection trial is independent → massive parallelism.
Expected: 1000x speedup for scan test coverage analysis.