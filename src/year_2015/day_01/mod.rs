pub mod part1 {
    pub fn challange(path: String) -> String {
        let contents =
            std::fs::read_to_string(path.clone()).expect(format!("Cant read {}", path).as_str());
        let ups: i32 = contents
            .split("")
            .into_iter()
            .filter(|c| *c == "(")
            .map(|c| 1)
            .sum();
        let downs: i32 = contents
            .split("")
            .into_iter()
            .filter(|c| *c == ")")
            .map(|c| 1)
            .sum();
        format!("{}", ups - downs)
    }
}

pub mod part2 {
    pub fn challange(path: String) -> String {
        let contents =
            std::fs::read_to_string(path.clone()).expect(format!("Cant read {}", path).as_str());
        "TODO".to_owned()
    }
}
