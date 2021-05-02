#[allow(unused_imports)]
use crate::models::errors::*;
#[allow(unused_imports)]
use crate::game_data_reader::*;
#[allow(unused)]
use std::env::current_dir;

#[test]
fn import_game_data() -> TmouResult<()> {
    let fname = current_dir()?.join("sample_game_data.xml");
    let game_data = read_game_data(fname.to_str().unwrap())?;
    assert_eq!(game_data.items.len(), 17);
    assert_eq!(game_data.nodes_items.len(), 17);
    assert_eq!(game_data.bonuses.len(), 10);
    Ok(())
}
