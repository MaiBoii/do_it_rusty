use dirs::document_dir;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn save_to_document_dir(){
    if let Some(document_dir) = document_dir(){
        // 해당 컴퓨터 문서 폴더명 특정
        let document_path = document_dir.to_str().unwrap();
        // 저장할 폴더 경로 지정
        let folder_path = document_path + String::from("/todos");
    } else {
        //만약에 없으면 만들기
        if !Path::new(folder_path).exists() {
            fs::create_dir_all(folder_path).expect("디렉토리를 만드는데 실패해버렸으");
        }
    }
    // 파일 경로
    let file_path = format!("{}/todo.json", folder_path);

    // JSON 파일 생성 및 데이터 저장
    let mut file = File::create(file_path).expect("Failed to create file");
    file.write_all(json_data.as_bytes()).expect("Failed to write data to file");

    println!("JSON 데이터를 저장했습니다.");
}