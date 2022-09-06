pub fn assert_strings_roughly_equal(a: impl Into<String>, b: impl Into<String>) {
    let undressed_a = (a.into() as String).as_str().replace(['\n', ' ', '\t'], "");
    let undressed_b = (b.into() as String).as_str().replace(['\n', ' ', '\t'], "");
    assert_eq!(undressed_a, undressed_b);
}
