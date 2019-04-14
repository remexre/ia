use crate::IQM;
use std::fs::read;

#[test]
fn parse_all_assets() {
    for entry in glob::glob("../../assets/**/*.iqm").unwrap() {
        let path = entry.unwrap();
        println!("Parsing {}...", path.display());
        let data = read(path).unwrap();
        assert!(IQM::parse_from(&data).is_some());
    }
}
