use crate::grammar::str_helper::*;
use crate::grammar::*;

pub fn get_ruby_grammar() -> Grammar<'static> {
    let mut ruby_grammar: Grammar = HashMap::new();
    ruby_grammar.insert(
        "<start>".to_string(),
        vec![Union::OnlyA("@Define:Primitive;\n@Define:Iterable;\n<statements>".to_string())],
    );
    ruby_grammar.insert(
        "<statements>".to_string(),
        vec![
            Union::OnlyA("<statement>".to_string()),
            Union::OnlyA("<statements>\n<statements>".to_string()),
        ],
    );
    ruby_grammar.insert(
        "<statement>".to_string(),
        vec![
            Union::OnlyA("<for-statement>".to_string()),
            Union::OnlyA("<condition-statement>".to_string()),
            Union::OnlyA("<assign-statement>".to_string()),
            Union::OnlyA("<call-statement>".to_string()),
        ],
    );
    ruby_grammar.insert(
        "<condition-statement>".to_string(),
        vec![Union::OnlyA("<if-statement>".to_string()),
        Union::OnlyA("<unless-statement>".to_string())]
    );
    ruby_grammar.insert(
        "<if-statement>".to_string(),
        vec![Union::OnlyA("if <bool-expr>{\n<statements>}\nend".to_string()),
        Union::OnlyA("if <bool-expr>{\n<statements>}\nels<if-statement>".to_string()),
        Union::OnlyA("if <bool-expr>{\n<statements>}\nelse{\n<statements>}\nend".to_string()),
        ]
    );
    ruby_grammar.insert(
        "<unless-statement>".to_string(),
        vec![
        Union::OnlyA("unless <bool-expr>{\n<statements>}\nend".to_string()),
        Union::OnlyA("unless <bool-expr>{\n<statements>}\nelse{\n<statements>}\nend".to_string())
        ]
    );
    ruby_grammar.insert(
        "<bool-expr>".to_string(),
        vec![
            Union::OnlyA("<term> != <term>".to_string()),
            Union::OnlyA("<term> == <term>".to_string()),
            Union::OnlyA("<term> < <term>".to_string()),
            Union::OnlyA("<term> > <term>".to_string()),
            Union::OnlyA("<term> <= <term>".to_string()),
            Union::OnlyA("<term> >= <term>".to_string()),
        ],
    );
    ruby_grammar.insert(
        "<for-statement>".to_string(),
        vec![
            Union::OnlyA("for @Define:ForRange; in 1..<non-zero-digit>{\n<statements>}\nend".to_string()),
            Union::OnlyA("for @Define:ForIter; in @Refer:Iterable;{\n<statements>}\nend".to_string()),
        ],
    );
    ruby_grammar.insert(
        "<assign-statement>".to_string(),
        vec![
            Union::OnlyA("@Assign:Iterable;".to_string()),
            Union::OnlyA("@Assign:Primitive;".to_string()),
            Union::OnlyA("@Define:Iterable;".to_string()),
            Union::OnlyA("@Define:Primitive;".to_string()),
        ],
    );
    ruby_grammar.insert(
        "<call-statement>".to_string(),
        vec![Union::OnlyA("<func-expr>".to_string())],
    );
    ruby_grammar.insert(
        "<func-expr>".to_string(),
        vec![Union::OnlyA("<func-ident> <func-vars>".to_string())],
    );
    ruby_grammar.insert(
        "<func-ident>".to_string(),
        vec![Union::OnlyA("puts".to_string())],
    );
    ruby_grammar.insert(
        "<func-vars>".to_string(),
        vec![
            Union::OnlyA("@Refer:Iterable;".to_string()),
            Union::OnlyA("@Refer:Primitive;".to_string()),
            Union::OnlyA("<func-vars>".to_string()),
        ],
    );
    ruby_grammar.insert(
        "<expr>".to_string(),
        vec![
            Union::OnlyA("<term> + <expr>".to_string()),
            Union::OnlyA("<term> - <expr>".to_string()),
            Union::OnlyA("<term>".to_string()),
        ],
    );
    ruby_grammar.insert(
        "<term>".to_string(),
        vec![
            Union::OnlyA("<factor> * <term>".to_string()),
            Union::OnlyA("<factor> / <term>".to_string()),
            Union::OnlyA("<factor>".to_string()),
        ],
    );
    ruby_grammar.insert(
        "<factor>".to_string(),
        vec![
            Union::OnlyA("+<integer>".to_string()),
            Union::OnlyA("-<integer>".to_string()),
            Union::OnlyA("+<integer>.<integer>".to_string()),
            Union::OnlyA("-<integer>.<integer>".to_string()),
            Union::OnlyA("(<expr>)".to_string()),
            Union::OnlyA("<integer>.<integer>".to_string()),
            Union::OnlyA("<integer>".to_string()),
            Union::OnlyA("@Refer:Primitive;".to_string()),
        ],
    );
    ruby_grammar.insert(
        "<integer>".to_string(),
        vec![
            Union::OnlyA("<non-zero-digit><integer>".to_string()),
            Union::OnlyA("<non-zero-digit>".to_string()),
        ],
    );
    ruby_grammar.insert("<non-zero-digit>".to_string(), range_chars_as_str(CharRange::NonZeroDigit));
    ruby_grammar.insert(
        "<letter>".to_string(),
        range_chars_as_str(CharRange::Letters),
    );
    return ruby_grammar;
}
pub fn get_python_grammar() -> Grammar<'static> {
    let mut python_grammar: Grammar = HashMap::new();
    python_grammar.insert(
        "<start>".to_string(),
        vec![Union::OnlyA("@Define:Primitive;\n@Define:Iterable;\n<statements>".to_string())],
    );
    python_grammar.insert(
        "<statements>".to_string(),
        vec![
            Union::OnlyA("<statement>".to_string()),
            Union::OnlyA("<statements>\n<statements>".to_string()),
        ],
    );
    python_grammar.insert(
        "<statement>".to_string(),
        vec![
            Union::OnlyA("<for-statement>".to_string()),
            Union::OnlyA("<if-statement>".to_string()),
            Union::OnlyA("<assign-statement>".to_string()),
            Union::OnlyA("<call-statement>".to_string()),
        ],
    );
    python_grammar.insert(
        "<if-statement>".to_string(),
        vec![Union::OnlyA("if <bool-expr>:{\n<statements>}\nel<if-statement>".to_string()),
        Union::OnlyA("if <bool-expr>:{\n<statements>}".to_string()),
        Union::OnlyA("if <bool-expr>:{\n<statements>}\nelse:{\n<statements>}".to_string())
        ],
    );
    python_grammar.insert(
        "<bool-expr>".to_string(),
        vec![
            Union::OnlyA("<term> != <term>".to_string()),
            Union::OnlyA("<term> == <term>".to_string()),
            Union::OnlyA("<term> < <term>".to_string()),
            Union::OnlyA("<term> > <term>".to_string()),
            Union::OnlyA("<term> <= <term>".to_string()),
            Union::OnlyA("<term> >= <term>".to_string()),
        ],
    );
    python_grammar.insert(
        "<for-statement>".to_string(),
        vec![
            Union::OnlyA("for @Define:ForRange; in range(1,<non-zero-digit>):{\n<statements>}".to_string()),
            Union::OnlyA("for @Define:ForIter; in @Refer:Iterable;:{\n<statements>}".to_string()),
        ],
    );
    python_grammar.insert(
        "<assign-statement>".to_string(),
        vec![
            Union::OnlyA("@Assign:Iterable;".to_string()),
            Union::OnlyA("@Assign:Primitive;".to_string()),
            Union::OnlyA("@Define:Iterable;".to_string()),
            Union::OnlyA("@Define:Primitive;".to_string()),
        ],
    );
    python_grammar.insert(
        "<call-statement>".to_string(),
        vec![Union::OnlyA("<func-expr>".to_string())],
    );
    python_grammar.insert(
        "<func-expr>".to_string(),
        vec![Union::OnlyA("<func-ident>(<func-vars>)".to_string())],
    );
    python_grammar.insert(
        "<func-ident>".to_string(),
        vec![Union::OnlyA("print".to_string())],
    );
    python_grammar.insert(
        "<func-vars>".to_string(),
        vec![
            Union::OnlyA("@Refer:Iterable;".to_string()),
            Union::OnlyA("@Refer:Primitive;".to_string()),
            Union::OnlyA("<func-vars>".to_string()),
        ],
    );
    python_grammar.insert(
        "<expr>".to_string(),
        vec![
            Union::OnlyA("<term> + <expr>".to_string()),
            Union::OnlyA("<term> - <expr>".to_string()),
            Union::OnlyA("<term>".to_string()),
        ],
    );
    python_grammar.insert(
        "<term>".to_string(),
        vec![
            Union::OnlyA("<factor> * <term>".to_string()),
            Union::OnlyA("<factor> / <term>".to_string()),
            Union::OnlyA("<factor>".to_string()),
        ],
    );
    python_grammar.insert(
        "<factor>".to_string(),
        vec![
            Union::OnlyA("+<integer>".to_string()),
            Union::OnlyA("-<integer>".to_string()),
            Union::OnlyA("+<integer>.<integer>".to_string()),
            Union::OnlyA("-<integer>.<integer>".to_string()),
            Union::OnlyA("(<expr>)".to_string()),
            Union::OnlyA("<integer>.<integer>".to_string()),
            Union::OnlyA("<integer>".to_string()),
            Union::OnlyA("@Refer:Primitive;".to_string()),
        ],
    );
    python_grammar.insert(
        "<integer>".to_string(),
        vec![
            Union::OnlyA("<non-zero-digit><integer>".to_string()),
            Union::OnlyA("<non-zero-digit>".to_string()),
        ],
    );
    python_grammar.insert("<non-zero-digit>".to_string(), range_chars_as_str(CharRange::NonZeroDigit));
    python_grammar.insert(
        "<letter>".to_string(),
        range_chars_as_str(CharRange::Letters),
    );
    return python_grammar;
}
pub fn get_array_grammar() -> Grammar<'static> {
    let mut array_grammar: Grammar = HashMap::new();

    array_grammar.insert(
        "<array>".to_string(),
        vec![Union::OnlyA("[<elems>]".to_string())],
    );
    array_grammar.insert(
        "<elems>".to_string(),
        vec![
            Union::OnlyA("<elem>".to_string()),
            Union::OnlyA("<elems>,<elem>".to_string()),
        ],
    );
    array_grammar.insert(
        "<elem>".to_string(),
        vec![Union::OnlyA("<integer>".to_string())],
    );
    array_grammar.insert(
        "<integer>".to_string(),
        vec![
            Union::OnlyA("<non-zero-digit><integer>".to_string()),
            Union::OnlyA("<non-zero-digit>".to_string()),
        ],
    );
    array_grammar.insert("<non-zero-digit>".to_string(), range_chars_as_str(CharRange::NonZeroDigit));
    return array_grammar;
}
pub fn get_expr_grammar() -> Grammar<'static> {
    let mut expr_grammar: Grammar = HashMap::new();
    expr_grammar.insert(
        "<start>".to_string(),
        vec![Union::OnlyA("<expr>".to_string())],
    );
    expr_grammar.insert(
        "<expr>".to_string(),
        vec![
            Union::OnlyA("<term> + <expr>".to_string()),
            Union::OnlyA("<term> - <expr>".to_string()),
            Union::OnlyA("<term>".to_string()),
        ],
    );
    expr_grammar.insert(
        "<term>".to_string(),
        vec![
            Union::OnlyA("<factor> * <term>".to_string()),
            Union::OnlyA("<factor> / <term>".to_string()),
            Union::OnlyA("<factor>".to_string()),
        ],
    );
    expr_grammar.insert(
        "<factor>".to_string(),
        vec![
            Union::OnlyA("+<integer>".to_string()),
            Union::OnlyA("-<integer>".to_string()),
            Union::OnlyA("+<integer>.<integer>".to_string()),
            Union::OnlyA("-<integer>.<integer>".to_string()),
            Union::OnlyA("(<expr>)".to_string()),
            Union::OnlyA("<integer>.<integer>".to_string()),
            Union::OnlyA("<integer>".to_string()),
        ],
    );
    expr_grammar.insert(
        "<integer>".to_string(),
        vec![
            Union::OnlyA("<non-zero-digit><integer>".to_string()),
            Union::OnlyA("<non-zero-digit>".to_string()),
        ],
    );
    expr_grammar.insert("<non-zero-digit>".to_string(), range_chars_as_str(CharRange::NonZeroDigit));
    return expr_grammar;
}

pub fn get_xml_grammar() -> Grammar<'static> {
    let mut xml_grammar: Grammar = HashMap::new();
    xml_grammar.insert(
        "<start>".to_string(),
        vec![Union::OnlyA("<xml-tree>".to_string())],
    );
    xml_grammar.insert(
        "<xml-tree>".to_string(),
        vec![
            Union::OnlyA("<text>".to_string()),
            Union::OnlyA("<xml-open-tag><xml-tree><xml-close-tag>".to_string()),
            Union::OnlyA("<xml-openclose-tag>".to_string()),
            Union::OnlyA("<xml-tree><xml-tree>".to_string()),
        ],
    );
    xml_grammar.insert(
        "<xml-open-tag>".to_string(),
        vec![
            Union::OnlyA("<<id>>".to_string()),
            Union::OnlyA("<<id> <xml-attribute>>".to_string()),
        ],
    );
    xml_grammar.insert(
        "<xml-openclose-tag>".to_string(),
        vec![
            Union::OnlyA("<<id>/>".to_string()),
            Union::OnlyA("<<id> <xml-attribute>/>".to_string()),
        ],
    );
    xml_grammar.insert(
        "<xml-close-tag>".to_string(),
        vec![Union::OnlyA("</<id>>".to_string())],
    );
    xml_grammar.insert(
        "<xml-attribute>".to_string(),
        vec![
            Union::OnlyA("<id>=<id>".to_string()),
            Union::OnlyA("<xml-attribute> <xml-attribute>".to_string()),
        ],
    );
    xml_grammar.insert(
        "<id>".to_string(),
        vec![
            Union::OnlyA("<letter>".to_string()),
            Union::OnlyA("<id><letter>".to_string()),
        ],
    );

    xml_grammar.insert(
        "<text>".to_string(),
        vec![
            Union::OnlyA("<text><letter-space>".to_string()),
            Union::OnlyA("<letter-space>".to_string()),
        ],
    );

    let mut vec1 = range_chars_as_str(CharRange::Digit);
    vec1.append(&mut range_chars_as_str(CharRange::Letters));
    vec1.append(&mut vec![
        Union::OnlyA("\\".to_string()),
        Union::OnlyA("'".to_string()),
    ]);

    let mut letter_vec = vec1.clone();
    letter_vec.push(Union::OnlyA("'".to_string()));

    let mut letter_space_vec = vec1.clone();
    letter_space_vec.append(&mut vec![
        Union::OnlyA("\t".to_string()),
        Union::OnlyA(" ".to_string()),
    ]);

    xml_grammar.insert("<letter>".to_string(), letter_vec);
    xml_grammar.insert("<letter-space>".to_string(), letter_space_vec);
    return xml_grammar;
}