# Fretwise

A guitar practice toolkit that runs in the browser. Everything lives in one file, `index.html`: no build step, no install, no account. Your settings and your song are saved in the browser.

**Live site:** GitHub Pages for this repo (Settings → Pages → Source: GitHub Actions).

## What's inside

| Page | What it does |
|---|---|
| **Fretboard** | Any scale, mode or triad on the neck, in any root and tuning. Practice by whole neck, one octave, the five boxes, or full-neck runs. A backing band (**Jam**, beta) plays your Song Builder progression or a stock one, rings each chord shape on the neck, and can play a lead line over it. |
| **Licks** (on the Fretboard page) | 15 original blues, rock and country licks, drawn as tab and on the neck in your key. Slow them down, loop them, or trade them with the band: it plays the lick, you answer. |
| **Chord Finder** | Tap notes on the neck (open or muted strings too) and get the chord name, plus what to add to complete it. |
| **Chord Shapes** | Every shape of a chord up to fret 15, grouped by position, with inversions, finger numbers, a CAGED filter and a triads view. |
| **Song Builder** | Type or tap chords. You get the key, Roman numerals, a key wheel of chords that fit, a play-along neck, nine strumming and fingerstyle patterns, time signatures, ties, and a chord builder on the neck. **My songs** saves and reopens songs. |
| **Tab Player** (beta) | Open a plain-text tab and play it back. |
| **Tuner** | 26 tunings and a microphone tuner. Hidden from the menu for now; open `#tuner`. |

Note names switch between English (C D E) and Italian (Do Re Mi). Dark and light themes, seven fretboard woods, left-handed and tab-style neck views.

## Save songs in a database (local Rust server)

```sh
cd server
cargo run --release
```

Then open http://127.0.0.1:8080. Songs go into a SQLite file on your computer instead of the browser. See [server/README.md](server/README.md) for options and the API. Without the server, the app saves songs in the browser.

## Docs

- [User guide](docs/USER_GUIDE.md): every page and feature, how to use it.
- [Licks](docs/LICKS.md): the lick list and the format for adding your own.
- [Developer notes](docs/DEVELOPER.md): how the file is organised, state, audio, testing.

## Run locally

Open `index.html` in a browser, or run the server above. The microphone tuner needs `https://` or `localhost` (the server is on localhost, so it works there).

## Deploy

Pushing to `main` deploys to GitHub Pages via `.github/workflows/pages.yml`.
