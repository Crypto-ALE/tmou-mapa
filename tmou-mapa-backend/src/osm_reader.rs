use super::osm_models::*;
use super::errors::*;
use std::fs::read_to_string;
use std::collections::HashMap;


pub fn read_osm(path: &str) -> TmouResult<Osm>
{
    let xml = read_to_string(path)?;
    let doc = roxmltree::Document::parse(&xml)?;
    let mut osm = Osm{nodes:HashMap::new(), ways:HashMap::new()};
    for n in doc.root().first_child().unwrap().children()
    {
        match n.tag_name().name()
        {
            "node" => 
            {
                let on = read_node(&n);
                match on 
                {
                    Some(n) => { osm.nodes.insert(n.0, n.1); ()},
                    None => ()

                }
            },
            "way" => 
            {
                let on = read_way(&n);
                match on 
                {
                    Some(n) => { osm.ways.insert(n.0, n.1); ()},
                    None => ()

                }
           },
            _ => ()
        };
    }
    Ok(osm)
}


fn read_node(n: &roxmltree::Node) -> Option<(String, Node)>
{
    let id = n.attribute("id")?;
    let lat = n.attribute("lat").and_then(|l| l.parse::<f32>().ok())?;
    let lon = n.attribute("lon").and_then(|l| l.parse::<f32>().ok())?;
    Some((id.to_string(), 
      Node{id:id.to_string(), lat, lon}))
}

fn read_way(n: &roxmltree::Node) -> Option<(String, Way)>
{
    let id = n.attribute("id")?;
    let nodes = n.children().filter(|a| a.tag_name().name() == "nd" && a.has_attribute("ref")).map(|a| a.attribute("ref").unwrap().to_string()).collect();
    Some((id.to_string(), Way{id:id.to_string(), nodes}))
}
