pub fn build_proverb(list: Vec<&str>) -> String {
    let mut proverb = String::new();
    let (first, rest) = list.split_first().unwrap();

    list.iter().zip(rest.iter()).map ( |a, b|
        proverb.push_back(format!("For want of a {} the {} was lost.\n", a, b));
    )

    proverb.push_back(format!("All for want of a {}.", list[0]))
}
