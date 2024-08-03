use docker_api::opts::{ContainerCreateOpts,ContainerRestartOpts, ContainerStopOpts, ExecCreateOpts};
use docker_api::{conn::TtyChunk, Docker};
use futures::StreamExt;
use tokio::time::{sleep, Duration};
use std::str;

use crate::grammar::predef_grammars::{get_python_grammar, get_ruby_grammar};
use crate::grammar::str_helper::replace_scope_with_indent;
use crate::grammar::Union;
use crate::grammar_fuzzer::var_ctx::ir_to_ctx;
use crate::grammar_fuzzer::GrammarsFuzzer;

use super::IndicateBar;


pub struct PrintResult{
    pub stdout: String,
    pub stderr: String
}

pub enum PlatformType{
    V8, SpiderMonkey, JavascriptCore, Native
}
pub enum LanguageType{
    Python, Ruby
}
pub struct TestInfo{
    pub target_platform:PlatformType,
    pub target_language:LanguageType
}
pub async fn exec_test(docker:&mut Docker, test_count: u32, init_seed: u64, object_interpreter:LanguageType){
    let mut cli_progress_bar = IndicateBar::new(test_count);
    let language_grammar = match object_interpreter{
        LanguageType::Python => &get_python_grammar(),
        LanguageType::Ruby => &get_ruby_grammar()
    };
    let testcase_file_ext = match object_interpreter{
        LanguageType::Python => ".py",
        LanguageType::Ruby => ".rb"
    };

    let test_exec_infos = match object_interpreter{
        LanguageType::Python =>  &[
            TestInfo{
            target_platform:PlatformType::V8,
            target_language:LanguageType::Python
            },
             TestInfo{
            target_platform:PlatformType::JavascriptCore,
            target_language:LanguageType::Python
            }, 
             TestInfo{
            target_platform:PlatformType::SpiderMonkey,
            target_language:LanguageType::Python
            }, 
            TestInfo{
            target_platform:PlatformType::Native,
            target_language:LanguageType::Python
            }
        ],
        LanguageType::Ruby =>  &[
            TestInfo{
            target_platform:PlatformType::V8,
            target_language:LanguageType::Ruby
            },
             TestInfo{
            target_platform:PlatformType::JavascriptCore,
            target_language:LanguageType::Ruby
            }, 
             TestInfo{
            target_platform:PlatformType::SpiderMonkey,
            target_language:LanguageType::Ruby
            }, 
            TestInfo{
            target_platform:PlatformType::Native,
            target_language:LanguageType::Ruby
            }
        ]
    };
    let mut testcase_generator = GrammarsFuzzer::new(
        &language_grammar,
        "<start>",
        10,
        100,
        Union::OnlyA(false),
    );
    let mut rand_seed = init_seed.clone();
    
    for i in 0..test_count{ 
        cli_progress_bar.progress_one();
        let testcase = testcase_generator.fuzz(&mut rand_seed);
        let test_input = replace_scope_with_indent(&ir_to_ctx(&testcase, &mut rand_seed.clone()));
        let mut results:Vec<PrintResult> = Vec::new(); 


        for test_info in test_exec_infos{
            let result = execute_test(docker, test_info, test_input.as_bytes()).await;
            results.push(result);
        }
    
        let is_v8_and_jsc_result_same = results[0].stdout == results[1].stdout;
        let is_v8_and_spidermonkey_result_same = results[0].stdout == results[2].stdout;
        let is_jsc_and_spidermonkey_result_same = results[1].stdout == results[2].stdout;
        
        let are_js_engines_result_same = (is_v8_and_jsc_result_same && is_v8_and_spidermonkey_result_same) && is_jsc_and_spidermonkey_result_same;

        let is_v8_and_native_result_same = results[0].stdout == results[3].stdout.clone() + &results[3].stderr;
        let is_jsc_and_native_result_same = results[1].stdout == results[3].stdout.clone() + &results[3].stderr;
        let is_spidermonkey_and_native_result_same = results[2].stdout == results[3].stdout.clone() + &results[3].stderr;
        
        let is_native_wasm_same = (is_v8_and_native_result_same && is_jsc_and_native_result_same) && is_spidermonkey_and_native_result_same;
       

        if !are_js_engines_result_same{

            if is_jsc_and_spidermonkey_result_same{

                println!("⚠Warning!: V8 result is different with other engines!");
                std::fs::write("./issues/v8/log".to_string() + &i.to_string() + ".txt", &results[0].stdout).unwrap();
                std::fs::write("./issues/v8/testcase".to_string() + &i.to_string() + testcase_file_ext, &test_input).unwrap();

            }else if is_v8_and_spidermonkey_result_same{

                println!("⚠Warning!: JavascriptCore result is different with other engines!");
                std::fs::write("./issues/jsc/log".to_string() + &i.to_string() + ".txt", &results[1].stdout).unwrap();
                std::fs::write("./issues/jsc/testcase".to_string() + &i.to_string() + testcase_file_ext, &test_input).unwrap();

            }else if is_v8_and_jsc_result_same{

                println!("⚠Warning!: SpiderMonkey result is different with other engines!");
                std::fs::write("./issues/spm/log".to_string() + &i.to_string() + ".txt", &results[2].stdout).unwrap();
                std::fs::write("./issues/spm/testcase".to_string() + &i.to_string() + testcase_file_ext, &test_input).unwrap();

            }else{

                println!("⚠Warning!: Each result of js engines different with other engines!");
                std::fs::write("./issues/unknown/log".to_string() + &i.to_string() + ".txt", 
                format!("d8(V8):\n{}\n", results[0].stdout)
                + &format!("jsc(JavascriptCore):\n{}\n", results[1].stdout)
                + &format!("JsShell(SpiderMonkey):\n{}\n", results[2].stdout)).unwrap();
                std::fs::write("./issues/unknown/testcase".to_string() + &i.to_string() + testcase_file_ext, &test_input).unwrap();

            }

        }else if !is_native_wasm_same{
            std::fs::write("./issues/native_vs_wasm/log".to_string() + &i.to_string() + ".txt", 
            format!("d8(V8):\n{}\n", results[0].stdout)
            + &format!("Native:\n{}\n", results[3].stdout.clone() + &results[3].stderr)).unwrap();
            std::fs::write("./issues/native_vs_wasm/testcase".to_string() + &i.to_string() + testcase_file_ext, &test_input).unwrap();
            //omit_testcase_or_other_name("./issued_testcases/native_vs_wasm", &test_input, &LanguageType::Python);
        }
    }
}

pub async fn execute_test<'a>(docker:&mut Docker, test_target:&TestInfo, testcase: &'a [u8])-> PrintResult{
    let container_name = "testcase_execution_container".to_string();
    let mut image_name = String::with_capacity(6);
    //["./jsc", "module-ready-wasi-py.js", "--", "none"]
    let mut execution_command: Vec<& 'a str> = Vec::with_capacity(3);
    let mut container_testcase_path: String = "/root/".to_string();

  match test_target.target_platform{
        PlatformType::V8 => {
            image_name += "d8_";
            execution_command.push("./d8");
        },
        PlatformType::SpiderMonkey => {
            image_name += "js_";
            execution_command.push("./js");
        },
        PlatformType::JavascriptCore => {
            image_name += "jsc_";
            execution_command.push("./jsc");
        },
        PlatformType::Native => {
            image_name += "na_";
        }
    };

    match test_target.target_language{
        LanguageType::Python => {
            image_name += "py";

            match test_target.target_platform{
                PlatformType::Native => {
                    container_testcase_path += "py-native-sandbox/testcase.py";
                    execution_command.push("./bin/python3");
                    execution_command.push("./testcase.py");
                },
                _ =>{
                    container_testcase_path += "py-wasi-sandbox/testcase.py";
                    execution_command.push("module-ready-wasi-py.js");
                }
            };
        },
        LanguageType::Ruby => {
            image_name += "rb";
            match test_target.target_platform{
                PlatformType::Native => {
                    container_testcase_path += "rb-native-sandbox/testcase.rb";
                    execution_command.push("./bin/ruby");
                    execution_command.push("./testcase.rb");
                },
                _ =>{
                    container_testcase_path += "rb-wasi-sandbox/testcase.rb";
                    execution_command.push("module-ready-wasi-rb.js");
                }
            };
        }
    };

    match test_target.target_platform { 
        PlatformType::JavascriptCore =>{
            execution_command.push("--");
            execution_command.push("none");
        },
        _ => {}
    };

    let opts = ContainerCreateOpts::builder()
                    .image(image_name)
                    .name(container_name)
                    .build();
            
            let container_creation_result = docker.containers().create(&opts).await;
            let container_id = container_creation_result.unwrap().id().clone();
            let container = docker.containers().get(container_id);


            if let Err(e) = container.copy_file_into(&container_testcase_path, &testcase).await
            {
                eprintln!("Error: {e}")
            }

            let opts = ContainerRestartOpts::builder();

            if let Err(e) = container.restart(&opts.build()).await {
                eprintln!("Error: {e}");
            }

            let options = ExecCreateOpts::builder()
                .command(&execution_command)
                .attach_stdout(true)
                .attach_stderr(true)
                .build();
            let mut stdout = String::with_capacity(32);
            let mut stderr = String::with_capacity(32);
            let opt = Default::default();
            tokio::select! {
                exec_res =  container.exec(&options, &opt)=>{
                    match exec_res{
                        Err(e) => {eprintln!("{e}");},
                        Ok(mut stream) => {
                            loop{
                                tokio::select! {
                                    stream_or_none = stream.next()=> {
                                        if let Some(exec_result) = stream_or_none{
                                            match exec_result {
                                                Ok(chunk) => {
                                                    match chunk {
                                                        TtyChunk::StdOut(bytes) => {
                                                            stdout.push_str(str::from_utf8(&bytes).unwrap_or_default())
                                                        }
                                                        TtyChunk::StdErr(bytes) => {
                                                            stderr.push_str(str::from_utf8(&bytes).unwrap_or_default())
                                                        }
                                                        TtyChunk::StdIn(_) => {break;},
                                                    }
                                                },
                                                Err(_) => {break;},
                                               }
                                        }else{
                                            break;
                                        }
                                        //Request completed within 10 seconds.;
                                    }
                                    _ = sleep(Duration::from_secs(10)) => {
                                        println!("10 seconds elapsed. skiping.");
                                        break;
                                }
                                }
                            }
                        }
                    } 
              
                   
                },
                _ = sleep(Duration::from_secs(10)) => {
                    println!("10 seconds elapsed. skiping.");
                }
            }
            

            let opts = ContainerStopOpts::builder()
                .wait(std::time::Duration::from_secs(0));

            if let Err(e) =  container.stop(&opts.build()).await {
                eprintln!("Error: {e}");
            };

            if let Err(e) = container.remove(&Default::default()).await {
                eprintln!("Error: {e}")
            }
        PrintResult{
            stdout:stdout,
            stderr:stderr
        }
}
