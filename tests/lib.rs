extern crate pdb;

use std::fs::File;
use std::path::Path;

use pdb::Header;
use pdb::Title;

#[test]
fn test_parse_header() {
    let path = Path::new("tests/fixtures/1CKT.pdb");
    let mut file = File::open(&path).unwrap();
    let header = Header::from_reader(&mut file).unwrap();
    assert_eq!(header.id_code, "1CKT");
    assert_eq!(header.dep_date, "23-APR-99");
    assert_eq!(header.classification, "GENE REGULATION/DNA");
}

#[test]
fn test_parse_title() {
    let path = Path::new("tests/fixtures/1CKT.pdb");
    let mut file = File::open(&path).unwrap();
    let title = Title::from_reader(&mut file).unwrap();
    assert_eq!(title.title,
               "CRYSTAL STRUCTURE OF HMG1 DOMAIN A BOUND TO A CISPLATIN-MODIFIED DNA DUPLEX");
    assert_eq!(title.continuation, 2);
}
