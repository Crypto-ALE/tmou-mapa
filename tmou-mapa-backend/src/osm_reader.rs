use super::osm_models::*;
use super::errors::*;
use std::fs::read_to_string;
use std::collections::HashMap;


fn get_osm_element<'a>(doc:&'a roxmltree::Document)->TmouResult<roxmltree::Node<'a,'a>>
{
    doc.root().first_child().ok_or(TmouError{message:"OSM data do not contain <osm>".to_string(), response:404})
}

pub fn read_osm_from_file(path: &str) -> TmouResult<Osm>
{
    let xml = read_to_string(path)?;
    read_osm_from_string(&xml)
}

pub fn read_osm_from_string(xml: &str) -> TmouResult<Osm>
{
    let doc = roxmltree::Document::parse(&xml)?;
    let mut osm = Osm{nodes:HashMap::new(), ways:HashMap::new()};
    for el in get_osm_element(&doc)?.children()
    {
        match el.tag_name().name()
        {
            "node" => 
            {
                let on = read_node(&el);
                match on 
                {
                    Some(n) => { osm.nodes.insert(n.0, n.1); ()},
                    None => { println!("Malformed node: {:?}", el); ()}

                }
            },
            "way" => 
            {
                let on = read_way(&el);
                match on 
                {
                    Some(n) => { osm.ways.insert(n.0, n.1); ()},
                    None => { println!("Malformed node: {:?}", el); () }

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
    let nodes = n
        .children()
        .filter(|a| a.tag_name().name() == "nd" && a.has_attribute("ref"))
        .map(|a| a.attribute("ref")
        .unwrap().to_string()).collect();
    Some((id.to_string(), Way{id:id.to_string(), nodes}))
}
