use wasi_pyrb_diff_test::grammar::{str_helper::replace_scope_with_indent,predef_grammars::get_python_grammar, Union};
use wasi_pyrb_diff_test::grammar_fuzzer::var_ctx::ir_to_ctx;
use wasi_pyrb_diff_test::grammar_fuzzer::GrammarsFuzzer;
use wasi_pyrb_diff_test::testcase_driver::exec::{execute_test, PrintResult, PYTHON_TEST_INFOS};
use wasi_pyrb_diff_test::testcase_driver::*;


#[tokio::main]
async fn main() {
            let test_count = 10000;
            let mut cli_progress_bar = IndicateBar::new(test_count);
            let mut testcase_generator = GrammarsFuzzer::new(
                &get_python_grammar(),
                "<start>",
                10,
                100,
                Union::OnlyA(false),
            );
            let mut docker = docker_api::Docker::new("unix:///var/run/docker.sock").unwrap();
            //let mut rand_seed = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() & ((1<<65) - 1)) as u64;
            let mut rand_seed = 17526186317047798642;
            for i in 0..test_count{   
                let testcase = testcase_generator.fuzz(&mut rand_seed);
                let test_input = replace_scope_with_indent(&ir_to_ctx(&testcase, &mut rand_seed.clone()));
                let mut results:Vec<PrintResult> = Vec::new(); 

                for test_info in PYTHON_TEST_INFOS{
                    let result = execute_test(&mut docker, test_info, test_input.as_bytes()).await;
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
                        std::fs::write("./issues/v8/testcase".to_string() + &i.to_string() + ".py", &test_input).unwrap();

                    }else if is_v8_and_spidermonkey_result_same{

                        println!("⚠Warning!: JavascriptCore result is different with other engines!");
                        std::fs::write("./issues/jsc/log".to_string() + &i.to_string() + ".txt", &results[1].stdout).unwrap();
                        std::fs::write("./issues/jsc/testcase".to_string() + &i.to_string() + ".py", &test_input).unwrap();

                    }else if is_v8_and_jsc_result_same{

                        println!("⚠Warning!: SpiderMonkey result is different with other engines!");
                        std::fs::write("./issues/spm/log".to_string() + &i.to_string() + ".txt", &results[2].stdout).unwrap();
                        std::fs::write("./issues/spm/testcase".to_string() + &i.to_string() + ".py", &test_input).unwrap();

                    }else{

                        println!("⚠Warning!: Each result of js engines different with other engines!");
                        std::fs::write("./issues/unknown/log".to_string() + &i.to_string() + ".txt", 
                        format!("d8(V8):\n{}\n", results[0].stdout)
                        + &format!("jsc(JavascriptCore):\n{}\n", results[1].stdout)
                        + &format!("JsShell(SpiderMonkey):\n{}\n", results[2].stdout)).unwrap();
                        std::fs::write("./issues/unknown/testcase".to_string() + &i.to_string() + ".py", &test_input).unwrap();

                    }

                }else if !is_native_wasm_same{
                    std::fs::write("./issues/native_vs_wasm/log".to_string() + &i.to_string() + ".txt", 
                    format!("d8(V8):\n{}\n", results[0].stdout)
                    + &format!("Native:\n{}\n", results[3].stdout.clone() + &results[3].stderr)).unwrap();
                    std::fs::write("./issues/native_vs_wasm/testcase".to_string() + &i.to_string() + ".py", &test_input).unwrap();
                    //omit_testcase_or_other_name("./issued_testcases/native_vs_wasm", &test_input, &LanguageType::Python);
                }
                cli_progress_bar.progress_one();
            }

}
