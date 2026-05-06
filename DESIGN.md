# DESIGN.md

Visual identity for the gdidiot frontend.

## Direction

Refined minimal. Closer to Apple's own apps, Linear, Capacities, OpenAI's
ChatGPT desktop than to dashboard SaaS. Restraint is the point — character
comes from typography, spacing, and atmosphere, not from heavy chrome or
loud animation.

The app reads as a tool, not a brand showcase. The single bold choice is
a warm amber ambient gradient that anchors the dark canvas and gives the
whole surface a soft glow.

## What we are NOT

- No Inter / Roboto / system-font default look.
- No purple gradient on white. No "AI launchpad" template.
- No skeuomorphic depth. No glassy buttons. No neumorphism.
- No scattered micro-interactions (button bounces, input wiggles, etc.).
- No light mode until the warm ambient palette has a true light counterpart
  — NOT a token flip; the warmth would feel wrong on white.

## Tokens

Defined in `src/app.css`. Use the variables; never hardcode.

### Color

```
--bg-base          #0b0b0c               page base
--bg-glow-warm     orange/0.18           ambient glow, bottom-left
--bg-glow-cool     blue/0.05             ambient glow, top-right (counterweight)

--surface          rgba(22,22,24,0.62)   floating cards (frosted)
--surface-strong   rgba(28,28,30,0.88)   reserved for modals
--surface-hover    rgba(255,255,255,0.035)

--border           rgba(255,255,255,0.07)  default
--border-strong    rgba(255,255,255,0.12)  emphasis
--border-focus     rgba(255,138,76,0.45)   focused inputs

--text             rgba(255,255,255,0.94)
--text-dim         rgba(255,255,255,0.62)
--text-faint       rgba(255,255,255,0.38)
--text-whisper     rgba(255,255,255,0.22)

--accent           #ff8a4c               warm orange — single accent
--accent-soft      rgba(255,138,76,0.14) soft fills, focus rings, mark
```

**Single-accent rule**: orange only. If something needs to stand out, lean
on text contrast or borders before introducing a second hue. Status colors
(error red, success green) wait until a feature genuinely needs them.

### Typography

- **UI / body**: `Geist Variable` — distinctive but restrained. Fits the
  Apple/Linear feel without being SF or Inter.
- **Mono**: `Geist Mono Variable` — paths, kbd, mode badge, anything that
  reads as "data" or "code".
- `letter-spacing: -0.005em` baseline; `-0.012em` on the search input
  (display-sized text reads tighter).
- `font-feature-settings: "ss01", "cv11"` — Geist's open digits and
  alternate lowercase. Subtle but worth it.
- Avoid bold on body text; weight range is **350 / 400 / 500**.

### Radii

```
--radius-card    14px   floating surfaces
--radius-pill    6px    badges, kbd
items inside cards  10px (slightly less than the card)
```

### Shadow

One shadow recipe for every floating surface:

```
0 1px 0 rgba(255,255,255,0.04) inset,    /* top edge highlight */
0 24px 60px -20px rgba(0,0,0,0.55)        /* long, soft drop */
```

Don't stack multiple shadows. Don't introduce harder shadows.

### Spacing rhythm

Loose, but consistent.

- 24px gutter on the shell
- 8px between bar and results
- 14px / 18px paddings inside cards
- 16px above the hint footer
- `14vh` top margin on the shell — leaves headroom; not vertically centered.

## Layout

Single-column centered palette, max-width **720px**. No sidebar yet. The
shell is what's on screen — the app IS the palette right now.

When folders/tags ship (slice 3) and the editor lands (slice 4), the shell
will grow into a full app with sidebar + main pane + status bar. At that
point the palette becomes a Cmd-K modal **over** the shell, not a separate
route.

## Components

- **`.shell`** — outer wrapper. `max-width: 720px`, `margin-top: 14vh`,
  rise-fade entrance (320ms cubic-bezier(0.2, 0.8, 0.2, 1)).
- **`.bar`** — frosted card holding the input + mode badge. `:focus-within`
  shows a 4px soft accent ring on the parent.
- **`input.search`** — transparent inside `.bar`. `caret-color: var(--accent)`.
- **`.mode-badge`** — mono pill. Two states: `.mode-literal` (gray) and
  `.mode-fuzzy` (orange).
- **`.results`** — frosted card; `<li>` items with 10px inner radius and
  hover background tint.
- **`.path`** — mono, faint, ellipsis on overflow.
- **`.snippet`** — line-height 1.55. `<mark>` uses a bottom-up gradient
  (highlighter, not box).
- **`.empty`** — frosted card; same surface tokens.
- **`.hint`** — kbd-style key caps for shortcuts at the bottom.

## Animation principles

- One entrance animation on the shell. No per-item stagger (yet).
- All hover/focus transitions are **120–200ms, ease**.
- No springy / bouncy easings. Cubic-bezier(0.2, 0.8, 0.2, 1) for entrance,
  ease for state changes.
- Reserve elaborate motion for moments of genuine delight (first run,
  command-execution feedback). Never decorate.

## When to break the rules

If you're adding a feature the current system can't express:

- prefer **adding a new token** to redefining an existing one
- prefer **adding a component** to overloading an existing one
- prefer **extending** the radii / spacing scale to one-off values
- never duplicate the warm/cool gradient — it is the canvas signature

If you find yourself reaching for a green checkmark or a red error blob,
stop and propose a token in the design discussion before adding it.
