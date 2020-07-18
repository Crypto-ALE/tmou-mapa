use super::db_models::*;
use super::errors::*;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};


// for in-memory db
use std::fs::{File, read_to_string};
use std::io::prelude::*;




// database control holds the state of the game for teams
// the map data are handled by map module

pub trait DbControl
{
    fn new() -> Self;
    fn init(&mut self, conn: &str) -> TmouResult<()>;
    fn get_team(&self, phrase: &str) -> Option<Team>;
    fn put_team(&mut self, team: Team) -> TmouResult<()>;
    fn get_pois_for_team(&self, phrase: &str) -> Option<Vec<Poi>>;
    fn put_pois_for_team(&mut self, pois:Vec<Poi>) -> ();
}

#[derive(Serialize, Deserialize)]
pub struct MemoryDbControl
{
    teams: HashMap<String, Team>,
    pois: HashMap<String, Vec<Poi>>,
    filename: String
}

impl MemoryDbControl
{

    fn load(&mut self)->TmouResult<()>
    {
        // TODO: more concise way using and_then, etc.?
        match read_to_string(&self.filename)
        {
            Ok(str) => 
            {
                match serde_json::from_str(&str)
                {
                    Ok(obj) => 
                    { 
                        *self = obj;
                        Ok(())
                    }
                    _ => Ok(())
                }
            },
            _ => Ok(()) // swallow
        }
    }

    fn save(&self)->TmouResult<()>
    {
        let mut file = File::create(&self.filename)?;
        let serialized = serde_json::to_string(self).unwrap();
        file.write_all(&serialized.as_bytes())?;
        Ok(())
    }

}

impl DbControl for MemoryDbControl
{
    fn new() -> MemoryDbControl
    {
        MemoryDbControl{teams: HashMap::new(), pois: HashMap::new(), filename:"***".to_string()}
    }

    fn init(&mut self, conn: &str) -> TmouResult<()>
    {
        self.filename = conn.to_string();
        self.load()?;
        Ok(())
    }

    fn get_team(&self, phrase: &str) -> Option<Team>
    {
        let t = self.teams.get(phrase)?;
        Some(t.clone())
    }

    fn put_team(&mut self, team: Team) -> TmouResult<()>
    {
        let t = self.teams.entry(team.phrase.clone()).or_insert(Team::default());
        *t = team;
        self.save()?;
        Ok(())
    }

    fn get_pois_for_team(&self, phrase: &str) -> Option<Vec<Poi>>
    {   
        None
    }

    fn put_pois_for_team(&mut self, pois:Vec<Poi>) -> ()
    {
    }
}