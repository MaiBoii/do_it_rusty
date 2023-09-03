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

    println!("{}", file_path);
    
    let todo = Todo {
        id: 0,
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

pub fn complete_task(task_id: usize) -> Result<(), io::Error> {
    let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));

    let file = File::open(file_path.clone())?;
    let mut parsed: Date = serde_json::from_reader(BufReader::new(file))?;

    parsed.todos[task_id].completed = true;

    let file = File::create(file_path)?;
    serde_json::to_writer(file, &parsed)?;

    Ok(())
}

pub fn delete_task(task_id: usize) -> Result<(), io::Error> {
    let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));

    let file = File::open(file_path.clone())?;
    let mut parsed: Date = serde_json::from_reader(BufReader::new(file))?;

    parsed.todos.remove(task_id);

    let file = File::create(file_path)?;
    serde_json::to_writer(file, &parsed)?;

    Ok(())
}

pub fn update_task(task_id: usize, task: String) -> Result<(), io::Error> {
    let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));

    let file = File::open(file_path.clone())?;
    let mut parsed: Date = serde_json::from_reader(BufReader::new(file))?;

    parsed.todos[task_id].content = task;

    let file = File::create(file_path)?;
    serde_json::to_writer(file, &parsed)?;

    Ok(())
}

pub fn get_completed_tasks() -> Result<Vec<String>, io::Error> {
    let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));

    let file = File::open(file_path.clone())?;
    let parsed: Date = serde_json::from_reader(BufReader::new(file))?;

    let mut todos: Vec<String> = Vec::new();

    for todo in parsed.todos {
        if todo.completed {
            todos.push(todo.content);
        }
    }

    Ok(todos)
}

pub fn get_incomplete_tasks() -> Result<Vec<String>, io::Error> {
    let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));

    let file = File::open(file_path.clone())?;
    let parsed: Date = serde_json::from_reader(BufReader::new(file))?;

    let mut todos: Vec<String> = Vec::new();

    for todo in parsed.todos {
        if !todo.completed {
            todos.push(todo.content);
        }
    }

    Ok(todos)
}

pub fn get_all_tasks() -> Result<Vec<String>, io::Error> {
    let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));

    let file = File::open(file_path.clone())?;
    let parsed: Date = serde_json::from_reader(BufReader::new(file))?;

    let mut todos: Vec<String> = Vec::new();

    for todo in parsed.todos {
        todos.push(todo.content);
    }

    Ok(todos)
}

