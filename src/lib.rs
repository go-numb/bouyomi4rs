use std::io::{Read, Write};
use std::net::TcpStream;

use std::thread::sleep;
use std::time::Duration;

/// ## 棒読みちゃんのクライアント
/// アプリ連携機能（TCP）を使用
/// デフォルト: 127.0.0.1:50001
pub struct BouyomiClient {
    host: String,
    port: String,

    config: TalkConfig,
}

/// ## 棒読みちゃんの発声設定
pub struct TalkConfig {
    pub code: u8,
    pub voice: i16,
    pub volume: i16,
    pub speed: i16,
    pub tone: i16,
}

/// ## カスタムエラー型
#[derive(Debug)]
pub enum MyError {
    IoError(std::io::Error),
    OtherError(String), // エラーメッセージを含める
}

/// ## Result型のエイリアスを定義
type Result<T> = std::result::Result<T, MyError>;

impl Default for TalkConfig {
    fn default() -> Self {
        Self {
            code: 0,    // デフォルトコードは 0
            voice: 0,   // default=0 (1-8: AquesTalk, 10001-: SAPI5)
            volume: 80, // default=-1 (0-100)
            speed: 100, // default=-1 (50-300)
            tone: 100,  // default=-1 (50-200)
        }
    }
}


impl TalkConfig {
    // 新しいTalkConfigの生成
    pub fn new() -> Self {
        Self::default()
    }


    // 声種の設定
    pub fn set_voice(&mut self, voice: i16) -> &mut Self {
        self.voice = voice;
        self
    }

    // 音量の設定
    pub fn set_volume(&mut self, volume: i16) -> &mut Self {
        self.volume = volume;
        self
    }

    // 話速の設定
    pub fn set_speed(&mut self, speed: i16) -> &mut Self {
        self.speed = speed;
        self
    }

    // 高さの設定
    pub fn set_tone(&mut self, tone: i16) -> &mut Self {
        self.tone = tone;
        self
    }
}

impl Default for BouyomiClient {
    fn default() -> Self {
        BouyomiClient {
            host: String::from("127.0.0.1"),
            port: String::from("50001"),
            config: TalkConfig::new(),
        }
    }
}

impl BouyomiClient {
    // 新しいクライアントの生成
    pub fn new() -> Self {
        Self::default()
    }

    // Builderパターンを使用
    pub fn set_host(mut self, host: impl AsRef<str>) -> Self {
        self.host = host.as_ref().to_owned();
        self
    }

    pub fn set_port(mut self, port: impl AsRef<str>) -> Self {
        self.port = port.as_ref().to_owned();
        self
    }

    pub fn set_config(mut self, config: TalkConfig) -> Self {
        self.config = config;
        self
    }

    // デフォルト設定で話す
    pub fn talk(&self, message: impl AsRef<str>) -> std::result::Result<(), String> {
        match self.talk_with_config(message, &self.config) {
            Ok(_) => Ok(()),
            Err(err) => {
                let e = match err {
                    MyError::IoError(e) => e.to_string(),
                    MyError::OtherError(e) => e,
                };
                Err(e)
            }
        }
    }

    // 設定を指定して話す
    pub fn talk_with_config(
        &self,
        message: impl AsRef<str>,
        talk_config: &TalkConfig,
    ) -> std::result::Result<(), MyError> {
        let mut stream = match TcpStream::connect(format!("{}:{}", self.host, self.port)) {
            Ok(s) => s,
            Err(e) => {
                println!("failed to connect to BouyomiChan: {}", e);
                return Err(MyError::IoError(e));
            }
        };

        let message_bytes = message.as_ref().as_bytes();
        let message_length: u32 = message_bytes.len() as u32;
        let talk_command: i16 = 1; // 発声コマンド

        let talk_command_bytes = talk_command.to_le_bytes();
        let speed_bytes = talk_config.speed.to_le_bytes();
        let tone_bytes = talk_config.tone.to_le_bytes();
        let volume_bytes = talk_config.volume.to_le_bytes();
        let voice_bytes = talk_config.voice.to_le_bytes();
        let code_bytes = [talk_config.code];
        let message_length_bytes = message_length.to_le_bytes();

        stream.write_all(&talk_command_bytes).unwrap();
        stream.write_all(&speed_bytes).unwrap();
        stream.write_all(&tone_bytes).unwrap();
        stream.write_all(&volume_bytes).unwrap();
        stream.write_all(&voice_bytes).unwrap();
        stream.write_all(&code_bytes).unwrap();
        stream.write_all(&message_length_bytes).unwrap();
        stream.write_all(message_bytes).unwrap();

        match stream.flush() {
            Ok(_) => {}
            Err(e) => {
                println!();
                return Err(MyError::IoError(e));
            }
        };

        Ok(())
    }

    // 棒読みちゃんの一時停止
    pub fn pause(&self) -> Result<()> {
        self.send_simple_command(0x10)
    }

    // 棒読みちゃんの再開
    pub fn resume(&self) -> Result<()> {
        self.send_simple_command(0x20)
    }

    // タスクをスキップ
    pub fn skip(&self) -> Result<()> {
        self.send_simple_command(0x30)
    }

    // タスクをクリア
    pub fn clear(&self) -> Result<()> {
        self.send_simple_command(0x40)
    }

    // 一時停止状態を取得
    pub fn is_pause(&self) -> Result<bool> {
        match self.send_command_with_response(0x110) {
            Ok(res) => {
                if res == 1 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            },
            Err(_) => Ok(false),
        }
    }

    // 再生中かどうかを取得
    pub fn is_now_playing(&self) -> Result<bool> {
        match self.send_command_with_response(0x120) {
            Ok(res) => {
                if res == 0 {
                    Ok(false)
                } else {
                    Ok(true)
                }
            }
            Err(_) => Ok(false),
        }
    }

    // 再生中ならば待機
    pub fn wait(&self, limit_sec: i16) {
        for _ in 1..limit_sec {
            match self.is_now_playing() {
                Ok(is_playing) => {
                    if !is_playing {
                        break;
                    }
                }
                Err(_) => {
                    println!("failed to get playing status.");
                    break;
                }
            }

            sleep(Duration::from_millis(1000));
        }
    }

    // 残りのタスク数を取得
    pub fn get_remaining_tasks(&self) -> Result<u32> {
        let res = self.send_command_with_response(0x130)?;
        Ok(res as u32)
    }

    // 単純なコマンドの送信
    fn send_simple_command(&self, command_id: i16) -> Result<()> {
        let mut stream = match TcpStream::connect(format!("{}:{}", self.host, self.port)) {
            Ok(s) => s,
            Err(e) => {
                println!("failed to connect to BouyomiChan: {}", e);
                return Err(MyError::IoError(e));
            }
        };

        let talk_command_bytes = command_id.to_le_bytes();
        stream.write_all(&talk_command_bytes).unwrap();
        stream.flush().unwrap();

        Ok(())
    }

    // 応答が必要なコマンドの送信
    fn send_command_with_response(&self, command_id: i16) -> Result<u8> {
        let mut stream = match TcpStream::connect(format!("{}:{}", self.host, self.port)) {
            Ok(s) => s,
            Err(e) => {
                println!("failed to connect to BouyomiChan: {}", e);
                return Err(MyError::IoError(e));
            }
        };

        let talk_command_bytes = command_id.to_le_bytes();
        stream.write_all(&talk_command_bytes).unwrap();
        stream.flush().unwrap();

        let mut res = Vec::new();
        match stream.read_to_end(&mut res) {
            Ok(_) => {}
            Err(e) => {
                println!("failed to read response: {}", e);
                return Err(MyError::IoError(e));
            }
        };

        Ok(res[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // reimu client object
        let mut config = TalkConfig::default();
        config
            .set_voice(1)
            .set_volume(100)
            .set_speed(100)
            .set_tone(100);
        let reimu = BouyomiClient::new().set_config(config);

        println!(
            "host: {}, port: {}, voice: {}, volume: {}, speed: {}, tone: {}",
            reimu.host,
            reimu.port,
            reimu.config.voice,
            reimu.config.volume,
            reimu.config.speed,
            reimu.config.tone   
        );

        //  marisa client object
        let mut config = TalkConfig::default();
        config.set_voice(2);
        let marisa = BouyomiClient::new()
            .set_host("127.0.0.1")
            .set_port("50001")
            .set_config(config);

        // test talk
        match reimu.talk("ねえねえ、魔理沙、何してるの？") {
            Ok(_) => {}
            Err(e) => {
                // 棒読みちゃんを起動していないなどのエラーは無視する場合が多いため、このように捨てることができる
                println!("failed to talk: {}", e);
            }
        }

        // wait for end the task
        reimu.wait(60);

        marisa
            .talk("おすおす、霊夢、お昼ごはんを作ってたぜ？")
            .unwrap();
        marisa.wait(60);

        println!("success!");
    }
}
