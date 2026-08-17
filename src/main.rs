pub mod cards;
pub mod dice;

use crate::cards::cards;
use crate::dice::dice;
use std::fs;

pub const BIP39_WORDS: usize = 2048;

fn main() {
    let text = fs::read_to_string("bip39.txt").expect("read bip39.txt");
    let words: Vec<String> = text.lines().map(String::from).collect();
    assert_eq!(words.len(), BIP39_WORDS);

    fs::write("cards.html", cards(&words)).unwrap();
    fs::write("dice.html", dice(&words)).unwrap();
}

pub fn html_table(data: Vec<Vec<String>>) -> String {
    let mut html = String::from("<table border=\"1\" rules=\"rows\" cellpadding=\"5\">\n");

    let header = data.first().unwrap();
    html.push_str("  <tr>\n");
    for header in header {
        html.push_str(&format!("    <th>{}</th>\n", header));
    }
    html.push_str("  </tr>\n");

    for row in data.iter().skip(1) {
        html.push_str("  <tr>\n");
        for cell in row {
            html.push_str(&format!("    <td>{}</td>\n", cell));
        }
        html.push_str("  </tr>\n");
    }

    html.push_str("</table>");
    html
}
