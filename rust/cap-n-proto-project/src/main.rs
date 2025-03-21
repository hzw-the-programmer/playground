use capnp::message::Builder;
use capnp::message::ReaderOptions;
use capnp::serialize;

pub mod person_schema_capnp;

// https://blog.logrocket.com/using-capn-proto-rust-serialize-deserialize-objects/

fn main() {
    // Creating object
    let mut message = Builder::new_default();
    let mut person = message.init_root::<person_schema_capnp::person::Builder>();
    person.set_name("John");
    person.set_age(23);

    // Serializing object
    let data = serialize::write_message_to_words(&message);
    println!("{:?}", data);

    // Deserializing object
    let reader = serialize::read_message(data.as_slice(), ReaderOptions::new()).unwrap();

    let person = reader
        .get_root::<person_schema_capnp::person::Reader>()
        .unwrap();
    let name = person.get_name().unwrap();
    println!("Name: {name}");
}
