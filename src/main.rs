mod expect_exit;
mod mojang_options;

use std::env::args;
use std::fmt::Write;
use std::io::{ErrorKind, Result};
use std::path::{Path, PathBuf};

use crate::expect_exit::ExpectExit;
use crate::mojang_options::mojang_options;
use rusty_leveldb::{DBIterator, LdbIterator, Options, DB};

static ACTOR_PREFIX_HEADER: &str = "actorprefix";

enum EditMode {
    Print,
    Revive,
}

fn main() -> Result<()> {
    println!("invincible-reviver");

    let args: Vec<String> = args().collect();

    let world_dir: &Path = Path::new(
        args.get(1)
            .expect_exit("Please pass the world folder you'd like to modify"),
    );

    let mode: EditMode = match args
        .get(2)
        .expect_exit("Please specify the action you'd like to make; '--print' or '--revive'")
        .as_str()
    {
        "--print" => Ok(EditMode::Print),
        "--revive" => Ok(EditMode::Revive),
        _ => Err(ErrorKind::InvalidInput),
    }
    .expect_exit("Invalid action; '--print' or '--revive'");

    let db_dir: PathBuf = world_dir.join("db");

    let mut options: Options = mojang_options();
    options.create_if_missing = false;

    println!("Opening world {:?}\n", world_dir);

    let mut db: DB = DB::open(db_dir, options).expect_exit("Failed to open database");

    match mode {
        EditMode::Print => print_mode(&mut db)?,
        EditMode::Revive => revive_mode(&mut db)?,
    }

    db.close().expect_exit("Failed to close database");

    Ok(())
}

fn print_mode(db: &mut DB) -> Result<()> {
    let mut iter: DBIterator = db
        .new_iter()
        .expect_exit("Failed to create database iterator");
    iter.seek_to_first();

    println!("Searching for entity entries...");

    while iter.valid() {
        let (key, value): (Vec<u8>, Vec<u8>) = match iter.next() {
            Some(entry) => entry,
            None => break,
        };

        if !key.starts_with(ACTOR_PREFIX_HEADER.as_bytes()) {
            continue;
        }

        let key_id: &[u8] = &key[ACTOR_PREFIX_HEADER.as_bytes().len()..];
        let key_id_str: String = to_hex_string(&key_id);
        let key_str: String = format!("{}_{}", ACTOR_PREFIX_HEADER, key_id_str);

        println!("{}", key_str);

        // print!("{:?}", value);
    }

    Ok(())
}

fn revive_mode(db: &mut DB) -> Result<()> {
    Ok(())
}

fn to_hex_string(bytes: &[u8]) -> String {
    let mut hex_string: String = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut hex_string, "{:02x}", byte).unwrap();
    }
    hex_string
}
