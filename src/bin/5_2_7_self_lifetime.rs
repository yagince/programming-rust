struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    // fn find_by_prefix<'a, 'b>(&'a self, prefix: &'b str) -> Option<&'a String> {
    fn find_by_prefix(&self, prefix: &str) -> Option<&String> {
        // for i in 0..self.elements.len() {
        //     if self.elements[i].starts_with(prefix) {
        //         return Some(&self.elements[i]); // これは、selfのelementsから直接取り出しているので、lifetimeがselfと同じになる？
        //     }
        // }
        // for s in self.elements {
        //     if s.starts_with(prefix) {
        //         return Some(&s); // これは、sのlifetimeがfor内になるので、生存期間がselfと一致しないので、コンパイルエラー
        //     }
        // }
        for s in self.elements.iter() { // iterはレシーバをmoveせずに、要素の参照のIteratorを返す
            if s.starts_with(prefix) {
                return Some(s);
            }
        }
        None
    }
}

fn main() {
    let st = StringTable {
        elements: vec![
            "japan".to_string(),
            "russia".to_string(),
            "china".to_string(),
        ],
    };

    println!("{:?}", st.find_by_prefix("ja"));
    println!("{:?}", st.find_by_prefix("ch"));
    println!("{:?}", st.find_by_prefix("ho"));
    println!("{:?}", st.find_by_prefix("blah"));
}
