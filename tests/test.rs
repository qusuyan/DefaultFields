#[macro_use]
extern crate default_fields;

#[derive(DefaultFields)]
struct Test {
    field1: bool,
    field2: String,
}

impl Default for Test {
    fn default() -> Self {
        Self {
            field1: true,
            field2: String::from("test"),
        }
    }
}

#[test]
pub fn get_field1() {
    assert!(Test::get_default_field1() == true);
}

#[test]
pub fn get_field2() {
    assert!(Test::get_default_field2() == String::from("test"));
}
