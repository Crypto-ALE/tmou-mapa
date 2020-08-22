#[cfg(test)]

use super::osm_models as osm;
#[allow(unused_imports)]
use super::osm_reader::*;
#[allow(unused_imports)]
use super::osm_logic::*;
#[allow(unused_imports)]
use super::errors::*;

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
    let way = osm.ways.get("1000").unwrap();
    assert_eq!(way.nodes, vec!["1".to_string(),"2".to_string(),"3".to_string()]);
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
    let node = osm.nodes.get("1").unwrap();
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
    let way = osm.ways.get("1002").unwrap();
    assert_eq!(way.nodes, vec!["1".to_string()]);
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
/// Logic
/////////////////////////////////////////////////////////////////////

#[test]
fn osm_logic_get_ways_going_through_node_id_returns_correct_ways()->TmouResult<()> 
{
    let xml =
    r#"<osm>
         <node id="0" lat="0" lon="0"/>
         <node id="1" lat="0" lon="0"/>
         <node id="2" lat="0" lon="0"/>
         <node id="3" lat="0" lon="0"/>
         <node id="4" lat="0" lon="0"/>
         <node id="4" lat="0" lon="0"/>
         <way id="01">
          <nd ref="0"/>
          <nd ref="1"/>
          <tag k="highway" v="x"/>
          </way>
         <way id="12">
           <nd ref="1"/>
           <nd ref="2"/>
           <tag k="highway" v="x"/>
         </way>
         <way id="23">
           <nd ref="2"/>
           <nd ref="3"/>
           <tag k="highway" v="x"/>
         </way>
         <way id="34">
           <nd ref="3"/>
           <nd ref="4"/>
           <tag k="highway" v="x"/>
         </way>
         <way id="45">
           <nd ref="4"/>
           <nd ref="5"/>
           <tag k="highway" v="x"/>
        </way>
       </osm>"#;
    let osm = read_osm_from_string(xml)?;
    let ways: Vec<& osm::Way> = get_ways_going_through_node_id(&osm, "4".to_string());
    assert_eq!(ways.len(), 2);
    assert_eq!(ways.iter().any(|w| w.id == "34"), true);
    assert_eq!(ways.iter().any(|w| w.id == "45"), true);
    Ok(())
}

#[test]
fn osm_logic_get_reachable_ways_for_node_id_returns_correct_ways()->TmouResult<()> 
{
    let xml =
    r#"<osm>
         <node id="0" lat="0" lon="0"/>
         <node id="1" lat="0" lon="0"/>
         <node id="2" lat="0" lon="0"/>
         <node id="3" lat="0" lon="0"/>
         <node id="4" lat="0" lon="0"/>
         <node id="4" lat="0" lon="0"/>
         <way id="01">
          <nd ref="0"/>
          <nd ref="1"/>
         </way>
         <way id="12">
           <nd ref="1"/>
           <nd ref="2"/>
           <tag k="highway" v="x"/>
         </way>
         <way id="23">
           <nd ref="2"/>
           <nd ref="3"/>
           <tag k="highway" v="x"/>
         </way>
         <way id="34">
           <nd ref="3"/>
           <nd ref="4"/>
           <tag k="highway" v="x"/>
         </way>
         <way id="45">
           <nd ref="4"/>
           <nd ref="5"/>
           <tag k="highway" v="x"/>
         </way>
       </osm>"#;
    let osm = read_osm_from_string(xml)?;
    let ways: Vec<& osm::Way> = get_reachable_ways_for_node_id(&osm, "3".to_string());
    assert_eq!(ways.len(), 4);
    assert_eq!(ways.iter().any(|w| w.id == "12"), true);
    assert_eq!(ways.iter().any(|w| w.id == "23"), true);
    assert_eq!(ways.iter().any(|w| w.id == "34"), true);
    assert_eq!(ways.iter().any(|w| w.id == "45"), true);
    Ok(())
}

