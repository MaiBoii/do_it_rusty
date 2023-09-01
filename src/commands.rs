use std::{collections::HashMap};
use std::io;
use chrono::{Local, Datelike};

static COUNT: u32 = 0;

pub fn get_now_time() -> String {
    let local_time = Local::now();

    // 연도, 월, 일을 가져와서 문자열로 변환
    let year = local_time.year();
    let month = local_time.month();
    let day = local_time.day();

    // 문자열로 변환
    let today_date = format!("{}-{:02}-{:02}", year, month, day);
    let todo_id = format!("{}{:02}{:02}{}", year,month,day,COUNT);

    today_date
}

enum Action {
    Add,
    Modify,
    Delete,
    Complete,
    List,
}

pub fn intoduction(){
    let mut tasks: HashMap<String, bool> = HashMap::new();

    loop {
        println!("{:?}", get_now_time());
        println!("할 일 관리 도구");
        println!("1. 할 일 추가");
        println!("2. 할 일 완료");
        println!("3. 할 일 편집 ");
        println!("4. 할 일 삭제");
        println!("5. 할 일 목록");
        println!("6. 종료");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("읽기 오류");
        let choice = choice.trim().parse::<u32>().unwrap_or(0);

        match choice {
            1 => manage_task(&mut tasks, Action::Add),
            2 => manage_task(&mut tasks, Action::Complete),
            3 => manage_task(&mut tasks, Action::Modify),
            4 => manage_task(&mut tasks, Action::Delete),
            5 => manage_task(&mut tasks, Action::List),
            6 => break,
            _ => println!("유효하지 않은 선택입니다."),
        }
}}

fn manage_task(tasks: &mut HashMap<String, bool>, action: Action) {
    println!("작업 이름을 입력하세요:");
    let mut task_name = String::new();
    io::stdin().read_line(&mut task_name).expect("읽기 오류");
    let task_name = task_name.trim().to_string();

    match action {
        Action::Add => {
            tasks.insert(task_name, false);
            println!("작업이 추가되었습니다.");
        }
        Action::Complete => {
            if let Some(task) = tasks.get_mut(&task_name) {
                *task = true;
                println!("작업이 완료 처리되었습니다.");
            } else {
                println!("작업을 찾을 수 없습니다.");
            }
        }
        Action::Delete => {
            if tasks.remove(&task_name).is_some() {
                println!("작업이 삭제되었습니다.");
            } else {
                println!("작업을 찾을 수 없습니다.");
            }
        }
        Action::List => {
            println!("할 일 목록:");
            for(task, completed) in tasks{
                let status = if *completed{ "완료" } else { "미완료" };
                println!("{} - {}", task, status);
            }
        }
        _ => {}
    }
}