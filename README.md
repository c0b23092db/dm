# Download Mover（だむ）
```bash
dm.exe
```
**ダウンロードディレクトリにあるファイルとディレクトリを、カレントディレクトリに移動させるCLI**

## 実行環境

### 検証済
- [x] Windows 11(64bit)

### 未検証
- [ ] Linux
- [ ] Mac

## インストール

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

### バイナリ
Windows - https://github.com/c0b23092db/dm/releases/download/v1.2.0/dm.exe

## コマンド
```
> dm help
CLI: Moving File and Folder in Download Directory at Current Directory

Usage: dm.exe [OPTIONS] [count] [COMMAND]

Commands:
  ls      Check files in the download directory
  remove  Remove files in the download directory
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
- 正の数: 更新日が新しいファイルを移動させる
- 負の数: 更新日が古いファイルを移動させる
- 0を指定: すべてのファイルを移動させる

### SubCommands
- `remove`, `rm`
  動作を「ファイルを削除する」に変える。
- `ls`, `list`, `dir`
  ダウンロードディレクトリにあるファイルをリスト形式で出力する。
- `path`
  ダウンロードディレクトリのパスを出力する。

### Options
- `-s`, `--specify`
  指定した場所のファイルを移動する
- `-h`, `--help`
  ヘルプを表示する。
- `-V`, `--version`
  バージョンを表示する。

## 実例

### `dm`

更新日付が新しいファイルを一つ移動させる。
```bash
dm
```

更新日付が新しいファイルを三つ移動させる。
```bash
dm 3
```

更新日付が古いファイルを五つ移動させる。
```bash
dm -5
```

更新日付が新しい順で三番目のファイルを移動させる。
```bash
dm -s 3
```

### `dm remove`

更新日付が新しいファイルを一つ削除する。
```bash
dm rm
```

更新日付が新しいファイルを三つ削除する。
```bash
dm rm 3
```

更新日付が古いファイルを五つ削除する。
```bash
dm rm -5
```

更新日付が新しい順で三番目のファイルを削除する。
```bash
dm rm -s 3
```

### `dm ls`
ダウンロードディレクトリにあるファイルをリスト形式で出力する。
```bash
dm ls
```

### `dm path`
ダウンロードディレクトリのパスを出力する。
```bash
dm path
```

## 開発者
- ikata

## ライセンス
[MIT License](./LICENSE) / <http://opensource.org/licenses/MIT>
