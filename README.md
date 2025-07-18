# Command Line Tool : Moving File and Folder in Download Directory at Current Directory
```batch
dm.exe
```
(*^^)v＜ダウンロードディレクトリにあるファイルとディレクトリをカレントディレクトリに移動させるコマンドラインツールです。

## 概要
- ダウンロードディレクトリからカレントディレクトリへファイルを移動

## 実行環境

### 検証済
- [x] Windows 11(64bit)
- [x] Linux

### 非検証
- [ ] Mac

## インストール

### バイナリ
Windows - https://github.com/c0b23092db/dm/releases/download/v1.0.3/dm.exe

### cargo
```batch
cargo install --git https://github.com/c0b23092db/dm
```


## 使用方法
```
Usage: dm [sum | help | dir/ls]
    sum     : 移動するファイルの数
    dir/ls  : 移動するファイルの一覧を表示
    help    : ヘルプを表示
```

**`[sum]`**
- 指定した数値分、更新日が新しいファイルを移動させる。
- 負の数を指定すると、更新日が古いファイルを移動させる。
- 0を指定すると、すべてのファイルを移動させる。

**`[option]`**

- `help`
  ヘルプを表示する。

- `dir` or `ls`
  ダウンロードフォルダにあるファイルを一覧で表示する。

### 例

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
dm dir
```

ヘルプを表示する。
```batch
dm help
```

## 実装予定
- [x] バイナリを配布する
- [ ] clapを使う

## 開発者
- いかた゚ : [](url)

## ライセンス
[MIT Licence](./LICENCE.md) / <http://opensource.org/licenses/MIT>
