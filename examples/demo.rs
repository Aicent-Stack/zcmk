//! ZCMK Blood Demo
//! Zero-commission DePIN computing power value transfer demonstration

use zcmk::circulatory::{ComputeNode, Market};

fn main() {
    println!("🩸 ZCMK - The Blood of Aicent Stack");
    println!("   Global idle compute → living value flow.");

    let mut market = Market::new();
    
    let node = ComputeNode {
        id: "edge-882".to_string(),
        available_gflops: 4200,
        price_per_million: 0.0008,
    };

    market.register_node(node);
    let cleared = market.run_auction(5000); // Requirement: 5000 GFLOPS

    println!("🏪 Auction completed:");
    println!("   Matched nodes: {}", cleared.len());
    println!("   Total value settled: ${:.4}", cleared.iter().map(|n| n.price_per_million).sum::<f64>());

    println!("\n✅ ZCMK circulatory system flowing - Compute blood is alive.");
}
