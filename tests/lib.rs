extern crate pdb;

use std::fs::File;
use std::path::Path;

use pdb::Header;

#[test]
fn test_parse_header() {
    let path = Path::new("tests/fixtures/1CKT.pdb");
    let mut file = File::open(&path).unwrap();
    let header = Header::from_reader(&mut file).unwrap();
    assert_eq!(header.id_code, "1CKT");
    assert_eq!(header.dep_date, "23-APR-99");
    assert_eq!(header.classification, "GENE REGULATION/DNA");
}
