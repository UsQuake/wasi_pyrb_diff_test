use wasi_pyrb_diff_test::grammar::{str_helper::replace_scope_with_indent,predef_grammars::get_python_grammar, Union};
use wasi_pyrb_diff_test::grammar_fuzzer::var_ctx::ir_to_ctx;
use wasi_pyrb_diff_test::grammar_fuzzer::GrammarsFuzzer;
use wasi_pyrb_diff_test::test_executor::{execute_test, PrintResult, PYTHON_TEST_INFOS};



#[tokio::main]
async fn main() {
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

            
            let testcase = f.fuzz(&mut rand_seed);
            std::fs::write(&testcase_path,  replace_scope_with_indent(&ir_to_ctx(&testcase, &mut rand_seed.clone()))).unwrap();
            let mut results:Vec<PrintResult> = Vec::new(); 

            for test_info in PYTHON_TEST_INFOS{
              let result = execute_test(&mut docker, test_info, &testcase_path).await;
              results.push(result);
            }
            
            assert!(results[0].stdout == results[1].stdout);
            assert!(results[1].stdout == results[2].stdout);
            assert!(results[2].stdout == results[3].stdout.clone() + &results[3].stderr);

}
