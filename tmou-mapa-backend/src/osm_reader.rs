use std::collections::HashMap;
use std::fs::read_to_string;

use crate::models::errors::*;
use crate::osm_models::*;

fn get_osm_element<'a>(doc: &'a roxmltree::Document) -> TmouResult<roxmltree::Node<'a, 'a>> {
    doc.root().first_child().ok_or(TmouError {
        message: "OSM data do not contain <osm>".to_string(),
        response: 404,
    })
}

pub fn read_osm_from_file(path: &str) -> TmouResult<Osm> {
    let xml = read_to_string(path)?;
    read_osm_from_string(&xml)
}

pub fn read_osm_from_string(xml: &str) -> TmouResult<Osm> {
    let doc = roxmltree::Document::parse(&xml)?;
    let mut osm = Osm {
        nodes: HashMap::new(),
        ways: HashMap::new(),
    };
    for el in get_osm_element(&doc)?.children() {
        match el.tag_name().name() {
            "node" => {
                let on = read_node(&el);
                match on {
                    Some(n) => {
                        osm.nodes.insert(n.0, n.1);
                        ()
                    }
                    None => {
                        println!("Malformed node: {:?}", el);
                        ()
                    }
                }
            }
            "way" => {
                let on = read_way(&el);
                match on {
                    Some(n) => {
                        osm.ways.insert(n.0, n.1);
                        ()
                    }
                    None => {
                        /* no log, some ways can be ignored */
                        ()
                    }
                }
            }
            _ => (),
        }
    }
    add_node_types(&mut osm);
    Ok(osm)
}

fn add_node_types(osm: &mut Osm) {
    let node_iter = osm.ways.iter().flat_map(|(_, w)| &w.nodes);
    let mut nodes = node_iter.map(|n| *n).collect::<Vec<i64>>();
    nodes.sort();
    let mut m = HashMap::new();
    let mut last_n = 0;
    let mut last_count = 0;
    for n in nodes {
        if n == last_n {
            last_count = last_count + 1;
        } else {
            if last_n != 0 {
                m.insert(last_n, last_count);
            }
            last_count = 1;
            last_n = n;
        }
    }
    if last_count != 0 {
        m.insert(last_n, last_count);
    }
    for (_, mut n) in &mut osm.nodes {
        n.r#type = match m.get(&n.id) {
            Some(cnt) if cnt > &1 => "junction".to_string(),
            Some(cnt) if cnt == &1 => "ordinary".to_string(),
            _ => "pruned".to_string(),
        };
    }
    osm.nodes.retain(|_, n| n.r#type != "pruned".to_string());
}

fn read_node(n: &roxmltree::Node) -> Option<(i64, Node)> {
    let id = n.attribute("id").and_then(|l| l.parse::<i64>().ok())?;
    let lat = n.attribute("lat").and_then(|l| l.parse::<f32>().ok())?;
    let lon = n.attribute("lon").and_then(|l| l.parse::<f32>().ok())?;
    Some((
        id,
        Node {
            id,
            lat,
            lon,
            r#type: "ordinary".to_string(),
            tag: None, //TODO: get tag as parameter
        },
    ))
}

fn read_way(n: &roxmltree::Node) -> Option<(i64, Way)> {
    let id = n.attribute("id").and_then(|i| i.parse::<i64>().ok())?;
    if !n.children().any(|a| {
        a.tag_name().name() == "tag"
            && a.has_attribute("k")
            && a.attribute("k").unwrap().to_string() == "highway"
    }) {
        return None;
    }

    let nodes = n
        .children()
        .filter(|a| a.tag_name().name() == "nd" && a.has_attribute("ref"))
        .map(|a| a.attribute("ref").unwrap().parse::<i64>().unwrap())
        .collect();
    Some((id, Way { id, nodes, tag:None })) //TODO: get tag as parameter
}
