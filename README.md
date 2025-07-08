# Command Line Interface : Moving File and Folder in Current Directory from Download Directory

```batch
dm [sum] | [option]
```

ダウンロードディレクトリにあるファイルを現在いるカレントディレクトリに移動させるコマンドラインインターフェースのツールです。

## インストール

### Binary

Windows - https://github.com/c0b23092db/dm/releases/download/v1.0.3/dm.exe

### cargo

```batch
cargo install --git https://github.com/c0b23092db/dm
```

## 実行環境

### 検証済

- Windows 11(64bit)
- Linux

### 非検証
- Mac

## コマンドの説明

**`[sum]`**

- 指定した数値分、更新日が新しいファイルを移動させる。
- 負の数を指定すると、更新日が古いファイルを移動させる。
- 0を指定すると、すべてのファイルを移動させる。

**`[option]`**

- `help`
  ヘルプを表示する。

- `dir` or `ls`
  ダウンロードフォルダにあるファイルを一覧で表示する。

## 使い方

```batch
dm
```

更新日付が新しいファイルを一つ移動させる。

```batch
dm
```

更新日付が新しいファイルを三つ移動させる。

```batch
dm 3
```

更新日付が古いファイルを五つ移動させる。

```batch
dm -5
```

ダウンロードフォルダにあるファイルを一覧表示する。

```batch
dm help
```

ヘルプを表示する。

```batch
dm dir
```

## 今後について

- [x] バイナリを配布する
- [ ] clapを使う
