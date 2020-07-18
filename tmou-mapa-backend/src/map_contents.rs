use super::errors::*;
use super::db_models::*;



pub fn get_contents_of_node(node_id: &String) -> TmouResult<NodeContents>
{
    match node_id.as_str()
    {
        "539563487" => Ok(puzzle("7-mysi_a_syr.pdf")),
        _=> Ok(nothing())
    }
}

fn puzzle(url: &str) -> NodeContents
{
    NodeContents{r#type:"Puzzle".to_string(), data:format!("/static/puzzles/{}",url.to_string())}
}

fn nothing() -> NodeContents
{
    NodeContents{r#type:"None".to_string(), data:"Tady nic není".to_string()}
}