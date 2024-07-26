use wasi_pyrb_diff_test::grammar::{str_helper::replace_scope_with_indent,predef_grammars::get_python_grammar, Union};
use wasi_pyrb_diff_test::grammar_fuzzer::var_ctx::ir_to_ctx;
use wasi_pyrb_diff_test::grammar_fuzzer::GrammarsFuzzer;
use wasi_pyrb_diff_test::testcase_driver::exec::{execute_test, LanguageType, PrintResult, PYTHON_TEST_INFOS};
use wasi_pyrb_diff_test::testcase_driver::omit_testcase_or_other_name;


#[tokio::main]
async fn main() {
            let test_count = 83;
            let mut f = GrammarsFuzzer::new(
                &get_python_grammar(),
                "<start>",
                10,
                100,
                Union::OnlyA(false),
            );
            let mut docker = docker_api::Docker::new("unix:///var/run/docker.sock").unwrap();
            //let mut rand_seed = (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() & ((1<<65) - 1)) as u64;
            let mut rand_seed = 17526186317047798642;
            let testcase_path = "./testcase.py";

            for _ in 0..test_count{   
                let testcase = f.fuzz(&mut rand_seed);
                let test_input = replace_scope_with_indent(&ir_to_ctx(&testcase, &mut rand_seed.clone()));
                std::fs::write(&testcase_path, &test_input).unwrap();
                let mut results:Vec<PrintResult> = Vec::new(); 

                for test_info in PYTHON_TEST_INFOS{
                    let result = execute_test(&mut docker, test_info, &testcase_path).await;
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
                    if is_v8_and_spidermonkey_result_same{
                        println!("⚠Warning!: JavascriptCore crashed!");
                    }else{
                        println!("⚠Warning!: results between js engines are different.");
                    }
                    omit_testcase_or_other_name("./issued_testcases/jsc", &test_input, &LanguageType::Python);
                }else if !is_native_wasm_same{
                    println!("⚠Warning!: results between native and wasm is different.");
                    omit_testcase_or_other_name("./issued_testcases/native_vs_wasm", &test_input, &LanguageType::Python);
                }
            }

}
