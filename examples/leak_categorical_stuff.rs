use std::{
    io::Cursor,
    sync::Arc,
    time::{Duration, Instant},
};

use polars::{
    io::csv,
    lazy::dsl::all,
    prelude::{DataType, Field, IntoLazy, SerReader},
};

fn main() {
    let seconds: u64 = std::env::args().nth(1).unwrap().parse().unwrap();

    let asd = csv::CsvReader::new(Cursor::new(include_bytes!("../res/example1.csv")))
        .with_dtypes(Some(Arc::new(
            [
                Field::new("foo_id", DataType::Categorical(None)),
                Field::new("bar_id", DataType::Categorical(None)),
                Field::new("stuff", DataType::UInt64),
                Field::new("thing", DataType::UInt64),
                Field::new("bar_stuff", DataType::UInt64),
                Field::new("bar_value", DataType::UInt64),
                Field::new("bar_thing", DataType::UInt64),
                Field::new("bar_total", DataType::UInt64),
                Field::new("capacity", DataType::UInt64),
                Field::new("foo_capacity", DataType::UInt64),
            ]
            .into_iter()
            .collect(),
        )))
        .has_header(true)
        .finish()
        .unwrap();

    let bsd = csv::CsvReader::new(Cursor::new(include_bytes!("../res/example2.csv")))
        .with_dtypes(Some(Arc::new(
            [
                Field::new("bar_id", DataType::Categorical(None)),
                Field::new("default_whatever", DataType::Float32),
                Field::new("min_whatever", DataType::Float32),
                Field::new("max_whatever", DataType::Float32),
                Field::new("is_true", DataType::Boolean),
            ]
            .into_iter()
            .collect(),
        )))
        .with_try_parse_dates(true)
        .has_header(true)
        .finish()
        .unwrap();

    let start = Instant::now();

    loop {
        let _asd = asd
            .clone()
            .lazy()
            .groupby_stable(["timestamp", "bar_id", "foo_id"])
            .agg([all().last()])
            .collect()
            .unwrap();

        let _bsd = bsd
            .clone()
            .lazy()
            .groupby_stable(["timestamp", "bar_id"])
            .agg([all().last()])
            .collect()
            .unwrap();

        if Instant::now().duration_since(start) > Duration::from_secs(seconds) {
            break;
        }
    }
}
