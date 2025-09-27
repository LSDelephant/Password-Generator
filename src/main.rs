use clap::Parser;
use rand::{distributions::Uniform, prelude::Distribution, rngs::ThreadRng, thread_rng};

/// Простий генератор паролів
#[derive(Parser, Debug)]
#[command(author, version, about = "CLI Password Generator на Rust")]
struct Args {
    /// Довжина пароля
    #[arg(short = 'l', long = "length", default_value_t = 16)]
    length: usize,

    /// Вимкнути великі літери (A-Z)
    #[arg(long = "no-uppercase", default_value_t = false)]
    no_uppercase: bool,

    /// Вимкнути малі літери (a-z)
    #[arg(long = "no-lowercase", default_value_t = false)]
    no_lowercase: bool,

    /// Вимкнути цифри (0-9)
    #[arg(long = "no-digits", default_value_t = false)]
    no_digits: bool,

    /// Вимкнути спеціальні символи
    #[arg(long = "no-special", default_value_t = false)]
    no_special: bool,
}

fn main() {
    let args = Args::parse();

    match build_charset(&args) {
        Ok(charset) => {
            if charset.is_empty() {
                eprintln!("Помилка: набір символів порожній. Увімкни хоча б одну категорію (letters, digits або special).");
                std::process::exit(1);
            }

            let password = generate_password(args.length, &charset, &mut thread_rng());
            println!("{}", password);
        }
        Err(e) => {
            eprintln!("Помилка: {}", e);
            std::process::exit(1);
        }
    }
}

/// Формує набір символів згідно з опціями
fn build_charset(args: &Args) -> Result<Vec<char>, &'static str> {
    let mut chars = Vec::new();

    if !args.no_lowercase {
        chars.extend(('a'..='z').into_iter());
    }
    if !args.no_uppercase {
        chars.extend(('A'..='Z').into_iter());
    }
    if !args.no_digits {
        chars.extend(('0'..='9').into_iter());
    }
    if !args.no_special {
        // набір спеціальних символів — можна розширити за бажанням
        let special = r###"!@#$%^&*()-_=+[]{};:,.<>?/|~`"###;
        chars.extend(special.chars());
    }

    Ok(chars)
}

/// Генерує пароль довжини `len` з `charset`
fn generate_password(len: usize, charset: &[char], rng: &mut ThreadRng) -> String {
    if charset.is_empty() || len == 0 {
        return String::new();
    }

    let dist = Uniform::from(0..charset.len());
    let mut out = String::with_capacity(len);

    for _ in 0..len {
        let idx = dist.sample(rng);
        out.push(charset[idx]);
    }

    out
}
