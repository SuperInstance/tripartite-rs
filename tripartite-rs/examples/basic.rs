//! Basic consensus example
//!
//! Demonstrates the simplest use of tripartite-rs:
//! - Create 3 simple agents
//! - Run consensus on a query
//! - Check if consensus was reached

use tripartite::{Agent, ConsensusEngine, ConsensusConfig, AgentInput, AgentOutput};
use async_trait::async_trait;
use std::sync::Arc;

/// Simple agent that returns a fixed response with fixed confidence
struct SimpleAgent {
    name: String,
    response: String,
    confidence: f32,
}

#[async_trait]
impl Agent for SimpleAgent {
    async fn process(&self, _input: AgentInput) -> Result<AgentOutput, tripartite::Error> {
        Ok(AgentOutput::new(
            &self.name,
            self.response.clone(),
            self.confidence,
        ))
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn role(&self) -> &str {
        "Simple response agent"
    }

    fn is_ready(&self) -> bool {
        true
    }

    fn model(&self) -> &str {
        "static"
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    println!("=== Basic Consensus Example ===\n");

    // Create three agents with high confidence
    let agents = vec![
        Arc::new(SimpleAgent {
            name: "Agent A".to_string(),
            response: "The answer is 42".to_string(),
            confidence: 0.90,
        }),
        Arc::new(SimpleAgent {
            name: "Agent B".to_string(),
            response: "The answer is 42".to_string(),
            confidence: 0.85,
        }),
        Arc::new(SimpleAgent {
            name: "Agent C".to_string(),
            response: "The answer is 42".to_string(),
            confidence: 0.88,
        }),
    ];

    println!("Created 3 agents:");
    for agent in &agents {
        println!("  - {} (confidence: {:.2})", agent.name(), agent.confidence);
    }
    println!();

    // Create consensus engine with default configuration
    // Default threshold: 0.85
    // Default max rounds: 3
    let engine = ConsensusEngine::new(ConsensusConfig::default(), agents);

    println!("Running consensus with query: \"What is the answer?\"\n");

    // Run consensus
    let outcome = engine.run("What is the answer?").await?;

    // Check results
    println!("=== Results ===");
    println!("Consensus Reached: {}", outcome.is_consensus());
    println!("Content: {}", outcome.content);
    println!("Aggregate Confidence: {:.2}", outcome.aggregate_confidence().unwrap_or(0.0));
    println!("Rounds: {}", outcome.rounds());
    println!("Duration: {}ms", outcome.total_duration_ms);

    // Show individual agent responses
    println!("\n=== Agent Responses ===");
    if let Some(ref pathos) = outcome.agent_0_response {
        println!("Agent A: {:.2} confidence", pathos.confidence);
    }
    if let Some(ref logos) = outcome.agent_1_response {
        println!("Agent B: {:.2} confidence", logos.confidence);
    }
    if let Some(ref ethos) = outcome.agent_2_response {
        println!("Agent C: {:.2} confidence", ethos.confidence);
    }

    Ok(())
}
