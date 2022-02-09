use diesel::prelude::*;

use crate::models::*;

type DbError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_cat_by_id(cat_id: i32, conn: &PgConnection) -> Result<Option<Cat>, DbError> {
    use crate::schema::cats::dsl::*;

    let cat = cats.filter(id.eq(cat_id)).first::<Cat>(conn).optional()?;

    Ok(cat)
}

pub fn create_new_cat(
    cat_name: &str,
    cat_color: &str,
    conn: &PgConnection,
) -> Result<Cat, DbError> {
    use crate::schema::cats::dsl::*;

    let new_cat = NewCat {
        name: cat_name.to_owned(),
        color: cat_color.to_owned(),
    };

    let cat = diesel::insert_into(cats)
        .values(&new_cat)
        .get_result(conn)
        .expect("Error creating cat");

    Ok(cat)
}
