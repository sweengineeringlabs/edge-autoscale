# Architecture — edge-autoscale

## Sequence

> `BenchRunner` drives concurrency ramp-up, collecting `StepResult` at each level; `KneeDetector` finds the saturation inflection point; `ThresholdAdvisor` translates it into HPA thresholds.

```mermaid
sequenceDiagram
    participant App
    participant BenchFacade
    participant BenchRunner
    participant Runner
    participant BenchHandler
    participant KneeDetector
    participant ThresholdAdvisor

    App->>BenchFacade: create_bench_runner(BenchConfig)
    BenchFacade-->>App: BenchRunner

    App->>BenchRunner: run(Arc<dyn BenchHandler>)

    loop for each concurrency step [1, 2, 4, 8, …, max_concurrency]
        BenchRunner->>Runner: run_step(concurrency)
        Runner->>BenchHandler: execute() × concurrency (parallel)
        BenchHandler-->>Runner: latency sample per call
        Runner-->>BenchRunner: StepResult{p50, p99, throughput, error_rate}
    end

    BenchRunner->>KneeDetector: detect(steps)
    KneeDetector-->>BenchRunner: Option<knee_index>

    BenchRunner->>ThresholdAdvisor: advise(steps, knee_index, safety_margin_pct)
    ThresholdAdvisor-->>BenchRunner: AutoscalePolicy{cpu_threshold, memory_threshold}

    BenchRunner-->>App: Result<Report, BenchError>
```

## Data Flow

> `BenchConfig` + a `BenchHandler` (the service under test) enter; a `Report` carrying HPA-ready thresholds exits.

```mermaid
flowchart LR
    A["BenchConfig\n───────────\nsteps: Vec<usize>\nduration_per_step_secs\ntarget_url: Url\nsafety_margin_pct: u8"] --> B["BenchRunner::run\n(Arc<dyn BenchHandler>)"]
    C["BenchHandler\n::execute()\n(service under test)"] --> B

    B --> D["Runner::run_step\n(concurrency: usize)"]
    D --> E["StepResult\n───────────\nconcurrency: usize\np50_ms, p99_ms\nthroughput_rps\nerror_rate_pct"]

    E --> F["Vec<StepResult>\n(one per step)"]
    F --> G["KneeDetector\n::detect(steps)"]
    G -->|found| H["knee_index: usize\n(saturation inflection)"]
    G -->|not found| I["fallback_policy()"]

    H --> J["ThresholdAdvisor\n::advise(steps, knee_idx,\nsafety_margin_pct)"]
    J --> K["AutoscalePolicy\n───────────\nmax_concurrency: usize\nrecommended_replicas: u32\nthreshold_pct: u8"]

    K --> L["Report\n───────────\nsteps: Vec<StepResult>\nknee_index: Option<usize>\npolicy: AutoscalePolicy"]
```
