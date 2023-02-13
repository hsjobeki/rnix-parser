#[macro_export]
#[rustfmt::skip]
macro_rules! T {

    ("Bool")   => ($crate::SyntaxKind::TOKEN_BOOL_TYPE);
    ("Int")   => ($crate::SyntaxKind::TOKEN_INT_TYPE);
    ("Float")   => ($crate::SyntaxKind::TOKEN_FLOAT_TYPE);
    ("String")   => ($crate::SyntaxKind::TOKEN_STRING_TYPE);
    ("Path")   => ($crate::SyntaxKind::TOKEN_PATH_TYPE);
    ("Null")   => ($crate::SyntaxKind::TOKEN_NULL_TYPE);
    // Composed & Reserved
    ("Number")   => ($crate::SyntaxKind::TOKEN_NUMBER_TYPE);
    ("Any")   => ($crate::SyntaxKind::TOKEN_ANY_TYPE);
    ("Derivation")   => ($crate::SyntaxKind::TOKEN_DERIVATION_TYPE);
    ("Package")   => ($crate::SyntaxKind::TOKEN_PACKAGE_TYPE);
    ("StorePath")   => ($crate::SyntaxKind::TOKEN_STOREPATH_TYPE);

    ('{')     => ($crate::SyntaxKind::TOKEN_ATTRSET_TYPE_START);
    ('}')     => ($crate::SyntaxKind::TOKEN_ATTRSET_TYPE_END);
    ('[')     => ($crate::SyntaxKind::TOKEN_LIST_TYPE_START);
    (']')     => ($crate::SyntaxKind::TOKEN_LIST_TYPE_END);
    ('(')     => ($crate::SyntaxKind::TOKEN_L_PAREN);
    (')')     => ($crate::SyntaxKind::TOKEN_R_PAREN);
    // operators
    (::)       => ($crate::SyntaxKind::TOKEN_DOUBLE_COLON);
    (...)     => ($crate::SyntaxKind::TOKEN_ELLIPSIS);
    (?)       => ($crate::SyntaxKind::TOKEN_QUESTION);
    (=)       => ($crate::SyntaxKind::TOKEN_ASSIGN);
    (;)       => ($crate::SyntaxKind::TOKEN_SEMICOLON);
    (->)      => ($crate::SyntaxKind::TOKEN_LAMBDA_TYPE);
    ("//")    => ($crate::SyntaxKind::TOKEN_UPDATE);
    (|)      => ($crate::SyntaxKind::TOKEN_PIPE);
    
    
    // not needed ?
    (assert)  => ($crate::SyntaxKind::TOKEN_ASSERT);
    (else)    => ($crate::SyntaxKind::TOKEN_ELSE);
    (if)      => ($crate::SyntaxKind::TOKEN_IF);
    (in)      => ($crate::SyntaxKind::TOKEN_IN);
    (inherit) => ($crate::SyntaxKind::TOKEN_INHERIT);
    (let)     => ($crate::SyntaxKind::TOKEN_LET);
    (or)      => ($crate::SyntaxKind::TOKEN_OR);
    (rec)     => ($crate::SyntaxKind::TOKEN_REC);
    (then)    => ($crate::SyntaxKind::TOKEN_THEN);
    (with)    => ($crate::SyntaxKind::TOKEN_WITH);
    
    
    (@)       => ($crate::SyntaxKind::TOKEN_AT);
    (:)       => ($crate::SyntaxKind::TOKEN_COLON);
    (,)       => ($crate::SyntaxKind::TOKEN_COMMA);
    (.)       => ($crate::SyntaxKind::TOKEN_DOT);
    (++)      => ($crate::SyntaxKind::TOKEN_CONCAT);
    (!)       => ($crate::SyntaxKind::TOKEN_INVERT);
    
    (+)       => ($crate::SyntaxKind::TOKEN_ADD);
    (-)       => ($crate::SyntaxKind::TOKEN_SUB);
    (*)       => ($crate::SyntaxKind::TOKEN_MUL);
    (/)       => ($crate::SyntaxKind::TOKEN_DIV);

    (&&)      => ($crate::SyntaxKind::TOKEN_AND_AND);
    (==)      => ($crate::SyntaxKind::TOKEN_EQUAL);
    
    (<)       => ($crate::SyntaxKind::TOKEN_LESS);
    (<=)      => ($crate::SyntaxKind::TOKEN_LESS_OR_EQ);
    (>)       => ($crate::SyntaxKind::TOKEN_MORE);
    (>=)      => ($crate::SyntaxKind::TOKEN_MORE_OR_EQ);
    (!=)      => ($crate::SyntaxKind::TOKEN_NOT_EQUAL);
    (||)      => ($crate::SyntaxKind::TOKEN_OR_OR);
    
    ($kind:ident) => ($crate::SyntaxKind::$kind);
}
