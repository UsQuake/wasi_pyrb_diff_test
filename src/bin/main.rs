use wasi_pyrb_diff_test::test_executor::{execute_test, LanguageType, PlatformType, PrintResult, TestInfo};



fn main(){
    tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(
        async{
            let ruby_test_infos =  [
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
            let python_test_infos =  [
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

            let mut docker = docker_api::Docker::new("unix:///var/run/docker.sock").unwrap();

            let mut results:Vec<PrintResult> = Vec::new(); 

            for test_info in python_test_infos{
              let result = execute_test(&mut docker, test_info).await;
              results.push(result);
            }
            
            println!("{:?}",results[0].stdout);
            println!("{:?}",results[1].stdout);
            println!("{:?}",results[2].stdout);
            println!("{:?}",results[3].stdout);
            assert!(results[0].stdout == results[1].stdout);
            assert!(results[1].stdout == results[2].stdout);
            assert!(results[2].stdout == results[3].stdout);
        }
    );
}

