# Developer notes

Fretwise is one self-contained HTML file (`index.html`): markup, CSS and JavaScript, no framework, no build. Fonts come from Google Fonts; everything else is inline.

## Layout of the file

1. `<style>`: design tokens on `:root` (dark first), light theme under `html.fw-light`, fretboard woods via `html[data-wood]`.
2. Markup: header with the menu, then one `<section class="view" id="v-…">` per page (`explore`, `finder`, `keys`, `song`, `tabs`, `tuner`). Only one is visible; `show(view)` switches and sets `location.hash`.
3. One `<script>` with, roughly in order:
   - **Theory**: note spelling (`pcOf`, `spellIv`, `pretty` for Italian names), the chord dictionary `CH`, `parseChord`, interval table `IV`.
   - **Voicings**: `voicings()` generates playable shapes; `diagramSVG()` draws chord boxes; `bestVoicingMidis/Frets` pick the easiest.
   - **Audio**: `ac()` builds the master chain (EQ, compressor, convolution reverb). `ksPlay()` is a cached Karplus-Strong string with bends, releases, slides, vibrato and legato; `pluck()` and `strum()` wrap it.
   - **Neck**: `boardSVG()` draws any neck (geometry in `NECK_G`: nut at x = 82, 56 px per fret, 24 px per string); `mirrorSVG()` makes the left-handed view.
   - **Fretboard page**: `renderExplorer()`, practice modes (`scaleBoxes`, `fullRuns`, `octGroups`), the sequencer (`SEQ`), the jam engine (`JAM`, `jamTick`, `jamPattern`, `jamMark`, `jamLeadBar`), the lick library (`LICKS`, `lickLayout`, `lickTabSVG`, `lickSchedule`, `renderLicks`).
   - **Chord Finder**: `finderSVG`, `identify`.
   - **Song Builder**: `renderSong`, key detection `detectKey`, chord analysis `classify`, key wheel `renderCompass`, play-along `renderSongNeck` and builder `renderSongBuild`, playback styles `sgPattern` with time signatures `SGT`, drag and drop (`makeSortable`, plus the drag-in handler on `[data-add]`).
   - **Chord Shapes**: `renderKeys`, `cagedOf`, `triadShapes`.
   - **UI helpers**: `$`, `$$`, `el(tag, attrs, html)`, tips (`TIPS`, `tipBtn`), chord pop-ups, `helpBox` for collapsible help.

## State

Everything the user sets is in the object `S`, saved to `localStorage['fretwise']` by `save()`. Defaults are in `DEF`. The song itself is the text `S.song`; chips, analysis and playback are derived from it every time with `curChords()`, so the text box and the chips can never disagree.

Song tokens can carry extras: `~` prefix = tied bar, `@x07555` suffix = exact shape. `parseChord` returns `{pc, type, bpc, raw, tied, shape}`.

## Server and song library

`server/` is a Rust crate (Axum + rusqlite) that serves `index.html` and a JSON API for songs and a settings backup; see `server/README.md`. In the page, `libInit()` calls `/api/health`: if it answers, the library (`LIB`, `libSave`, `libOpen`, `libDelete`, `libImport`, `libExport`) talks to `/api/songs`; otherwise it uses `localStorage['fretwise-songs']` with the same record shape. The open song is `S.songId` / `S.songTitle`; "unsaved" compares `songPayload()` with the stored record.

## Audio timing

All sound is scheduled on the Web Audio clock (`ac().currentTime`). Visual highlights use `setTimeout` to the same time. The jam runs a 30 ms look-ahead loop (`jamTick`) that schedules whole bars 350 ms ahead.

## Testing

The server has a test suite: `cd server && cargo test`. The page has no test suite in the repo. Changes were checked with Playwright scripts that load the page at 1400 px and 390 px (phone), fail on any page error, check there is no horizontal overflow, click through features and take screenshots. A minimal check:

```python
from playwright.sync_api import sync_playwright
with sync_playwright() as pw:
    page = pw.chromium.launch().new_page(viewport={'width': 390, 'height': 844})
    errors = []; page.on('pageerror', lambda e: errors.append(str(e)))
    page.goto('file:///path/to/index.html#song'); page.wait_for_timeout(800)
    assert not errors, errors
    assert page.evaluate('document.documentElement.scrollWidth') <= 390
```
