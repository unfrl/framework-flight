use serde::{Deserialize, Serialize};

use crate::schema::cats;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Cat {
    pub id: i32,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewCat {
    pub name: String,
    pub color: String,
}
