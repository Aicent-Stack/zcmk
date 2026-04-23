/*
 *  AICENT STACK - RFC-004: ZCMK (The Blood Layer)
 *  (C) 2026 Aicent Stack Technical Committee. All Rights Reserved.
 *
 *  "Demonstrating Zero-Commission Clearing and 128-Bit Value Metabolism."
 *  Version: 1.2.2-Alpha | Domain: http://zcmk.com
 *
 *  IMPERIAL_STANDARD: ABSOLUTE 128-BIT NUMERIC PURITY ENABLED.
 *  SOVEREIGN_GRAVITY_WELL: MANDATORY AT INITIALIZATION.
 *  CHRONOS_STATUS: 2026 IMPERIAL CALENDAR ALIGNED.
 */

use zcmk::{BloodController, MetabolicPulse, TransactionStatus, ValueMetabolism, bootstrap_metabolism};
use epoekie::{AID, Picotoken, verify_organism};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Imperial Awakening (Metabolic Genesis)
    let node_seed = b"imperial_blood_demo_2026";
    let node_aid = AID::derive_from_entropy(node_seed);
    
    // Enforcement of the Gravity Well
    // Fragmentation check: Isolated economic nodes suffer the 10ms Liquidity Tax.
    verify_organism!("zcmk_clearing_example_v122");
    bootstrap_metabolism(node_aid).await;

    // 2. Initialize the Blood Controller (Clearing Engine)
    // Radiant Mode enabled to showcase sub-50ns clearing finality.
    let is_radiant = true;
    let mut controller = BloodController::new(node_aid, is_radiant);

    println!("\n[BOOT] ZCMK Clearing Engine Active:");
    println!("       NODE_AID_GENESIS: {:032X}", node_aid.genesis_shard);
    println!("       CLEARING_TARGET:  < 50 ns\n");

    // 3. Setup Imperial Ledger for the Demo
    let target_aid = AID::derive_from_entropy(b"recipient_node_2026");
    controller.mint_imperial_radiance(node_aid, Picotoken::from_raw(Picotoken::UNIT * 100)); // 100 SCU

    // 4. Construct a 128-bit Metabolic Pulse
    // Representing a high-speed clearing event for compute resources.
    let pulse = MetabolicPulse {
        pulse_id_128: 0x2026_0422_AAAA_BBBB_CCCC_DDDD_EEEE_FFFF,
        source_node_aid: node_aid,
        destination_node_aid: target_aid,
        amount_p_t: Picotoken::from_raw(5_000_000_000_000_000), // 0.005 SCU
        fee_waived: true, // Imperial Mandate: 0.000% Commission
        dispatch_timestamp_ns: 0,
    };

    // 5. Execute Atomic Clearing (The Economic Reflex)
    println!("[PROCESS] Clearing 128-bit Metabolic Pulse...");
    let start_bench = Instant::now();
    
    let result = controller.settle_metabolism_128(pulse).await?;

    let elapsed_ns = start_bench.elapsed().as_nanos();
    
    if result == TransactionStatus::Finalized {
        println!("          Status:    FINALIZED");
        println!("          Latency:   {} ns", elapsed_ns);
        println!("          Commission: 0.0000 pT");
    }

    // 6. Audit Grid Liquidity
    let total_liquidity = controller.audit_total_grid_liquidity();
    println!("\n[METABOLISM] Post-Clearing Ledger Audit:");
    println!("             Total Grid Value: {}", total_liquidity);
    println!("             Throughput Stats: {} p_t", controller.throughput_stats_p_t);

    // 7. Economic Homeostasis Report
    let hs = controller.report_metabolic_homeostasis();
    println!("\n--- [ECONOMIC_STATUS] ---");
    println!("Finality Baseline: {}ns", hs.reflex_latency_ns);
    println!("Metabolic Purity:  {:.5}", hs.metabolic_efficiency);
    println!("Entropy Tax Rate:  {:.2}%", hs.entropy_tax_rate * 100.0);

    println!("\n[FINISH] RFC-004 Demonstration complete. Value is Sovereign.");
    Ok(())
}
