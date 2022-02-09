use diesel::prelude::*;

use crate::models::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_cat_by_id(cat_id: i32, conn: &PgConnection) -> Result<Option<Cat>, DbError> {
    use crate::schema::cats::dsl::*;

    let cat = cats.filter(id.eq(cat_id)).first::<Cat>(conn).optional()?;

    Ok(cat)
}
