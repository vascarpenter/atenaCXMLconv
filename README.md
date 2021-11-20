# AtenaCXMLconv

- 宛名職人26のContactXML出力を CSVに出力するプログラム
- Rust版に大幅に書き直し
- CSVで出力すると年賀状履歴が消えるためこちらに変更
- atxBaseYear, X-NYCardHistory に年賀状履歴が入っていそうだが、まだ解析していない

# コンパイル

- `cargo build'

# 使い方

- 宛名職人26から
  - ファイル＞書き出しを選択
  - ファイル形式：ContactXML1.1形式
  - エンコード：Unicode (UTF-8)
  - 改行コード：LF(標準)
  - 書き出す対象：全ての宛名
  - 上記設定で書き出す

- `./target/debug/atenaCXMLconv XXX.xml`
- 標準出力にCSVが出力されるのでリダイレクトするなりして。
