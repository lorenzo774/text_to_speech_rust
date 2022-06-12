use tts_rust::{languages::Languages, GTTSClient};
use whoami::realname;

pub struct Speaker {
    client: GTTSClient,
}
impl Speaker {
    pub fn new(lang: Languages) -> Speaker {
        Speaker {
            client: GTTSClient {
                volume: 1.0,
                language: lang,
            },
        }
    }
    pub fn say(&self, txt: &String) -> String {
        let res = match txt.to_lowercase().trim() {
            "q" => {
                let res = ["Bye", realname().as_str()].concat();
                let _ = &self.client.speak(&res.as_str());
                "Bye 👋"
            }
            _ => {
                let _ = &self.client.speak(txt);
                txt.trim()
            }
        };
        res.to_string()
    }
}
