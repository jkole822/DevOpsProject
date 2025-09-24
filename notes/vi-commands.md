# Vi/Vim Command Cheatsheet

## Modes

| Mode | How to Enter | Purpose |
||--||
| Normal (Command) | `Esc` | Navigation, editing commands |
| Insert | `i` / `a` / `o` | Insert text |
| Visual | `v` / `V` / `Ctrl+v` | Select text |
| Command-line | `:` | Execute commands like save, quit, search |

## Basic Navigation

| Command | Description |
||-|
| `h` | Move left |
| `j` | Move down |
| `k` | Move up |
| `l` | Move right |
| `0` | Beginning of line |
| `^` | First non-blank character |
| `$` | End of line |
| `w` | Jump forward to next word |
| `e` | Jump to end of word |
| `b` | Jump backward to beginning of word |
| `gg` | Go to beginning of file |
| `G` | Go to end of file |
| `:n` | Go to line `n` |

## Insert Mode

| Command | Description |
||-|
| `i` | Insert before cursor |
| `I` | Insert at beginning of line |
| `a` | Insert after cursor |
| `A` | Insert at end of line |
| `o` | Open new line below |
| `O` | Open new line above |
| `Esc` | Exit insert mode |

## Editing

| Command | Description |
||-|
| `x` | Delete character under cursor |
| `X` | Delete character before cursor |
| `dd` | Delete current line |
| `D` | Delete to end of line |
| `dw` | Delete word |
| `cw` | Change word (delete + insert) |
| `cc` | Change entire line |
| `u` | Undo |
| `Ctrl+r` | Redo |
| `p` | Paste after cursor |
| `P` | Paste before cursor |

## Visual Mode (Selection)

| Command | Description |
||-|
| `v` | Start character-wise selection |
| `V` | Start line-wise selection |
| `Ctrl+v` | Start block-wise (column) selection |
| `y` | Yank (copy) selection |
| `d` | Delete selection |
| `>`, `<` | Indent / unindent selection |

## Searching

| Command | Description |
||-|
| `/pattern` | Search forward |
| `?pattern` | Search backward |
| `n` | Repeat search in same direction |
| `N` | Repeat search in opposite direction |
| `:%s/old/new/g` | Replace all occurrences in file |
| `:s/old/new/g` | Replace all occurrences in current line |

## File Commands

| Command | Description |
||-|
| `:w` | Save file |
| `:wq` | Save and quit |
| `:q` | Quit |
| `:q!` | Quit without saving |
| `:e filename` | Open file |
| `:r filename` | Insert file content at cursor |

## Marks and Jumps

| Command | Description |
||-|
| `m{a-z}` | Set mark `{a-z}` at cursor |
| `'{a-z}` | Jump to beginning of line of mark `{a-z}` |
| `` `{a-z} `` | Jump to exact cursor position of mark `{a-z}` |
| `Ctrl+o` | Jump back in jump list |
| `Ctrl+i` | Jump forward in jump list |

## Miscellaneous

| Command | Description |
||-|
| `.` | Repeat last command |
| `~` | Toggle case of character |
| `J` | Join line below with current line |
| `:set number` | Show line numbers |
| `:set nonumber` | Hide line numbers |
| `:syntax on` | Enable syntax highlighting |
