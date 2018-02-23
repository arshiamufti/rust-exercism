pub fn build_proverb(list: Vec<&str>) -> String {
    if list.is_empty() {
        return String::from("")
    }
    let mut proverb = String::new();
    let (_, rest) = list.split_first().unwrap();

     for (a, b) in list.iter().zip(rest.iter()) {
        let s = format!("For want of a {} the {} was lost.\n", a, b);
        proverb.push_str(&s);
     }

    proverb.push_str(&(format!("And all for the want of a {}.", list[0])));
    proverb
}
