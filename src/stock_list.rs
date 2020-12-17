use calamine::{open_workbook, DataType, Reader, Xlsx};
use futures_lite::io::AsyncReadExt;
use std::fs;
use url::form_urlencoded;

use crate::error::Error;

const TMPFILENAME: &str = "tmp.xlsx";
const CODECOLID: u32 = 4;

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
