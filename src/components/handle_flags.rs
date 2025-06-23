use sqlx:: PgPool;


use crate::hash;
use crate::handle_db;
use crate::handle_encryption;

pub async fn handle_insert(args: Vec<String>, db: &PgPool, gem: &str) -> Result<(), sqlx::Error>
{
    if &args[2] == "--encrypt" {
        handle_encryption::crypt_pass(args, &db, gem, true).await?;
    } else {
        let (key, data) = (&args[2], &args[3]);
        let hash_value: i32 = hash::pre_hash(key, gem);
    
        handle_db::add_to_db(hash_value, data, &db).await?;
    }
    Ok(())
}

pub async fn handle_remove(args: Vec<String>, db: &PgPool, gem: &str) -> Result<(), sqlx::Error>
{
    let key: &String = &args[2];
    let hash_value: i32 = hash::pre_hash(key, gem);

    handle_db::delete_from_db(hash_value, &db).await?;
    Ok(())
}

pub async fn handle_find(args: Vec<String>, db: &PgPool, gem:&str) -> Result<(), sqlx::Error>
{
    let key: &String = &args[2];
    let hash_value: i32 = hash::pre_hash(key, gem);

    handle_db::get_db_row(hash_value, &db).await?;
    Ok(())
}

pub async fn handle_show(args: Vec<String>, db: &PgPool) -> Result<(), sqlx::Error>
{
    let table: &String = &args[2];

    handle_db::show_db(table, &db).await?;
    Ok(())
}

pub async fn handle_update(args: Vec<String>, db: &PgPool, gem: &str) -> Result<(), sqlx::Error>
{
    if &args[2] == "--encrypt" {
        handle_encryption::crypt_pass(args, db, gem, false).await?;
    } else {
        let (key, data) = (&args[2], &args[3]);
        let hash_value: i32 = hash::pre_hash(key, gem);
    
        handle_db::update_row(hash_value, data, &db).await?;
    }
    Ok(())
}
