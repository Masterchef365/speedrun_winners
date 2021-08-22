use anyhow::Result;
use speedruns::juniper::cli::read_table;
use speedruns::models::{Game, Run, User, RunPlayer};
use std::collections::HashMap;

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
    println!("Reading users table");
    let users: Vec<User> = read_table("../speedruns/data/imported/users.jsonl").unwrap();

    // Convert users into a map of ID to username
    println!("Building username table");
    let usernames: HashMap<u64, String> = users.into_iter().map(|u| (u.id, u.name)).collect();

    // Search for the game id of the given slug
    println!("Finding game...");
    let game = match games.iter().find(|g| g.slug == game_slug) {
        Some(id) => id,
        None => {
            println!("Slug {} not found!", game_slug);
            return Ok(());
        }
    };

    dbg!(&game.primary_timing);

    // Select runs for just this slug
    println!("Filtering runs...");
    let mut selected_runs: Vec<Run> = runs
        .iter()
        .filter(|r| r.game_id == game.id)
        .filter(|r| r.times_ms.get(&game.primary_timing).is_some())
        .filter(|r| r.created.is_some())
        .cloned()
        .collect();

    dbg!(selected_runs.len());

    // Sort by ascending date
    println!("Sorting runs...");
    selected_runs.sort_unstable_by_key(|r| r.created().unwrap());
    //selected_runs.sort_by_key(|r| r.created().unwrap());

    // Run through the date-ordered runs and check if each is a winner, if so update `best`
    println!("Discovering winning runs...");
    let mut winners = vec![];
    let mut best = u64::MAX;

    for run in selected_runs {
        let time = run.times_ms.get(&game.primary_timing).unwrap();
        if time < best {
            winners.push(run);
            best = time;
        }
    }

    // Display winners
    let unknown = "unknown".to_string();
    for run in winners {
        let time = run.times_ms.get(&game.primary_timing).unwrap();
        let names = run.players.iter().map(|p| match p {
            RunPlayer::UserId(id) => usernames.get(id).unwrap_or(&unknown),
            RunPlayer::GuestName(g) => g,
        }).collect::<Vec<&String>>();
        println!("Winner: {}ms by {:?} on {}", time, names, run.created.unwrap());
    }

    Ok(())
}
