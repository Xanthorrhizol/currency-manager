#![allow(dead_code)]
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CurrencyRateResponse {
    pub(crate) data_body: ResDataBody,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct ResDataBody {
    #[serde(rename = "R_RIBF3733_1")]
    pub(crate) currency_info: Vec<CurrencySellBuyInfo>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct CurrencySellBuyInfo {
    #[serde(rename = "통화CODE")]
    pub(crate) currency_code: String,
    #[serde(rename = "지폐매입환율")]
    pub(crate) bill_buy_exchange_rate: f32,
    #[serde(rename = "지폐매입환율_display")]
    pub(crate) bill_buy_exchange_rate_display: String,
    #[serde(rename = "지폐매도환율")]
    pub(crate) bill_sell_exchange_rate: f32,
    #[serde(rename = "지폐매도환율_display")]
    pub(crate) bill_sell_exchange_rate_display: String,
    #[serde(rename = "매매기준환율")]
    pub(crate) standard_exchange_rate: f32,
    #[serde(rename = "매매기준환율_display")]
    pub(crate) standard_exchange_rate_display: String,
}
