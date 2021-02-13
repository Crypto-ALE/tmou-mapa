#[allow(unused)]
use std::env::current_dir;
#[allow(unused_imports)]
use crate::models::errors::*;
#[allow(unused_imports)]
use crate::osm_models as osm;
#[allow(unused_imports)]
use crate::osm_reader::*;

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
