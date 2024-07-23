fn make_random_string(len: u32) -> String {
    let rng = rand::thread_rng();
    let str: String = rng
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect();
    str
}
