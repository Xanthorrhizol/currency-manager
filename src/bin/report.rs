use csv::Reader;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <csv file path>", args[0]);
    }
    let path = &args[1];
    let mut reader = Reader::from_path(path).unwrap();

    println!(
        "date      \t  krw_diff\t  jpy_diff\tusd_diff\tjpy_to_krw\tusd_to_krw\tcurrent_value(krw)\tincome(krw)",
    );
    let mut krw = 0.0;
    let mut jpy = 0.0;
    let mut usd = 0.0;
    let mut first = true;
    let mut first_income = 0.0;

    for record in reader.records() {
        let record = record.unwrap();
        let date = record[0].to_string();
        let krw_diff: f32 = record[1].parse().unwrap();
        let jpy_diff: f32 = record[2].parse().unwrap();
        let usd_diff: f32 = record[3].parse().unwrap();
        let jpy_to_krw: f32 = record[4].parse().unwrap();
        let usd_to_krw: f32 = record[5].parse().unwrap();
        krw += krw_diff;
        jpy += jpy_diff;
        usd += usd_diff;
        let current_value = jpy * jpy_to_krw / 100.0 + usd * usd_to_krw;
        let mut income = current_value + krw;
        if first {
            first = false;
            first_income = income;
            income = 0.0;
            println!(
                "{:10 }\t{:10 }\t{:10 }\t{:11 }\t{:10 }\t{:10 }\t{:18 }\t{:11 }",
                date, krw_diff, jpy_diff, usd_diff, jpy_to_krw, usd_to_krw, current_value, income,
            );
        } else {
            println!(
                "{:10 }\t{:10 }\t{:10 }\t{:11 }\t{:10 }\t{:10 }\t{:18 }\t{:11 }",
                date,
                krw_diff,
                jpy_diff,
                usd_diff,
                jpy_to_krw,
                usd_to_krw,
                current_value,
                income - first_income,
            );
        }
    }
}
