#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SyntaxKind {
    // Type Keywords
    // Basic types
    TOKEN_BOOL_TYPE,
    TOKEN_INT_TYPE,
    TOKEN_FLOAT_TYPE,
    TOKEN_STRING_TYPE,
    TOKEN_PATH_TYPE,
    TOKEN_NULL_TYPE,
    // Composed Types
    TOKEN_ATTRSET_TYPE_START,
    TOKEN_ATTRSET_TYPE_END,

    TOKEN_LIST_TYPE_START,
    TOKEN_LIST_TYPE_END,

    TOKEN_LAMBDA_TYPE,
    // Predefined / reserved types
    TOKEN_NUMBER_TYPE,
    TOKEN_ANY_TYPE,
    TOKEN_DERIVATION_TYPE,
    TOKEN_STOREPATH_TYPE,
    TOKEN_PACKAGE_TYPE,

    // TYPE Operators
    TOKEN_PIPE,

    // Internals
    TOKEN_COMMENT, /* */
    //
    TOKEN_ERROR,      // Error
    TOKEN_WHITESPACE, // " "

    // Keywords
    TOKEN_ASSERT,  // "assert"
    TOKEN_ELSE,    // "else"
    TOKEN_IF,      // "if"
    TOKEN_IN,      // "in"
    TOKEN_INHERIT, // "inherit"
    TOKEN_LET,     // "let"
    TOKEN_OR,      // "or"
    TOKEN_REC,     // "rec"
    TOKEN_THEN,    // "then"
    TOKEN_WITH,    // "with"

    // Symbols
    TOKEN_L_BRACE,      // "{"
    TOKEN_R_BRACE,      // "}"
    TOKEN_L_BRACK,      // "["
    TOKEN_R_BRACK,      // "]"
    TOKEN_ASSIGN,       // "="
    TOKEN_AT,           // "@"
    TOKEN_COLON,        // ":"
    TOKEN_DOUBLE_COLON, // "a :: Int"
    TOKEN_COMMA,        // ","
    TOKEN_DOT,          // "."
    TOKEN_ELLIPSIS,     // "..."
    TOKEN_QUESTION,     // "?"
    TOKEN_SEMICOLON,    // ";"

    // Operators
    TOKEN_L_PAREN, // "("
    TOKEN_R_PAREN, // ")"
    TOKEN_CONCAT,  // "[] ++ []"
    TOKEN_INVERT,  // "!"
    TOKEN_UPDATE,  // "//"

    TOKEN_ADD, // "+"
    TOKEN_SUB, // "-"
    TOKEN_MUL, // "*"
    TOKEN_DIV, // "/"

    TOKEN_AND_AND,     // "&&"
    TOKEN_EQUAL,       // "=="
    TOKEN_IMPLICATION, // "->"
    TOKEN_LESS,        // "<"
    TOKEN_LESS_OR_EQ,  // "<="
    TOKEN_MORE,        // ">"
    TOKEN_MORE_OR_EQ,  // ">="
    TOKEN_NOT_EQUAL,   // "!="
    TOKEN_OR_OR,       // "||"

    // Identifiers and values
    TOKEN_FLOAT,          // "1.1" -> "FLOAT(1.1)"
    TOKEN_IDENT, // "foo" -> "Ident(Foo)" -> 'a'..='z' | 'A'..='Z' | '_' but not one of reserved keywords: ["let" "in" "inherit" "assert" ...]
    TOKEN_INTEGER, // "1" -> "Int(1)"
    TOKEN_INTERPOL_END, // "}" -> if Context::Interpol -> destroy Context::Interpol
    TOKEN_INTERPOL_START, //  "${" -> create Context::Interpol
    TOKEN_PATH,  // "<Path>" -> "<...validPath....>"  only if kind is IdentType::Store
    TOKEN_URI,   // 'a'..='z' | 'A'..='Z' | '_'
    TOKEN_STRING_CONTENT, //  '' -> create [a-Z]... -> create
    TOKEN_STRING_END, //  " or ''
    TOKEN_STRING_START, // " or '' -> create Context::StringBody

    // Parser ?
    NODE_TYPE,
    NODE_DECLARE,
    NODE_ASSIGN,
    NODE_TYPE_UNION,
    //
    NODE_APPLY,
    NODE_ASSERT,
    NODE_ATTRPATH,
    NODE_DYNAMIC,
    NODE_ERROR,
    NODE_IDENT,
    NODE_IF_ELSE,
    NODE_SELECT,
    NODE_INHERIT,
    NODE_INHERIT_FROM,
    NODE_STRING,
    NODE_INTERPOL,
    NODE_LAMBDA,
    NODE_IDENT_PARAM,
    // An old let { x = 92; body = x; } syntax
    NODE_LEGACY_LET,
    NODE_LET_IN,
    NODE_LIST,
    NODE_BIN_OP,
    NODE_PAREN,
    NODE_PATTERN,
    NODE_PAT_BIND,
    NODE_PAT_ENTRY,
    NODE_ROOT,
    NODE_ATTR_SET,
    NODE_ATTRPATH_VALUE,
    NODE_UNARY_OP,
    NODE_LITERAL,
    NODE_WITH,
    NODE_PATH,
    // Attrpath existence check: foo ? bar.${baz}."bux"
    NODE_HAS_ATTR,

    #[doc(hidden)]
    __LAST,
}

use SyntaxKind::*;

impl SyntaxKind {
    /// Returns true if this token is a literal, such as an integer or a string
    pub fn is_literal(self) -> bool {
        matches!(self, TOKEN_FLOAT | TOKEN_INTEGER | TOKEN_PATH | TOKEN_URI)
    }

    /// Returns true if this token should be used as a function argument.
    /// ```ignore
    /// Example:
    /// add 1 2 + 3
    /// ^   ^ ^ ^
    /// |   | | +- false
    /// |   | +- true
    /// |   +- true
    /// +- true
    /// ```
    pub fn is_fn_arg(self) -> bool {
        match self {
            TOKEN_REC | TOKEN_L_BRACE | TOKEN_L_BRACK | TOKEN_L_PAREN | TOKEN_STRING_START
            | TOKEN_IDENT => true,
            _ => self.is_literal(),
        }
    }
    /// Returns true if this token is a comment, whitespace, or similar, and
    /// should be skipped over by the parser.
    pub fn is_trivia(self) -> bool {
        matches!(self, TOKEN_COMMENT | TOKEN_ERROR | TOKEN_WHITESPACE)
    }
}
