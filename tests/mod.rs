extern crate is_svg;

#[test]
fn test_png() {
    assert_eq!(
        is_svg::is_svg("./images/example.png"),
        "please check imghdr to check this file"
    );
}
#[test]
fn test_jpeg() {
    assert_eq!(
        is_svg::is_svg("./images/example.svg"),
        "yes it is a svg file"
    );
}
