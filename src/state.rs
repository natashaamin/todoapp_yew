use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};
use yew::html::IntoPropValue;
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
        pub entries: Vec<Entry>,
        pub filter: Filter,
}

impl State {
    pub fn total(&self) -> usize {
        self.entries.len()
    }

    pub fn total_completed(&self) -> usize {
        self.entries.iter().filter(|e| Filter::Completed.fits(e)).count()
    }
 }

 #[derive(Debug, Serialize, Deserialize)]
 pub struct Entry {
    pub description: String,
    pub completed: bool,
    pub editing: bool,
 }

 #[derive(Debug, Clone, Copy, EnumIter, Display, PartialEq, Serialize, Deserialize, Eq)]
 pub enum Filter {
    All,
    Active, 
    Completed
 }

 impl Filter {
    pub fn fits(&self, entry: &Entry) -> bool {
        match &self {
            Filter::All => true,
            Filter::Active => !entry.completed,
            Filter::Completed => entry.completed
        }
    }
 }