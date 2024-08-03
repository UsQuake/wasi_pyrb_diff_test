use docker_api::{models::ContainerSummary, opts::{ContainerListOpts, ContainerStopOpts}};
use exec::{exec_test, LanguageType};
use tokio::signal;
use wasi_pyrb_diff_test::testcase_driver::*;


#[tokio::main]
async fn main() {
    let test_count = 10000;
    let mut docker = docker_api::Docker::new("unix:///var/run/docker.sock").unwrap();
    //let rand_seed = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() & ((1<<65) - 1)) as u64;
    let rand_seed = 17526186317047798642;
    let test_exec = exec_test(&mut docker, test_count, rand_seed, LanguageType::Python);
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

