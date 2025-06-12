use sqlx:: PgPool;


use crate::hash;
use crate::handle_db;


pub async fn crypt_pass(args: Vec<String>, db: &PgPool, gem: &str, insert: bool) -> Result<(), sqlx::Error>
{
    let (key, data);
    let hash_value: i32;
    let hex_val;

    if insert == true {
        (key, data) = (&args[3], &args[4]);
        hash_value = hash::pre_hash(key, gem);
    
        hex_val = hash::crypt_data(data, true);
        if let Some(hex_data) = hex_val {
            handle_db::add_to_db(hash_value, &hex_data, db).await?;
        }
    } else {
        (key, data) = (&args[3], &args[4]);
        hash_value = hash::pre_hash(key, gem);

        hex_val = hash::crypt_data(data, true);
        if let Some(hex_data) = hex_val {
            handle_db::update_row(hash_value, &hex_data, &db).await?;
        }
    }
    Ok(())
}
