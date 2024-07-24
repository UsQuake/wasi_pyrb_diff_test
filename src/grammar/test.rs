#[cfg(test)]
mod tests {
    use crate::grammar::*;
    use regex::*;
    use std::collections::*;

    #[test]

    fn test_is_valid_grammar() {
        let mut grammars: Grammar = HashMap::new();
        grammars.insert("<start>".to_string(), vec![Union::OnlyA("<x>".to_string())]);
        grammars.insert("<y>".to_string(), vec![Union::OnlyA("1".to_string())]);
        assert!(!is_valid_grammar(&grammars, "<start>", BTreeSet::new()))
    }
    #[test]
    fn test_parenthesized_expressions() {
        let pred = parenthesized_expressions(&Union::OnlyA(
            "(<foo>)* (<foo><bar>)+ (+<foo>)? <integer>(.<integer>)?".to_string(),
        ));
        let answer = ["(<foo>)*", "(<foo><bar>)+", "(+<foo>)?", "(.<integer>)?"];
        assert_eq!(pred.len(), answer.len());
        for i in 0..pred.len() {
            assert_eq!(pred[i], answer[i]);
        }
    }
    #[test]
    fn test_extended_nonterminals() {
        let pred = extended_nonterminals(&Union::OnlyA("<foo>* <bar>+ <elem>? <none>".to_string()));
        let answer = vec![
            "<foo>*".to_string(),
            "<bar>+".to_string(),
            "<elem>?".to_string(),
        ];
        assert_eq!(pred.len(), answer.len());
        for i in 0..pred.len() {
            assert_eq!(pred[i], answer[i]);
        }
    }
    #[test]
    fn test_new_symbol() {
        let mut grammars: Grammar = HashMap::new();

        grammars.insert(
            "<start>".to_string(),
            vec![Union::OnlyA("<expr>".to_string())],
        );
        grammars.insert(
            "<expr>".to_string(),
            vec![
                Union::OnlyA("<term> + <expr>".to_string()),
                Union::OnlyA("<term> - <expr>".to_string()),
                Union::OnlyA("<term>".to_string()),
            ],
        );
        grammars.insert(
            "<term>".to_string(),
            vec![
                Union::OnlyA("<factor> * <term>".to_string()),
                Union::OnlyA("<factor> / <term>".to_string()),
                Union::OnlyA("<factor>".to_string()),
            ],
        );
        grammars.insert(
            "<factor>".to_string(),
            vec![
                Union::OnlyA("+<factor>".to_string()),
                Union::OnlyA("-<factor>".to_string()),
                Union::OnlyA("(<expr>)".to_string()),
                Union::OnlyA("<integer>.<integer>".to_string()),
                Union::OnlyA("<integer>".to_string()),
            ],
        );
        grammars.insert(
            "<integer>".to_string(),
            vec![
                Union::OnlyA("<digit><integer>".to_string()),
                Union::OnlyA("<digit>".to_string()),
            ],
        );
        grammars.insert(
            "<digit>".to_string(),
            vec![
                Union::OnlyA("0".to_string()),
                Union::OnlyA("1".to_string()),
                Union::OnlyA("2".to_string()),
                Union::OnlyA("3".to_string()),
                Union::OnlyA("4".to_string()),
                Union::OnlyA("5".to_string()),
                Union::OnlyA("6".to_string()),
                Union::OnlyA("7".to_string()),
                Union::OnlyA("8".to_string()),
                Union::OnlyA("9".to_string()),
            ],
        );

        assert_eq!(
            new_symbol(&grammars, &String::from("<expr>")),
            String::from("<expr-1>")
        );
    }
}