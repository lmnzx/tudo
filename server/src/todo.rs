use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}
