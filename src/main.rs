use std::fs::File;
use std::io;
use std::time::{Duration, Instant};

fn download_file(url: &str, modo: &str, filename: &str) {
    let start = Instant::now();
    println!("Baixando {}", &filename);
    let mut resp = reqwest::blocking::get(format!("{}/{}", url, filename)).expect("request failed");
    let mut out =
        File::create(format!("./downloads/{}_{}", modo, filename)).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
    let duration: Duration = start.elapsed();
    println!(
        "    concluido download de {} em {}",
        &filename,
        duration.as_millis() as f32 / 1000.
    );
}

const URL: &str = "http://arquivos.afonsomiguel.com";

fn main() {
    // Download Sequencial
    let start = Instant::now();
    for f_cont in 0..=9 {
        download_file(
            URL,
            "sequencial",
            format!("arquivo_{}.jpg", f_cont).as_str(),
        );
    }
    let duration: Duration = start.elapsed();
    println!(
        "Download sequencial em {:.1} segundos",
        duration.as_millis() as f32 / 1000.
    );
}
