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

## Sync history

**2026-11-16: merged upstream/main (v0.5.0 nightly line, 752 commits).**
The merge retired three of the five patches — upstream built the same
features, in most places better:

- **Global shortcuts** (was 321201ca): upstream ships
  `register_global_shortcut` / `unregister_global_shortcut` /
  `unregister_all_global_shortcuts` on the desktop element with
  registration answers (`Answer<bool>` — the desktop's accept/refuse is
  delivered, not guessed), tray support, and the Linux portal flow
  documented down to the Hyprland bind. Our simpler fn is deleted.
- **Rust-app capabilities** (was d690f0a4): upstream reads
  `nsh.json` `capabilities` for Rust apps (`app_access` →
  `capabilities_ref`); the runner-side `policy_from_manifest` is gone.
- **`baseParts`** (was 03770fba): upstream's `rust_sets()` serves a Rust
  app `nsh/base`'s core part only — "registering every part would link
  every component into the module, the ones the app never makes
  included" — which is our size win (bench: 10.78 MB → 5.15 MB main
  wasm, brotli 2.63 MB → 1.33 MB) without the knob. Apps drop
  `"baseParts"` from `nsh.json`; the key is unknown now.
- Upstream also gained the input surface mdrv games asked for:
  continuous `on_pinch` (`PinchEvent.delta` per event), `on_key_down` /
  `on_key_up` (`KeyEvent` with the DOM's `key`/`code`), `on_mouse_up`,
  `PinchPhase`.

## Patches

### linux: overlay windows on Wayland's layer shell (674350bf, pushed)

Overlays (toasts, popups, HUD) anchor through the compositor's layer shell
instead of stacking ordinary windows. Verified on Hyprland 0.57: glass HUD
on overlay (layer 3), bar on level 2, anchor math exact. Hyprland 0.57
configures binds in Lua (`hl.bind` in `hyprland.lua`); `hyprctl keyword
bind` is "unknown request" and `wtype`'s virtual keyboard does not fire
compositor binds. Survived the merge untouched except the
`gtk4-layer-shell` dep line (upstream made gstreamer optional in the same
section; both kept).

### cli: lighter dev profiles for the generated workspace

`write_workspace_manifest` adds `[profile.dev.package."*"] debug = false`
next to the existing per-crate `opt-level = 3` overrides.

The problem is real and measured: an app whose crates pull Bevy
(`nsh-bevy-test`) linked its development runner for over an hour and
peaked ~10 GB of RSS — dependency debuginfo alone (bevy_render, wgpu,
hickory, ...) dwarfs everything else, and the describe program links the
same tree again. With debuginfo left out of dependencies, links drop to
minutes and the memory spike goes with them; the app's own crates keep
their debuginfo (where breakpoints live anyway), and TSX apps feel it
only as faster dev rebuilds of the same binaries.

Pair it with mold in the app (`.cargo/config.toml`,
`rustflags = ["-C", "link-arg=-fuse-ld=mold"]`) — that part is per-app,
not patched into the CLI.

- _upstream?_ yes — every Rust app with heavy dependencies benefits; TSX
  apps pay nothing.

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

- `nsh-bevy-test` (`/g/nsh-bevy-test`) — Bevy 3D viewport embedded in an
  nsh app (drag-orbit, pinch zoom, WASD walks the duck); the game-input
  eval app.
- `nsh-bench` (`/g/_ext/nsh-bench`) — the visual/scroll benchmark and the
  web-size eval app.
- `nsh-layer-test` (`/g/nsh-layer-test`) — layer-shell +
  global-shortcut demo, the verification app for the layer-shell patch.

## Considered, not patched

- **Renaming the fork** (`mdrv-nsh`): cosmetic, high churn, no functional
  reason yet. Revisit when the fork needs a public identity of its own.
- **Trimming nsh-engine / nsh-platform-web's `nsh-base` features**:
  impossible from the app side; upstream's `rust_sets()` delivers the win
  for Rust apps automatically.
