#[cfg(test)]

#[allow(unused)]
use std::env::current_dir;
#[allow(unused_imports)]
use std::cmp::Ordering;

#[allow(unused_imports)]
use chrono::prelude::*;
#[allow(unused_imports)]
use chrono::{Utc, Duration};

#[allow(unused_imports)]
use crate::controllers::standings;
#[allow(unused_imports)]
use crate::controllers::discovery as dis;
#[allow(unused_imports)]
use crate::models::errors::*;
#[allow(unused_imports)]
use crate::models::db as db;
#[allow(unused_imports)]
use crate::models::api as api;
#[allow(unused_imports)]
use crate::osm_models as osm;
#[allow(unused_imports)]
use crate::osm_reader::*;


 

/////////////////////////////////////////////////////////////////////
/// Reader
/////////////////////////////////////////////////////////////////////

#[test]
fn osm_reader_when_sample_file_given_four_nodes_and_two_ways_emitted()->TmouResult<()>
{
    let fname = current_dir()?.join("sample_osm_data.xml");
    let osm = read_osm_from_file(fname.to_str().unwrap())?;
    assert_eq!((&osm.nodes).len(), 4);
    assert_eq!((&osm.ways).len(), 2);
    Ok(())
}

#[test]
fn osm_reader_when_sample_file_given_way1000_contains_nodes123()->TmouResult<()> 
{
    let fname = current_dir()?.join("sample_osm_data.xml");
    let osm = read_osm_from_file(fname.to_str().unwrap())?;
    let way = osm.ways.get(&1000).unwrap();
    assert_eq!(way.nodes, vec![1,2,3]);
    Ok(())
}


#[test]
fn osm_reader_node_is_correctly_parsed()->TmouResult<()> 
{
    let xml = 
    r#"<osm>
         <node id="1" lat="2.5" lon="3.5"/>
         <way id="1000">
           <nd ref="1"/>
           <tag k="highway" v="x"/>
         </way>
       </osm>"#;
    let osm = read_osm_from_string(xml)?;
    assert_eq!(osm.nodes.len(), 1);
    let node = osm.nodes.get(&1).unwrap();
    assert_eq!(node.lat, 2.5);
    assert_eq!(node.lon, 3.5);
    Ok(())
}

#[test]
fn osm_reader_node_is_ignored_when_not_in_way()->TmouResult<()> 
{
    let xml = 
    r#"<osm>
        <node id="0" lat="2.5" lon="3.5"/>
        <node id="1" lat="2.5" lon="3.5"/>
         <way id="1000">
           <nd ref="1"/>
           <tag k="highway" v="x"/>
         </way>
       </osm>"#;
    let osm = read_osm_from_string(xml)?;
    assert_eq!(osm.nodes.len(),1);
    Ok(())
}

#[test]
fn osm_reader_when_malformed_node_supplied_reader_gracefully_continues()->TmouResult<()> 
{
    let xml = 
    r#"<osm>
         <node malformed="blablabla"/>
         <node id="0" lat="0" lon="0"/>
         <way id="1000">
           <nd ref="0"/>
           <tag k="highway" v="x"/>
         </way>
       </osm>"#;
    let osm = read_osm_from_string(xml)?;
    assert_eq!(osm.nodes.len(),1);
    Ok(())
}

#[test]
fn osm_reader_when_malformed_way_supplied_reader_gracefully_continues()->TmouResult<()> 
{
    let xml =
    r#"<osm>
         <node id="0" lat="0" lon="0"/>
         <node id="1" lat="0" lon="0"/>
         <way malformed="blablabla"/>
         <way id="1000">
           <nd reg="1"/>
           <tag k="highway" v="x"/>
         </way>
         <way id="1001">
           <nd ref="1"/>
         </way>
         <way id="1002">
           <nd ref="1"/>
           <tag k="highway" v="x"/>
         </way>
       </osm>"#;
    let osm = read_osm_from_string(xml)?;
    assert_eq!(osm.ways.len(),2);
    let way = osm.ways.get(&1002).unwrap();
    assert_eq!(way.nodes, vec![1]);
    Ok(())
}

#[test]
fn osm_reader_when_malformed_xml_suplied_fails_gracefully()->TmouResult<()> 
{
    let xml =
    r#"<osm>
         <node>
       </osm>"#;
    match read_osm_from_string(xml)
    {
        Ok(_) => panic!("malformed OSM parsing succeeded (should not)"),
        Err(_) => Ok(())
    }
}

/////////////////////////////////////////////////////////////////////
/// controllers::discovery
/////////////////////////////////////////////////////////////////////

#[allow(unused)]
fn item(t: &str, l: i16, n: &str)->db::Item
{
    db::Item{type_: t.to_string(), url:"Dummy".to_string(), level: l, name: n.to_string(), description: None}
}

#[test]
fn controllers::discovery_returns_unchanged_inventory_when_nothing_found()->TmouResult<()> 
{
    // ready for picking level 2:
    let inventory = vec![
        item("puzzles", 1, "puzzles-1"),
        item("badge", 1, "badge-1-1"),
        item("badge", 1, "badge-1-2"),
        item("badge", 1, "badge-1-3")];

    // empty node
    let node_contents = Vec::new();

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::Nothing);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}

#[test]
fn controllers::discovery_returns_level_0_puzzles_at_start()->TmouResult<()> 
{
    let inventory = Vec::new();

    // checkpoint with puzzles
    let node_contents = vec![
        item("puzzles", 0, "puzzles-0")];

    let expected_inventory = vec![
        item("puzzles", 0, "puzzles-0")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::PuzzlesFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(evt.newly_discovered_items, vec![item("puzzles", 0, "puzzles-0")]);
    Ok(())
}

#[test]
fn controllers::discovery_returns_level_1_puzzles_when_in_level_0()->TmouResult<()> 
{
    let inventory = vec![
        item("puzzles", 0, "puzzles-0")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("puzzles", 1, "puzzles-1")];

    let expected_inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::PuzzlesFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(evt.newly_discovered_items, vec![item("puzzles", 1, "puzzles-1")]);
    Ok(())
}

#[test]
fn controllers::discovery_returns_level_4_puzzles_when_in_level_4()->TmouResult<()> 
{
    let inventory = vec![
        item("puzzles", 4, "puzzles-4a")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("puzzles", 4, "puzzles-4b")];

    let expected_inventory = vec![
        item("puzzles", 4, "puzzles-4a"),
        item("puzzles", 4, "puzzles-4b")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::PuzzlesFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(evt.newly_discovered_items, vec![item("puzzles", 4, "puzzles-4b")]);
    Ok(())
}

#[test]
fn controllers::discovery_returns_empty_when_puzzle_level_4_found_and_in_level_5()->TmouResult<()> 
{
    let inventory = vec![
        item("puzzles", 5, "puzzles-5")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("puzzles", 4, "puzzles-4b")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::PuzzlesFound);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}

 
#[test]
fn controllers::discovery_returns_nothing_on_level_1_puzzles_at_start()->TmouResult<()> 
{
    let inventory = Vec::new();

    // checkpoint with puzzles
    let node_contents = vec![
        item("puzzles", 1, "puzzles-1")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::Nothing);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}

#[test]
fn controllers::discovery_returns_badge_level_when_found_at_start()->TmouResult<()> 
{
    // ready for a new badge
    let inventory = Vec::new();

    // new badge
    let node_contents = vec![item("badge", -1, "badge-1-3")];

    let expected_inventory = vec![item("badge", -1, "badge-1-3")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::BadgeFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(evt.newly_discovered_items, vec![item("badge", -1, "badge-1-3")]);
    Ok(())
}

#[test]
fn controllers::discovery_returns_badge_level_when_found_when_inventory_somehow_populated()->TmouResult<()> 
{
    // ready for a new badge
    let inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5"),
        item("badge", -1, "badge-1-1"),
        item("badge", -1, "badge-1-2"),
        item("badge", -1, "badge-1-4"),
        item("badge", -1, "badge-1-5"),
        item("badge", -1, "badge-1-6"),
        item("badge", -1, "badge-1-7"),
        item("badge", -1, "badge-1-8"),
        item("badge", -1, "badge-1-9")];

    // new badge
    let node_contents = vec![item("badge", -1, "badge-1-3")];

    let expected_inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5"),
        item("badge", -1, "badge-1-1"),
        item("badge", -1, "badge-1-2"),
        item("badge", -1, "badge-1-4"),
        item("badge", -1, "badge-1-5"),
        item("badge", -1, "badge-1-6"),
        item("badge", -1, "badge-1-7"),
        item("badge", -1, "badge-1-8"),
        item("badge", -1, "badge-1-9"),
        item("badge", -1, "badge-1-3")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::BadgeFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(evt.newly_discovered_items, vec![item("badge", -1, "badge-1-3")]);
    Ok(())
}

#[test]
fn controllers::discovery_returns_nothing_when_inventory_already_contains_badge()->TmouResult<()> 
{
    // ready for a new badge
    let inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5"),
        item("badge", -1, "badge-1-1"),
        item("badge", -1, "badge-1-2"),
        item("badge", -1, "badge-1-4"),
        item("badge", -1, "badge-1-5"),
        item("badge", -1, "badge-1-6"),
        item("badge", -1, "badge-1-7"),
        item("badge", -1, "badge-1-8"),
        item("badge", -1, "badge-1-9")];

    // new badge
    let node_contents = vec![item("badge", -1, "badge-1-4")];

    let expected_inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5"),
        item("badge", -1, "badge-1-1"),
        item("badge", -1, "badge-1-2"),
        item("badge", -1, "badge-1-4"),
        item("badge", -1, "badge-1-5"),
        item("badge", -1, "badge-1-6"),
        item("badge", -1, "badge-1-7"),
        item("badge", -1, "badge-1-8"),
        item("badge", -1, "badge-1-9")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::BadgeFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}

#[test]
fn controllers::discovery_returns_final_badge_when_on_proper_level()->TmouResult<()> 
{
    // ready for a new badge
    let inventory = vec![item("puzzles", 14, "puzzles-14")];

    // new badge
    let node_contents = vec![item("badge", 14, "final-badge")];

    let expected_inventory = vec![
        item("puzzles", 14, "puzzles-14"),
        item("badge", 14, "final-badge")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::BadgeFound);
    assert_eq!(evt.updated_inventory, expected_inventory);
    assert_eq!(evt.newly_discovered_items, vec![item("badge", 14, "final-badge")]);
    Ok(())
}


#[test]
fn controllers::discovery_returns_nothing_when_on_badge_but_insufficient_level()->TmouResult<()> 
{
    // ready for a new badge
    let inventory = vec![item("puzzles", 13, "puzzles-13")];

    // new badge
    let node_contents = vec![item("badge", 14, "final-badge")];

    let evt = dis::discover_node(Utc::now(), &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::Nothing);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}



#[test]
fn controllers::discovery_returns_fakes_on_checkpoint_when_eligible_nothing_owned()->TmouResult<()> 
{
    let inventory = vec![
        item("puzzles", 0, "puzzles-0")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint-start", 0, "checkpoint-start"),
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake")
        ];

    let expected_new_items = vec![
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake")
        ];
    
    let time = Utc.ymd(2020, 11, 06).and_hms(21, 0, 0) - Duration::hours(1);
    let evt = dis::discover_node(time, &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::CheckpointStartVisited);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, expected_new_items);
    Ok(())
}

#[test]
fn controllers::discovery_returns_nothing_on_checkpoint_when_not_eligible_nothing_owned()->TmouResult<()> 
{
    let inventory = vec![
        item("puzzles", 0, "puzzles-0")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint-start", 0, "checkpoint-start"),
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake")
        ];
   
    let time = Utc.ymd(2020, 11, 06).and_hms(20, 59, 59) - Duration::hours(1);
    let evt = dis::discover_node(time, &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::CheckpointStartVisited);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}

#[test]
fn controllers::discovery_returns_subset_on_checkpoint_when_eligible_some_owned()->TmouResult<()> 
{
    let inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1a"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1e-fake"),
        item("puzzles-fake", 1, "puzzles-1f-fake")
        ];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint-start", 0, "checkpoint-start"),
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1c-fake"),
        item("puzzles-fake", 1, "puzzles-1d-fake"),
        ];

    let expected_new_items = vec![
        item("puzzles-fake", 1, "puzzles-1c-fake"),
        item("puzzles-fake", 1, "puzzles-1d-fake")
        ];
    
    let time = Utc.ymd(2020, 11, 06).and_hms(22, 30, 0) - Duration::hours(1);
    let evt = dis::discover_node(time, &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::CheckpointStartVisited);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, expected_new_items);
    Ok(())
}

#[test]
fn controllers::discovery_returns_nothing_on_checkpoint_when_not_eligible_some_owned()->TmouResult<()> 
{
    let inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles-fake", 1, "puzzles-1a"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1e-fake"),
        item("puzzles-fake", 1, "puzzles-1f-fake")
        ];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint-start", 0, "checkpoint-start"),
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1c-fake"),
        item("puzzles-fake", 1, "puzzles-1d-fake"),
        ];

   
    let time = Utc.ymd(2020, 11, 06).and_hms(22, 29, 59) - Duration::hours(1);
    let evt = dis::discover_node(time, &inventory, &node_contents)?;
    assert_eq!(evt.event, dis::EventType::CheckpointStartVisited);
    assert_eq!(evt.updated_inventory, inventory);
    assert_eq!(evt.newly_discovered_items, Vec::new());
    Ok(())
}

#[test]
fn format_skip_limit_honors_declinations()->TmouResult<()>
{
    assert_eq!(dis::format_skip_limit(0,7,200), String::from(" 0 bonusů: 200 týmů;"));
    assert_eq!(dis::format_skip_limit(1,7,200), String::from(" 1 bonus: 200 týmů;"));
    assert_eq!(dis::format_skip_limit(2,7,200), String::from(" 2 bonusy: 200 týmů;"));
    assert_eq!(dis::format_skip_limit(3,7,200), String::from(" 3 bonusy: 200 týmů;"));
    assert_eq!(dis::format_skip_limit(4,7,200), String::from(" 4 bonusy: 200 týmů;"));
    assert_eq!(dis::format_skip_limit(5,7,200), String::from(" 5 bonusů: 200 týmů;"));
    assert_eq!(dis::format_skip_limit(6,7,200), String::from(" 6 bonusů: 200 týmů;"));
    assert_eq!(dis::format_skip_limit(7,7,200), String::from(" 7 a více bonusů: 200 týmů;"));
    Ok(())
}

#[test]
fn get_puzzle_welcome_message_returns_nonskippable_before_start()->TmouResult<()>
{
    let game_state = vec!(100, 90, 80, 70);
    let inventory = Vec::new();
    let msg = dis::get_puzzle_welcome_message(game_state, inventory).unwrap();
    assert_eq!(msg, String::from("Vítejte před hrou! Jste tu 100. Tuto šifru nelze přeskočit."));
    Ok(())
}

#[test]
fn get_puzzle_welcome_message_returns_nonskippable_on_start()->TmouResult<()>
{
    let game_state = vec!(100, 90, 80, 70);
    let inventory = vec![item("puzzles", 0, "šifra 0a")];
    let msg = dis::get_puzzle_welcome_message(game_state, inventory).unwrap();
    assert_eq!(msg, String::from("Vítejte na startu! Jste tu 100. Tuto šifru nelze přeskočit."));
    Ok(())
}

#[test]
fn get_puzzle_welcome_message_returns_skip_sequence_on_1()->TmouResult<()>
{
    let game_state = vec!(100, 90, 80, 70);
    let mut item_with_desc = item("puzzles", 1, "puzzle-1");
    item_with_desc.description = Some("šifra 1".to_string());
    let inventory = vec![item("puzzles", 0, "puzzles-0"), item_with_desc];
    let msg = dis::get_puzzle_welcome_message(game_state, inventory).unwrap();
    assert_eq!(msg, String::from("Vítejte na další šifře! Přibyla vám šifra 1. Jste tu 90. K přeskočení šifry potřebujete, aby šifrou prošlo pro: \
                                  0 bonusů: 250 týmů; 1 a více bonusů: 200 týmů;"));
    Ok(())
}

#[test]
fn discover_fake_puzzle_succeeds_when_eligible_some_owned()->TmouResult<()> 
{
    let inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1a"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1e-fake"),
        item("puzzles-fake", 1, "puzzles-1f-fake")
        ];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint-start", 0, "checkpoint-start"),
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1c-fake"),
        item("puzzles-fake", 1, "puzzles-1d-fake"),
        ];

    let expected_inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1a"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1e-fake"),
        item("puzzles-fake", 1, "puzzles-1f-fake"),
        item("puzzles-fake", 1, "puzzles-1d-fake")
        ];
        
    let time = Utc.ymd(2020, 11, 06).and_hms(22, 20, 0) - Duration::hours(1);
    let updated_inventory = dis::discover_fake_puzzle(time, &inventory, 
        &node_contents, &String::from("puzzles-1d-fake"))?;
    assert_eq!(updated_inventory, expected_inventory);
    Ok(())
}

#[test]
fn discover_fake_puzzle_fails_on_checkpoint_when_not_eligible_some_owned()->TmouResult<()> 
{
    let inventory = vec![
        item("puzzles", 0, "puzzles-0"),
        item("puzzles", 1, "puzzles-1a"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1e-fake"),
        item("puzzles-fake", 1, "puzzles-1f-fake")
        ];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint-start", 0, "checkpoint-start"),
        item("puzzles-fake", 1, "puzzles-1a-fake"),
        item("puzzles-fake", 1, "puzzles-1b-fake"),
        item("puzzles-fake", 1, "puzzles-1c-fake"),
        item("puzzles-fake", 1, "puzzles-1d-fake"),
        ];

    let time = Utc.ymd(2020, 11, 06).and_hms(22, 19, 0) - Duration::hours(1);
    let updated_inventory = dis::discover_fake_puzzle(time, &inventory, 
        &node_contents, &String::from("puzzles-1d-fake"));
    assert!(!updated_inventory.is_ok());
    Ok(())
}

////////////////////////////////////////////////////////
/// controllers::standings
////////////////////////////////////////////////////////

#[allow(unused)]
fn team(rank: u16, name: &str, ps: Vec<(u16, bool, u32)>) -> api::TeamStanding
{
    let start_puzzles_solved = ps.iter().filter(|(level,_,_)| *level == 1).count() as u16;
    api::TeamStanding{
        rank: rank,
        name: name.to_string(),
        puzzles: ps.into_iter().map(|(level, dead, min)| (level, api::PuzzleResult {
            dead: dead,
            timestamp: chrono::NaiveDate::from_ymd(2020, 11, 6).and_hms(22, min, 00)
        })).collect(),
        badge_count: 0,
        start_puzzles_solved 
    }
}

#[test]
fn is_better_team_returns_alphabetical_for_empty_teams()->TmouResult<()> 
{
    let a = team(0, "Absolutno", Vec::new());
    let b = team(0, "Bazinga", Vec::new());
    assert_eq!(controllers::standings::is_better_team(&a, &b), Ordering::Less);
    Ok(())
}

#[test]
fn is_better_team_returns_alphabetical_for_equivalent_teams()->TmouResult<()> 
{
    let a = team(0, "Absolutno", vec![(1,true,1), (2,true,17), (3,true,23), (4,false,51), (5,true,59)]);
    let b = team(0, "Bazinga", vec![(1,true,1), (2,true,17), (3,true,23), (4,false,51), (5,true,59)]);
    assert_eq!(controllers::standings::is_better_team(&a, &b), Ordering::Less);
    Ok(())
}


#[test]
fn is_better_team_does_not_prefer_puzzle_visit_over_nothing()->TmouResult<()> 
{
    let a = team(0, "Absolutno", Vec::new());
    let b = team(0, "Bazinga", vec![(1,false,0)]);
    assert_eq!(controllers::standings::is_better_team(&a, &b), Ordering::Less);
    Ok(())
}

#[test]
fn is_better_team_does_not_prefer_dead()->TmouResult<()> 
{
    let a = team(0, "Absolutno", vec![(1,false,0)]);
    let b = team(0, "Bazinga", vec![(1,true,0)]);
    assert_eq!(controllers::standings::is_better_team(&a, &b), Ordering::Less);
    Ok(())
}

#[test]
fn is_better_team_prefers_team_with_solved_puzzle()->TmouResult<()> 
{
    let a = team(0, "Absolutno", vec![(1,true,0), (2,true,0)]);
    let b = team(0, "Bazinga", vec![(1,false,1), (2,true,1)]);
    assert_eq!(controllers::standings::is_better_team(&a, &b), Ordering::Greater);
    Ok(())
}

#[test]
fn is_better_team_prefers_faster_team()->TmouResult<()> 
{
    let a = team(0, "Absolutno", vec![(1,false,0), (2,false,1)]);
    let b = team(0, "Bazinga", vec![(1,false,1), (2,false,0)]);
    assert_eq!(controllers::standings::is_better_team(&a, &b), Ordering::Greater);
    Ok(())
}

#[test]
fn is_better_team_prefers_faster_team_on_last_solved()->TmouResult<()> 
{
    let a = team(0, "Absolutno", vec![(1,true,0), (2,true,1), (3,false,1), (4,true,1), (5,false,1), (6,true,1), (7,true,1), (8,false,1)]);
    let b = team(0, "Bazinga",   vec![(1,true,0), (2,true,1), (3,true,1), (4,false,1), (5,true,1), (6,false,1), (7,true,2), (8,false,2)]);
    assert_eq!(controllers::standings::is_better_team(&a, &b), Ordering::Greater);
    Ok(())
}


#[test]
fn is_better_team_prefers_team_with_more_solved()->TmouResult<()> 
{
    let a = team(0, "Absolutno", vec![(1,true,1), (2,true,1), (3,true,1), (4,false,1), (5,true,1)]);
    let b = team(0, "Bazinga", vec![(1,false,0), (2,false,5), (3,true,7)]);
    assert_eq!(controllers::standings::is_better_team(&a, &b), Ordering::Greater);
    Ok(())
}

////////////////////////////////////////////////////
#[allow(unused)]
fn db_team(name: &str, ps: Vec<(&str, i16, u32)>) -> Vec<db::TeamStandingsItem>
{
    match ps.is_empty()
    {
        true => vec![db::TeamStandingsItem {
            team_name: name.to_string(),
            type_: None,
            level: None,
            name: None,
            description: None,
            timestamp: None}],
        false =>     ps.into_iter().map(|(typ, levl, min)| db::TeamStandingsItem {
            team_name: name.to_string(),
            type_: Some(typ.to_string()),
            level: Some(levl),
            name: Some("item".to_string()),
            description: Some("item".to_string()),
            timestamp: Some(chrono::NaiveDate::from_ymd(2020, 11, 6).and_hms(22, min, 00))
        }).collect()
    }
}




#[test]
fn calculate_teams_controllers::standings_outputs_2_empty_teams_sorted()->TmouResult<()> 
{
    let mut a = db_team("Absolutno", Vec::new());
    let mut b = db_team("Bazinga", Vec::new());
    a.append(&mut b);

    let expected = vec![
        team(1, "Absolutno", Vec::new()),
        team(2, "Bazinga", Vec::new())
    ];

    let st = controllers::standings::calculate_teams_controllers::standings(a)?;
    assert_eq!(st.controllers::standings, expected);
    Ok(())
}

#[test]
fn calculate_teams_controllers::standings_outputs_2_complex_teams_sorted()->TmouResult<()> 
{
    let mut a = db_team("Absolutno", vec![
        ("puzzles-fake", 1, 0),
        ("puzzles-fake", 1, 0),
        ("puzzles", 1, 5),
        ("puzzles", 1, 6),
        ("puzzles", 1, 8),
        ("puzzles", 1, 7),
        ("dead", 1, 10),
        ("puzzles", 2, 20),
        ("dead", 2, 25),
        ("puzzles", 3, 30),
        ("puzzles", 4, 50),
    ]);
    let mut b = db_team("Bazinga", vec![
        ("puzzles-fake", 1, 0),
        ("puzzles-fake", 1, 0),
        ("puzzles", 1, 5),
        ("puzzles", 1, 6),
        ("puzzles", 2, 20),
        ("dead", 2, 25),
        ("puzzles", 3, 30),
        ("puzzles", 4, 50),
    ]);
    a.append(&mut b);

    let mut res_1 = team(1, "Bazinga", vec![
        (1, false, 5),
        (2, true, 20),
        (3, false, 30),
        (4, false, 50)
    ]);
    res_1.start_puzzles_solved = 2;

    let mut res_2 = team(2, "Absolutno", vec![
        (1, true, 5),
        (2, true, 20),
        (3, false, 30),
        (4, false, 50)
    ]);
    res_2.start_puzzles_solved = 4;


    let expected = vec![res_1, res_2];

    let st = controllers::standings::calculate_teams_controllers::standings(a)?;
    assert_eq!(st.controllers::standings, expected);
    Ok(())
}

#[test]
fn calculate_teams_controllers::standings_outputs_correct_badge_counts()->TmouResult<()> 
{
    let mut a = db_team("Absolutno", vec![
        ("puzzles-fake", 1, 0),
        ("puzzles-fake", 1, 0),
        ("puzzles", 1, 5),
        ("puzzles", 1, 6),
        ("badge", 0, 21),
        ("puzzles", 1, 7),
        ("puzzles", 1, 8),
        ("dead", 1, 10),
        ("puzzles", 2, 20),
        ("badge", 0, 29),
        ("dead", 2, 25),
        ("puzzles", 3, 30),
        ("badge", 0, 11),
        ("puzzles", 4, 50),
        ("badge", 0, 38),
        ("badge", 0, 20),
    ]);
    let mut b = db_team("Bazinga", vec![
        ("puzzles-fake", 1, 0),
        ("badge", 0, 21),
        ("puzzles-fake", 1, 0),
        ("puzzles", 1, 5),
        ("puzzles", 1, 6),
        ("puzzles", 2, 20),
        ("dead", 2, 25),
        ("puzzles", 3, 30),
        ("puzzles", 4, 50),
    ]);
    let mut c = db_team("Corn Flakes", vec![
        ("puzzles-fake", 1, 0),
        ("puzzles-fake", 1, 0),
        ("puzzles", 1, 5),
        ("puzzles", 1, 6),
        ("puzzles", 2, 59),
    ]);
    let mut d = db_team("Degen a spol", Vec::new());
    a.append(&mut b);
    a.append(&mut c);
    a.append(&mut d);

    let st = controllers::standings::calculate_teams_controllers::standings(a)?;
    assert_eq!(st.controllers::standings.len(), 4);
    assert_eq!(st.controllers::standings[0].name, "Bazinga".to_string());
    assert_eq!(st.controllers::standings[0].badge_count, 1);
    assert_eq!(st.controllers::standings[1].name, "Absolutno".to_string());
    assert_eq!(st.controllers::standings[1].badge_count, 5);
    assert_eq!(st.controllers::standings[2].name, "Corn Flakes".to_string());
    assert_eq!(st.controllers::standings[2].badge_count, 0);
    assert_eq!(st.controllers::standings[3].name, "Degen a spol".to_string());
    assert_eq!(st.controllers::standings[3].badge_count, 0);
    Ok(())
}

