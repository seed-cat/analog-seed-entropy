use crate::{html_table, BIP39_WORDS};

const DICE: [&str; 6] = [
    "⚀", "⚁", "⚂", "⚃", "⚄", "⚅",
];

pub fn dice(words: &[String]) -> String {
    // Generic code for generating tuples (should be easily portable to most languages)
    let mut tuples = vec![];
    for i0 in 0..6 {
        for i1 in 0..6 {
            for i2 in 0..6 {
                for i3 in 0..6 {
                    for i4 in 0..2 {
                        // Skip doubles
                        if i0 == i1 && i0 < 4 {
                            continue;
                        }
                        if i2 == i3 && i2 < 4 {
                            continue;
                        }

                        tuples.push(vec![i0, i1, i2, i3, i4]);
                    }
                }
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
    header.push("4th".to_string());
    header.push("5th".to_string());
    table.push(header);
    for i in 0..BIP39_WORDS {
        let mut row = vec![];
        row.push(format!("{:04}", i + 1));
        row.push(DICE[tuples[i][0]].to_string());
        row.push(DICE[tuples[i][1]].to_string());
        row.push(DICE[tuples[i][2]].to_string());
        row.push(DICE[tuples[i][3]].to_string());
        row.push(last_die(tuples[i][4]));
        row.push(words[i].to_string());
        table.push(row);
    }
    html_table(table)
}

fn last_die(index: usize) -> String {
    if index == 0 {
        format!("{}|{}|{}", DICE[0], DICE[1], DICE[2])
    } else {
        format!("{}|{}|{}", DICE[3], DICE[4], DICE[5])
    }
}
