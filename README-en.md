# Download Mover（だむ）
```bash
dm.exe
```
**CLI: moves files and folders from your Downloads directory to the current directory.**

## Environment

### Tested
- [x] Windows 11 (64-bit)

### Not Tested
- [ ] Linux
- [ ] macOS

## Installation

### cargo
```bash
cargo install download_mover
```

```bash
cargo binstall download_mover
```

```bash
cargo install --git https://github.com/c0b23092db/dm
```

### Binary
Windows - https://github.com/c0b23092db/dm/releases/download/v1.2.0/dm.exe

## Commands
```
> dm help
CLI: Moving File and Folder in Download Directory at Current Directory

Usage: dm.exe [OPTIONS] [count] [COMMAND]

Commands:
  ls      Check files in the download directory
  remove  Remove files in the download directory
  back    Move files in the download directory
  path    Get Path of the download directory
  help    Print this message or the help of the given subcommand(s)

Arguments:
  [count]  Number of files being moved

Options:
  -s, --specify  Move files at the specified number
  -h, --help     Print help
  -V, --version  Print version
```

### Arguments
**`[count]`**
- Positive number: Moves newer files.
- Negative number: Moves older files.
- `0`: Moves all files.

### Subcommands
- `remove`, `rm`
  Switches behavior to removing files.
- `ls`, `list`, `dir`
  Prints files in the Downloads directory in list format.
- `path`
  Prints the Downloads directory path.
- `back`, `bk`
  Switches behavior to moving files from the current directory back to the Downloads directory.

### Options
- `-s`, `--specify`
  Moves the file at the specified position.
- `-h`, `--help`
  Prints help.
- `-V`, `--version`
  Prints version.

## Examples

### `dm`

Moves the newest file.
```bash
dm
```

Moves the 3 newest files.
```bash
dm 3
```

Moves the 5 oldest files.
```bash
dm -5
```

Moves the 3rd file in newest-first order.
```bash
dm -s 3
```

### `dm remove`

Removes the newest file.
```bash
dm remove
```

Removes the 3 newest files.
```bash
dm rm 3
```

Removes the 5 oldest files.
```bash
dm rm -5
```

Removes the 3rd file in newest-first order.
```bash
dm rm -s 3
```

### `dm back`

Moves `README.md` to the Downloads directory.
```bash
dm back README.md
```

Moves `README.md` and `Cargo.toml` to the Downloads directory.
```bash
dm bk README.md Cargo.toml
```

### `dm ls`
Prints files in the Downloads directory in list format.
```bash
dm ls
```

### `dm path`
Prints the Downloads directory path.
```bash
dm path
```

## Developer
- ikata

## License
[MIT License](./LICENSE) / <http://opensource.org/licenses/MIT>
