#[s_macro::test_attribute(100)]
fn bar() -> u8 {
    println!("YES");
    1
}
