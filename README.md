# RBMenu

Rust Bookmark (d)Menu is a dmenu/ncurses based interface to manage bookmarks independently of your web browser. It also supports file/folder bookmarks

## Features
- Insert Bookmark
- List Bookmark(s) \[With coloured output]

## FAQ
**Location of Bookmark file ?**
The Bookmark file for `rbmenu` is stored in `~/.local/share/rbmenu/`

**File format of the file ?**
The Bookmark file is stored in `json` format.

**More features ?**
Yes, more features are on the way. Some planned ones are, copy to clipboard, modify bookmarks and so on.

## Installation
`rbmenu` is available on [crates.io](https://crates.io/crates/rbmenu)

**Arch Linux** : Available on AUR, `rbmenu` for manual compilation from release and `rbmenu-bin` for precompiled binary

**Manual Installation**
- Install the rust toolchain. `cargo` should be on the `$PATH`
- Clone the repo: `git clone https://github.com/DevHyperCoder/rbmenu.git`. Change directory (`cd`) into the `rbmenu` folder
- Build the code: `cargo build --release`
- Copy the binary to a location on $PATH. Binary is in `./target/release/rbmenu`
- For operation with cargo, `cargo run -- <options>`.

## CLI - Options

| Option / Flags   | Description                |
| ---------------- | -------------------------- |
| `-h` `--help`    | Prints help information    |
| `-i`             | Insert a new bookmark      |
| `-l`             | List all bookmarks         |
| `-n` `--name`    | Name of the bookmark       |
| `-V` `--version` | Prints version information |

## Examples
**Insert a new bookmark**

`rbmenu` reads from `stdin`, therefore, you are able to pipe text into it.
`-n` is the name of the bookmark.

`echo "https://discord.com/app" | rbmenu -in "Discord"`
> Scripts working with `dmenu` or `rofi` would be published soon.

**List bookmarks**

Without the name option, `rbmenu -l` displays all the available bookmarks. Give a regex string to the `-n` flag to filter out the bookmarks

`rbmenu -ln "git*"` 

## License

RBMenu is licensed under the GPL-3 license.
