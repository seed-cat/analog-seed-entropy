use crate::{BIP39_WORDS, html_table};

const CARDS: [&str; 14] = [
    "A♥", "A♠", "2♠", "3♠", "4♠", "5♠", "6♠", "7♠", "8♠", "9♠", "10♠", "J♠", "Q♠", "K♠",
];

pub fn cards(words: &[String]) -> String {
    // Generic code for generating tuples (should be easily portable to most languages)
    let mut tuples = vec![];
    for i0 in 0..14 {
        for i1 in 0..14 {
            for i2 in 0..14 {
                // iterate over all combinations without repetition
                if i0 == i1 || i0 == i2 || i1 == i2 {
                    continue;
                }

                // skip tuples
                if i0 == 0 && (i1 > 2 || i1 == 2 && i2 > 9) {
                    continue;
                }

                // add the tuple
                tuples.push(vec![i0, i1, i2]);
            }
        }
    }

    // Check tuples for duplicates
    assert_eq!(tuples.len(), BIP39_WORDS);
    tuples.dedup();
    assert_eq!(tuples.len(), BIP39_WORDS);

    // Format for Markdown / HTML files
    let mut table = vec![];
    let mut header = vec![];
    header.push("#".to_string());
    header.push("1st".to_string());
    header.push("2nd".to_string());
    header.push("3rd".to_string());
    table.push(header);
    for i in 0..BIP39_WORDS {
        let mut row = vec![];
        row.push(format!("{:04}", i + 1));
        row.push(html_card(tuples[i][0]));
        row.push(html_card(tuples[i][1]));
        row.push(html_card(tuples[i][2]));
        row.push(words[i].to_string());
        table.push(row);
    }
    html_table(table)
}

fn html_card(index: usize) -> String {
    if index == 0 {
        format!("<span style=\"color: red\">{}</span>", CARDS[index])
    } else {
        format!("{}", CARDS[index])
    }
}
