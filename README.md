# Fretwise

A single-page guitar toolkit. Everything lives in `index.html`: no build step, no dependencies.

- **Fretboard**: scales, modes, chords and triads on the neck in any root and tuning. Intervals are colour-coded, with an octave selector and same-pitch finder.
- **Chord Finder**: place notes on the neck (mark strings open or muted) and get the chord name plus suggestions to complete it.
- **Chord Shapes**: pick a note and a chord to see every shape on the neck for it and all its variants, grouped by position, with optional inversions.
- **Tuner**: 26 tunings (standard, drop, open, 7-string, bass), reference tones and a live microphone tuner. A4 = 440 Hz, adjustable.
- **Circle of Fifths**: an interactive circle showing key signatures, diatonic chords and explanations.
- **Song Builder**: type chords to get the key, Roman numerals, borrowed and secondary chords, next-chord ideas, scales to solo with and starter progressions.

## Run locally

Open `index.html` in a browser. The microphone tuner needs `https://` or `localhost`. GitHub Pages provides https.

## Deploy

Pushing to `main` deploys to GitHub Pages via `.github/workflows/pages.yml`.
