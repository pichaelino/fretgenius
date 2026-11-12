# Licks

The lick library on the Fretboard page. All licks are original phrases written for Fretwise in common blues, rock and country styles; none is a transcription of a recording.

## The list

All examples are in A (root at fret 5 on the low E string). In the app every lick moves to the key you pick.

| Lick | Fits | Over | Level | Teaches |
|---|---|---|---|---|
| Box 1 opener | minor | I | ● | Walking down Box 1, a whole-step bend, vibrato |
| Bend, release, resolve | minor | any | ● | Bending in tune and releasing in time |
| Box 1 climb | minor | IV | ● | Alternate picking up Box 1 |
| Blue-note grind | minor | IV | ● | Hammer-on and pull-off through the ♭5 |
| Unison bend | minor | I | ●● | Bending to match a held note |
| Upper blues box | minor | IV | ●● | The small box five frets above Box 1 |
| Slide up to Box 2 | minor | any | ●● | Slides that change position |
| Six-note repeat | minor | any | ●●● | Fast groups of three over a beat of four |
| Slow-blues cry | minor | I | ●●● | Long bends, half-step "blue" bend, space |
| Classic turnaround | any | last bar | ●● | Chromatic line against a ringing root |
| Rock 'n' roll double-stops | major | I | ● | Two-string stabs and a sliding 6th |
| Major pentatonic run | major | I | ●● | A fast major-pentatonic descent, bend to the 3rd |
| Minor-to-major hammer | major | I | ● | ♭3 → 3 hammer-on |
| Pedal-steel bend | major | I | ●●● | Bending one string under a held note |
| Mixolydian ♭7 lick | major | IV | ●● | Adding the ♭7 to major pentatonic |

**Fits** decides which licks are offered first: *minor* licks for minor scales, pentatonic and blues; *major* licks for major scales and modes. A lick of the other kind is played in the relative key, so its notes still match the scale on screen.

**Over** is used by the jam's *Licks* lead: on the I chord it picks I or any-chord licks, on IV it picks IV licks, on V it also allows the turnaround.

## Format

Licks live in the `LICKS` array in `index.html`. Each note is one token:

```
string:fret:length[:technique]
```

| Part | Meaning |
|---|---|
| `string` | 1 = high e … 6 = low E |
| `fret` | Offset from **R**, the fret of the key's root on the low E string (A → 5, so `0` = fret 5, `3` = fret 8, `-1` = fret 4) |
| `length` | In 16th notes: 1 = 16th, 2 = 8th, 4 = beat, 16 = a 4/4 bar. Use `0` with `+` |

| Technique | Meaning |
|---|---|
| `b1` `b2` `b3` | Bend up ½, 1 or 1½ steps |
| `br2` | Bend up a whole step and release in the same note |
| `r2` | Start pre-bent a whole step and release |
| `h` / `p` | Hammer-on / pull-off from the previous note on that string |
| `/` / `\` | Slide up / down into the note |
| `~` | Vibrato (can be combined: `b2~`) |
| `+` | Played together with the previous note (double-stop), length 0 |

Example, the Box 1 opener:

```
1:3:2 1:0:2 2:3:2 2:0:2 3:2:2:b2 3:0:2 4:2:4:~
```

In A: high e fret 8 → fret 5, B string fret 8 → fret 5, G string fret 7 bent a whole step, fret 5, D string fret 7 with vibrato. Eighth notes, then a quarter note: 16 sixteenths, one bar.

### Adding a lick

```js
{id:'mylick', name:'My lick', q:'minor', lvl:2, ctx:'any', bars:1, style:'Blues',
 learn:'One or two sentences on what it teaches.',
 n:'…tokens…'}
```

- `q`: `minor`, `major` or `any`. `ctx`: `I`, `IV`, `V`, `turn` or `any`. `lvl`: 1–3.
- Make the lengths add up to 16 per bar (`bars` × 16).
- The app picks R so the lick stays between frets 0 and 22 in every key, and moves it for other tunings.
