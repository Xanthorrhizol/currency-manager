use chrono::Utc;
use json::JsonValue;
use std::collections::HashMap;
use std::{fs::File, io::Write};

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("Usage: {} <csv file path>", args[0]);
    }
    let path = &args[1];

    let date = Utc::now().naive_local().date().to_string();
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
