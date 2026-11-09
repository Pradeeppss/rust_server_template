use sqlx::{ Pool, Sqlite, sqlite};


pub async fn sqlite_db() -> Result<Pool<Sqlite>, sqlx::Error> {
    let opt = sqlite::SqliteConnectOptions::new().filename("test.db").create_if_missing(true);

    let connection = sqlite::SqlitePool::connect_with(opt).await?;

    create_tables(&connection).await?;

    Ok(connection)
}

async fn create_tables(connection: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query("
        CREATE TABLE IF NOT EXISTS  tasks (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        title TEXT,
        done BOOLEAN DEFAULT 0)
    ").execute(connection).await?;

    Ok(())
}