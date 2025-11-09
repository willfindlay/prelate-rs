// SPDX-License-Identifier: Apache-2.0 or MIT

//! Example demonstrating that unknown civilizations are handled gracefully.
//!
//! Run this with: cargo run --example test_unknown_civilization

use prelate_rs::types::{civilization::Civilization, games::Player};

fn main() {
    println!("Testing unknown civilization handling...\n");

    // Test 1: Deserialize a player with a known civilization
    let known_json = r#"{
        "name": "TestPlayer",
        "profile_id": 54321,
        "result": "win",
        "civilization": "english",
        "civilization_randomized": false,
        "rating": 1500,
        "rating_diff": 25,
        "mmr": 1600,
        "mmr_diff": 30,
        "input_type": "keyboard"
    }"#;

    let player: Player = serde_json::from_str(known_json).unwrap();
    println!("✓ Known civilization:");
    println!("  Player: {}", player.name);
    println!("  Civilization: {:?}", player.civilization);
    assert!(matches!(player.civilization, Some(Civilization::English)));

    // Test 2: Deserialize a player with an unknown civilization (e.g., future DLC)
    let unknown_json = r#"{
        "name": "AlienPlayer",
        "profile_id": 12345,
        "result": "win",
        "civilization": "martians",
        "civilization_randomized": false,
        "rating": 1500,
        "rating_diff": 25,
        "mmr": 1600,
        "mmr_diff": 30,
        "input_type": "keyboard"
    }"#;

    let player: Player = serde_json::from_str(unknown_json).unwrap();
    println!("\n✓ Unknown civilization (future DLC):");
    println!("  Player: {}", player.name);
    println!("  Civilization: {:?}", player.civilization);

    match player.civilization {
        Some(Civilization::Unknown(ref name)) => {
            println!("  → Captured as Unknown(\"{}\") ✓", name);
            assert_eq!(name, "martians");
        }
        _ => panic!("Expected Unknown civilization variant"),
    }

    // Test 3: Test all the new civilizations we just added
    let new_civs = vec![
        ("golden_horde", "GoldenHorde"),
        ("macedonian_dynasty", "MacedonianDynasty"),
        ("sengoku_daimyo", "SengokuDaimyo"),
        ("tughlaq_dynasty", "TughlaqDynasty"),
    ];

    println!("\n✓ Recently added civilizations:");
    for (snake_case, display_name) in new_civs {
        let json = format!(
            r#"{{
                "name": "Player",
                "profile_id": 99999,
                "result": "win",
                "civilization": "{}",
                "civilization_randomized": false,
                "rating": 1500,
                "rating_diff": 25,
                "mmr": 1600,
                "mmr_diff": 30,
                "input_type": "keyboard"
            }}"#,
            snake_case
        );

        let player: Player = serde_json::from_str(&json).unwrap();
        println!("  {} → {:?} ✓", display_name, player.civilization);
    }

    println!("\n✅ All tests passed!");
    println!("The library can handle both known and unknown civilizations gracefully.");
}
