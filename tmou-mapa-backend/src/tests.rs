#[cfg(test)]
use super::osm_reader::read_osm;
use super::errors::*;

#[test]
fn osm_reader_when_sample_file_given_four_nodes_and_two_ways_emitted()->TmouResult<()> 
{
    let fname = concat!(env!("CARGO_MANIFEST_DIR"), r"\sample_osm_data.xml");
    let osm = read_osm(fname)?;
    assert_eq!((&osm.nodes).len(), 4);
    assert_eq!((&osm.ways).len(), 2);
    Ok(())
}

#[test]
fn osm_reader_when_sample_file_given_way1000_contains_nodes123()->TmouResult<()> 
{
    let fname = concat!(env!("CARGO_MANIFEST_DIR"), r"\sample_osm_data.xml");
    let osm = read_osm(fname)?;
    let way = osm.ways.get("1000").ok_or_else(|| TmouError{message:"Way not found".to_string(), response:404})?;
    assert_eq!(way.nodes, vec!["1".to_string(),"2".to_string(),"3".to_string()]);
    Ok(())
}

