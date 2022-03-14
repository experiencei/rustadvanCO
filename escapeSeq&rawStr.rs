// escape sequence

let msg = "Hello\nworld";
let msg = "hello\tworld";
let msg = "left\\right";
let msg = "Over\"there\"";
let smiley = "\u{1f642}"; //unicode smileface


// Raw string

let msg = r"Hello
World!";
let msg = r"Hello        world!";
let msg = r"left\right";
let msg = r#"over "there""#;
let msg = r##"Over #"#there#"#"##;
let smiley = r"):";
