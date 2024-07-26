pub mod analyze;
pub mod exec;
use crate::testcase_driver::exec::*;

pub fn omit_testcase_or_other_name<'a>(path: &'a str, content: &'a str, interpreter_type:&LanguageType){
    
    let testcase_ext = match interpreter_type{
        LanguageType::Python => ".py",
        LanguageType::Ruby => ".rb"
    };

    let mut testcase_filename = "testcase".to_string();
    let mut name_id = 0;
    let mut already_exist_filenames: Vec<String> = Vec::new();
    for ent in std::fs::read_dir(path).unwrap(){
            let ent = ent.unwrap();
            if let Ok(file_name) = ent.file_name().into_string(){
                already_exist_filenames.push(file_name);
            }
    }
    testcase_filename += &name_id.to_string();
    testcase_filename += testcase_ext;
    while already_exist_filenames.contains(&testcase_filename){
        name_id +=1;
        testcase_filename = "testcase".to_string();
        testcase_filename += &name_id.to_string();
        testcase_filename += testcase_ext;
    }
    let mut path = path.to_string();
    if !path.ends_with('/'){
        path.push('/');
    }
    path += &testcase_filename;
    std::fs::write(&path, content).unwrap();

}