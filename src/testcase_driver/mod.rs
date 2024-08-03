pub mod analyze;
pub mod exec;
use crate::testcase_driver::exec::*;
use std::fmt;
use std::io::{self, Write};

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


pub struct IndicateBar 
{
    pub max_progress: u32,
    pub progress: u32
}
impl fmt::Display for IndicateBar
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut progressed_bar = String::from("[");
        for _ in 0..self.progress * 15 / self.max_progress
        {
            progressed_bar.push('\u{1f525}');
        }
        if self.progress < self.max_progress
        {
            progressed_bar.push('\u{1f680}');
        }
       
        for _ in self.progress * 15 / self.max_progress..14
        {
            progressed_bar.push('-');
        }
        progressed_bar.push_str("");
        return write!(f, "{}] [{} exec/{} exec]\r", progressed_bar, self.progress, self.max_progress);
    }
}
impl IndicateBar
{
    pub fn new(max: u32) ->Self
    {
        Self
        {
            max_progress: max,
            progress: 0
        }
    }

    pub fn progress(&mut self, add_progress: u32)
    {
        self.progress += add_progress;
        print!("Test progress : \x1b[96m{}\x1b[0m\r", self);
        io::stdout().flush().unwrap();
    } 
    pub fn progress_one(&mut self)
    {
        self.progress(1);
    }
}