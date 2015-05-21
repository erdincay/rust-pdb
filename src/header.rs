use std::io::{BufRead, BufReader, Read};

use record::RecordType;

pub struct Header {
    pub classification: String,
    pub dep_date: String,
    pub id_code: String,
}

impl Header {
    pub fn from_reader(f: &mut Read) -> Option<Header> {
        let reader = BufReader::new(f);
        let mut header = None;
        for line in reader.lines() {
            let line = line.unwrap();
            let mut elems: Vec<&str> = line.split_whitespace().collect();
            let record_string = elems.remove(0);
            let record_type = RecordType::parse_record_type(record_string);
            header = match record_type {
                Some(RecordType::HEADER) => {
                    let id_code = elems.pop().unwrap().to_string(); 
                    let dep_date = elems.pop().unwrap().to_string();
                    let classification = elems.connect(" ");
                    Some(Header {
                        id_code: id_code,
                        dep_date: dep_date,
                        classification: classification,
                    })
                },
                _ => break 
            };
        }
        header
    }
}
