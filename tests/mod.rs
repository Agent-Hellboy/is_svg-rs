#[test]
fn test_png() {
    assert_eq!(is_svg::is_svg("./images/example.png").unwrap(), false);
    // should it be false? maybe this should be changed accordingly
}
#[test]
fn test_jpeg() {
    assert_eq!(is_svg::is_svg("./images/example.svg").unwrap(), true);
}
