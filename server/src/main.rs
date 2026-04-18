use prost::Message;

pub mod user {
    include!(concat!(env!("OUT_DIR"), "/user.rs"));
}

fn main() {
    let original_user = user::User {
        id: 1,
        name: "Rust User".to_string(),
        email: "rust@example.com".to_string(),
    };

    // serialize
    let mut buf = Vec::new();
    buf.reserve(original_user.encoded_len());
    original_user.encode(&mut buf).unwrap();

    let binary_string = buf
        .iter()
        .map(|b| format!("{:08b}", b))
        .collect::<String>();

    println!("Serialized (binary): {}", binary_string);

    let deserialized_user = user::User::decode(&buf[..]).unwrap();
    assert_eq!(original_user, deserialized_user);
}