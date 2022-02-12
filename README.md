# aviutl-keyconfig

[AviUtl](http://spring-fragrance.mints.ne.jp/aviutl/)
のショートカットキー設定を見やすく一覧にするツールです。

## インストール

[Releases](https://github.com/karoterra/aviutl-keyconfig/releases)
から最新版の ZIP ファイルをダウンロードし、好きな場所に展開してください。

アンインストール時には展開したフォルダを削除してください。

## 使い方

### コマンドラインを使わない場合

`aviutl.key` を `aviutl-keyconfig.exe` にドラッグ&ドロップしてください。
`aviutl.key` と同じ場所に `aviutl.key.html` が生成されます。

### コマンドラインから使う場合

```
aviutl-keyconfig 0.1.0
Generate AviUtl key config list

USAGE:
    aviutl-keyconfig.exe [OPTIONS] <INPUT_FILE>

ARGS:
    <INPUT_FILE>    AviUtl key config file (*.key)

OPTIONS:
    -h, --help                         Print help information
    -o, --output-file <OUTPUT_FILE>    Output path
    -t, --template <TEMPLATE>          Template file name [default: default.hbs]
    -V, --version                      Print version information
```

`--template` オプションには `template` ディレクトリ内にあるファイル名を指定してください。

## ライセンス

このソフトウェアは、MIT ライセンスのもとで公開されます。
詳細は [LICENSE](LICENSE) を参照してください。

## 更新履歴

更新履歴は [CHANGELOG](CHANGELOG.md) を参照してください。

## 参考

- [Auls - .keyファイル構造](http://auls.client.jp/doc/aviutl_keyfileformat.html)
