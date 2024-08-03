pub mod options;
pub mod predef_grammars;
pub mod str_helper;
mod test;

use self::options::{exp_opts, Option};

use lazy_static::lazy_static;
use regex::Regex;
use rustc_hash::FxHasher;
use std::{
    collections::{BTreeSet, HashMap},
    hash::{Hash, Hasher},
    time::{SystemTime, UNIX_EPOCH},
};

// #[derive(Clone)]
// pub enum Any{
//     Usize(usize), Str(String), Tuple((Vec<String>, String))
// }

#[derive(Clone)]
pub enum Union<A, B> {
    OnlyA(A),
    OnlyB(B),
}

pub type Expansion<'l_use> = Union<String, (String, Option<'l_use>)>;
pub type Grammar<'l_use> = HashMap<String, Vec<Expansion<'l_use>>>;

lazy_static! {
    pub static ref RE_PARENTHESIZED_EXPR: Regex = Regex::new(r"\([^()]*\)[?+*]").unwrap();
    pub static ref RE_NONTERMINAL: Regex = Regex::new(r"(<[^<> ]*>)").unwrap();
    pub static ref RE_EXTENDED_NONTERMINAL: Regex = Regex::new(r"(<[^<> ]*>[?+*])").unwrap();
}

lazy_static! {}
static START_SYMBOL: &'static str = "<start>";

pub fn is_nonterminal<'l_use>(symbol: &'l_use str) -> bool {
    return RE_NONTERMINAL.is_match(symbol);
}
pub fn nonterminals<'l_use>(expansion: &Expansion<'l_use>) -> Vec<String> {
    let expansion = match expansion {
        Union::OnlyA(only_str) => only_str.to_string(),
        Union::OnlyB(str_and_opt) => str_and_opt.0.to_string(),
    };

    let ret = RE_NONTERMINAL
        .find_iter(&expansion)
        .map(|m| m.as_str().to_string())
        .collect();

    return ret;
}

pub fn extended_nonterminals<'l_use>(expansion: &Expansion<'l_use>) -> Vec<String> {
    let expansion = match expansion {
        Union::OnlyA(only_str) => only_str.to_string(),
        Union::OnlyB(str_and_opt) => str_and_opt.0.to_string(),
    };

    let ret = RE_EXTENDED_NONTERMINAL
        .find_iter(&expansion)
        .map(|m| m.as_str().to_string())
        .collect();
    return ret;
}
pub fn extend_grammar<'l_use>(
    grammar: &Grammar<'l_use>,
    extension: &Grammar<'l_use>,
) -> Grammar<'l_use> {
    let mut result = grammar.clone();
    for extension_tuple in extension {
        match result.get_mut(extension_tuple.0) {
            Some(ref_result) => {
                *ref_result = extension_tuple.1.clone();
            }
            None => {
                result.insert(extension_tuple.0.to_string(), extension_tuple.1.clone());
            }
        }
    }
    result
}
fn parenthesized_expressions<'l_use>(expansion: &Expansion<'l_use>) -> Vec<String> {
    let expansion = match expansion {
        Union::OnlyA(s) => s.clone(),
        Union::OnlyB(s_and_map) => s_and_map.0.clone(),
    };
    let ret = RE_PARENTHESIZED_EXPR
        .find_iter(&expansion)
        .map(|m| m.as_str().to_string())
        .collect();
    return ret;
}

pub fn convert_ebnf_parentheses<'l_use>(ebnf_grammar: &Grammar<'l_use>) -> Grammar<'l_use> {
    let mut grammar = extend_grammar(&ebnf_grammar, &HashMap::new());
    for nonterminal_and_vector in ebnf_grammar {
        let expansions = nonterminal_and_vector.1;

        for i in 0..expansions.len() {
            let expansion = &expansions[i];
            let mut expansion: Expansion<'l_use> = match expansion {
                Union::OnlyA(st) => Union::OnlyA(st.clone()),
                Union::OnlyB(st_and_vec) => Union::OnlyA(st_and_vec.0.clone()),
            };

            loop {
                let parenthesized_exprs = parenthesized_expressions(&expansion);
                if parenthesized_exprs.len() == 0 {
                    break;
                }

                for expr in parenthesized_exprs {
                    let operator = &expr[expr.len() - 1..expr.len()];
                    let contents = &expr[1..expr.len() - 2];
                    let new_sym = new_symbol(&grammar, "<symbol>");

                    let exp = &grammar.get_mut(nonterminal_and_vector.0).unwrap()[i];
                    match exp {
                        Union::OnlyA(only_exp) => {
                            let exp_copy = only_exp.clone();

                            let sub_exp =
                                exp_copy.replacen(&expr, &(new_sym.clone() + operator), 1);
                            expansion = Union::OnlyA(sub_exp);
                            grammar.get_mut(nonterminal_and_vector.0).unwrap()[i] =
                                expansion.clone();
                        }
                        Union::OnlyB(tuple) => {
                            let (exp, opts) = tuple;

                            let exp_copy = exp.clone();

                            let sub_exp =
                                exp_copy.replacen(&expr, &(new_sym.clone() + operator), 1);
                            expansion = Union::OnlyA(sub_exp.clone());
                            if opts.is_empty() {
                                grammar.get_mut(nonterminal_and_vector.0).unwrap()[i] =
                                    Union::OnlyA(sub_exp.clone());
                            } else {
                                grammar.get_mut(nonterminal_and_vector.0).unwrap()[i] =
                                    Union::OnlyB((sub_exp.clone(), opts.clone()));
                            }
                        }
                    }

                    grammar.insert(new_sym.clone(), vec![Union::OnlyA(contents.to_string())]);
                }
            }
        }
    }
    return grammar;
}

pub fn convert_ebnf_operators<'l_use>(ebnf_grammar: &Grammar<'l_use>) -> Grammar<'l_use> {
    let mut grammar = extend_grammar(&ebnf_grammar, &HashMap::new());
    for nonterminal_and_exps in ebnf_grammar {
        let expansions = nonterminal_and_exps.1;

        for i in 0..expansions.len() {
            let mut expansion = expansions[i].clone();
            let extended_symbols = extended_nonterminals(&expansion);

            for extended_symbol in extended_symbols {
                let operator = &extended_symbol[extended_symbol.len() - 1..extended_symbol.len()];
                let original_symbol = &extended_symbol[0..extended_symbol.len() - 1];

                let new_sym = new_symbol(&grammar, original_symbol);

                let exp = &grammar.get_mut(nonterminal_and_exps.0).unwrap()[i];
                match exp {
                    Union::OnlyA(only_exp) => {
                        let exp_copy = only_exp.clone();

                        let sub_exp = exp_copy.replacen(&extended_symbol, &new_sym, 1);
                        expansion = Union::OnlyA(sub_exp.clone());
                        grammar.get_mut(nonterminal_and_exps.0).unwrap()[i] = expansion.clone();
                    }
                    Union::OnlyB(tuple) => {
                        let (exp, opts) = tuple;

                        let exp_copy = exp.clone();

                        let sub_exp = exp_copy.replacen(&extended_symbol, &new_sym, 1);
                        expansion = Union::OnlyA(sub_exp.clone());
                        if opts.is_empty() {
                            grammar.get_mut(nonterminal_and_exps.0).unwrap()[i] =
                                Union::OnlyA(sub_exp.clone());
                        } else {
                            grammar.get_mut(nonterminal_and_exps.0).unwrap()[i] =
                                Union::OnlyB((sub_exp.clone(), opts.clone()));
                        }
                    }
                }

                if operator == "?" {
                    grammar.insert(
                        new_sym,
                        vec![
                            Union::OnlyA("".to_string()),
                            Union::OnlyA(original_symbol.to_string()),
                        ],
                    );
                } else if operator == "*" {
                    grammar.insert(
                        new_sym.clone(),
                        vec![
                            Union::OnlyA("".to_string()),
                            Union::OnlyA(original_symbol.to_string() + &new_sym),
                        ],
                    );
                } else if operator == "+" {
                    grammar.insert(
                        new_sym.clone(),
                        vec![
                            Union::OnlyA(original_symbol.to_string()),
                            Union::OnlyA(original_symbol.to_string() + &new_sym),
                        ],
                    );
                }
            }
        }
    }

    return grammar;
}

pub fn convert_ebnf_grammar<'l_use>(ebnf_grammar: &Grammar<'l_use>) -> Grammar<'l_use> {
    return convert_ebnf_operators(&convert_ebnf_parentheses(ebnf_grammar));
}

pub fn new_symbol<'l_use>(grammar: &Grammar, symbol_name: &'l_use str) -> String {
    match grammar.get(symbol_name) {
        Some(_) => {
            let mut count = 1;
            loop {
                let mut symbol_copy = String::from(symbol_name);
                symbol_copy.pop();
                let tentative_symbol_name = format!("{}-{}>", symbol_copy, count);
                if !grammar.contains_key(tentative_symbol_name.as_str()) {
                    return tentative_symbol_name;
                }
                count += 1
            }
        }
        None => {
            return String::from(symbol_name);
        }
    }
}

fn def_used_nonterminals<'l_use>(
    grammar: &Grammar,
    start_symbol: &'l_use str,
) -> (
    std::option::Option<BTreeSet<String>>,
    std::option::Option<BTreeSet<String>>,
) {
    let mut defined_nonterminals = BTreeSet::new();
    let mut used_nonterminals = BTreeSet::new();
    used_nonterminals.insert(start_symbol.to_string());

    for defined_nonterminal in grammar {
        defined_nonterminals.insert(defined_nonterminal.0.clone());
        let expansions = grammar.get(defined_nonterminal.0);

        match expansions {
            None => {
                print!("{} : expansion is not a list", defined_nonterminal.0);
                return (None, None);
            }

            Some(expansions) => {
                if expansions.is_empty() {
                    print!("{} : expansion is empty", defined_nonterminal.0);
                    return (None, None);
                }

                for expansion in expansions {
                    let expansion = match expansion {
                        Union::OnlyA(st) => st.clone(),
                        Union::OnlyB(str_and_map) => str_and_map.0.clone(),
                    };

                    for used_nonterminal in nonterminals(&Union::OnlyA(expansion)) {
                        used_nonterminals.insert(used_nonterminal);
                    }
                }
            }
        }
    }

    return (Some(defined_nonterminals), Some(used_nonterminals));
}
fn reachable_nonterminals<'l_use>(
    grammar: &Grammar,
    start_symbol: &'l_use str,
) -> BTreeSet<String> {
    let mut reachable = BTreeSet::new();
    _find_reachable_nonterminals(&mut reachable, grammar, start_symbol.to_string());
    return reachable;
}

fn _find_reachable_nonterminals<'l_use>(
    reachable: &mut BTreeSet<String>,
    grammar: &Grammar<'l_use>,
    symbol: String,
) {
    reachable.insert(symbol.to_string());
    let exps = match grammar.get(&symbol) {
        None => Vec::new(),
        Some(expansions) => expansions.clone(),
    };
    for expansion in exps {
        for nonterminal in nonterminals(&expansion) {
            if !reachable.contains(&nonterminal) {
                _find_reachable_nonterminals(reachable, grammar, nonterminal);
            }
        }
    }
}

fn unreachable_nonterminals<'l_use>(
    grammar: &Grammar<'l_use>,
    start_symbol: &'l_use str,
) -> BTreeSet<String> {
    let grammar_keys_set: BTreeSet<String> =
        grammar.keys().map(|m| m.to_string()).into_iter().collect();
    let reachable_nonterms_set = reachable_nonterminals(grammar, start_symbol);
    return &grammar_keys_set - &reachable_nonterms_set;
}

fn opts_used<'l_use>(grammar: &Grammar<'l_use>) -> BTreeSet<String> {
    let mut used_opts = BTreeSet::new();
    for symbol in grammar.keys() {
        for expansion in grammar.get(symbol).unwrap() {
            used_opts.append(
                &mut exp_opts(expansion)
                    .keys()
                    .map(|m| m.to_string())
                    .into_iter()
                    .collect(),
            );
        }
    }
    return used_opts;
}
pub fn is_valid_grammar<'l_use>(
    grammar: &Grammar,
    start_symbol: &'l_use str,
    supported_opts: BTreeSet<String>,
) -> bool {
    let (defined_nonterminals, used_nonterminals) = def_used_nonterminals(grammar, start_symbol);
    if defined_nonterminals.is_none() || used_nonterminals.is_none() {
        return false;
    }
    let defined_nonterminals = defined_nonterminals.unwrap();
    let mut used_nonterminals = used_nonterminals.unwrap();

    if grammar.contains_key(START_SYMBOL) {
        used_nonterminals.insert(START_SYMBOL.to_string());
    }

    for unused_nonterminal in &defined_nonterminals - &used_nonterminals {
        println!("{unused_nonterminal}: defined, but not used. Consider applying trim_grammar() on the grammar");
    }

    for undefined_nonterminal in &used_nonterminals - &defined_nonterminals {
        println!("{undefined_nonterminal}: used, but not defined")
    }

    let mut unreachable = unreachable_nonterminals(grammar, start_symbol);
    let mut msg_start_symbol = start_symbol.to_string();

    if grammar.contains_key(START_SYMBOL) {
        unreachable = &unreachable - &reachable_nonterminals(grammar, START_SYMBOL);
    }

    if start_symbol != START_SYMBOL {
        msg_start_symbol = msg_start_symbol + " or " + START_SYMBOL;
    }

    for unreachable_nonterminal in &unreachable {
        println!("{unreachable_nonterminal}: unreachable from {msg_start_symbol}. Consider applying trim_grammar() on the grammar");
    }

    let mut used_but_not_supported_opts = BTreeSet::new();
    if supported_opts.len() > 0 {
        used_but_not_supported_opts = opts_used(&grammar)
            .difference(&supported_opts)
            .cloned()
            .collect();
    }

    for opt in used_but_not_supported_opts {
        println!("warning: option {opt} is not supported");
    }

    return used_nonterminals == defined_nonterminals && unreachable.len() == 0;
}

fn trim_grammar<'l_use>(grammar: &Grammar<'l_use>, start_symbol: &'l_use str) -> Grammar<'l_use> {
    let mut new_grammar = extend_grammar(&grammar, &HashMap::new());
    let (defined_nonterminals, used_nonterminals) = def_used_nonterminals(grammar, start_symbol);

    if defined_nonterminals.is_none() || used_nonterminals.is_none() {
        return new_grammar;
    }
    let defined_nonterminals = defined_nonterminals.unwrap();
    let used_nonterminals = used_nonterminals.unwrap();

    let unused = &defined_nonterminals - &used_nonterminals;
    let unreachable = unreachable_nonterminals(grammar, start_symbol);
    for nonterminal in unused.intersection(&unreachable) {
        new_grammar.remove(nonterminal);
    }

    return new_grammar;
}
pub fn get_rand(seed: &mut u64) -> usize {
    let mut hasher = FxHasher::default();
    seed.hash(&mut hasher);
    *seed = hasher.finish();
    return (*seed >> 32) as usize;
}
pub fn simple_grammar_fuzzer<'l_use>(
    syntax: &Grammar,
    start_symbol: &'l_use str,
    max_nonterminals: usize,
    max_expansion_trials: usize,
    log: bool,
) -> Result<String, &'static str> {
    let mut term = String::from(start_symbol);
    let mut expansion_trials = 0;
    let mut timestamp = (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        & ((1 << 33) - 1)) as u64;

    while nonterminals(&Union::OnlyA(term.clone())).len() > 0 {
        let sub_nonterminals = term.clone();

        let none_terminals = nonterminals(&Union::OnlyA(sub_nonterminals.clone()));

        let rand_var = get_rand(&mut timestamp) % none_terminals.len();
        let symbol_to_expand = &none_terminals[rand_var];

        let expansions = &syntax[symbol_to_expand];

        let rand_var2 = get_rand(&mut timestamp) % expansions.len();
        let expansion = &expansions[rand_var2];

        let expansion = match expansion {
            Union::OnlyA(a) => a.clone(),
            Union::OnlyB(b) => b.0.clone(),
        };

        let new_term = (&term).replacen(symbol_to_expand.as_str(), &expansion, 1);

        if nonterminals(&Union::OnlyA(new_term.clone())).len() < max_nonterminals {
            term = new_term;
            if log {
                println!(
                    "{:<40} {}",
                    format!("{symbol_to_expand} -> {expansion}"),
                    &term
                )
            }
            expansion_trials = 0;
        } else {
            expansion_trials += 1;
            if expansion_trials >= max_expansion_trials {
                return Err("Cannot expand more!");
            }
        }
    }

    return Ok(term);
}