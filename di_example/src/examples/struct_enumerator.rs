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

// Example Output
//
// my_struct = MyStruct { field_1: 0, field_1_2: 0, field_2: 0, field_3: 0 }
// struct = MyStruct, field = field_1, type = init_listener
// struct = MyStruct, field = field_1_2, type = init_listener
// struct = MyStruct, field = field_1_2, type = start_listener
// struct = MyStruct, field = field_3, type = start_listener

pub fn run() {
    let my_struct = MyStruct::default();
    println!("my_struct = {:?}", my_struct);

    enumerate_tags_MyStruct_init_listener!(my_callback);
    enumerate_tags_MyStruct_start_listener!(my_callback);
}
