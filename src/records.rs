use std::fs;
// Read /records.csv
// zone_id, record_id

fn read_records() -> String {
    let file = match fs::read("./records.csv") {
        Ok(v) => String::from_utf8(v),
        _ => panic!("There is no records.csv!")
    };

    match file {
        Ok(v) => v,
        _ => panic!("records.csv is not a csv file")
    }
}

fn parse_records(records: String) -> Vec<[String; 2]>  {
    let mut destructured_record: Vec<[String; 2]> = Vec::new();
    for str_record in records.split("\n") {
        let values = str_record.split(",");

        let values: Vec<&str> = values.collect();
        if values.len() == 2 {
            destructured_record.push([
                values[0].to_string(),
                values[1].to_string(),
            ]);
        }
    }

    destructured_record
}

pub fn get_records() -> Vec<[String; 2]> {
    let csv_records = read_records();
    
    parse_records(csv_records)
}