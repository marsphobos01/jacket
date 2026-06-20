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
    let Some(release) = result.releases.first() else {
        println!("No matches found.");
        return Ok(());
    };
    println!("Found: {} ({})", release.title, release.id);
    let art_url = format!("https://coverartarchive.org/release/{}/front", release.id);
    let art_bytes = client.get(&art_url).send()?.bytes()?;
    let path = format!("{}/cover.jpg", args.out);
    std::fs::write(&path, art_bytes)?;
    Ok(())
}
