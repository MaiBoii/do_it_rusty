use std::fs::{self, File};
use std::io::{self, BufReader, Write};
use crate::json::todos::{Date, Todo};
use lazy_static::lazy_static;

lazy_static! {
    // 'rust_todo' 폴더 경로를 저장할 전역 변수
    pub static ref TODO_FOLDER_PATH: String = {
        if let Some(document_dir) = dirs::document_dir() {
            let todo_folder_path = document_dir.join("rust_todo");

            if !todo_folder_path.exists() {
                fs::create_dir(&todo_folder_path).expect("폴더 생성 실패");
            }
            todo_folder_path.to_string_lossy().into_owned()
        } else {
            panic!("Document 디렉토리를 찾을 수 없습니다.");
        }
    };
}

pub mod date;
pub mod todos;

//오늘자 json 파일이 없으면 만들고 있으면 넘어가는 함수
pub fn create_or_skip_todays_json_file() -> Result<(), io::Error> {

    let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));

    let date = Date{
        today: date::get_now_time("todo_id"),
        todos: Vec::new(),
    };
    
    // JSON 파일을 열거나 생성합니다.
    let file = File::open(file_path.clone());
    let _file = match file {
        Ok(file) => file,
        Err(_) => {
            // 파일이 없는 경우 새로운 파일을 생성합니다.
            let mut new_file = File::create(file_path.clone())?;

            let json_string = serde_json::to_string_pretty(&date).unwrap();

            new_file.write_all(json_string.as_bytes()).expect("실패한듯.");
            return Ok(());
        }
    };
    Ok(())
}

pub fn add_task(task: String) -> Result<(), io::Error> {
    let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));
    
    let todo = Todo {
        id: generate_task_id()? as usize,
        content: task.to_string(),
        completed: false 
    };

     // 파일이 없는 경우 새로운 JSON 객체를 생성합니다.
     let mut parsed: Date = match File::open(&file_path) {
        Ok(file) => serde_json::from_reader(BufReader::new(file))?,
        Err(_) => Date {
            today: date::get_now_time("todo_id"),
            todos: Vec::new(),
        },
    };

    // 입력받은 문자열을 data 필드에 추가
    parsed.todos.push(todo);

    // 파일 열기 및 데이터 쓰기
    let file = File::create(&file_path)?;
    serde_json::to_writer(file, &parsed)?;

    Ok(())
}

pub fn get_all_tasks() -> Result<Vec<(usize, String)>, io::Error> {
    let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));

    let file = File::open(file_path.clone())?;
    let parsed: Date = serde_json::from_reader(BufReader::new(file))?;

    let mut todos: Vec<(usize, String)> = Vec::new();

    for mut todo in parsed.todos {
        if todo.completed{
            todo.content = format!("{} ✅", todo.content);
        } else {
            todo.content = format!("{} ❌", todo.content);
        }
        todos.push((todo.id,todo.content));
    }

    Ok(todos)
}

pub fn complete_incomplete_task(task_id: usize) -> Result<(), io::Error> {
    let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));

    let file = File::open(file_path.clone())?;
    let mut parsed: Date = serde_json::from_reader(BufReader::new(file))?;

    let target_index = parsed.todos.iter().position(|x| x.id == task_id).unwrap();

    if parsed.todos[target_index].completed {
        parsed.todos[target_index].completed = false;
    } else {
        parsed.todos[target_index].completed = true;
    }

    let file = File::create(file_path)?;
    serde_json::to_writer(file, &parsed)?;

    Ok(())
}

pub fn delete_task(task_id: usize) -> Result<(), io::Error> {
    let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));

    let file = File::open(file_path.clone())?;
    let mut parsed: Date = serde_json::from_reader(BufReader::new(file))?;


    let target_index = parsed.todos.iter().position(|x| x.id == task_id).unwrap();
    parsed.todos.remove(target_index);

    let file = File::create(file_path)?;
    serde_json::to_writer(file, &parsed)?;

    Ok(())
}

pub fn update_task(task_id: usize, task: String) -> Result<(), io::Error> {
    let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));

    let file = File::open(file_path.clone())?;
    let mut parsed: Date = serde_json::from_reader(BufReader::new(file))?;

    let target_index = parsed.todos.iter().position(|x| x.id == task_id).unwrap();

    parsed.todos[target_index].content = task;

    let file = File::create(file_path)?;
    serde_json::to_writer(file, &parsed)?;

    Ok(())
}

//make todo's id by static variable
fn generate_task_id() -> Result<usize, io::Error> {
    let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));

    let file = File::open(file_path.clone())?;

    let parsed: Date = serde_json::from_reader(BufReader::new(file))?;

    let mut id: usize = 0;

    for todo in parsed.todos {
        if todo.id > id {
            id = todo.id as usize;
        }
    }

    Ok(id + 1)
}