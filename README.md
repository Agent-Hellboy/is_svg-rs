# is_svg

A library that determines whether the image type is SVG or not.



## Examples

Check the file directly:

```rust
use is_svg::is_svg;

fn main() {
    let res = is_svg("path/to/file");
    let res = match res {
        Ok(true) => println!("The file is in SVG format"),
        Ok(false) => println!("The file isn't in SVG format"),
        Err(e) => println!("Error while checking: {}", e),
    }
}
    
```
