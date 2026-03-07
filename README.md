# `explora`
A small, friendly, terminal file explorer written in rust.

## Use
To run `explora`, simply run the executable in your shell and terminal of choice.
```sh
explora
```
And the `explora` interface will appear!

Navigation is simple, just refer to the guide on the bottom of the tui screen.
* Up / Down to navigate through a directory.
* Right / Left to navigate from parent to child.
* Enter to open a file in your `$EDITOR` (defaults to `vim`).
* Esc to quit.

## Features
* Easy navigation, and hints at the bottom of the window.
* Open files and directories in your `$EDITOR`, then get right back to browsing.
* Color coded file and directory types, with a reference at the top of the window.

## Installation
Installation is easy through `cargo`.
```sh
cargo install explora
```
> Support for cargo-binstall is planned!

## Roadmap
[ ] Command line options: opening `explora` in other directories, ignoring permission errors.

[X] Customizable file type colors through `TOML` themes.

[ ] File / directory size and permission info.
