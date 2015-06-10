use std::io::{BufRead, BufReader, Read};

use record::RecordType;

pub struct Title {
    pub title: String,
    pub continuation: u8,
}

impl Title { 
    pub fn from_reader(f: &mut Read) -> Option<Title> {
        let reader = BufReader::new(f);
        let mut title = String::new();
        let mut continuation = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let mut elems: Vec<&str> = line.split_whitespace().collect();
            let record_string = elems.remove(0);
            let record_type = RecordType::parse_record_type(record_string);
            match record_type {
                Some(RecordType::TITLE) => {
                    continuation = continuation + 1;
                    if continuation > 1 {
                        title = title + " ";
                        elems.remove(0);
                    }
                    title = title + &elems.connect(" ");
                },
                _ => if continuation > 0 { break }
            };
        };
        if continuation > 0 {
            Some(Title {
                title: title,
                continuation: continuation,
            })
        } else {
            None
        }
    }
}
