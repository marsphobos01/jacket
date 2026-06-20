# jacket

A small CLI tool that fetches album artwork given an artist and album name.

## What it does

```
jacket --artist "Tame Impala" --album "Currents"
```

Looks up the album, grabs the artwork, and saves it to disk. No accounts, no API keys, no nonsense.

## Why

Most "album art downloader" tools are bloated GUI apps or buried inside larger music management suites. This is just the one thing, done simply, from the terminal.

## Usage

```
jacket --artist <ARTIST> --album <ALBUM> [--out <DIR>]
```

| Flag | Description | Default |
|------|-------------|---------|
| `--artist` | Artist name | required |
| `--album` | Album name | required |
| `--out` | Output directory | current directory |

## Building

```
cargo build --release
```

## License

Personal project, no license applied yet.
