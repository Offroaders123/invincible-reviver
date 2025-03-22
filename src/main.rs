mod expect_exit;
mod hex_string;
mod mojang_options;
mod nbt_files;
mod world_backup;
mod zip;

use std::env::args;
use std::fs::remove_file;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

use crate::expect_exit::ExpectExit;
use crate::hex_string::HexString;
use crate::mojang_options::mojang_options;
use crate::nbt_files::{read_nbt, write_nbt};
use crate::world_backup::create_world_backup;
use nbt::{Blob, Value};
use rusty_leveldb::{DBIterator, LdbIterator, Options, DB};

static ACTOR_PREFIX_HEADER: &str = "actorprefix";

#[derive(Debug)]
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

    println!("Mode: {:?}\n", mode);

    let backup: bool = !args.iter().any(|arg| arg == "--no-backup");

    match mode {
        EditMode::Revive => {
            if backup {
                create_world_backup(world_dir).expect_exit("Could not create world backup");
            } else {
                println!("<backup skipped>");
            }
        }
        _ => (),
    };

    let db_dir: PathBuf = world_dir.join("db");

    let mut options: Options = mojang_options();
    options.create_if_missing = false;

    println!("Opening world {:?}\n", world_dir);

    let mut db: DB = DB::open(&db_dir, options).expect_exit("Failed to open database");

    match mode {
        EditMode::Print => print_mode(&mut db).expect_exit("Failed to run print mode"),
        EditMode::Revive => revive_mode(&mut db).expect_exit("Failed to run revive mode"),
    }

    db.close().expect_exit("Failed to close database");

    let lock_path: PathBuf = db_dir.join("LOCK");

    // This is temporary hopefully, seems to be a bug with `rusty-leveldb`.
    // https://github.com/dermesser/leveldb-rs/issues/54
    remove_file(lock_path).expect_exit("Could not remove database LOCK file");

    println!("\nDone!");

    Ok(())
}

fn print_mode(db: &mut DB) -> Result<()> {
    println!("Searching for entity entries...");

    let entities: Vec<(Vec<u8>, Vec<u8>)> = find_entity_entries(db)?;

    for (key, value) in entities {
        match handle_entity(&key, value) {
            Err(err) => {
                eprintln!("{err}");
                continue;
            }
            _ => (),
        };
    }

    Ok(())
}

fn revive_mode(db: &mut DB) -> Result<()> {
    println!("Searching for entity entries...");

    let entities: Vec<(Vec<u8>, Vec<u8>)> = find_entity_entries(db)?;

    for (key, value) in entities {
        let (key_str, mut nbt, dead): (String, Blob, bool) = match handle_entity(&key, value) {
            Ok(value) => value,
            Err(err) => {
                eprintln!("{err}");
                continue;
            }
        };

        if !dead {
            continue;
        }

        println!("Reviving...");

        nbt.insert("Dead", Value::Byte(0))?;

        let recompile: Vec<u8> = write_nbt(&nbt)?;

        match db.put(&key, &recompile) {
            Err(err) => {
                eprintln!("DB writing issue for {:?}: {err}", key_str);
                continue;
            }
            _ => (),
        };
    }

    Ok(())
}

fn find_entity_entries(db: &mut DB) -> Result<Vec<(Vec<u8>, Vec<u8>)>> {
    let mut entities: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();

    let mut iter: DBIterator = db
        .new_iter()
        .expect_exit("Failed to create database iterator");
    iter.seek_to_first();

    while iter.valid() {
        let (key, value): (Vec<u8>, Vec<u8>) = match iter.next() {
            Some(entry) => entry,
            None => break,
        };

        if !key.starts_with(ACTOR_PREFIX_HEADER.as_bytes()) {
            continue;
        }

        entities.push((key, value));
    }

    Ok(entities)
}

fn handle_entity(key: &Vec<u8>, value: Vec<u8>) -> Result<(String, Blob, bool)> {
    let key_str: String = to_pretty_key(&key);

    let nbt: Blob = read_nbt(value).map_err(|err| {
        Error::new(
            ErrorKind::InvalidData,
            format!("NBT parsing issue for {:?}: {}", key_str, err),
        )
    })?;

    let dead: bool =
        get_dead_state(&nbt).map_err(|err| Error::new(err.kind(), format!("{key_str}: {err}")))?;

    let decorator: &str = if dead { "💀" } else { "🌱" };

    println!("{key_str} {decorator}");
    // println!("{:#?}", nbt);
    // println!("'Dead': {}", dead);

    Ok((key_str, nbt, dead))
}

fn to_pretty_key(key: &[u8]) -> String {
    let key_id: &[u8] = &key[ACTOR_PREFIX_HEADER.as_bytes().len()..];
    let key_id_str: &String = &key_id.to_hex_lowercase();
    let key_str: String = format!("{}_{}", ACTOR_PREFIX_HEADER, key_id_str);
    key_str
}

fn get_dead_state(nbt: &Blob) -> Result<bool> {
    match nbt.get("Dead") {
        Some(value) => match value {
            Value::Byte(value) => Ok(*value != 0),
            value => {
                let tag_name: &str = value.tag_name();
                Err(Error::new(ErrorKind::InvalidData, format!("'Dead' value is not the correct type, expected 'TAG_Byte', encountered '{tag_name}'")))
            }?,
        },
        None => Err(Error::new(ErrorKind::NotFound, "'Dead' key not found")),
    }
}
