use chrono::{Local, Datelike};

//오늘 날짜 반환
pub fn get_now_time(input: &str) -> String {
    let local_time = Local::now();

    // 연도, 월, 일을 가져와서 문자열로 변환
    let year = local_time.year();
    let month = local_time.month();
    let day = local_time.day();

    // 문자열로 변환
    let today_date_ko = format!("{}년 {:02}월 {:02}일", year, month, day);
    let today_date = format!("{}{:02}{:02}", year, month, day);

    //매개변수에 따라 id 작성
    match input {
        "intro" => today_date_ko,
        "todo_id" => today_date,
        _ => todo!()
    }   
}

