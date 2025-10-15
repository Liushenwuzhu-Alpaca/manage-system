use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub score: i32,
}


impl PartialEq for Student {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.name == other.name
    }
}