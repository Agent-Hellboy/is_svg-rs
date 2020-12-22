#[test]
fn test_is_svg() -> Result<(),bool> {
        assert_eq!(is_svg::is_svg("./images/example.svg").unwrap(), true);
        Ok(())
    }
    
#[test]
#[should_panic]
fn test_is_not_svg() {
        assert_eq!(is_svg::is_svg("./images/example.png").unwrap(), false);
    }

