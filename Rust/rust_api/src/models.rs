#[derive(serde::Deserialize, Insertable, Debug)]
#[diesel(table_name = crate::schema::todos)]
pub struct NewTodo {
    pub title: String,
    pub body: String,
}
