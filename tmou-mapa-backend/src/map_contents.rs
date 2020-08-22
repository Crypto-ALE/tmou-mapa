use super::errors::*;
use super::db_models::*;



pub fn get_contents_of_node(node_id: i64) -> TmouResult<Vec<Item>>
{
    match node_id
    {
        539563487 => Ok(vec![puzzle("7-mysi_a_syr.pdf")]),
        _=> Ok(Vec::new())
    }
}

fn puzzle(url: &str) -> Item
{
    Item{
        r#type:"Puzzle".to_string(), 
        url:format!("/static/puzzles/{}",url.to_string()),
        label:"puzzles-1".to_string(),
        description:"šifry sady 1".to_string(),
        level:1,
    }
}
