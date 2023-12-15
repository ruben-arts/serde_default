use serde::{Deserialize, Deserializer};
#[derive(Debug, Deserialize)]
struct Foos {
    foo: String,
}
#[derive(Debug, Deserialize)]
struct Test {
    #[serde(deserialize_with = "deserialize_foos")]
    pub foo: Option<Foos>,
    pub bar: String,
}

fn deserialize_foos<'de, D>(deserializer: D) -> Result<Option<Foos>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;

    // Do some magic with the data
    match s {
        Some(str_val) => Ok(Some(Foos { foo: str_val })),
        None => Ok(None),
    }
}

fn main() {
    let succeeding_contents = format!(
        r#"
        foo = "foo"
        bar = "test"
        "#
    );
    let _ = toml_edit::de::from_str::<Test>(&succeeding_contents)
        .expect("parsing should as foo is optional succeed!");

    let unsucceeding_contents = format!(
        r#"
    bar = "test"
    "#
    );

    let _ = toml_edit::de::from_str::<Test>(&unsucceeding_contents)
        .expect("parsing should as foo is optional succeed!");
}
