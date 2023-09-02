use cliclack::log;
use std::process;
use console::style;

mod json;

fn main() -> std::io::Result<()> {

    cliclack::clear_screen()?;

    let today:String = format!("ToDo-Rustyy - {}", json::date::get_now_time("intro"));

    cliclack::intro(style(today).on_yellow().black())?;
    log::step("1. 할 일 추가\n2. 할 일 완료\n3. 할 일 수정\n4. 할 일 조회\n5. 할 일 삭제\n6. 종료")?;

    let selection: String = cliclack::input("무슨 작업을 하실래요?")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("작업을 선택해주세요.")
            } else if !(input == "1" || input == "2" || input == "3" || input == "4" || input == "5" || input == "6"){
                Err("정해진 선택지 내에서 정해주세요.")
            } else {
                Ok(())
            }
        })
        .interact()?;

    //선택지에 따른 match문 갈래
    match selection.as_str(){
        "1" => {
            let _kind = cliclack::select(format!("무슨 일정을 추가하시겠어요?'{selection}'"))
            .initial_value("ts")
            .item("ts", "TypeScript", "")
            .item("js", "JavaScript", "")
            .item("coffee", "CoffeeScript", "oh no")
            .interact()?;
        }
        "2" => {
            let _tools = cliclack::multiselect("무슨 일정을 완료하시겠어요?")
            .initial_values(vec!["prettier", "eslint"])
            .item("prettier", "Prettier", "recommended")
            .item("eslint", "ESLint", "recommended")
            .item("stylelint", "Stylelint", "")
            .item("gh-action", "GitHub Action", "")
            .interact()?;
        }
        "3" => {
            let _kind = cliclack::select(format!("무슨 일정을 수정하실래요?'{selection}'"))
            .initial_value("ts")
            .item("ts", "TypeScript", "")
            .item("js", "JavaScript", "")
            .item("coffee", "CoffeeScript", "oh no")
            .interact()?;
        }
        "4" => {
            let _kind = cliclack::select(format!("계획된 예정입니다!'{selection}'"))
            .initial_value("ts")
            .item("ts", "TypeScript", "")
            .item("js", "JavaScript", "")
            .item("coffee", "CoffeeScript", "oh no")
            .interact()?;
        }
        "5" => {
            let _kind = cliclack::select(format!("무슨 일정을 삭제하실래요?'{selection}'"))
            .initial_value("ts")
            .item("ts", "TypeScript", "")
            .item("js", "JavaScript", "")
            .item("coffee", "CoffeeScript", "oh no")
            .interact()?;
        }
        "6" => {
            println!("GOOD BYE!");
            process::exit(0);
        }
        _ => println!("blahh blahhh"),
}
    Ok(())
}
