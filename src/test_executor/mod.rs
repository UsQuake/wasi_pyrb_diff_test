use docker_api::opts::{ContainerCreateOpts,ContainerRestartOpts, ContainerStopOpts, ExecCreateOpts};
use docker_api::{conn::TtyChunk, Docker};
use futures::StreamExt;
use std::{str, fs::File, io::Read};
use std::time::Instant;
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

pub async fn execute_test<'a>(docker:&mut Docker, test_target:TestInfo)-> PrintResult{
    let container_name = "testcase_execution_container".to_string();
    let mut image_name = String::with_capacity(6);
    //["./jsc", "module-ready-wasi-py.js", "--", "none"]
    let mut execution_command: Vec<& 'a str> = Vec::with_capacity(3);
    let mut host_testcase_path: String = "/opt/experiment3/testcase_volume/testcase.".to_string();
    let mut container_testcase_path: String = "/root/".to_string();

  match test_target.target_platform{
        PlatformType::V8 => {
            image_name += "d8_";
            execution_command.push("./d8");
            println!("V8");
        },
        PlatformType::SpiderMonkey => {
            image_name += "js_";
            execution_command.push("./js");
            println!("Spidermonkey");
        },
        PlatformType::JavascriptCore => {
            image_name += "jsc_";
            execution_command.push("./jsc");
            println!("JavascriptCore");
        },
        PlatformType::Native => {
            image_name += "na_";
            println!("Native");
        }
    };

    match test_target.target_language{
        LanguageType::Python => {
            image_name += "py";
            host_testcase_path += "py";

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
            host_testcase_path += "rb";
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
           


            let mut file = File::open(&host_testcase_path)
            .unwrap();
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)
                .expect("Cannot read file on the localhost.");


            if let Err(e) = container.copy_file_into(&container_testcase_path, &bytes).await
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

            let mut stream = container
               .exec(&options, &Default::default())
               .await
               .expect("exec stream");
            
            let mut stdout = String::with_capacity(32);
            let mut stderr = String::with_capacity(32);
            
            let now = Instant::now();
            while let Some(exec_result) = stream.next().await {
              match exec_result {
                Ok(chunk) => {
                    match chunk {
                        TtyChunk::StdOut(bytes) => {
                            stdout.push_str(str::from_utf8(&bytes).unwrap_or_default())
                        }
                        TtyChunk::StdErr(bytes) => {
                            stderr.push_str(str::from_utf8(&bytes).unwrap_or_default())
                        }
                        TtyChunk::StdIn(_) => unreachable!(),
                    }
                

                },
                Err(e) => eprintln!("Error: {e}"),
               }
            }
            println!("exec 소요시간: {}", now.elapsed().as_secs());

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
