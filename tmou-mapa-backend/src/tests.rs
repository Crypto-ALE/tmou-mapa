#[cfg(test)]

#[allow(unused_imports)]
use super::osm_models as osm;
#[allow(unused_imports)]
use super::osm_reader::*;
#[allow(unused_imports)]
use super::errors::*;
#[allow(unused_imports)]
use super::discovery as dis;
#[allow(unused_imports)]
use super::db_models as db;


#[allow(unused)]
use std::env::current_dir;
 

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
/// Discovery
/////////////////////////////////////////////////////////////////////

#[allow(unused)]
fn item(t: &str, l: i16, n: &str)->db::Item
{
    db::Item{type_: t.to_string(), url:"Dummy".to_string(), level: l, name: n.to_string(), description: None}
}

#[test]
fn discovery_returns_unchanged_inventory_when_nothing_found()->TmouResult<()> 
{
    // ready for picking level 2:
    let inventory = vec![
        item("puzzles", 1, "puzzles-1"),
        item("badge", 1, "badge-1-1"),
        item("badge", 1, "badge-1-2"),
        item("badge", 1, "badge-1-3")];

    // empty node
    let node_contents = Vec::new();

    let (new_inventory,items) = dis::discover_node(&inventory, &node_contents)?;
    assert_eq!(new_inventory, inventory);
    assert_eq!(items, Vec::new());
    Ok(())
}

#[test]
fn discovery_returns_level_2_puzzles_when_3_badges_level_1_presented_in_checkpoint()->TmouResult<()> 
{
    // ready for picking level 2:
    let inventory = vec![
        item("puzzles", 1, "puzzles-1"),
        item("badge", 1, "badge-1-1"),
        item("badge", 1, "badge-1-2"),
        item("badge", 1, "badge-1-3")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint",0,"checkpoint"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5")];

    let expected_inventory = vec![
        item("puzzles", 1, "puzzles-1"),
        item("badge", 1, "badge-1-1"),
        item("badge", 1, "badge-1-2"),
        item("badge", 1, "badge-1-3"),
        item("puzzles", 2, "puzzles-2")];

    let (new_inventory,items) = dis::discover_node(&inventory, &node_contents)?;
    assert_eq!(new_inventory, expected_inventory);
    assert_eq!(items, vec![item("checkpoint",0,"checkpoint")]);
    Ok(())
}

#[test]
fn discovery_returns_level_3_puzzles_when_2_badges_level_2_presented_in_checkpoint()->TmouResult<()> 
{
    // ready for picking level 2:
    let inventory = vec![
        item("puzzles", 2, "puzzles-2"),
        item("badge", 2, "badge-2-1"),
        item("badge", 2, "badge-2-2")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint",0,"checkpoint"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5")];

    let expected_inventory = vec![
        item("puzzles", 2, "puzzles-2"),
        item("badge", 2, "badge-2-1"),
        item("badge", 2, "badge-2-2"),
        item("puzzles", 3, "puzzles-3")];

    let (new_inventory,items) = dis::discover_node(&inventory, &node_contents)?;
    assert_eq!(new_inventory, expected_inventory);
    assert_eq!(items, vec![item("checkpoint",0,"checkpoint")]);
    Ok(())
}


#[test]
fn discovery_returns_level_4_puzzles_when_1_badge_level_3_presented_in_checkpoint()->TmouResult<()> 
{
    // ready for picking level 2:
    let inventory = vec![
        item("puzzles", 3, "puzzles-3"),
        item("badge", 3, "badge-3-2")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint",0,"checkpoint"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5")];

    let expected_inventory = vec![
        item("puzzles", 3, "puzzles-3"),
        item("badge", 3, "badge-3-2"),
        item("puzzles", 4, "puzzles-4")];

    let (new_inventory,items) = dis::discover_node(&inventory, &node_contents)?;
    assert_eq!(new_inventory, expected_inventory);
    assert_eq!(items, vec![item("checkpoint",0,"checkpoint")]);
    Ok(())
}


#[test]
fn discovery_returns_level_5_puzzles_when_1_badge_level_4_presented_in_checkpoint()->TmouResult<()> 
{
    // ready for picking level 2:
    let inventory = vec![
        item("puzzles", 4, "puzzles-4"),
        item("badge", 4, "badge-4-1")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint",0,"checkpoint"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5")];

    let expected_inventory = vec![
        item("puzzles", 4, "puzzles-4"),
        item("badge", 4, "badge-4-1"),
        item("puzzles", 5, "puzzles-5")];

    let (new_inventory,items) = dis::discover_node(&inventory, &node_contents)?;
    assert_eq!(new_inventory, expected_inventory);
    assert_eq!(items, vec![item("checkpoint",0,"checkpoint")]);
    Ok(())
}

#[test]
fn discovery_does_nothing_when_too_few_badges()->TmouResult<()> 
{
    // ready for picking level 2:
    let inventory = vec![
        item("puzzles", 1, "puzzles-1"),
        item("badge", 1, "badge-1-1"),
        item("badge", 1, "badge-1-3")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint",0,"checkpoint"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5")];


    let (new_inventory,items) = dis::discover_node(&inventory, &node_contents)?;
    assert_eq!(new_inventory, inventory);
    assert_eq!(items, vec![item("checkpoint",0,"checkpoint")]);
    Ok(())
}

#[test]
fn discovery_does_nothing_when_badges_from_wrong_level()->TmouResult<()> 
{
    // ready for picking level 2:
    let inventory = vec![
        item("puzzles", 1, "puzzles-1"),
        item("badge", 2, "badge-2-1"),
        item("badge", 1, "badge-1-2")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint",0,"checkpoint"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 2, "puzzles-2"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5")];


    let (new_inventory,items) = dis::discover_node(&inventory, &node_contents)?;
    assert_eq!(new_inventory, inventory);
    assert_eq!(items, vec![item("checkpoint",0,"checkpoint")]);
    Ok(())
}


#[test]
fn discovery_does_nothing_when_puzzles_not_in_checkpoint()->TmouResult<()> 
{
    // ready for picking level 2:
    let inventory = vec![
        item("puzzles", 1, "puzzles-1"),
        item("badge", 1, "badge-1-1"),
        item("badge", 1, "badge-1-2"),
        item("badge", 1, "badge-1-3")];

    // checkpoint with puzzles
    let node_contents = vec![
        item("checkpoint",0,"checkpoint"),
        item("puzzles", 1, "puzzles-1"),
        item("puzzles", 3, "puzzles-3"),
        item("puzzles", 4, "puzzles-4"),
        item("puzzles", 5, "puzzles-5")];


    let (new_inventory,items) = dis::discover_node(&inventory, &node_contents)?;
    assert_eq!(new_inventory, inventory);
    assert_eq!(items, vec![item("checkpoint",0,"checkpoint")]);
    Ok(())
}

#[test]
fn discovery_adds_badge_level_1_when_player_in_level_1_and_not_on_max()->TmouResult<()> 
{
    // ready for a new badge
    let inventory = vec![
        item("puzzles", 1, "puzzles-1"),
        item("badge", 1, "badge-1-1"),
        item("badge", 1, "badge-1-2")];

    // new badge
    let node_contents = vec![item("badge", 1, "badge-1-3")];

    // expected: level 2 puzzles and nothing more
    let expected_inventory = vec![
        item("puzzles", 1, "puzzles-1"),
        item("badge", 1, "badge-1-1"),
        item("badge", 1, "badge-1-2"),
        item("badge", 1, "badge-1-3")];

    let (new_inventory,items) = dis::discover_node(&inventory, &node_contents)?;
    assert_eq!(new_inventory, expected_inventory);
    assert_eq!(items, node_contents);
    Ok(())
}

#[test]
fn discovery_does_not_add_badge_when_insufficient_level()->TmouResult<()> 
{
    // ready for a new badge
    let inventory = vec![
        item("puzzles", 1, "puzzles-1"),
        item("badge", 1, "badge-1-1"),
        item("badge", 1, "badge-1-2")];

    // new badge
    let node_contents = vec![item("badge", 2, "badge-2-3")];

    let (new_inventory,items) = dis::discover_node(&inventory, &node_contents)?;
    assert_eq!(new_inventory, inventory);
    assert_eq!(items, Vec::new());
    Ok(())
}

#[test]
fn discovery_does_not_add_badge_when_level_too_high()->TmouResult<()> 
{
    // ready for a new badge
    let inventory = vec![
        item("puzzles", 2, "puzzles-2"),
        item("badge", 2, "badge-2-1")];

    // new badge
    let node_contents = vec![item("badge", 1, "badge-1-1")];

    let (new_inventory,items) = dis::discover_node(&inventory, &node_contents)?;
    assert_eq!(new_inventory, inventory);
    assert_eq!(items, node_contents);
    Ok(())
}


#[test]
fn discovery_does_not_add_badge_when_already_found()->TmouResult<()> 
{
    // ready for a new badge
    let inventory = vec![
        item("puzzles", 1, "puzzles-1"),
        item("badge", 1, "badge-1-1"),
        item("badge", 1, "badge-1-2")];

    // new badge
    let node_contents = vec![item("badge", 1, "badge-1-2")];

    let (new_inventory,items) = dis::discover_node(&inventory, &node_contents)?;
    assert_eq!(new_inventory, inventory);
    assert_eq!(items, node_contents);
    Ok(())
}

#[test]
fn discovery_does_not_add_badge_when_on_max()->TmouResult<()> 
{
    // ready for a new badge
    let inventory = vec![
        item("puzzles", 1, "puzzles-1"),
        item("badge", 1, "badge-1-1"),
        item("badge", 1, "badge-1-2"),
        item("badge", 1, "badge-1-3")];

    // new badge
    let node_contents = vec![item("badge", 1, "badge-1-4")];

    let (new_inventory,items) = dis::discover_node(&inventory, &node_contents)?;
    assert_eq!(new_inventory, inventory);
    assert_eq!(items, node_contents);
    Ok(())
}
