# DefaultFields

[![license-badge](https://img.shields.io/crates/l/default_fields)](https://github.com/qusuyan/DefaultFields/blob/main/LICENSE.txt)
[![crates-badge](https://img.shields.io/crates/v/default_fields)](https://crates.io/crates/default_fields)

Simple Rust derive macro that, for each field in the struct, creates a function that returns its default value (requires the struct to extend `Default` trait). This is useful if you want to parse struct from JSON and fill in the missing elements with defaults. For example, the following code:

```rust
#[derive(Deserialize)]
struct TestObj {
    #[serde(default = "TestObj::get_default_field1")]
    pub field1: bool,
    #[serde(default = "TestObj::get_default_field2")]
    pub field2: u64,
    #[serde(default = "TestObj::get_default_field3")]
    pub field3: String,
}

impl TestObj {

    pub fn get_default_field1() -> bool {
        true
    }

    pub fn get_default_field2() -> u64 {
        100u64
    }

    pub fn get_default_field3() -> String {
        String::from("test_str")
    }

}
```
can be simplified to:
```rust
#[derive(Deserialize, DefaultFields)]
struct TestObj {
    #[serde(default = "TestObj::get_default_field1")]
    pub field1: bool,
    #[serde(default = "TestObj::get_default_field2")]
    pub field2: u64,
    #[serde(default = "TestObj::get_default_field3")]
    pub field3: String,
}

impl Default for TestObj {
    fn default -> Self {
        Self {
            field1: true,
            field2: 100u64,
            field3: String::from("test_str"),
        }
    }
}
```