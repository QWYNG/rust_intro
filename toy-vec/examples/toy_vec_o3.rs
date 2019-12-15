use toy_vec::ToyVec;

fn main() {
    let mut v = ToyVec::new();
    v.push("Thresh".to_string());
    v.push("Alister".to_string());

    let mut iter = v.iter();

    assert_eq!(iter.next(), Some(&"Thresh".to_string()));
    v.push("Soraka".to_string());
}
