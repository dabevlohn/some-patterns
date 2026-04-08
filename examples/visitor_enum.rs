use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use std::fmt;

// Тип, который может быть либо строкой, либо числом
#[derive(Debug, PartialEq)]
enum StringOrInt {
    String(String),
    Int(i64),
}

// Visitor для обработки обоих случаев
struct StringOrIntVisitor;

impl<'de> Visitor<'de> for StringOrIntVisitor {
    type Value = StringOrInt;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("строку или целое число")
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrInt::Int(value))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrInt::Int(value as i64))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrInt::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(StringOrInt::String(value))
    }
}

impl<'de> Deserialize<'de> for StringOrInt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Используем deserialize_any для JSON, который может быть строкой или числом
        deserializer.deserialize_any(StringOrIntVisitor)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Тест 1: десериализация из числа
    let json_int = r#"42"#;
    let val_int: StringOrInt = serde_json::from_str(json_int)?;
    println!("Из числа: {:?}", val_int); // Int(42)

    // Тест 2: десериализация из строки
    let json_str = r#""hello""#;
    let val_str: StringOrInt = serde_json::from_str(json_str)?;
    println!("Из строки: {:?}", val_str); // String("hello")

    // Тест 3: в составе структуры
    #[derive(Deserialize, Debug)]
    struct Config {
        name: String,
        value: StringOrInt,
    }

    let json_config = r#"{"name": "test", "value": 100}"#;
    let config: Config = serde_json::from_str(json_config)?;
    println!("Структура с числом: {:?}", config);

    let json_config2 = r#"{"name": "test", "value": "dynamic"}"#;
    let config2: Config = serde_json::from_str(json_config2)?;
    println!("Структура со строкой: {:?}", config2);

    Ok(())
}
