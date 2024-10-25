use std::io::{self, Write};
use std::time::{Duration, Instant};
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    loop {
        println!("=== Gos Typing CLI ===");
        println!("1. Mulai Permainan");
        println!("2. Keluar");
        print!("Pilih opsi: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "1" => {
                if let Err(e) = start_game() {
                    println!("Error: {}", e);
                }
            }
            "2" => {
                println!("Terima kasih telah bermain!");
                break;
            }
            _ => println!("Opsi tidak valid, coba lagi."),
        }
    }
}

fn start_game() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("E:/Developer/Program/Rust/typing_game/src/word/word.txt")?;
    let reader = BufReader::new(file);
    let mut words = String::new();
    for line in reader.lines() {
        words.push_str(&line?);
        words.push(' ');
    }

    let words: Vec<&str> = words.split_whitespace().collect();
    let mut rng = rand::thread_rng();

    // Pilih 10 kata secara acak dan unik
    let chosen_words: Vec<&str> = words.choose_multiple(&mut rng, 10).cloned().collect();
    let mut score = 0;
    let mut total_duration = Duration::new(0, 0);

    println!("Mulai Permainan! Ketik kata yang muncul secepat mungkin. Ketik 'exit' untuk keluar.");
    println!();

    for word in chosen_words {
        println!("Kata: {}", word);

        let start = Instant::now();
        let mut input = String::new();

        print!("Ketik: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let duration = start.elapsed();
        total_duration += duration;
        if input == word {
            score += 1;
            println!("Benar! Waktu: {:.2?}. Skor: {}", duration, score);
        } else {
            println!("Salah. Kata yang benar adalah '{}'. Skor: {}", word, score);
        }
        println!();
    }

    println!("Permainan selesai! Total waktu yang dihabiskan: {:.2?}", total_duration);
    Ok(())
}

