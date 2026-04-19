# compe-rust

rust勉強用+競プロ用

## フォルダ構成

src配下にコンテスト等ごとにフォルダを切り、問題ごとにファイルを作成。  
ファイル作成は以下のコマンドを参照
## コマンド
cargo.tomlに実行できるように設定する
ファイルの作成とcargo.tomlへの登録を行う
- ```./add_problem.sh {フォルダ名} ファイル名```
- ```./add_problem.sh sample_folder main```

ファイルを実行
- ```cargo run --bin {フォルダ名}_{ファイル名}```
    