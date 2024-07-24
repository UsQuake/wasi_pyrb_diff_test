use crate::grammar::{Expansion, Grammar, Union};
use std::any::Any;
use std::collections::HashMap;

pub type Option<'l_use> = HashMap<&'l_use str, &'l_use dyn Any>;

pub fn opts<'l_use>(a: &[(&'l_use str, &'l_use dyn Any)]) -> Option<'l_use> {
    let mut result = HashMap::new();
    for it in a {
        result.insert(it.0, it.1);
    }
    result
}

pub fn exp_string<'l_use>(expansion: &Expansion<'l_use>) -> String {
    match expansion {
        Union::OnlyA(str_only) => str_only.clone(),
        Union::OnlyB(str_and_map) => str_and_map.0.clone(),
    }
}

pub fn exp_opts<'l_use>(expansion: &Expansion<'l_use>) -> Option<'l_use> {
    match expansion {
        Union::OnlyA(_) => HashMap::new(),
        Union::OnlyB(str_and_map) => str_and_map.1.clone(),
    }
}

pub fn exp_opt<'l_use>(
    expansion: &Expansion<'l_use>,
    attribute: &'l_use str,
) -> std::option::Option<&'l_use dyn Any> {
    match exp_opts(&expansion).get(attribute) {
        Some(ref_ref) => Some(ref_ref.clone()),
        None => None,
    }
}

fn set_opts<'l_use>(
    grammar: &mut Grammar<'l_use>,
    symbol: &'l_use str,
    expansion: &Expansion,
    opts: &Option<'l_use>,
) -> Result<(), &'static str> {
    let expansions = &grammar[symbol];

    for i in 0..expansions.len() {
        let exp = &expansions[i];
        if exp_string(exp) != exp_string(expansion) {
            continue;
        }

        let mut new_opts = exp_opts(exp);
        if opts.is_empty() || new_opts.is_empty() {
            new_opts = opts.clone();
        } else {
            for key_opt in opts {
                new_opts.insert(key_opt.0, opts[key_opt.0]);
            }
        }

        if new_opts.is_empty() {
            grammar.get_mut(symbol).unwrap()[i] = Union::OnlyA(exp_string(&exp.clone()));
        } else {
            grammar.get_mut(symbol).unwrap()[i] = Union::OnlyB((exp_string(exp), new_opts));
        }

        return Ok(());
    }
    return Err("panic!");
}