mod types;

use chrono::Utc;
use csv::Reader;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs::File, io::Write};
use structopt::StructOpt;
use types::{args::Args, request::CurrencyRateRequest, response::CurrencyRateResponse};

#[tokio::main]
async fn main() {
    let args: Args = Args::from_args();
    match args {
        Args::Report { csv_path } => report(csv_path),
        Args::Update { csv_path, date } => update(csv_path, date).await,
    }
}

async fn update(path: PathBuf, date: Option<String>) {
    let dates = match date {
        Some(d) => vec![d],
        None => {
            let mut reader = Reader::from_path(path.clone()).unwrap();
            let mut date = Utc::now().naive_local().date();
            let mut dates = Vec::new();
            match reader.records().last() {
                Some(Ok(record)) => {
                    let start_date =
                        chrono::NaiveDate::parse_from_str(&record[0], "%Y-%m-%d").unwrap();
                    while date > start_date {
                        dates.push(date.to_string());
                        date -= chrono::Duration::days(1);
                    }
                }
                _ => dates.push(date.to_string()),
            };
            dates.reverse();
            dates
        }
    };
    let mut file = File::options().append(true).open(path).unwrap();
    for date in dates.iter() {
        let shortdate = format!("{}{}{}", &date[0..4], &date[5..7], &date[8..10]);
        let request = CurrencyRateRequest::new(&shortdate);
        let client = reqwest::Client::new();
        let raw_res = client
            .post("https://bank.shinhan.com/serviceEndpoint/httpDigital")
            .body(serde_json::to_string(&request).unwrap())
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        let res = {
            let mut map = HashMap::new();
            for currency_info in serde_json::from_str::<CurrencyRateResponse>(&raw_res)
                .unwrap()
                .data_body
                .currency_info
                .iter()
            {
                map.insert(
                    currency_info.currency_code.clone(),
                    currency_info.bill_buy_exchange_rate,
                );
            }
            map
        };

        let krw_diff = 0;
        let jpy_diff = 0;
        let usd_diff = 0;
        let jpy_to_krw = res.get("JPY").unwrap();
        let usd_to_krw = res.get("USD").unwrap();
        let _ = file.write(
            format!(
                "{},{},{},{},{},{}\n",
                date, krw_diff, jpy_diff, usd_diff, jpy_to_krw, usd_to_krw,
            )
            .as_bytes(),
        );
    }
}

fn report(path: PathBuf) {
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
