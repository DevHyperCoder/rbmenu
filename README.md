# RBMenu

Rust Bookmark (d)Menu is a dmenu/ncurses based interface to manage bookmarks independently of your web browser. It also supports file/folder bookmarks

## Features
- Insert Bookmark
- List Bookmark(s)

## FAQ
**Location of Bookmark file ?**
The Bookmark file for `rbmenu` is stored in `~/.local/share/rbmenu/`

**File format of the file ?**
The Bookmark file is stored in `json` format.

**More features ?**
Yes, more features are on the way. Some planned ones are, copy to clipboard, modify bookmarks and so on.

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

Without any options, `rbmenu` displays all the available bookmarks. Pipe regex string into `stdin`

`echo "git*" | rbmenu -l` 

## License

RBMenu is licensed under the GPL-3 license.
