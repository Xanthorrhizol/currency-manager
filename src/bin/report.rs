use csv::Reader;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <csv file path>", args[0]);
    }
    let path = &args[1];
    let mut reader = Reader::from_path(path).unwrap();

    println!(
        "date      \t  won_diff\t  yen_diff\tdollar_diff\tyen_to_won\tdollar_to_won\tcurrent_value(won)\tincome(won)",
    );
    let mut won = 0.0;
    let mut yen = 0.0;
    let mut dollar = 0.0;
    let mut first = true;
    let mut first_income = 0.0;

    for record in reader.records() {
        let record = record.unwrap();
        let date = record[0].to_string();
        let won_diff: f32 = record[1].parse().unwrap();
        let yen_diff: f32 = record[2].parse().unwrap();
        let dollar_diff: f32 = record[3].parse().unwrap();
        let yen_to_won: f32 = record[4].parse().unwrap();
        let dollar_to_won: f32 = record[5].parse().unwrap();
        won += won_diff;
        yen += yen_diff;
        dollar += dollar_diff;
        let current_value = yen * yen_to_won / 100.0 + dollar * dollar_to_won;
        let mut income = current_value + won;
        if first {
            first = false;
            first_income = income;
            income = 0.0;
            println!(
                "{:10 }\t{:10 }\t{:10 }\t{:11 }\t{:10 }\t{:13 }\t{:18 }\t{:11 }",
                date,
                won_diff,
                yen_diff,
                dollar_diff,
                yen_to_won,
                dollar_to_won,
                current_value,
                income,
            );
        } else {
            println!(
                "{:10 }\t{:10 }\t{:10 }\t{:11 }\t{:10 }\t{:13 }\t{:18 }\t{:11 }",
                date,
                won_diff,
                yen_diff,
                dollar_diff,
                yen_to_won,
                dollar_to_won,
                current_value,
                income - first_income,
            );
        }
    }
}
