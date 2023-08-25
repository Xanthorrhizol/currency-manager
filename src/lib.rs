pub(crate) struct AccountInfo {}

pub(crate) struct OpenBankingApiClient {
    account_info: AccountInfo,
    oauth_serv: OAuthCallbackServer,
}

struct OAuthCallbackServer {
    port: u16,
}
