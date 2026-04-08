use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use std::fmt;

// Структура, которая может десериализоваться из массива [i32; 2] или объекта {"a": i32, "b": i32}
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct PointVisitor;

impl<'de> Visitor<'de> for PointVisitor {
    type Value = Point;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("массив из 2 чисел или объект с полями x и y")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Point, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let x = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;
        let y = seq
            .next_element()?
            .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
        Ok(Point { x, y })
    }

    fn visit_map<A>(self, mut map: A) -> Result<Point, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut x = None;
        let mut y = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "x" => x = Some(map.next_value()?),
                "y" => y = Some(map.next_value()?),
                _ => return Err(serde::de::Error::unknown_field(&key, &["x", "y"])),
            }
        }

        let x = x.ok_or_else(|| serde::de::Error::missing_field("x"))?;
        let y = y.ok_or_else(|| serde::de::Error::missing_field("y"))?;

        Ok(Point { x, y })
    }
}

impl<'de> Deserialize<'de> for Point {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PointVisitor)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Из массива
    let json_arr = r#"[10, 20]"#;
    let p1: Point = serde_json::from_str(json_arr)?;
    println!("{:?}", p1); // Point { x: 10, y: 20 }

    // Из объекта
    let json_obj = r#"{"x": 30, "y": 40}"#;
    let p2: Point = serde_json::from_str(json_obj)?;
    println!("{:?}", p2); // Point { x: 30, y: 40 }

    Ok(())
}
