# BouyomiClient for Rust

BouyomiClientは、棒読みちゃんのアプリ連携機能（TCP）を使用して、テキストを音声で読み上げるためのRustライブラリです。


[棒読みちゃん](https://chi.usamimi.info/Program/Application/BouyomiChan/)を起動してご利用ください。  

## Feature

- 棒読みちゃんの発声設定をカスタマイズ可能
- 棒読みちゃんの一時停止、再開、スキップ、クリアなどの操作をサポート
- 再生中の状態や残りのタスク数を取得可能
- Builderパターンを使用して、設定を柔軟に変更可能

## Usage

```rust
use rs_bouyomi::{BouyomiClient, TalkConfig};

let mut config = TalkConfig::default();
config.set_voice(1).set_volume(100).set_speed(100).set_tone(100);
let client = BouyomiClient::new().set_config(config);

client.talk("こんにちは、世界");
client.wait(60);

```

## APIリスト

- [x] `BouyomiClient::new()`: 新しい`BouyomiClient`インスタンスを作成します。
- [x] `BouyomiClient::set_config(config: TalkConfig)`: `BouyomiClient`の設定を更新します。
- [x] `BouyomiClient::talk(message: &str)`: 指定したメッセージを棒読みちゃんに読み上げさせます。
- [x] `BouyomiClient::wait(seconds: u64)`: 指定した秒数を上限に読み上げが終わるまで待機します。
- [x] `TalkConfig::default()`: デフォルトの`TalkConfig`を作成します。
- [x] `TalkConfig::set_voice(voice: i16)`: 発声設定を更新します。
- [x] `TalkConfig::set_volume(volume: i16)`: 音量設定を更新します。
- [x] `TalkConfig::set_speed(speed: i16)`: 速度設定を更新します。
- [x] `TalkConfig::set_tone(tone: i16)`: トーン設定を更新します。

## via golang client
[go-bouyomichan@go-numb](https://github.com/go-numb/go-bouyomichan)

## Author

[@_numbP](https://twitter.com/_numbP)

## License

[MIT](https://github.com/go-numb/rust-bouyomichan/blob/master/LICENSE)