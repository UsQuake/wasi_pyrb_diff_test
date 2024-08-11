use std::{env::args, time::{SystemTime, UNIX_EPOCH}};

use docker_api::{models::ContainerSummary, opts::{ContainerListOpts, ContainerStopOpts}};
use exec::{exec_test, LanguageType};
use tokio::signal;
use wasi_pyrb_diff_test::testcase_driver::*;


#[tokio::main]
async fn main() {
    let arguments:Vec<_> = args().collect();

    if arguments.len() != 3{
        eprintln!("Please use command ./command_exec test_execution_count target_interpreter_type(py or rb)!");
        return;
    }
    let test_count = 
    match arguments[1].parse::<usize>(){
        Ok(s) =>{
            s
        },
        Err(e) => {
            eprintln!("Error: {e}\nPlease use command ./command_exec test_execution_count target_interpreter_type(py or rb)!");
            return;
        }
    };

    let test_target_interpreter_type = 
    if arguments[2] == "py"{
            LanguageType::Python
    }else if arguments[2] == "rb"{
            LanguageType::Ruby
    }else{
            eprintln!("Please choose valid target interpreter type[py or rb]!");
            return;
    };

    let mut docker = docker_api::Docker::new("unix:///var/run/docker.sock").unwrap();
    let rand_seed = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() & ((1<<65) - 1)) as u64;
    tokio::fs::write("./issues/latest_seed.txt", rand_seed.to_string()).await.unwrap();

    let test_exec = exec_test(&mut docker, test_count, rand_seed, test_target_interpreter_type);
    tokio::select! {
        _ = test_exec=>{},

        _ = signal::ctrl_c()  => {
            let opts = ContainerListOpts::builder()
                .all(true)
                .build();

            match docker.containers().list(&opts).await {

                Ok(containers)=>{

                let cont_names:Vec<ContainerSummary> = containers.into_iter()
                .filter(|container| {container.clone().names.map(|n| n[0].to_owned()).unwrap_or_default() == "/testcase_execution_container"})
                .collect();

                if cont_names.len() == 1{
                    let container = docker.containers().get(cont_names[0].id.clone().unwrap());
                    let opts = ContainerStopOpts::builder()
                            .wait(std::time::Duration::from_secs(0))
                            .build();
                    if let Err(e) =  container.stop(&opts).await {
                        eprintln!("Error: {e}");
                    }
        
                    if let Err(e) = container.remove(&Default::default()).await {
                        eprintln!("Error: {e}");
                    }
                
                }

                },

                Err(e) =>{eprintln!("{e}");}
        
            }

        }    
    }
}

