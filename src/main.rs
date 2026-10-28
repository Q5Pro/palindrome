//! palindrome: Palindrom kontrolü, en uzun palindrom alt dizisi bulma
//! ve palindrom istatistikleri.
//!
//! Kullanım:
//!     palindrome check "kayak"
//!     palindrome longest "racecarabacaba"
//!     palindrome stats "A man a plan a canal Panama"
use std::env;
use std::process;

fn normalize(s: &str) -> String {
    s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect()
}

fn is_palindrome(s: &str) -> bool {
    let n = normalize(s);
    let r: String = n.chars().rev().collect();
    n == r
}

fn longest_palindrome(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 { return String::new(); }

    let expand = |mut l: i64, mut r: i64| -> usize {
        while l >= 0 && r < n as i64 && chars[l as usize] == chars[r as usize] {
            l -= 1; r += 1;
        }
        (r - l - 1) as usize
    };

    let mut start = 0;
    let mut max_len = 1;
    for i in 0..n {
        let l1 = expand(i as i64, i as i64);
        let l2 = if i+1 < n { expand(i as i64, (i+1) as i64) } else { 0 };
        let best = l1.max(l2);
        if best > max_len {
            max_len = best;
            start = i - (best - 1) / 2;
        }
    }
    chars[start..start+max_len].iter().collect()
}

fn stats(s: &str) {
    let words: Vec<&str> = s.split_whitespace().collect();
    let total = words.len();
    let pal_words: Vec<&&str> = words.iter().filter(|w| is_palindrome(w)).collect();
    let longest = longest_palindrome(s);

    println!("Metin: \"{}\"", s);
    println!("─────────────────────────────────");
    println!("Palindrom mu?       : {}", if is_palindrome(s) { "✓ Evet" } else { "✗ Hayır" });
    println!("Toplam kelime       : {}", total);
    println!("Palindrom kelimeler : {} ({:?})", pal_words.len(),
        pal_words.iter().map(|w| w.to_string()).collect::<Vec<_>>());
    println!("En uzun palindrom   : \"{}\" ({} karakter)", longest, longest.chars().count());
    println!("Karakter sayısı     : {}", s.chars().count());
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        println!("Kullanım:");
        println!("  palindrome check \"metin\"");
        println!("  palindrome longest \"metin\"");
        println!("  palindrome stats \"metin\"");
        return;
    }
    match args[0].as_str() {
        "check" => {
            let text = args.get(1).map(|s| s.as_str()).unwrap_or("");
            if is_palindrome(text) {
                println!("✓ \"{}\" bir palindromdur.", text);
            } else {
                println!("✗ \"{}\" bir palindrom değildir.", text);
            }
        }
        "longest" => {
            let text = args.get(1).map(|s| s.as_str()).unwrap_or("");
            let p = longest_palindrome(text);
            println!("En uzun palindrom: \"{}\" ({} karakter)", p, p.chars().count());
        }
        "stats" => {
            let text = args[1..].join(" ");
            stats(&text);
        }
        _ => { eprintln!("check, longest veya stats kullanın."); process::exit(1); }
    }
}
