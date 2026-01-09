// Benchmark consensus performance with varying configurations
//
// These benchmarks measure:
// - Consensus speed with 3 agents (standard tripartite)
// - Consensus speed with 10 agents (extended council)
// - Multi-round consensus performance
// - Voting aggregation overhead

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tripartite::{Agent, AgentInput, AgentOutput, ConsensusEngine, ConsensusConfig};
use async_trait::async_trait;
use std::time::Duration;

// Mock agent for benchmarking
struct MockAgent {
    name: String,
    latency_ms: u64,
}

#[async_trait]
impl Agent for MockAgent {
    fn name(&self) -> &str {
        &self.name
    }

    fn role(&self) -> &str {
        "Mock agent for benchmarking"
    }

    async fn process(&self, _input: AgentInput) -> tripartite::Result<AgentOutput> {
        // Simulate processing latency
        if self.latency_ms > 0 {
            tokio::time::sleep(Duration::from_millis(self.latency_ms)).await;
        }

        Ok(AgentOutput::new(
            &self.name,
            format!("Response from {}", self.name),
            0.9,
        ))
    }

    fn is_ready(&self) -> bool {
        true
    }

    fn model(&self) -> &str {
        "mock-model"
    }
}

fn create_mock_agents(count: usize, latency_ms: u64) -> Vec<std::sync::Arc<dyn Agent>> {
    (0..count)
        .map(|i| {
            std::sync::Arc::new(MockAgent {
                name: format!("agent_{}", i),
                latency_ms,
            }) as std::sync::Arc<dyn Agent>
        })
        .collect()
}

fn bench_consensus_three_agents(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("consensus/3_agents");

    // Benchmark with zero latency
    group.bench_function("zero_latency", |b| {
        b.to_async(&rt).iter(|| async {
            let agents = create_mock_agents(3, 0);
            let config = ConsensusConfig::default();
            let engine = ConsensusEngine::new(agents, config);

            let input = AgentInput::new(tripartite::A2AManifest::new("Test query".to_string()));

            // Run consensus
            let _outcome = engine.reach_consensus(input).await;
        });
    });

    // Benchmark with 1ms latency per agent
    group.bench_function("1ms_latency", |b| {
        b.to_async(&rt).iter(|| async {
            let agents = create_mock_agents(3, 1);
            let config = ConsensusConfig::default();
            let engine = ConsensusEngine::new(agents, config);

            let input = AgentInput::new(tripartite::A2AManifest::new("Test query".to_string()));

            let _outcome = engine.reach_consensus(input).await;
        });
    });

    group.finish();
}

fn bench_consensus_many_agents(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("consensus/many_agents");

    for agent_count in [3, 5, 7, 10].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(agent_count),
            agent_count,
            |b, &count| {
                b.to_async(&rt).iter(|| async {
                    let agents = create_mock_agents(count, 0);
                    let config = ConsensusConfig::default();
                    let engine = ConsensusEngine::new(agents, config);

                    let input =
                        AgentInput::new(tripartite::A2AManifest::new("Test query".to_string()));

                    let _outcome = engine.reach_consensus(input).await;
                });
            },
        );
    }

    group.finish();
}

fn bench_consensus_multi_round(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("consensus/multi_round");

    // Benchmark with different thresholds (affects number of rounds)
    for threshold in [0.5, 0.7, 0.85, 0.95].iter() {
        group.bench_with_input(
            BenchmarkId::new("threshold", threshold),
            threshold,
            |b, &threshold| {
                b.to_async(&rt).iter(|| async {
                    let agents = create_mock_agents(3, 0);
                    let config = ConsensusConfig {
                        threshold,
                        max_rounds: 5,
                        ..Default::default()
                    };
                    let engine = ConsensusEngine::new(agents, config);

                    let input =
                        AgentInput::new(tripartite::A2AManifest::new("Test query".to_string()));

                    let _outcome = engine.reach_consensus(input).await;
                });
            },
        );
    }

    group.finish();
}

fn bench_voting_aggregation(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    c.bench_function("consensus/voting_aggregation", |b| {
        b.to_async(&rt).iter(|| async {
            let agents = create_mock_agents(10, 0);
            let config = ConsensusConfig::default();
            let engine = ConsensusEngine::new(agents, config);

            let input = AgentInput::new(tripartite::A2AManifest::new("Test query".to_string()));

            // Just measure the voting aggregation (assuming instant agent responses)
            let _outcome = engine.reach_consensus(input).await;
        });
    });
}

criterion_group!(
    benches,
    bench_consensus_three_agents,
    bench_consensus_many_agents,
    bench_consensus_multi_round,
    bench_voting_aggregation
);
criterion_main!(benches);
