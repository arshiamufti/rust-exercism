pub fn raindrops(n: usize) -> String {
    let is_pling = |n| n % 3 == 0;
    let is_plang = |n| n % 5 == 0;
    let is_plong = |n| n % 7 == 0;
    let mut raindrop_speak = String::new();
    if is_pling(n) {
        raindrop_speak.push_str("Pling");
    }
    if is_plang(n) {
        raindrop_speak.push_str("Plang");
    }
    if is_plong(n) {
        raindrop_speak.push_str("Plong");
    }
    if raindrop_speak.is_empty() {
        n.to_string()
    } else {
        raindrop_speak
    }
}
