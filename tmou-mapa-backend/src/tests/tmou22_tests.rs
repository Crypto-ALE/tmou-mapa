#[allow(unused_imports)]
use crate::controllers::tmou22::*;
#[allow(unused_imports)]
use crate::models::api as api;
#[allow(unused_imports)]
use crate::models::errors::*;
#[allow(unused_imports)]
use super::api_item;

#[allow(unused)]
fn node(id: i64, tag: &str) ->api::Node {
    api::Node {
        id: id,
        x: 0.0,
        y: 0.0,
        r#type: String::from("ordinary"),
        data: String::from(""),
        tag: Some(String::from(tag))
    }
}

#[allow(unused)]
fn way(id: i64, n1: i64, n2: i64, tag: &str) ->api::Way {
    api::Way {
        id: id,
        nodes: vec![n1,n2],
        tag: Some(String::from(tag))
    }
}

fn default_pois() -> api::Pois {
    api::Pois{
        nodes: vec![
            node(1, "Europe"),
            node(2, "Europe"),
            node(3, "Africa"),
            node(4, "Africa"),
            node(5, "Asia"),
            node(6, "Asia"),
            node(7, "Australia"),
            node(8, "Australia"),
            node(9, "America"),
            node(10, "America"),
            node(11, "Sifra11"),
        ],
        ways: vec![
            way(1,1,2, "Europe"),
            way(2,2,3, "Europe,Africa"),
            way(3,3,4, "Africa"),
            way(4,4,5, "Africa,Asia"),
            way(5,5,6, "Asia"),
            way(6,6,7, "Asia,Australia"),
            way(7,7,8, "Australia"),
            way(8,8,9, "Australia,America"),
            way(9,9,10, "America"),
            way(10,1,11, "Sifra11")
        ]
    }
}


#[test]
fn tmou22_filter_returns_europe_for_level_minus1() -> TmouResult<()> {
    let pois = default_pois();
    let items = api::Items{
        items: Vec::new()
    };

    let expected_pois = api::Pois{
        nodes: vec![
            node(1, "Europe"),
            node(2, "Europe"),
        ],
        ways: vec![
            way(1,1,2, "Europe"),
        ]
    };
    let new_pois = filter_pois_by_tag(pois, &items)?;
    assert_eq!(new_pois, expected_pois);
    Ok(())
}

#[test]
fn tmou22_filter_returns_europe_for_level_0() -> TmouResult<()> {
    let pois = default_pois();
    let items = api::Items{
        items: vec![
            api_item("puzzle", 0, "puzzles-12")
        ]
    };

    let expected_pois =     api::Pois{
        nodes: vec![
            node(1, "Europe"),
            node(2, "Europe"),
        ],
        ways: vec![
            way(1,1,2, "Europe"),
        ]
    };

    let new_pois = filter_pois_by_tag(pois, &items)?;
    assert_eq!(new_pois, expected_pois);
    Ok(())
}

#[test]
fn tmou22_filter_returns_europe_and_africa_for_level1() -> TmouResult<()> {
    let pois = default_pois();
    let items = api::Items{
        items: vec![
            api_item("puzzle", 0, "puzzles-12"),
            api_item("puzzle", 1, "puzzles-21")
        ]
    };

    let expected_pois = api::Pois{
        nodes: vec![
            node(1, "Europe"),
            node(2, "Europe"),
            node(3, "Africa"),
            node(4, "Africa"),
        ],
        ways: vec![
            way(1,1,2, "Europe"),
            way(2,2,3, "Europe,Africa"),
            way(3,3,4, "Africa"),
        ]
    };

    let new_pois = filter_pois_by_tag(pois, &items)?;
    assert_eq!(new_pois, expected_pois);
    Ok(())
}


#[test]
fn tmou22_filter_returns_europe_africa_asia_for_level2() -> TmouResult<()> {
    let pois = default_pois();
    let items = api::Items{
        items: vec![
            api_item("puzzle", 0, "puzzles-12"),
            api_item("puzzle", 1, "puzzles-21"),
            api_item("puzzle", 2, "puzzles-31")
        ]
    };

    let expected_pois = api::Pois{
        nodes: vec![
            node(1, "Europe"),
            node(2, "Europe"),
            node(3, "Africa"),
            node(4, "Africa"),
            node(5, "Asia"),
            node(6, "Asia"),
        ],
        ways: vec![
            way(1,1,2, "Europe"),
            way(2,2,3, "Europe,Africa"),
            way(3,3,4, "Africa"),
            way(4,4,5, "Africa,Asia"),
            way(5,5,6, "Asia"),
        ]
    };

    let new_pois = filter_pois_by_tag(pois, &items)?;
    assert_eq!(new_pois, expected_pois);
    Ok(())
}

#[test]
fn tmou22_filter_returns_europe_africa_asia_australia_for_level3() -> TmouResult<()> {
    let pois = default_pois();
    let items = api::Items{
        items: vec![
            api_item("puzzle", 0, "puzzles-12"),
            api_item("puzzle", 1, "puzzles-21"),
            api_item("puzzle", 2, "puzzles-31"),
            api_item("puzzle", 3, "puzzles-41")
        ]
    };

    let expected_pois =     api::Pois{
        nodes: vec![
            node(1, "Europe"),
            node(2, "Europe"),
            node(3, "Africa"),
            node(4, "Africa"),
            node(5, "Asia"),
            node(6, "Asia"),
            node(7, "Australia"),
            node(8, "Australia"),
        ],
        ways: vec![
            way(1,1,2, "Europe"),
            way(2,2,3, "Europe,Africa"),
            way(3,3,4, "Africa"),
            way(4,4,5, "Africa,Asia"),
            way(5,5,6, "Asia"),
            way(6,6,7, "Asia,Australia"),
            way(7,7,8, "Australia"),
        ]
    };

    let new_pois = filter_pois_by_tag(pois, &items)?;
    assert_eq!(new_pois, expected_pois);
    Ok(())
}

#[test]
fn tmou22_filter_returns_all_for_level4() -> TmouResult<()> {
    let pois = default_pois();
    let items = api::Items{
        items: vec![
            api_item("puzzle", 0, "puzzles-12"),
            api_item("puzzle", 1, "puzzles-21"),
            api_item("puzzle", 2, "puzzles-31"),
            api_item("puzzle", 3, "puzzles-41"),
            api_item("puzzle", 4, "puzzles-51")
        ]
    };

    let expected_pois =     api::Pois{
        nodes: vec![
            node(1, "Europe"),
            node(2, "Europe"),
            node(3, "Africa"),
            node(4, "Africa"),
            node(5, "Asia"),
            node(6, "Asia"),
            node(7, "Australia"),
            node(8, "Australia"),
            node(9, "America"),
            node(10, "America"),
            node(11, "Sifra11"),
        ],
        ways: vec![
            way(1,1,2, "Europe"),
            way(2,2,3, "Europe,Africa"),
            way(3,3,4, "Africa"),
            way(4,4,5, "Africa,Asia"),
            way(5,5,6, "Asia"),
            way(6,6,7, "Asia,Australia"),
            way(7,7,8, "Australia"),
            way(8,8,9, "Australia,America"),
            way(9,9,10, "America"),
            way(10,1,11, "Sifra11")
        ]
    };

    let new_pois = filter_pois_by_tag(pois, &items)?;
    assert_eq!(new_pois, expected_pois);
    Ok(())
}

#[test]
fn tmou22_filter_returns_europe_sifra11_for_level0_sifra11() -> TmouResult<()> {
    let pois = default_pois();
    let items = api::Items{
        items: vec![
            api_item("puzzle", 0, "puzzles-11")
        ]
    };

    let expected_pois =     api::Pois{
        nodes: vec![
            node(1, "Europe"),
            node(2, "Europe"),
            node(11, "Sifra11"),
        ],
        ways: vec![
            way(1,1,2, "Europe"),
            way(10,1,11, "Sifra11")
        ]
    };

    let new_pois = filter_pois_by_tag(pois, &items)?;
    assert_eq!(new_pois, expected_pois);
    Ok(())
}
