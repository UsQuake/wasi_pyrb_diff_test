use lazy_static::lazy_static;
use regex::*;
use std::collections::*;

use crate::grammar::get_rand;

use super::{
    predef_grammars::{get_array_grammar, get_expr_grammar},
    GrammarsFuzzer,
};

type Ident = String;
type Stack<T> = Vec<T>;
type Trait = HashSet<Ident>;
//type Var = (Ident, Trait);
type Vars = HashMap<Ident, Trait>;
type Context = Stack<Vars>;

lazy_static! {
    pub static ref RE_VAR_TYPE_EXPR: Regex = Regex::new(r"@[^;]+;").unwrap();
}
fn generate_unique_var_name(already_define_vars: &Vars, prefix:String) -> String {
   
    let mut count = 0;
    let mut result = prefix.clone()+ &count.to_string();
    while already_define_vars.get(&result).is_some() {
        result = prefix.clone() + &count.to_string();
        count = count + 1;
    }
    result
}

fn get_var_info_from_ir(ir: &String) -> (String, HashSet<String>) {
    let var_info: Vec<String> = ir
        .clone()
        .split_off(0)
        .split(':')
        .map(|m| m.to_string())
        .collect();
    let (var_state, var_traits) = (&var_info[0], &var_info[1]);
    let mut var_traits = var_traits.clone();
    var_traits.pop();
    let var_traits: HashSet<String> = var_traits.split(',').map(|m| m.to_string()).collect();
    (var_state.clone(), var_traits)
}

fn get_splitted_ir_code(ir_code: &String) -> Vec<String> {
    let mut split_ir: Vec<String> = Vec::with_capacity(8);
    let mut last_match_end = 0;
    for mat in RE_VAR_TYPE_EXPR.find_iter(&ir_code) {
        let start_index = mat.start();
        let matched_text = mat.as_str();
        let unmatched_text = &ir_code[last_match_end..start_index];
        if !unmatched_text.is_empty() {
            split_ir.push(unmatched_text.to_string());
        }
        split_ir.push(matched_text.to_string());
        last_match_end = mat.end();
    }
    let unmatched_text_after_last_match = &ir_code[last_match_end..];
    if !unmatched_text_after_last_match.is_empty() {
        split_ir.push(unmatched_text_after_last_match.to_string());
    }
    split_ir
}
pub fn ir_to_ctx(ir_code: &String, seed: &mut u64 /* , predefined_ctx:Context*/) -> String {
    let mut fuzz_expr = GrammarsFuzzer::new(
        &get_expr_grammar(),
        "<start>",
        3,
        5,
        super::Union::OnlyA(false),
    );
    let mut fuzz_arr = GrammarsFuzzer::new(
        &get_array_grammar(),
        "<array>",
        10,
        0,
        super::Union::OnlyA(false),
    );
    let mut res_stack: Context = Vec::new();
    res_stack.push(HashMap::new());
    let splitted_ir_code = get_splitted_ir_code(ir_code);
    let mut result_ir_code = splitted_ir_code.clone();
    let mut current_vars = res_stack.last().unwrap().clone();
    let mut for_iter_var  = None;
    for (idx, str) in splitted_ir_code.iter().enumerate() {
        
        if str.chars().nth(0) == Some('@') {
            let (var_state, var_traits) = get_var_info_from_ir(str);

            let referable_vars: Vec<_> = current_vars
                .iter()
                .filter(|(_, already_defined_var_traits)| {
                    var_traits.is_subset(already_defined_var_traits)
                })
                .map(|(already_defined_var_name, _)| already_defined_var_name)
                .collect();

            match var_state.as_str() {
                "@Refer" => {
                    if referable_vars.len() != 0 {
                        let rand_num = get_rand(seed) % referable_vars.len();
                        result_ir_code[idx] = referable_vars[rand_num].clone();
                    } else {
                        println!("NULL!");
                    }
                }
                "@Define" => {
                    
                    if var_traits.contains("ForIter") || var_traits.contains("ForRange"){
                        let new_var_name = generate_unique_var_name(&current_vars, "it".to_string());
                        for_iter_var = Some(new_var_name.clone());
                        result_ir_code[idx] = new_var_name.clone();
                    }else if var_traits.contains("Iterable") {
                        let new_var_name = generate_unique_var_name(&current_vars, "vec".to_string());
                        let mut generated_statement = new_var_name.clone() + " = ";
                        current_vars.insert(new_var_name, HashSet::from(["Iterable".to_string()]));
                        generated_statement = generated_statement + &fuzz_arr.fuzz(seed);
                        result_ir_code[idx] = generated_statement;
                    } else if var_traits.contains("Primitive"){
                        let new_var_name = generate_unique_var_name(&current_vars, "var".to_string());
                        let mut generated_statement = new_var_name.clone() + " = ";
                        current_vars.insert(new_var_name, HashSet::from(["Primitive".to_string()]));
                        generated_statement = generated_statement + &fuzz_expr.fuzz(seed);
                        result_ir_code[idx] = generated_statement;
                    }
                }
                "@Assign" => {
                    if referable_vars.len() != 0 {
                        let rand_num = get_rand(seed) % referable_vars.len();
                        let mut generated_statement = referable_vars[rand_num].clone() + " = ";
                        if var_traits.contains("Primitive") {
                            generated_statement = generated_statement + &fuzz_expr.fuzz(seed);
                        } else if var_traits.contains("Iterable") {
                            generated_statement = generated_statement + &fuzz_arr.fuzz(seed);
                        }
                        result_ir_code[idx] = generated_statement;
                    } else {
                        println!("NULL!");
                    }
                }
                _ => {}
            }
        } else {
            for ch in str.chars() {
                if ch == '{' {
                    res_stack.push(current_vars.clone());
                    if let Some(new_var_name) = for_iter_var{
                        current_vars.insert(new_var_name, HashSet::from(["Primitive".to_string()]));
                        for_iter_var= None;
                    }
                } else if ch == '}' {
                    current_vars = res_stack.pop().unwrap();
                }
            }
        }
    }
    result_ir_code.join("")
}