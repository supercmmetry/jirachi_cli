mod schema;
mod seed;

#[macro_use]
extern crate diesel;

use schema::seeds::dsl::*;
use diesel::{RunQueryDsl, PgConnection, Connection};
use dotenv;
use std::env;
use crate::seed::Seed;

fn add(conn: &PgConnection, other_prefix: String) -> anyhow::Result<()> {
    diesel::insert_into(seeds)
        .values(&Seed {
            prefix: other_prefix,
            index: 0
        }).execute(conn)?;

    Ok(())
}

fn print_usage() {
    println!("Usage: \n\
    jirachi [add] key1 key2 ...\n\
    \n\
    add: Adds one or more keys to the jirachi database")
}


fn main() {
    dotenv::dotenv().ok();

    let db_url = env::var("JIRACHI_DB_URL");
    if db_url.is_err() {
        println!("error: JIRACHI_DB_URL was not found in the environment.");
        return;
    }

    let conn = PgConnection::establish(db_url.unwrap().as_str());

    if let Err(e) = conn {
        println!("error: Connection to database failed .. \n{}", e);
        return;
    }

    let conn = conn.unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    match args[1].as_str() {
        "add" => {
            for i in 2..args.len() {
                if let Err(e) = add(&conn, args[i].clone()) {
                    println!("error: Could not add key \n{}", e);
                    return;
                }
                println!("success: Added key {} to jirachi database.", args[i]);
            }
        },
        _ => print_usage()
    }

}
