use docker_api::opts::{ContainerCreateOpts,ContainerRestartOpts, ContainerStopOpts, ExecCreateOpts};
use docker_api::{conn::TtyChunk, Docker};
use futures::StreamExt;
use tokio::time::{sleep, Duration};
use std::str;

pub static PYTHON_TEST_INFOS:& 'static [TestInfo;4] =  &[
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
];

pub static RUBY_TEST_INFOS:& 'static [TestInfo;4] =  &[
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
]; 
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
