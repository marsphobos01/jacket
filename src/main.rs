use clap::Parser;
use serde::Deserialize;
#[derive(Parser)]
struct Args {
    #[arg(long)]
    artist: String,

    #[arg(long)]
    album: String,

    #[arg(long, default_value = ".")]
    out: String,

    #[arg(long)]
    save_dir: Option<String>,
}

#[derive(Deserialize)]
struct Release {
    id: String,
    title: String,
}

#[derive(Deserialize)]
struct SearchResult {
    releases: Vec<Release>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let config_path = dirs::config_dir()
        .ok_or("Could not find config directory.")?
        .join("jacket")
        .join("config");

    if let Some(dir) = &args.save_dir {
        std::fs::create_dir_all(config_path.parent().unwrap())?;
        std::fs::write(&config_path, dir)?;
        println!("Saved default directory: {}", dir);    }
    let url = format!(
        "https://musicbrainz.org/ws/2/release/?query=artist:{} AND release:{}&fmt=json",
        args.artist, args.album
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "jacket/0.1 ( morganbennett100@gmail.com )")
        .send()?;
    let body = response.text()?;
    let result: SearchResult = serde_json::from_str(&body)?;
    for (i, release) in result.releases.iter().enumerate() {
        println!("{}: {} ({})", i + 1, release.title, release.id);
    }
    println!("Please enter a number to choose your release:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let choice = input.trim().parse::<usize>()?;
    let index = choice - 1;
    let Some(release) = result.releases.get(index) else {
        println!("Invalid choice!");
        return Ok(());
    };
    let art_url = format!("https://coverartarchive.org/release/{}/front", release.id);
    let art_bytes = client.get(&art_url).send()?.bytes()?;
    let path = format!("{}/{} - {}.jpg", args.out, args.artist, release.title);
    std::fs::write(&path, art_bytes)?;
    println!("Your cover art was saved to '{}'.", path);
    Ok(())
}
\
