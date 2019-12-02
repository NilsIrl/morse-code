fn encode(character: char) -> &'static str {
    match character {
        'a' | 'A' => "._",
        'b' | 'B' => "_...",
        'c' | 'C' => "_._.",
        'd' | 'D' => "_..",
        'e' | 'E' => ".",
        'f' | 'F' => ".._.",
        'g' | 'G' => "__.",
        'h' | 'H' => "....",
        'i' | 'I' => "..",
        'j' | 'J' => ".___",
        'k' | 'K' => "_._",
        'l' | 'L' => "._..",
        'm' | 'M' => "__",
        'n' | 'N' => "_.",
        'o' | 'O' => "___",
        'p' | 'P' => ".__.",
        'q' | 'Q' => "__._",
        'r' | 'R' => "._.",
        's' | 'S' => "...",
        't' | 'T' => "_",
        'u' | 'U' => ".._",
        'v' | 'V' => "..._",
        'w' | 'W' => ".__",
        'x' | 'X' => "_.._",
        'y' | 'Y' => "_.__",
        'z' | 'Z' => "__..",
        '0' => "_____",
        '1' => ".____",
        '2' => "..___",
        '3' => "...__",
        '4' => "...._",
        '5' => ".....",
        '6' => "_....",
        '7' => "__...",
        '8' => "___..",
        '9' => "____.",
        ' ' => "/",
        '.' => "._._._",
        ',' => "__..__",
        '?' => "..__..",
        '\'' => ".____.",
        '!' => "_._.__",
        '/' => "_.._.",
        '(' => "_.__.",
        ')' => "_.__._",
        '&' => "._...",
        ':' => "___...",
        ';' => "_._._.",
        '=' => "_..._",
        '+' => "._._.",
        '-' => "_...._",
        '_' => "..__._",
        '"' => "._.._.",
        '$' => "..._.._",
        '@' => ".__._.",
        _ => unimplemented!(),
    }
}

fn decode(code: &str) -> char {
    match code {
        "._" => 'A',
        "_..." => 'B',
        "_._." => 'C',
        "_.." => 'D',
        "." => 'E',
        ".._." => 'F',
        "__." => 'G',
        "...." => 'H',
        ".." => 'I',
        ".___" => 'J',
        "_._" => 'K',
        "._.." => 'L',
        "__" => 'M',
        "_." => 'N',
        "___" => 'O',
        ".__." => 'P',
        "__._" => 'Q',
        "._." => 'R',
        "..." => 'S',
        "_" => 'T',
        ".._" => 'U',
        "..._" => 'V',
        ".__" => 'W',
        "_.._" => 'X',
        "_.__" => 'Y',
        "__.." => 'Z',
        "_____" => '0',
        ".____" => '1',
        "..___" => '2',
        "...__" => '3',
        "...._" => '4',
        "....." => '5',
        "_...." => '6',
        "__..." => '7',
        "___.." => '8',
        "____." => '9',
        "/" => ' ',
        "._._._" => '.',
        "__..__" => ',',
        "..__.." => '?',
        ".____." => '\'',
        "_._.__" => '!',
        "_.._." => '/',
        "_.__." => '(',
        "_.__._" => ')',
        "._..." => '&',
        "___..." => ':',
        "_._._." => ';',
        "_..._" => '=',
        "._._." => '+',
        "_...._" => '-',
        "..__._" => '_',
        "._.._." => '"',
        "..._.._" => '$',
        ".__._." => '@',
        _ => unimplemented!(),
    }
}

fn subcommand_with_name(name: &str) -> clap::App {
    clap::App::new(name)
        .arg(
            clap::Arg::with_name("dash")
                .long("dash")
                .takes_value(true)
                .default_value("_"),
        )
        .arg(
            clap::Arg::with_name("dot")
                .long("dot")
                .takes_value(true)
                .default_value("."),
        )
}

fn main() {
    use std::io::BufRead;
    let matches = clap::App::new("morse-code")
        .author("Nils <nils@nilsand.re>")
        .version("v0.0.1")
        .subcommand(subcommand_with_name("encode"))
        .subcommand(subcommand_with_name("decode"))
        .get_matches();

    println!(
        "{}",
        match matches.subcommand() {
            ("encode", Some(sub_m)) => std::io::stdin()
                .lock()
                .lines()
                .map(|line| line
                    .unwrap()
                    .chars()
                    .map(|character| encode(character))
                    .collect::<Vec<&str>>()
                    .join(" "))
                .collect::<Vec<String>>()
                .join("\n") // Morse code doesn't have newlines, so should the output not reflect that?
                .replace('_', sub_m.value_of("dash").unwrap())
                .replace('.', sub_m.value_of("dot").unwrap()),
            ("decode", Some(sub_m)) => std::io::stdin()
                .lock()
                .lines()
                .map(|line| line
                    .unwrap()
                    .split_whitespace()
                    .map(|code| decode(
                        &code
                            .replace(sub_m.value_of("dash").unwrap(), "_")
                            .replace(sub_m.value_of("dot").unwrap(), ".")
                    ))
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n"),
            _ => unreachable!(),
        }
    )
}
