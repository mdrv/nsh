# mdrv/nsh

> [!CAUTION]
> **Personal fork, not for general use.**
> This is my private working copy of Native Shell, maintained by and for
> me (mdrv). Development is heavily LLM-assisted. Most commits are written
> with AI agents and verified only against my own apps. No support, no
> stability guarantee, no release schedule. If you're not me, you'll have
> a far better time with
> [Longbridge's NSH](https://github.com/longbridge/nsh) ([nsh.dev](https://nsh.dev)).

## What's in the fork

See [MDRV.md](MDRV.md) for the full log. Highlights:

- Wayland layer-shell overlay windows (compositor-anchored toasts/popups/
  HUD), verified on Hyprland.
- `onPanMove` — the continuous drag stream between `onPanStart` and
  `onPanEnd`, targetting the pan's view wherever the pointer goes (a
  Blender-style orbit keeps updating past the viewport's edge).
- An `<Image>` whose `src` changes keeps its previous picture on screen
  until the new one loads, instead of flashing the placeholder.

Upstream has absorbed the fork's earlier capabilities, `baseParts`,
global-shortcut and dev-profile patches.

## Using it (if you insist)

Path deps for local development:

```toml
[dependencies]
nsh-core = { path = "/g/nsh/crates/nsh-core" }
```

The apps that use it build daily on Linux and the web. Git tags come when
the patch set stabilizes.

## License

Apache-2.0, same as upstream ([LICENSE-APACHE](LICENSE-APACHE)). The
warning at the top is a social notice, not a license change.
