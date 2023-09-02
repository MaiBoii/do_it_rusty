use std::fs::{self, File};
use std::io::Write;
use crate::json::todos::Date;
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

//Document/rust_todo 디렉토리에 오늘자 JSON 파일명 지정 및 저장
pub fn save_to_document_dir(date: Date){
        // 파일 경로지정
        let file_path = format!("{}/{}.json" ,*TODO_FOLDER_PATH,date::get_now_time("todo_id"));

        // JSON 파일 생성 및 데이터 저장
        let mut file = File::create(file_path).expect("파일을 생성하는데 실패했습니다.");

        //Stringify Pretty and unwrap
        let json_string = serde_json::to_string_pretty(&date).unwrap();

        //아까 문자열을 파일에다 작성
        file.write_all(json_string.as_bytes()).expect("파일에 내용을 작성하는데 실패했습니다.");
        
        //저장해부러쓰
        println!("JSON 데이터를 저장했습니다.");
}

