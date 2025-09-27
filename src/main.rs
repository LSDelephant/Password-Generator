use clap::Parser;
use rand::{seq::SliceRandom, thread_rng, Rng};
use arboard::Clipboard;

/// Простий генератор паролів з додатковими функціями
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

    /// Скопіювати результат у буфер обміну
    #[arg(short = 'c', long = "copy", default_value_t = false)]
    copy: bool,
}

fn main() {
    let args = Args::parse();

    // будуємо набори символів
    let (lower, upper, digits, special) = (
        ('a'..='z').collect::<Vec<_>>(),
        ('A'..='Z').collect::<Vec<_>>(),
        ('0'..='9').collect::<Vec<_>>(),
        r###"!@#$%^&*()-_=+[]{};:,.<>?/|~`"###.chars().collect::<Vec<_>>(),
    );

    // формуємо доступні категорії
    let mut categories: Vec<Vec<char>> = Vec::new();
    if !args.no_lowercase { categories.push(lower); }
    if !args.no_uppercase { categories.push(upper); }
    if !args.no_digits { categories.push(digits); }
    if !args.no_special { categories.push(special); }

    if categories.is_empty() {
        eprintln!("⚠️ Помилка: увімкни хоча б одну категорію символів.");
        std::process::exit(1);
    }

    // генеруємо пароль
    let password = generate_password(args.length, &categories);

    // вивід
    println!("{}", password);

    if args.copy {
        let mut clipboard = Clipboard::new().unwrap();
        clipboard.set_text(password).unwrap();
        println!("📋 Пароль скопійовано у буфер обміну.");
    }
}

/// Генератор паролів з гарантією хоча б одного символу з кожної категорії
fn generate_password(len: usize, categories: &[Vec<char>]) -> String {
    let mut rng = thread_rng();
    let mut password_chars: Vec<char> = Vec::new();

    // крок 1: обов’язково додаємо по одному символу з кожної категорії
    for category in categories {
        if let Some(c) = category.choose(&mut rng) {
            password_chars.push(*c);
        }
    }

    // крок 2: добираємо решту символів довжини
    let all_chars: Vec<char> = categories.iter().flatten().copied().collect();
    for _ in password_chars.len()..len {
        password_chars.push(*all_chars.choose(&mut rng).unwrap());
    }

    // крок 3: перемішуємо, щоб "обов’язкові" символи не були на початку
    password_chars.shuffle(&mut rng);

    password_chars.into_iter().collect()
}
