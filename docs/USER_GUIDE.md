# Fretwise user guide

Fretwise has five pages in the menu: **Fretboard**, **Chord Finder**, **Chord Shapes**, **Song Builder** and **Tab Player** (beta). On a phone the menu is the bar at the bottom.

Things that work everywhere:

- **C D E / Do Re Mi** (top right) switches every note and chord name between English and Italian.
- **☀ / ☾** switches dark and light theme.
- **ⓘ** buttons explain guitar terms. Tap one to open it, tap anywhere else to close it.
- Hover or tap a chord name to see its shape in a pop-up.
- **▸ How to …** links open longer explanations. They stay open or closed the way you left them.
- **⚙ Neck & display settings** (above every neck): which string is on top, left-handed, fretboard wood, number of frets (21 = classic Fender, the default; 22, 24, 20 acoustic, 19 classical), fit the whole neck on screen, keep the neck in view while scrolling.

Everything you set is saved in this browser.

---

## Fretboard

Pick a **root**, then **Scales / Modes / Triads** and a **shape**. Every note lights up on the neck. **Notes** shows names; **Intervals** shows R, ♭3, 5… Tap a dot to hear it and ring the same pitch everywhere else on the neck.

### Practice

| Mode | What you see |
|---|---|
| Whole neck | Every note of the scale. |
| One octave | One run from root to root. Pick which octave. |
| Boxes | The hand positions guitarists solo in. Pick a box from the **Box** menu or with ◀ ▶, or tap a fret to jump to the box there. |
| Full-neck runs | One path from the lowest note to the highest, four route styles. |

**▶ Play** plays what's lit. Choose low → high, high → low, or up & down. The neck scrolls to follow the note.

### Jam (beta)

A backing band to improvise over.

- **Progression**: if you have chords in the Song Builder, **Your song** is at the top of the list and is picked automatically, with your shapes, BPM and time signature. Otherwise pick a progression made for the scale you chose.
- If your song is in a different key than the scale on screen, a hint offers to switch to the right scale or its pentatonic.
- **Style, time signature, feel, bars per chord, tempo, click, count-in.**
- While it plays, the **shape the band is playing is ringed on the neck** (amber rings, thicker on the root, × on muted strings), and **Now / next** shows the chord.
- **Lead over the jam**: a second guitar plays on the notes lit on the neck.
  - *Scale run*: up and down the scale.
  - *Chord tones*: only the notes of each chord.
  - *Improvise*: a new phrase every bar that lands on a chord note.
  - *Call & answer*: one bar of phrase, one bar for you.
  - *Licks (you answer)*: a lick that fits the chord, then a free bar to copy it.
- When the **Start jam** button scrolls off screen, a **player bar** appears at the bottom with play/stop, the current chord, the lead choice and the tempo.

### Licks

Below the jam. Fifteen short phrases in blues, rock and country styles, written for this app (not transcriptions of records).

1. Pick a lick card. The filter shows licks that fit your scale first (**Minor & blues** or **Major & country**), or **All 15**.
2. Read what it teaches and which techniques it uses.
3. The **tab** shows it: lines are strings, high e on top; numbers are frets; *full* / *½* = bend, *r* = release, *h* / *p* = hammer-on / pull-off, a slanted line = slide, a wavy line = vibrato. Tap a number to hear that note.
4. The **neck** shows the same notes as blue circles, numbered in playing order. A small arrow marks bent notes.
5. **▶ Play lick** plays it; the tab number and the neck circle light up together. **Speed** 50 / 75 / 100 % of the jam tempo, **Loop** repeats it.
6. **▶ Trade it over the jam** starts the band: it plays this lick, then leaves you a bar to answer. **Random licks instead** picks a fitting lick for each chord.

A minor lick in a major key is played in the relative minor (same notes), and the other way round. The card says so.

---

## Chord Finder

Tap frets to place notes, one per string. Tap a string name to play it open or mute it. The chord name appears with its notes; if it's not a full chord yet you get suggestions, and dashed rings show where the missing note could go. Double-click a suggestion to put it on the neck.

## Chord Shapes

Pick a **note** and a **chord family** (major, minor, dominant, diminished), then a **variant** or *All*.

- Shapes are grouped by position (open, frets 1–4, 5–8…), easiest first, with finger numbers and the CAGED shape they come from.
- **Include inversions** adds shapes with another note in the bass.
- **CAGED shape** (standard tuning): show only the C, A, G, E or D shape of the chord.
- **Show: Triads** switches to three-note shapes on neighbouring strings (6-5-4, 5-4-3, 4-3-2, 3-2-1), each in root position, 1st and 2nd inversion.

Tap a diagram to hear it.

## Song Builder

The page has four parts; the buttons at the top jump to each one.

### 1 · Write your song

**My songs** (top of the section): type a **title** and press **Save** (or Ctrl/⌘ + S). After that, Save updates the same song; **Save as new** keeps the old one and makes a copy. **● unsaved** means you changed something since the last save. **＋ New** starts an empty song. **My songs ▾** lists everything you saved: tap one to open it (chords, tempo, time signature, playing style and chosen key come back), **Delete** removes it (with Undo), **Export all / Import JSON** move songs to another browser or into the database.

Where songs are kept is shown under the title box:

- *Saved in the database on your computer* (green dot): you opened the app through the Fretwise server; songs are in a SQLite file. **Back up / Restore settings** also store all your app settings there.
- *Saved in this browser only*: the app was opened as a file or from the website; songs live in this browser's storage.

- **Type** chords separated by spaces (`Am F C G`), or **tap / drag** them from the key wheel below. Dragging drops the chord exactly where the blue line shows (on a phone, press and hold first).
- **Chips**: drag ⠿ to reorder, × to delete (with Undo). Under each chip:
  - ⧉ repeat it in a new bar,
  - ⌒+ hold it for one more bar (tied, no new strum),
  - ⌒← tie it to an identical chord before it; tap ⌒ again to untie.
- **Transpose** −1 / +1. **🔍 Find the key** jumps to the key panel.
- **Time** (4/4, 3/4, 6/8, 12/8, 2/4), **Play as** (strum, bass + chord, five arpeggios, Travis picking, folk pinch, waltz), **Legato** for picked patterns, **BPM**.
- When you scroll past **Play progression**, a **player bar** stays at the bottom.

**Chords that fit your key**: a simplified circle of fifths turned so your key is at the top. The lit segments are the key's chords with their numerals; amber dots mark chords already in the song. Tap or drag to add. **Double-tap** a segment to make it the song's key. Next to it the same chords are sorted by role (home, moves away, pulls home) plus *Spice* (borrowed and secondary chords). **Nearby keys** turns the wheel to a neighbouring key.

### 2 · Play it

The play-along neck shows the current chord and rings the next one. It stays pinned at the top while you scroll and flashes on each chord during playback. Below it, every chord's shape with ◀ ▶ to pick another one.

**✎ Build a chord**: place notes on the neck to make a chord. The name appears as you go. **+ Add** puts it at the end of the song in exactly that shape; **↻ Replace** swaps it for the chord you opened the builder from. *Same notes, other names* lets you add it under the name you mean (e.g. C6 instead of Am/C).

### 3 · Key & theory

The detected key with how sure it is and why, the scale notes, other possible keys, a chord-by-chord analysis (in key, borrowed, secondary dominant, outside), what could come next, scales to solo with, and starter progressions.

### 4 · Circle of fifths

The full circle. Tap a key to hear it and see its chords; **double-tap** to make it the song's key.

### Text shortcuts

| You type | Meaning |
|---|---|
| `Am7` `G/B` `Dsus4` | Any chord name, slash chords too. Italian names work (`Lam`, `Sol/Si`). |
| `~Am` | Tied: Am keeps ringing for another bar. |
| `Am@x07555` | Am played with exactly this shape (low E to high e; `x` = muted). Use dots when a fret is above 9: `A@x.0.7.6.5.5`. |

---

## Tab Player (beta)

Paste or open a plain-text tab. Sections can be played whole. Rhythm is approximated because text tabs don't store it. This page is still being worked on.

## Tuner (hidden for now)

Open `index.html#tuner`. 26 tunings, reference tones and a microphone tuner, A4 adjustable.
