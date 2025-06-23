use sqlx:: {Row, PgPool};
use std::fs;
use std::path::Path;
use std::collections::HashSet;


pub async fn update_row(hash_value: i32, data:&String, db: &PgPool) -> Result<(), sqlx::Error>
{
    let check_row = sqlx::query("SELECT data FROM hashes WHERE hash_value = $1")
                .bind(hash_value)
                .fetch_optional(db)
                .await?;

    if check_row.is_none() {
        println!("Row doesn't exist");
        return  Ok(());
    }

    sqlx::query("UPDATE hashes SET data = $1 WHERE hash_value = $2")
            .bind(data)
            .bind(hash_value)
            .execute(db)
            .await?;
    println!("The data was successfully replaced with {} !", data);
    Ok(())
}


pub async fn show_db(table: &String, db: &PgPool) -> Result<(), sqlx::Error>
{
    let banned_tables = HashSet::from(["admin", "ADMIN", "passwords", "PASSWORDS", "users", "USERS"]);

    if banned_tables.contains(table.as_str()) {
        println!("BANNED TABLE DETECTED");
        return  Ok(());
    }
    let check_table = sqlx::query_scalar::<_, bool>("SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_schema = 'public' AND table_name= $1)")
        .bind(table)
        .fetch_one(db)
        .await?;

    if !check_table {
        println!("Table '{}' does not exist.", table);
        return Ok(());
    }

    let query = format!("SELECT * FROM {}", table);
    let rows = sqlx::query(&query)
        .fetch_all(db)
        .await?;

    for row in rows {
        let id: i32 = row.try_get("id")?;
        let hash_value: i32 = row.try_get("hash_value")?;
        let data: String = row.try_get("data")?;
        println!("id: {}, hash_value: {}, data: {}", id, hash_value, data);
    }
    Ok(())
}


pub async fn delete_from_db(key: i32, db: &PgPool) -> Result <(), sqlx::Error>
{

    let check_exist = sqlx::query("SELECT hash_value FROM hashes WHERE hash_value = $1")
        .bind(key)
        .fetch_optional(db)
        .await?;

    if check_exist.is_none() {
        println!("Key: {} does not exist", key);
        return  Ok(());
    }

    sqlx::query("DELETE FROM hashes WHERE hash_value = $1")
            .bind(key)
            .execute(db)
            .await?;
    println!("{} was removed from the hashes table", key);
    Ok(())
}


pub async fn get_db_row(hash_value: i32, db: &PgPool) -> Result< Option<String>, sqlx::Error>
{
    let row = sqlx::query("SELECT data FROM hashes WHERE hash_value = $1")
            .bind(hash_value)
            .fetch_optional(db)
            .await?;
    
    if let Some (row) = row {
        let data: String = row.get("data");
        println!("found: {}", data);
        Ok(Some(data))
    } else {
        println!("No data found for hash_value {}", hash_value);
        return Err(sqlx::Error::RowNotFound);
    }
}


pub async fn add_to_db(hash_value: i32, data: &String, db: &PgPool) -> Result<(), sqlx::Error>
{
    sqlx::query( "INSERT INTO hashes (hash_value, data) VALUES ($1, $2)")
        .bind(hash_value)
        .bind(data)
        .execute(db)
        .await?;

    println!("Inserted into DB:\n  Hash: {}\n  Data: {}", hash_value, data);
    Ok(())
}


pub async fn init_data_base(db: &PgPool) -> Result<(), sqlx::Error>
{
    let db_path = Path::new("src/data_base.sql");
    let db_script = fs::read_to_string(db_path).expect("Failed to read schema");

    sqlx::query(&db_script).execute(db).await?;
    Ok(())
}
