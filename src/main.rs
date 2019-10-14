mod csvprs;

fn main() -> Result<(), ()> {
    let f_result = std::fs::File::open("test.csv");
    let mut file;
    match f_result {
        Ok(f) => file = f,
        Err(_) => return Err(()),
    }
    let mut reader = csvprs::CsvReader::from_file(&mut file);
    let result = reader.process();
    if let Some(res) = result {
        println!("{:#?}", res.headers().unwrap());
        println!("{:#?}", res.data().unwrap());
    } else {
        println!("Failed");
    }

    Ok(())
}
