use chrono::Utc;
use csv::Reader;
use json::JsonValue;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fs::File, io::Write};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "update", about = "Update")]
enum Opt {
    Report {
        #[structopt(short, long)]
        csv_path: PathBuf,
    },
    Update {
        #[structopt(short, long)]
        csv_path: PathBuf,
        #[structopt(short, long)]
        date: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let opt: Opt = Opt::from_args();
    match opt {
        Opt::Report { csv_path } => report(csv_path),
        Opt::Update { csv_path, date } => update(csv_path, date).await,
    }
}

async fn update(path: PathBuf, date: Option<String>) {
    let date = match date {
        Some(d) => d,
        None => Utc::now().naive_local().date().to_string(),
    };
    let body = format!("{{\"dataBody\":{{\"ricInptRootInfo\":{{\"serviceType\":\"GU\",\"serviceCode\":\"F3733\",\"nextServiceCode\":\"\",\"pkcs7Data\":\"\",\"signCode\":\"\",\"signData\":\"\",\"useSign\":\"\",\"useCert\":\"\",\"permitMultiTransaction\":\"\",\"keepTransactionSession\":\"\",\"skipErrorMsg\":\"\",\"mode\":\"\",\"language\":\"ko\",\"exe2e\":\"\",\"hideProcess\":\"\",\"clearTarget\":\"\",\"callBack\":\"shbObj.fncF3733Callback\",\"exceptionCallback\":\"\",\"requestMessage\":\"\",\"responseMessage\":\"\",\"serviceOption\":\"\",\"pcLog\":\"\",\"preInqForMulti\":\"\",\"makesum\":\"\",\"removeIndex\":\"\",\"redirectUrl\":\"\",\"preInqKey\":\"\",\"_multi_transfer_\":\"\",\"_multi_transfer_count_\":\"\",\"_multi_transfer_amt_\":\"\",\"userCallback\":\"\",\"menuCode\":\"\",\"certtype\":\"\",\"fromMulti\":\"\",\"fromMultiIdx\":\"\",\"isRule\":\"N\",\"webUri\":\"/index.jsp\",\"gubun\":\"\",\"tmpField2\":\"\"}},\"조회구분\":\"\",\"조회일자\":\"{}{}{}\",\"고시회차\":\"\"}},\"dataHeader\":{{\"trxCd\":\"RSRFO0100A01\",\"language\":\"ko\",\"subChannel\":\"49\",\"channelGbn\":\"D0\"}}}}", &date[0..4], &date[5..7], &date[8..10]);
    let client = reqwest::Client::new();
    let res = if let JsonValue::Object(o) = json::parse(
        &client
            .post("https://bank.shinhan.com/serviceEndpoint/httpDigital")
            .body(body)
            .header("Content-Type", "application/json")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap(),
    )
    .unwrap()
    {
        let mut map = HashMap::new();
        if let Some(JsonValue::Object(o)) = o.get("dataBody") {
            if let Some(JsonValue::Array(array)) = o.get("R_RIBF3733_1") {
                for elem in array.iter() {
                    if let JsonValue::Object(o) = elem {
                        let currency_type =
                            o.get("통화CODE").unwrap().as_str().unwrap().to_string();
                        let to_krw = o.get("지폐매입환율").unwrap().as_f32().unwrap();
                        map.insert(currency_type, to_krw);
                    }
                }
            }
        }
        map
    } else {
        panic!("couldn't get data from shinhan bank");
    };

    let mut file = File::options().append(true).open(path).unwrap();
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
