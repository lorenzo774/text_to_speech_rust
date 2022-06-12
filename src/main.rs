use speaker::Speaker;
use std::io;
use std::io::Write;
use tts_rust::languages::Languages;

mod speaker;

fn processed_txt(txt: &String) -> String {
    String::from(txt.to_lowercase().trim())
}

fn main() {
    println!("\x1B[2J\x1B[1;1H");
    let speaker = Speaker::new(Languages::English);
    let res = speaker.say(&String::from(
        "Insert some text in your terminal, PRESS Q TO EXIT",
    ));
    println!("{}", res);

    let mut txt: String = String::new();

    while processed_txt(&txt) != "q" {
        txt.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut txt)
            .expect("Failed to read line");

        let res = speaker.say(&txt);
        println!("{}", res);
    }
}
