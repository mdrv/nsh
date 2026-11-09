# mdrv/nsh fork notes

This fork (github.com/mdrv/nsh) carries the patches the mdrv apps need on
top of [longbridge/nsh](https://github.com/longbridge/nsh) (upstream,
Native Shell / NSH). The changes are hard to keep as out-of-tree patches
and have not been merged upstream. Development is heavily LLM-assisted,
verified against mdrv's own apps.

Crates keep their upstream names for now. A rename (mdrv-nsh) is cosmetic,
touches hundreds of identifiers, and has no functional reason yet — it is
deliberately deferred (see the bottom of this file).

## Consuming the fork

Path deps for local development (the fork's own apps do this today):

    [dependencies]
    nsh-core = { path = "/g/nsh/crates/nsh-core" }
    nsh-base = { path = "/g/nsh/crates/nsh-base", default-features = false }

Nothing is published to crates.io; when consumers pin instead of
path-depending, they will pin a git tag (none cut yet — cut one when the
patch set stabilizes, in the mdrv-gpui style `mdrv-nsh-<version>`).

## Patches

### linux: overlay windows on Wayland's layer shell (674350bf, pushed)

Overlays (toasts, popups, HUD) anchor through the compositor's layer shell
instead of stacking ordinary windows. Verified on Hyprland 0.57: glass HUD
on overlay (layer 3), bar on level 2, anchor math exact. Hyprland 0.57
configures binds in Lua (`hl.bind` in `hyprland.lua`); `hyprctl keyword
bind` is "unknown request" and `wtype`'s virtual keyboard does not fire
compositor binds.

### engine: Rust apps declare capabilities in nsh.json (d690f0a4, pushed)

Rust apps had no way to ask for portal/device capabilities the way TSX
apps do; `nsh.json` is the app's manifest, so that is where the
declaration belongs.

### core: Global shortcuts for Rust apps (321201ca, pushed)

`desktop_el::global_shortcut("ctrl+alt+b", on_press)`. Verified with a
Hyprland bind. Dispatch probe when debugging: `hyprctl eval 'return
hl.dispatch(hl.dsp.global("appid:shortcut"))'`.

### cli: `baseParts` — serve a subset of `nsh/base` (03770fba)

The web size problem: the features `editor, highlight, plot, rich-editor,
term` are requested by `nsh-engine` and `nsh-platform-web`'s own manifests,
so nothing an app or the generated members do can remove them — feature
unification only ever adds. Manifest-level stripping is a dead end (three
attempts relinked byte-identical artifacts).

The working lever is nsh's own: `nsh_base::parts` — _what no part
registers is left out of the binary_, dependencies included (syntect,
ropey, vte), at link time. The generated runtimes hardcoded the full
`registry()`; `nsh.json "baseParts"` lets a Rust app list the parts it
serves instead.

- Files: `crates/nsh-cli/src/rust_app.rs` (field, parse, `base_set`),
  `crates/nsh-cli/src/package.rs` (`_base_parts` on `AppConfig`, so the
  validator does not warn — same pattern as `_theme`/`_resolve`).
- Rust apps only (a `"root"` must be set): TSX apps load base parts by
  gate (`nsh_part_*`) per chunk, so a trimmed registry would break their
  bundles at load. The guard lives in `base_set`.
- Default unchanged (`registry()`): every existing app builds exactly as
  before. One knob, all of the app's runtimes (web, native runners, test
  runner, TUI host) — one app, one base set.
- Measured on nsh-bench: packaged main wasm 10.78 MB → 5.15 MB, brotli
  2.63 MB → **1.33 MB**; site 11.4 MB → 6.1 MB. Renders fully in
  Chromium, empty console.
- _upstream?_ yes — the knob follows the lite runtime's own philosophy,
  and the CLI cannot know which parts a Rust app mounts, so the app must
  say.

## Tooling

No upstream-diff guard yet (mdrv-gpui has `script/check-upstream`).
Adopt one when the diverged-file count stops fitting in this file's
Patches sections.

## Branch policy

`layer-shell` carries the patches and is the fork's working line; push to
`origin` only when work has soaked. Sync with upstream:

    git fetch upstream && git merge upstream/main

Keep patches minimal, independent and re-submit upstream when feasible;
drop them from this file when they land. Eval notes and benchmarks live
out of tree (`/x/m/v270/nsh/`, never committed).

## Consumers

- `nsh-bench` (`/g/_ext/nsh-bench`) — the visual/scroll benchmark and the
  web-size eval app (`baseParts: ["core"]`).
- `nsh-layer-test` (`/g/nsh-layer-test`) — layer-shell + global-shortcut
  demo, the verification app for the first three patches.
- Future mdrv games — the reason `baseParts` exists.

## Considered, not patched

- **Renaming the fork** (`mdrv-nsh`): cosmetic, high churn, no functional
  reason yet. Revisit when the fork needs a public identity of its own.
- **Trimming nsh-engine / nsh-platform-web's `nsh-base` features**:
  impossible from the app side; would churn every platform crate for a
  win `baseParts` already delivers.
