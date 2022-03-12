use std::io;

const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const LYRICS: [&str; 14] = [
    "on the #### day of Christmas",
    "my true love sent to me",
    "a partridge in a pear tree",
    "two turtle doves",
    "three french hens",
    "four calling birds",
    "five golden rings!",
    "six geese a layin'",
    "seven swans a swimmin'",
    "eight maids a milkin'",
    "nine ladies dancin'",
    "ten lords a leapin'",
    "eleven pipers pipin'",
    "twelve drummers drummin'",
];

fn main() {
    println!("Type how many days has your Christmas to a max of 12 days!");
    loop {
        let mut days = String::new();
        io::stdin()
            .read_line(&mut days)
            .expect("Please type some value!");
        let days: u8 = match days.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Please type a valid value!({})", e);
                continue;
            }
        };
        if days == 0 || days > 12 {
            println!("Your option needs to be between 1 and 12!");
            continue;
        }

        for i in 0..days {
            println!("{}", build_verse(i));
        }
        break;
    }
}

fn build_verse(pos: u8) -> String {
    let mut verse = String::new();
    let mut base = String::from(LYRICS[0]).replace("####", DAYS[pos as usize]);
    base.push_str("\n");
    verse.push_str(base.as_str());
    verse.push_str(LYRICS[1]);
    verse.push_str("\n");
    for i in (0..(pos + 1)).rev() {
        if pos != 0 && i == 0 {
            verse.push_str("and ")
        }
        verse.push_str(LYRICS[(i + 2) as usize]);
        verse.push_str("\n");
    }
    captalize_all_lines(verse)
}

fn captalize_all_lines(text: String) -> String {
    let mut converted = String::new();
    let lines = text.lines();
    for line in lines {
        let split = line.split_at(1);
        converted.push_str(String::from(split.0).to_uppercase().as_str());
        converted.push_str(String::from(split.1).to_lowercase().as_str());
        converted.push_str("\n");
    }
    return converted;
}

#[test]
fn test_capitalize_sentences() {
    assert_eq!(captalize_all_lines(String::from("casa")), "Casa\n");
}
