use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Todo {
    pub title: String,
    pub done: bool,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo { title, done: false }
    }

    pub fn done(&mut self) {
        self.done = true;
    }

    pub async fn save(&self, conn: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO todo (title, done)
            VALUES (?, ?)
            "#,
        )
        .bind(&self.title)
        .bind(self.done)
        .execute(conn)
        .await?;
        Ok(())
    }
}
