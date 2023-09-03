use cliclack::{log, MultiSelect, Select};
use std::process;
use console::style;
use json::{TODO_FOLDER_PATH, todos::{Date, Todo}, date::get_now_time};

mod json;

fn main() -> std::io::Result<()> {

    //문서 폴더 아래있는 rust_todo 폴더 경로 전역변수
    let _ = *TODO_FOLDER_PATH;

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

    cliclack::clear_screen()?;

    //선택지에 따른 match문 갈래
    match selection.as_str(){
        //추가하기
        "1" => {
            let today:String = format!("ToDo-Rustyy - {}", json::date::get_now_time("intro"));
            cliclack::intro(style(today).on_yellow().black())?;

            let todo_content: String = cliclack::input("무슨 일정을 추가하시겠어요?")
            .placeholder("할 일을 입력해주세요.")
            .validate(|input: &String| {
                if input.is_empty() {
                    Err("일정을 입력해주세요.")
                } else {
                    Ok(())
                }
            })
            .interact()?;

            //오늘자 첫 번째 일정을 입력하면 오늘의 빈 Date 파일 생성, 없으면 넘기기
            json::create_or_skip_todays_json_file()?;

            //입력한 todo_content를 오늘자 Date에다 저장하기
            let _ = json::add_task(todo_content);

            cliclack::outro(
                "입력하신 일정을 추가하였습니다.\n",
            )?;
        }

        //완료 처리하기
        "2" => {
            // 1.오늘자 Date 날짜에 todos가 비어있으면 예외처리하기
            
            let today:String = format!("ToDo-Rustyy - {}", json::date::get_now_time("intro"));
            cliclack::intro(style(today).on_yellow().black())?;

            let mut multi_select = MultiSelect::new("무슨 일정을 완료하시겠어요?");
    
            for i in 1..=10 {
                multi_select = multi_select.item(i, format!("Number {}", i), "");
            }

            let _tools = multi_select.interact()?;

            cliclack::outro(
                "입력하신 일정을 완료 처리하였습니다.\n",
            )?;
        }

        //수정하기
        "3" => {
            let today:String = format!("ToDo-Rustyy - {}", json::date::get_now_time("intro"));
            cliclack::intro(style(today).on_yellow().black())?;

            let _kind = cliclack::select(format!("무슨 일정을 수정하실래요?'{selection}'"))
            .initial_value("ts")
            .item("ts", "TypeScript", "")
            .item("js", "JavaScript", "")
            .item("coffee", "CoffeeScript", "oh no")
            .interact()?;

            cliclack::outro(
                "해당 일정을 수정하였습니다.\n",
            )?;
        }

        //조회하기
        "4" => {
            let today:String = format!("ToDo-Rustyy - {}", json::date::get_now_time("intro"));
            cliclack::intro(style(today).on_yellow().black())?;

            println!("{:#?}",json::get_all_tasks().unwrap());

            //오늘자 todo 파일에 content 값 다 가져와서 문자열 변수에다 넣기
            log::success("This is a success")?;
            log::step("This is a submitted step")?;
            log::success("This is a success")?;
            log::step("This is a submitted step")?;
            
            cliclack::outro(
                "이상 n개의 일정이 남아있습니다.\n",
            )?;
        }


        //삭제하기 
        "5" => {
            let today:String = format!("ToDo-Rustyy - {}", json::date::get_now_time("intro"));
            cliclack::intro(style(today).on_yellow().black())?;
            
            let _tools = cliclack::multiselect("무슨 일정을 삭제하실래요?")
                .initial_values(vec!["prettier", "eslint"])
                .item("prettier", "Prettier", "recommended")
                .item("eslint", "ESLint", "recommended")
                .item("stylelint", "Stylelint", "")
                .item("gh-action", "GitHub Action", "")
                .interact()?;

            cliclack::outro(
                "해당 일정을 삭제하였습니다.\n",
            )?;
        }

        //종료하기 
        "6" => {
            println!("GOOD BYE!");
            process::exit(0);
        }
        _ => println!("blahh blahhh"),
    }
    Ok(())
}
