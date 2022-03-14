in Cargo.toml  file `
[dependencies]
serde = {Version = "1.0" , features = ["derive"]}
serde_json = "1.0"

// Demo serialization
use serde{Deserialize, Serialize};

#[derive(Serialize, Deserialize , Debug)]
struct Form {
    email: String,
    name: String,
    age: usize,
}
let form = Form {
    email: "sample@example.com".to_string(),
    name: "sample".to_string(),
    age: 23,
}

//serialize
let serialized = serde_json::to_string(&form)
        .expect("failed to serialize ");

        println!("{}", serialized)


// deSerialize
let  deserialized: Result<Form, _>;
deserialized = serde_json::from_str(&serialized)
     .expect("failed to deserialize ");

println!("{:?}", deserialized)