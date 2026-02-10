use md5;
use rand::{Rng, distributions::Alphanumeric};
use reqwest::blocking::Client;
use serde::Deserialize;

const ENDPOINT: &str = "https://fanyi-api.baidu.com/api/trans/vip/translate";

#[derive(Debug, Deserialize)]
struct TransResultItem {
    dst: String,
}

#[derive(Debug, Deserialize)]
struct TransResponse {
    trans_result: Option<Vec<TransResultItem>>,
    error_code: Option<String>,
    error_msg: Option<String>,
}

#[derive(Debug)]
pub enum TranslateError {
    Http(reqwest::Error),
    Json(reqwest::Error),
    Api { code: String, msg: String },
    EmptyResult,
}

pub fn translate(
    q: &str,
    from: &str,
    to: &str,
    appid: String,
    appkey: String,
) -> Result<String, TranslateError> {
    let salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    // sign = MD5(appid + q + salt + appkey)，q 不要 URL encode
    let sign_raw = format!("{}{}{}{}", appid, q, salt, appkey);
    let sign = format!("{:x}", md5::compute(sign_raw.as_bytes()));

    let form = [
        ("q", q.to_string()),
        ("from", from.to_string()),
        ("to", to.to_string()),
        ("appid", appid.to_string()),
        ("salt", salt),
        ("sign", sign),
    ];

    let resp: TransResponse = Client::new()
        .post(ENDPOINT)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form) // 会自动做表单编码（q 在这里才 encode）
        .send()
        .map_err(TranslateError::Http)?
        .json()
        .map_err(TranslateError::Json)?;

    if let Some(code) = resp.error_code {
        return Err(TranslateError::Api {
            code,
            msg: resp.error_msg.unwrap_or_default(),
        });
    }

    let items = resp.trans_result.ok_or(TranslateError::EmptyResult)?;
    if items.is_empty() {
        return Err(TranslateError::EmptyResult);
    }

    Ok(items
        .into_iter()
        .map(|x| x.dst)
        .collect::<Vec<_>>()
        .join("\n"))
}
