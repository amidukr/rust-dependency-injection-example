use di_macro::FieldEnumerator;

#[allow(dead_code)]
#[derive(Debug, FieldEnumerator, Default)]
pub struct MyStruct {
    #[tag(init_listener)]
    field_1: i32,
    #[tag(init_listener)]
    #[tag(start_listener)]
    field_1_2: i32,
    field_2: i32,
    #[tag(start_listener)]
    field_3: i32,
}

macro_rules! my_callback {
    ($struct_name:ident, $field_name:ident, $listener_type:ident) => {
        println!(
            "struct = {}, field = {}, type = {}",
            stringify!($struct_name),
            stringify!($field_name),
            stringify!($listener_type),
        )
    };
}

pub fn run() {
    let s = MyStruct::default();
    println!("S = {:?}", s);

    enumerate_tags_MyStruct_init_listener!(my_callback);
    enumerate_tags_MyStruct_start_listener!(my_callback);
}
