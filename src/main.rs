use smol::{block_on, process::Command};
use std::io::{self, Read, Write};
use tts_urls::google_translate;

fn main() {
    block_on(async {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();
        let mut buffer = String::new();
        stdin.read_to_string(&mut buffer).unwrap();
        buffer = buffer.replace("STEP", "■STEP");
        let sections = buffer.split('■');
        for (idx, section) in sections.enumerate() {
            let mut section = section.replace('\n', " ");
            if section.trim().starts_with("STEP") {
                section = format!(
                    "Step {}. {}",
                    section.split(' ').nth(1).unwrap(),
                    section.split(' ').skip(2).collect::<Vec<_>>().join(" ")
                );
            }
            section = section.replace(';', ".");
            for (inner_idx, sentence) in section.split('.').enumerate() {
                if sentence.len() > 200 {
                    println!("{}", sentence);
                    panic!()
                }
                let sentence = sentence.trim();
                if sentence.is_empty() {
                    continue;
                }
                let url = google_translate::url(&format!("{}.", sentence), "en");
                let data = surf::get(url).recv_bytes().await.unwrap();
                std::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(format!("tts_{:03}_{:03}.mp3", idx, inner_idx))
                    .unwrap()
                    .write_all(&data)
                    .unwrap();
            }
        }
        let mut command = Command::new("bash");
        command.arg("concat");
        command.spawn().unwrap();
    });
}
