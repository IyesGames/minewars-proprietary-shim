# MineWars Proprietary Shim

This repo contains shims for the proprietary functionality of MineWars, to allow
open-source builds of the game to be made without it.

The files in this repo, and the files in the real (private) proprietary repo, are
designed to be drop-in replacements for one another.

The `Cargo.toml` in the [main MineWars repo](https://github.com/IyesGames/minewars)
is configured to build using this repo by default, to make things easy for people
who want to make open-source builds.

This repo is under the MIT/Apache-2 license.
