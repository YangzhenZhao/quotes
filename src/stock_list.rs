use calamine::{open_workbook, DataType, Reader, Xlsx};
use futures::join;
use futures::AsyncReadExt;
use isahc::prelude::*;
pub use serde::{Deserialize, Serialize};
use std::fs;
use url::form_urlencoded;

use crate::error::Error;

const TMPFILENAME: &str = "tmp.xlsx";
const CODECOLID: u32 = 4;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
struct ShItem {
    security_code_a: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct ShResult {
    result: Vec<ShItem>,
}

pub async fn stock_list() -> Result<Vec<String>, Error> {
    let (list_sh, list_sz) = join!(stock_list_sh(), stock_list_sz());
    Ok([list_sh?, list_sz?].concat())
}

pub async fn stock_list_sh() -> Result<Vec<String>, Error> {
    let (list_type1, list_type8) = join!(stock_list_sh_by_type("1"), stock_list_sh_by_type("8"));
    Ok([list_type1?, list_type8?].concat())
}

pub async fn stock_list_sz() -> Result<Vec<String>, Error> {
    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("SHOWTYPE", "xlsx")
        .append_pair("CATALOGID", "1110")
        .append_pair("TABKEY", "tab1")
        .append_pair("random", "0.6935816432433362")
        .finish();
    let url = format!("http://www.szse.cn/api/report/ShowReport?{}", encoded);
    let response = isahc::get_async(url).await?;
    let mut buf = [0; 8192];
    let mut reader = response.into_body();
    let mut file_u8 = vec![];
    loop {
        match reader.read(&mut buf).await? {
            0 => break,
            len => {
                for &byte in &buf[..len] {
                    file_u8.push(byte);
                }
            }
        }
    }
    fs::write(TMPFILENAME, file_u8)?;
    let mut workbook: Xlsx<_> = open_workbook(TMPFILENAME)?;
    let range = workbook
        .worksheet_range_at(0)
        .ok_or(calamine::Error::Msg("Cannot find 'Sheet0'"))??;
    let mut codes = vec![];
    for i in 1..range.height() as u32 {
        let v = range.get_value((i, CODECOLID));
        if let Some(DataType::String(code)) = v {
            codes.push(code.clone());
        }
    }
    Ok(codes)
}

async fn stock_list_sh_by_type(stock_type: &str) -> Result<Vec<String>, Error> {
    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("jsonCallBack", "jsonpCallback66942")
        .append_pair("isPagination", "true")
        .append_pair("stockCode", "")
        .append_pair("csrcCode", "")
        .append_pair("areaName", "")
        .append_pair("stockType", stock_type)
        .append_pair("pageHelp.cacheSize", "1")
        .append_pair("pageHelp.beginPage", "1")
        .append_pair("pageHelp.pageSize", "2000")
        .append_pair("pageHelp.pageNo", "1")
        .append_pair("pageHelp.endPage", "11")
        .append_pair("_", "1589881387934")
        .finish();
    let url = format!(
        "http://query.sse.com.cn/security/stock/getStockListData.do?{}",
        encoded
    );
    let client = HttpClient::new()?;
    let request = Request::get(url)
        .header("Host", "query.sse.com.cn")
        .header("Pragma", "no-cache")
        .header("Referer", "http://www.sse.com.cn/assortment/stock/list/share/")
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/81.0.4044.138 Safari/537.36")
        .body("")?;
    let mut response = client.send_async(request).await?;
    let res_str = response.text()?;
    let begin_pos = match res_str.find('{') {
        None => return Err(Error::Msg("Responese text error!")),
        Some(p) => p,
    };
    let res_str = match res_str.get(begin_pos..res_str.len() - 1) {
        None => return Err(Error::Msg("Response text error!")),
        Some(s) => s,
    };
    let res: ShResult = serde_json::from_str(res_str)?;
    Ok(res.result.into_iter().map(|x| x.security_code_a).collect())
}
