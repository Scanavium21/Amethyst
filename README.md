# <img src="public/amethyst.svg" alt="Amethyst icon" height="32" width="32"> Amethyst

<div>
    <a href="https://repology.org/project/amethyst-player/versions">
        <img src="https://repology.org/badge/vertical-allrepos/amethyst-player.svg" alt="Packaging status" align="right">
    </a>
    <div>
        <img src="https://img.shields.io/discord/385387666415550474?label=Discord&logo=discord&style=flat">
        <img src="https://img.shields.io/github/repo-size/geoxor/amethyst?label=Size">
        <a title="Crowdin" target="_blank" href="https://crowdin.com/project/amethyst-player"><img src="https://badges.crowdin.net/amethyst-player/localized.svg"></a>
    </div>
</div>

Amethyst is an cross-platform audio player with a node-based audio routing system,
customization and lots of other features.

This branch is for a complete Rust rewrite of the original made in TypeScript and VueJS.
Using [Rust](https://rust-lang.org), [Tauri](https://tauri.app) and [Sycamore](https://sycamore.dev),
Amethyst will be much more performant and smaller, with better cross-platform support and with robust features.
Another goal is to make Amethyst web-hostable, allowing for central managment of your music files and
using it for a self-hosted streaming service, giving you control of everything. This however is completely optional.

**The rewrite is currently in development**

## ⌨️ Development

Amethyst will be mainly written in [Rust](https://rust-lang.org), so obviously familiarity with the language is recommended
as well as an IDE with Rust support (possible built-in or via extension/plugin).

- [Rust](https://rust-lang.org/tools/install)
- [Trunk](https://trunkrs.dev) via `cargo install --locked trunk`
- [Tauri CLI] via `cargo install --locked tauri-cli`
- [Tauri Setup](https://tauri.app/start/prerequisites)
