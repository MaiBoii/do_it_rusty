use dirs::document_dir;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

mod todos;
mod date;

//문서 폴더 아래에 todo 데이터들 저장할 디렉토리 특정 + 없으면 만들기
pub fn specific_save_dir() -> String{
        // Document Directory 경로 가져오기
        if let Some(document_dir) = dirs::document_dir() {
            // 저장할 디렉토리 경로 설정
            let save_directory = document_dir.join("rust_todo");
            // 디렉토리가 없으면 생성
            if !save_directory.exists() {
                fs::create_dir_all(&save_directory).expect("디렉토리를 만드는데 실패해버렸으");
            }
        }
        save_directory
    }

    //Document/rust_todo 디렉토리에 오늘자 JSON 파일명 지정 및 저장
pub fn save_to_document_dir(date: Date){
        // 파일 경로
        let file_path = format!("{}/{}.json", specific_save_dir, date::today);
        // JSON 파일 생성 및 데이터 저장
        let mut file = File::create(file_path).expect("파일을 생성하는데 실패했습니다.");
        file.write_all(json_data.as_bytes()).expect("Failed to write data to file");
        //저장해부러쓰ㄴ
        println!("JSON 데이터를 저장했습니다.");
    }

    //오늘 날짜에 해당하는 JSON 파일 특정하기
pub fn specific_todays_todo() {
    }