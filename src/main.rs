use std::fs::File;
use std::io::BufReader;
use anyhow::Result;
use speedruns::models::{Game, User, Run};
use speedruns::juniper::cli::read_table;

fn main() -> Result<()> {
    // Parse args
    let mut args = std::env::args();
    let program_name = args.next().unwrap();
    let game_slug = match args.next() {
        Some(s) => s,
        None => {
            println!("Usage: {} <game slug>", program_name);
            return Ok(());
        }
    };

    // Read tables
    println!("Reading games table");
    let games: Vec<Game> = read_table("../speedruns/data/imported/games.jsonl").unwrap();
    println!("Reading runs table");
    let runs: Vec<Run> = read_table("../speedruns/data/imported/runs.jsonl").unwrap();
    //let users: Vec<User> = read_table("../speedruns/data/imported/users.jsonl").unwrap();

    // Search for the game id of the given slug
    let game = match games.iter().find(|g| g.slug == game_slug) {
        Some(id) => id,
        None => {
            println!("Slug {} not found!", game_slug);
            return Ok(());
        }
    };

    let selected_runs: Vec<Run> = runs.iter().filter(|r| r.game_id == game.id).cloned().collect();

    dbg!(selected_runs.len());

    Ok(())
}
