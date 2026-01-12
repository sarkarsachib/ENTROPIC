use entropic_world_core::world::World;
use entropic_world_core::population::{NPC, Faction, Alignment, Relationship};

#[test]
fn test_npc_creation() {
    let mut world = World::new(
        "NPC Test".to_string(),
        "game_dna_1".to_string(),
        10,
        10,
    );

    let npc = NPC::new(
        "npc_1".to_string(),
        "Alice".to_string(),
        "entity_1".to_string(),
    );

    world.add_npc(npc);
    assert_eq!(world.total_npcs(), 1);
}

#[test]
fn test_npc_skills() {
    let mut npc = NPC::new(
        "npc_1".to_string(),
        "Bob".to_string(),
        "entity_1".to_string(),
    );

    npc.add_skill("archery".to_string(), 0.7);
    assert_eq!(npc.get_skill("archery"), 0.7);

    npc.improve_skill("archery", 0.2);
    assert_eq!(npc.get_skill("archery"), 0.9);

    npc.improve_skill("archery", 0.5);
    assert_eq!(npc.get_skill("archery"), 1.0);
}

#[test]
fn test_faction_creation() {
    let mut world = World::new(
        "Faction Test".to_string(),
        "game_dna_1".to_string(),
        10,
        10,
    );

    let faction = Faction::new(
        "faction_1".to_string(),
        "The Guild".to_string(),
        "npc_1".to_string(),
    );

    world.add_faction(faction);
    assert_eq!(world.factions.len(), 1);
}

#[test]
fn test_faction_members() {
    let mut faction = Faction::new(
        "faction_1".to_string(),
        "The Guild".to_string(),
        "npc_leader".to_string(),
    );

    faction.add_member("npc_1".to_string());
    faction.add_member("npc_2".to_string());
    assert_eq!(faction.member_count(), 2);

    faction.remove_member(&"npc_1".to_string());
    assert_eq!(faction.member_count(), 1);
}

#[test]
fn test_faction_relations() {
    let mut faction = Faction::new(
        "faction_1".to_string(),
        "The Guild".to_string(),
        "npc_leader".to_string(),
    );

    faction.add_ally("faction_2".to_string());
    faction.add_enemy("faction_3".to_string());

    assert!(faction.is_allied_with(&"faction_2".to_string()));
    assert!(faction.is_enemy_of(&"faction_3".to_string()));
}

#[test]
fn test_npc_relationships() {
    let mut npc = NPC::new(
        "npc_1".to_string(),
        "Alice".to_string(),
        "entity_1".to_string(),
    );

    let mut relationship = Relationship::new("npc_2".to_string());
    relationship.adjust_opinion(0.5);
    relationship.adjust_trust(0.7);

    npc.add_relationship(relationship);

    let rel = npc.get_relationship(&"npc_2".to_string()).unwrap();
    assert_eq!(rel.opinion, 0.5);
    assert_eq!(rel.trust, 0.7);
    assert!(rel.is_friendly());
}

#[test]
fn test_relationship_status() {
    let mut rel = Relationship::new("npc_2".to_string());

    rel.adjust_opinion(0.5);
    assert!(rel.is_friendly());

    rel.adjust_opinion(-1.0);
    assert!(rel.is_hostile());

    rel.adjust_opinion(0.5);
    assert!(rel.is_neutral());
}
