use csv;
use std::error::Error;

fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    if let Err(e) = read_from_path("./customers.csv") {
        eprintln!("{}", e);
    }
    0
}

fn read_from_path(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record.as_slice());
    }
    Ok(())
}
