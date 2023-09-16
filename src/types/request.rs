use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CurrencyRateRequest {
    data_header: ReqDataHeader,
    data_body: ReqDataBody,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ReqDataHeader {
    pub(crate) trx_cd: String,
    pub(crate) language: String,
    pub(crate) sub_channel: String,
    pub(crate) channel_gbn: String,
}

#[derive(Default, Clone, Debug, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct ReqDataBody {
    pub(crate) ric_inpt_root_info: RicInptRootInfo,
    #[serde(rename = "조회구분")]
    pub(crate) view_gbn: String,
    #[serde(rename = "조회일자")]
    pub(crate) view_date: String,
    #[serde(rename = "고시회차")]
    pub(crate) announce_round: String,
}

#[derive(Default, Clone, Debug, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub(crate) struct RicInptRootInfo {
    pub(crate) service_type: String,
    pub(crate) service_code: String,
    pub(crate) next_service_code: String,
    pub(crate) pkcs7_data: String,
    pub(crate) sign_code: String,
    pub(crate) sign_data: String,
    pub(crate) use_sign: String,
    pub(crate) use_cert: String,
    pub(crate) permit_multi_transaction: String,
    pub(crate) keep_transaction_session: String,
    pub(crate) skip_error_msg: String,
    pub(crate) mode: String,
    pub(crate) language: String,
    pub(crate) exe2e: String,
    pub(crate) hide_process: String,
    pub(crate) clear_target: String,
    pub(crate) call_back: String,
    pub(crate) exception_callback: String,
    pub(crate) request_message: String,
    pub(crate) response_message: String,
    pub(crate) service_option: String,
    pub(crate) pc_log: String,
    pub(crate) pre_inq_for_multi: String,
    pub(crate) makesum: String,
    pub(crate) remove_index: String,
    pub(crate) redirect_url: String,
    pub(crate) pre_inq_key: String,
    #[serde(skip)]
    pub(crate) _multi_transfer_: String,
    #[serde(skip)]
    pub(crate) _multi_transfer_count_: u32,
    #[serde(skip)]
    pub(crate) _multi_transfer_amt_: i32,
    pub(crate) user_callback: String,
    pub(crate) menu_code: String,
    pub(crate) certtype: String,
    pub(crate) from_multi: String,
    pub(crate) from_multi_idx: String,
    pub(crate) is_rule: String,
    pub(crate) web_uri: String,
    pub(crate) gubun: String,
}

impl CurrencyRateRequest {
    pub(crate) fn new(date: &str) -> Self {
        Self {
            data_body: ReqDataBody {
                ric_inpt_root_info: RicInptRootInfo {
                    service_type: "GU".to_string(),
                    service_code: "F3733".to_string(),
                    web_uri: "/index.jsp".to_string(),
                    is_rule: "N".to_string(),
                    call_back: "shbObj.fncF3733Callback".to_string(),
                    language: "ko".to_string(),
                    ..Default::default()
                },
                view_date: date.to_string(),
                ..Default::default()
            },
            data_header: ReqDataHeader {
                trx_cd: "RSRFO0100A01".to_string(),
                language: "ko".to_string(),
                sub_channel: "49".to_string(),
                channel_gbn: "D0".to_string(),
            },
        }
    }
}
