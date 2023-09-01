use chrono::{Local, Datelike};

pub fn get_now_time() -> String {
    let local_time = Local::now();

    // 연도, 월, 일을 가져와서 문자열로 변환
    let year = local_time.year();
    let month = local_time.month();
    let day = local_time.day();

    // 문자열로 변환
    let today_date = format!("{}년 {:02}월 {:02}일", year, month, day);

    today_date
}

