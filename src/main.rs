use std::env;
use std::process;
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;



mod handle_db;
mod hash;
mod help;
mod handle_flags;
mod handle_encryption;


pub const GEM: &str = "Phosphophyllite";


#[tokio:: main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    dotenv().ok();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter a flag");
        process::exit(-1);
    }

    let command: &str= &args[1];
    let db= PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL")?)
        .await?;
    handle_db::init_data_base(&db).await?;

    match command {
        "insert" => {
            if args.len() >= 4 {
                handle_flags::handle_insert(args, &db, GEM).await?;
            } else {
                println!("Please enter a key and data");
            }
        }
        "remove" => {
            if args.len() >= 3 {
                handle_flags::handle_remove(args, &db, GEM).await?;
            } else {
                println!("Please enter a key to delete from the table");
            }
        }
        "find" => {
            if args.len() >= 3 {
                handle_flags::handle_find(args, &db, GEM).await?;
            } else {
                println!("Please enter a key");
            }
        }
        "show" => {
            if args.len() >= 3 { 
                handle_flags::handle_show(args, &db).await?;
            } else {
                println!("Please enter a table to show");
            }
        }
        "update" => {
            if args.len() >= 4 {
                handle_flags::handle_update(args, &db, GEM).await?;
            } else {
                println!("Please enter key and data to update");
            }
        }
        _ => {
            help::help_flag();
        }
    }
    Ok(())
}
