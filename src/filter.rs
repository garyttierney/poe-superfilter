use ast;
use std::str::FromStr;
use tok::Location as TokenLocation;
use tok::Tok;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Filter {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast;
    use std::str::FromStr;
    use tok::Location as TokenLocation;
    use tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<> {
        Term_22_28_22(Tok),
        Term_22_29_22(Tok),
        Term_22_2a_22(Tok),
        Term_22_2b_22(Tok),
        Term_22_2d_22(Tok),
        Term_22_2f_22(Tok),
        Term_22_3c_22(Tok),
        Term_22_3c_3d_22(Tok),
        Term_22_3d_22(Tok),
        Term_22_3e_22(Tok),
        Term_22_3e_3d_22(Tok),
        Term_22Hide_22(Tok),
        Term_22Show_22(Tok),
        Term_22_5c_5cn_22(Tok),
        TermConstant(String),
        TermNum(i32),
        TermQuotedStrLiteral(String),
        NtBlock(ast::Block),
        NtBlock_2a(::std::vec::Vec<ast::Block>),
        NtBlock_2b(::std::vec::Vec<ast::Block>),
        NtColor(ast::Color),
        NtComparisonOperator(ast::ComparisonOperator),
        NtCondition(ast::Condition),
        NtFilter(Box<Vec<ast::Block>>),
        NtLine(ast::Instruction),
        NtLine_2a(::std::vec::Vec<ast::Instruction>),
        NtLine_2b(::std::vec::Vec<ast::Instruction>),
        NtNumExpression(ast::NumberExpression),
        NtNumExpression_2b(::std::vec::Vec<ast::NumberExpression>),
        NtStrLiteral(String),
        NtStrLiteral_2b(::std::vec::Vec<String>),
        NtValue(ast::Value),
        Nt____Filter(Box<Vec<ast::Block>>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        5, // on "Hide", goto 4
        6, // on "Show", goto 5
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -7, // on "Hide", reduce `Block+ = Block => ActionFn(27);`
        -7, // on "Show", reduce `Block+ = Block => ActionFn(27);`
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        5, // on "Hide", goto 4
        6, // on "Show", goto 5
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 3
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 4
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        8, // on "\\n", goto 7
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 5
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        9, // on "\\n", goto 8
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 6
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -8, // on "Hide", reduce `Block+ = Block+, Block => ActionFn(28);`
        -8, // on "Show", reduce `Block+ = Block+, Block => ActionFn(28);`
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 7
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -3, // on "Hide", reduce `Block = "Hide", "\\n" => ActionFn(35);`
        -3, // on "Show", reduce `Block = "Hide", "\\n" => ActionFn(35);`
        0, // on "\\n", error
        12, // on Constant, goto 11
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 8
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -1, // on "Hide", reduce `Block = "Show", "\\n" => ActionFn(33);`
        -1, // on "Show", reduce `Block = "Show", "\\n" => ActionFn(33);`
        0, // on "\\n", error
        12, // on Constant, goto 11
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 9
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -23, // on "Hide", reduce `Line+ = Line => ActionFn(29);`
        -23, // on "Show", reduce `Line+ = Line => ActionFn(29);`
        0, // on "\\n", error
        -23, // on Constant, reduce `Line+ = Line => ActionFn(29);`
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 10
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -4, // on "Hide", reduce `Block = "Hide", "\\n", Line+ => ActionFn(36);`
        -4, // on "Show", reduce `Block = "Hide", "\\n", Line+ => ActionFn(36);`
        0, // on "\\n", error
        12, // on Constant, goto 11
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 11
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        22, // on "<", goto 21
        23, // on "<=", goto 22
        24, // on "=", goto 23
        25, // on ">", goto 24
        26, // on ">=", goto 25
        0, // on "Hide", error
        0, // on "Show", error
        0, // on "\\n", error
        27, // on Constant, goto 26
        28, // on Num, goto 27
        29, // on QuotedStrLiteral, goto 28
        // State 12
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -2, // on "Hide", reduce `Block = "Show", "\\n", Line+ => ActionFn(34);`
        -2, // on "Show", reduce `Block = "Show", "\\n", Line+ => ActionFn(34);`
        0, // on "\\n", error
        12, // on Constant, goto 11
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 13
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -24, // on "Hide", reduce `Line+ = Line+, Line => ActionFn(30);`
        -24, // on "Show", reduce `Line+ = Line+, Line => ActionFn(30);`
        0, // on "\\n", error
        -24, // on Constant, reduce `Line+ = Line+, Line => ActionFn(30);`
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 14
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        0, // on "\\n", error
        27, // on Constant, goto 26
        28, // on Num, goto 27
        29, // on QuotedStrLiteral, goto 28
        // State 15
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        31, // on "\\n", goto 30
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 16
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        -26, // on "\\n", reduce `NumExpression+ = NumExpression => ActionFn(21);`
        0, // on Constant, error
        -26, // on Num, reduce `NumExpression+ = NumExpression => ActionFn(21);`
        0, // on QuotedStrLiteral, error
        // State 17
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        -32, // on "\\n", reduce `Value = NumExpression+ => ActionFn(6);`
        0, // on Constant, error
        28, // on Num, goto 27
        0, // on QuotedStrLiteral, error
        // State 18
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        -30, // on "\\n", reduce `StrLiteral+ = StrLiteral => ActionFn(19);`
        -30, // on Constant, reduce `StrLiteral+ = StrLiteral => ActionFn(19);`
        0, // on Num, error
        -30, // on QuotedStrLiteral, reduce `StrLiteral+ = StrLiteral => ActionFn(19);`
        // State 19
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        -33, // on "\\n", reduce `Value = StrLiteral+ => ActionFn(7);`
        27, // on Constant, goto 26
        0, // on Num, error
        29, // on QuotedStrLiteral, goto 28
        // State 20
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        34, // on "\\n", goto 33
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 21
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        0, // on "\\n", error
        -14, // on Constant, reduce `ComparisonOperator = "<" => ActionFn(14);`
        -14, // on Num, reduce `ComparisonOperator = "<" => ActionFn(14);`
        -14, // on QuotedStrLiteral, reduce `ComparisonOperator = "<" => ActionFn(14);`
        // State 22
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        0, // on "\\n", error
        -13, // on Constant, reduce `ComparisonOperator = "<=" => ActionFn(13);`
        -13, // on Num, reduce `ComparisonOperator = "<=" => ActionFn(13);`
        -13, // on QuotedStrLiteral, reduce `ComparisonOperator = "<=" => ActionFn(13);`
        // State 23
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        0, // on "\\n", error
        -15, // on Constant, reduce `ComparisonOperator = "=" => ActionFn(15);`
        -15, // on Num, reduce `ComparisonOperator = "=" => ActionFn(15);`
        -15, // on QuotedStrLiteral, reduce `ComparisonOperator = "=" => ActionFn(15);`
        // State 24
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        0, // on "\\n", error
        -12, // on Constant, reduce `ComparisonOperator = ">" => ActionFn(12);`
        -12, // on Num, reduce `ComparisonOperator = ">" => ActionFn(12);`
        -12, // on QuotedStrLiteral, reduce `ComparisonOperator = ">" => ActionFn(12);`
        // State 25
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        0, // on "\\n", error
        -11, // on Constant, reduce `ComparisonOperator = ">=" => ActionFn(11);`
        -11, // on Num, reduce `ComparisonOperator = ">=" => ActionFn(11);`
        -11, // on QuotedStrLiteral, reduce `ComparisonOperator = ">=" => ActionFn(11);`
        // State 26
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        -29, // on "\\n", reduce `StrLiteral = Constant => ActionFn(9);`
        -29, // on Constant, reduce `StrLiteral = Constant => ActionFn(9);`
        0, // on Num, error
        -29, // on QuotedStrLiteral, reduce `StrLiteral = Constant => ActionFn(9);`
        // State 27
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        -25, // on "\\n", reduce `NumExpression = Num => ActionFn(16);`
        0, // on Constant, error
        -25, // on Num, reduce `NumExpression = Num => ActionFn(16);`
        0, // on QuotedStrLiteral, error
        // State 28
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        -28, // on "\\n", reduce `StrLiteral = QuotedStrLiteral => ActionFn(8);`
        -28, // on Constant, reduce `StrLiteral = QuotedStrLiteral => ActionFn(8);`
        0, // on Num, error
        -28, // on QuotedStrLiteral, reduce `StrLiteral = QuotedStrLiteral => ActionFn(8);`
        // State 29
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        -16, // on "\\n", reduce `Condition = ComparisonOperator, Value => ActionFn(10);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 30
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -20, // on "Hide", reduce `Line = Constant, Condition, "\\n" => ActionFn(5);`
        -20, // on "Show", reduce `Line = Constant, Condition, "\\n" => ActionFn(5);`
        0, // on "\\n", error
        -20, // on Constant, reduce `Line = Constant, Condition, "\\n" => ActionFn(5);`
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        // State 31
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        -27, // on "\\n", reduce `NumExpression+ = NumExpression+, NumExpression => ActionFn(22);`
        0, // on Constant, error
        -27, // on Num, reduce `NumExpression+ = NumExpression+, NumExpression => ActionFn(22);`
        0, // on QuotedStrLiteral, error
        // State 32
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Show", error
        -31, // on "\\n", reduce `StrLiteral+ = StrLiteral+, StrLiteral => ActionFn(20);`
        -31, // on Constant, reduce `StrLiteral+ = StrLiteral+, StrLiteral => ActionFn(20);`
        0, // on Num, error
        -31, // on QuotedStrLiteral, reduce `StrLiteral+ = StrLiteral+, StrLiteral => ActionFn(20);`
        // State 33
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -19, // on "Hide", reduce `Line = Constant, Value, "\\n" => ActionFn(4);`
        -19, // on "Show", reduce `Line = Constant, Value, "\\n" => ActionFn(4);`
        0, // on "\\n", error
        -19, // on Constant, reduce `Line = Constant, Value, "\\n" => ActionFn(4);`
        0, // on Num, error
        0, // on QuotedStrLiteral, error
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -17, // on EOF, reduce `Filter =  => ActionFn(31);`
        -7, // on EOF, reduce `Block+ = Block => ActionFn(27);`
        -18, // on EOF, reduce `Filter = Block+ => ActionFn(32);`
        -34, // on EOF, reduce `__Filter = Filter => ActionFn(0);`
        0, // on EOF, error
        0, // on EOF, error
        -8, // on EOF, reduce `Block+ = Block+, Block => ActionFn(28);`
        -3, // on EOF, reduce `Block = "Hide", "\\n" => ActionFn(35);`
        -1, // on EOF, reduce `Block = "Show", "\\n" => ActionFn(33);`
        -23, // on EOF, reduce `Line+ = Line => ActionFn(29);`
        -4, // on EOF, reduce `Block = "Hide", "\\n", Line+ => ActionFn(36);`
        0, // on EOF, error
        -2, // on EOF, reduce `Block = "Show", "\\n", Line+ => ActionFn(34);`
        -24, // on EOF, reduce `Line+ = Line+, Line => ActionFn(30);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -20, // on EOF, reduce `Line = Constant, Condition, "\\n" => ActionFn(5);`
        0, // on EOF, error
        0, // on EOF, error
        -19, // on EOF, reduce `Line = Constant, Value, "\\n" => ActionFn(4);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, // on Block, goto 1
        0, // on Block*, error
        3, // on Block+, goto 2
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        4, // on Filter, goto 3
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 1
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 2
        7, // on Block, goto 6
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 3
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 4
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 5
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 6
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 7
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        10, // on Line, goto 9
        0, // on Line*, error
        11, // on Line+, goto 10
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 8
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        10, // on Line, goto 9
        0, // on Line*, error
        13, // on Line+, goto 12
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 9
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 10
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        14, // on Line, goto 13
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 11
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        15, // on ComparisonOperator, goto 14
        16, // on Condition, goto 15
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        17, // on NumExpression, goto 16
        18, // on NumExpression+, goto 17
        19, // on StrLiteral, goto 18
        20, // on StrLiteral+, goto 19
        21, // on Value, goto 20
        0, // on __Filter, error
        // State 12
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        14, // on Line, goto 13
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 13
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 14
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        17, // on NumExpression, goto 16
        18, // on NumExpression+, goto 17
        19, // on StrLiteral, goto 18
        20, // on StrLiteral+, goto 19
        30, // on Value, goto 29
        0, // on __Filter, error
        // State 15
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 16
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 17
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        32, // on NumExpression, goto 31
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 18
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 19
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        33, // on StrLiteral, goto 32
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 20
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 21
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 22
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 23
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 24
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 25
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 26
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 27
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 28
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 29
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 30
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 31
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 32
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
        // State 33
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on Line, error
        0, // on Line*, error
        0, // on Line+, error
        0, // on NumExpression, error
        0, // on NumExpression+, error
        0, // on StrLiteral, error
        0, // on StrLiteral+, error
        0, // on Value, error
        0, // on __Filter, error
    ];
    pub fn parse_Filter<
        __TOKEN: __ToTriple<Error=char>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens0: __TOKENS,
    ) -> Result<Box<Vec<ast::Block>>, __lalrpop_util::ParseError<TokenLocation,Tok,char>>
    {
        let __tokens = __tokens0.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        '__shift: loop {
            let __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(__lalrpop_util::ParseError::User { error: e }),
            };
            let __integer = match __lookahead {
                (_, Tok::LParen, _) if true => 0,
                (_, Tok::RParen, _) if true => 1,
                (_, Tok::Times, _) if true => 2,
                (_, Tok::Plus, _) if true => 3,
                (_, Tok::Minus, _) if true => 4,
                (_, Tok::Div, _) if true => 5,
                (_, Tok::Lt, _) if true => 6,
                (_, Tok::Lte, _) if true => 7,
                (_, Tok::Eql, _) if true => 8,
                (_, Tok::Gt, _) if true => 9,
                (_, Tok::Gte, _) if true => 10,
                (_, Tok::Hide, _) if true => 11,
                (_, Tok::Show, _) if true => 12,
                (_, Tok::NewLine, _) if true => 13,
                (_, Tok::Constant(_), _) if true => 14,
                (_, Tok::Num(_), _) if true => 15,
                (_, Tok::StrLiteral(_), _) if true => 16,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 17 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            __tok @ Tok::LParen => __Symbol::Term_22_28_22(__tok),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            __tok @ Tok::RParen => __Symbol::Term_22_29_22(__tok),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            __tok @ Tok::Times => __Symbol::Term_22_2a_22(__tok),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            __tok @ Tok::Plus => __Symbol::Term_22_2b_22(__tok),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            __tok @ Tok::Minus => __Symbol::Term_22_2d_22(__tok),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Tok::Div => __Symbol::Term_22_2f_22(__tok),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Tok::Lt => __Symbol::Term_22_3c_22(__tok),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Tok::Lte => __Symbol::Term_22_3c_3d_22(__tok),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Tok::Eql => __Symbol::Term_22_3d_22(__tok),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Tok::Gt => __Symbol::Term_22_3e_22(__tok),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            __tok @ Tok::Gte => __Symbol::Term_22_3e_3d_22(__tok),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Tok::Hide => __Symbol::Term_22Hide_22(__tok),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            __tok @ Tok::Show => __Symbol::Term_22Show_22(__tok),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Tok::NewLine => __Symbol::Term_22_5c_5cn_22(__tok),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            Tok::Constant(__tok0) => __Symbol::TermConstant(__tok0),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            Tok::Num(__tok0) => __Symbol::TermNum(__tok0),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            Tok::StrLiteral(__tok0) => __Symbol::TermQuotedStrLiteral(__tok0),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(__action, Some(&__lookahead.0), &mut __states, &mut __symbols) {
                        return r;
                    }
                } else {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(__action, None, &mut __states, &mut __symbols) {
                    return r;
                }
            } else {
                return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: vec![],
                });
            }
        }
    }
    pub fn __reduce<
    >(
        __action: i32,
        __lookahead_start: Option<&TokenLocation>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>,
    ) -> Option<Result<Box<Vec<ast::Block>>,__lalrpop_util::ParseError<TokenLocation,Tok,char>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Block = "Show", "\\n" => ActionFn(33);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Term_22Show_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                0
            }
            2 => {
                // Block = "Show", "\\n", Line+ => ActionFn(34);
                let __sym2 = __pop_NtLine_2b(__symbols);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Term_22Show_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action34(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                0
            }
            3 => {
                // Block = "Hide", "\\n" => ActionFn(35);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Term_22Hide_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                0
            }
            4 => {
                // Block = "Hide", "\\n", Line+ => ActionFn(36);
                let __sym2 = __pop_NtLine_2b(__symbols);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Term_22Hide_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action36(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                0
            }
            5 => {
                // Block* =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtBlock_2a(__nt), __end));
                1
            }
            6 => {
                // Block* = Block+ => ActionFn(26);
                let __sym0 = __pop_NtBlock_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBlock_2a(__nt), __end));
                1
            }
            7 => {
                // Block+ = Block => ActionFn(27);
                let __sym0 = __pop_NtBlock(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBlock_2b(__nt), __end));
                2
            }
            8 => {
                // Block+ = Block+, Block => ActionFn(28);
                let __sym1 = __pop_NtBlock(__symbols);
                let __sym0 = __pop_NtBlock_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtBlock_2b(__nt), __end));
                2
            }
            9 => {
                // Color = NumExpression, NumExpression, NumExpression, NumExpression => ActionFn(17);
                let __sym3 = __pop_NtNumExpression(__symbols);
                let __sym2 = __pop_NtNumExpression(__symbols);
                let __sym1 = __pop_NtNumExpression(__symbols);
                let __sym0 = __pop_NtNumExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action17(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtColor(__nt), __end));
                3
            }
            10 => {
                // Color = NumExpression, NumExpression, NumExpression => ActionFn(18);
                let __sym2 = __pop_NtNumExpression(__symbols);
                let __sym1 = __pop_NtNumExpression(__symbols);
                let __sym0 = __pop_NtNumExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action18(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtColor(__nt), __end));
                3
            }
            11 => {
                // ComparisonOperator = ">=" => ActionFn(11);
                let __sym0 = __pop_Term_22_3e_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                4
            }
            12 => {
                // ComparisonOperator = ">" => ActionFn(12);
                let __sym0 = __pop_Term_22_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                4
            }
            13 => {
                // ComparisonOperator = "<=" => ActionFn(13);
                let __sym0 = __pop_Term_22_3c_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                4
            }
            14 => {
                // ComparisonOperator = "<" => ActionFn(14);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                4
            }
            15 => {
                // ComparisonOperator = "=" => ActionFn(15);
                let __sym0 = __pop_Term_22_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                4
            }
            16 => {
                // Condition = ComparisonOperator, Value => ActionFn(10);
                let __sym1 = __pop_NtValue(__symbols);
                let __sym0 = __pop_NtComparisonOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCondition(__nt), __end));
                5
            }
            17 => {
                // Filter =  => ActionFn(31);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action31(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtFilter(__nt), __end));
                6
            }
            18 => {
                // Filter = Block+ => ActionFn(32);
                let __sym0 = __pop_NtBlock_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFilter(__nt), __end));
                6
            }
            19 => {
                // Line = Constant, Value, "\\n" => ActionFn(4);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_NtValue(__symbols);
                let __sym0 = __pop_TermConstant(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                7
            }
            20 => {
                // Line = Constant, Condition, "\\n" => ActionFn(5);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_NtCondition(__symbols);
                let __sym0 = __pop_TermConstant(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                7
            }
            21 => {
                // Line* =  => ActionFn(23);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action23(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtLine_2a(__nt), __end));
                8
            }
            22 => {
                // Line* = Line+ => ActionFn(24);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine_2a(__nt), __end));
                8
            }
            23 => {
                // Line+ = Line => ActionFn(29);
                let __sym0 = __pop_NtLine(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine_2b(__nt), __end));
                9
            }
            24 => {
                // Line+ = Line+, Line => ActionFn(30);
                let __sym1 = __pop_NtLine(__symbols);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action30(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLine_2b(__nt), __end));
                9
            }
            25 => {
                // NumExpression = Num => ActionFn(16);
                let __sym0 = __pop_TermNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumExpression(__nt), __end));
                10
            }
            26 => {
                // NumExpression+ = NumExpression => ActionFn(21);
                let __sym0 = __pop_NtNumExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumExpression_2b(__nt), __end));
                11
            }
            27 => {
                // NumExpression+ = NumExpression+, NumExpression => ActionFn(22);
                let __sym1 = __pop_NtNumExpression(__symbols);
                let __sym0 = __pop_NtNumExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action22(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtNumExpression_2b(__nt), __end));
                11
            }
            28 => {
                // StrLiteral = QuotedStrLiteral => ActionFn(8);
                let __sym0 = __pop_TermQuotedStrLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStrLiteral(__nt), __end));
                12
            }
            29 => {
                // StrLiteral = Constant => ActionFn(9);
                let __sym0 = __pop_TermConstant(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStrLiteral(__nt), __end));
                12
            }
            30 => {
                // StrLiteral+ = StrLiteral => ActionFn(19);
                let __sym0 = __pop_NtStrLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStrLiteral_2b(__nt), __end));
                13
            }
            31 => {
                // StrLiteral+ = StrLiteral+, StrLiteral => ActionFn(20);
                let __sym1 = __pop_NtStrLiteral(__symbols);
                let __sym0 = __pop_NtStrLiteral_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtStrLiteral_2b(__nt), __end));
                13
            }
            32 => {
                // Value = NumExpression+ => ActionFn(6);
                let __sym0 = __pop_NtNumExpression_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValue(__nt), __end));
                14
            }
            33 => {
                // Value = StrLiteral+ => ActionFn(7);
                let __sym0 = __pop_NtStrLiteral_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValue(__nt), __end));
                14
            }
            34 => {
                // __Filter = Filter => ActionFn(0);
                let __sym0 = __pop_NtFilter(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(__sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_28_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_3d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_3d_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22Hide_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Hide_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22Show_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Show_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5c_5cn_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5c_5cn_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermConstant<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, String, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermConstant(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermNum<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, i32, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermQuotedStrLiteral<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, String, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermQuotedStrLiteral(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBlock<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ast::Block, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBlock(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBlock_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBlock_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBlock_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBlock_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtColor<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ast::Color, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtColor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComparisonOperator<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ast::ComparisonOperator, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComparisonOperator(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtCondition<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ast::Condition, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtCondition(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFilter<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Box<Vec<ast::Block>>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFilter(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ast::Instruction, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumExpression<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ast::NumberExpression, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumExpression_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::vec::Vec<ast::NumberExpression>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumExpression_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStrLiteral<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, String, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStrLiteral(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStrLiteral_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::vec::Vec<String>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStrLiteral_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtValue<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ast::Value, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtValue(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Filter<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Box<Vec<ast::Block>>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Filter(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Filter::parse_Filter;

pub fn __action0<
>(
    (_, __0, _): (TokenLocation, Box<Vec<ast::Block>>, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    (__0)
}

pub fn __action1<
>(
    (_, __0, _): (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    Box::new(__0)
}

pub fn __action2<
>(
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, l, _): (TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation),
) -> ast::Block
{
    ast::Block::Show(Box::new(l))
}

pub fn __action3<
>(
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, l, _): (TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation),
) -> ast::Block
{
    ast::Block::Hide(Box::new(l))
}

pub fn __action4<
>(
    (_, n, _): (TokenLocation, String, TokenLocation),
    (_, v, _): (TokenLocation, ast::Value, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
) -> ast::Instruction
{
    ast::Instruction {
        name: n,
        value: ast::InstructionExpression::Value(v)
    }
}

pub fn __action5<
>(
    (_, n, _): (TokenLocation, String, TokenLocation),
    (_, c, _): (TokenLocation, ast::Condition, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
) -> ast::Instruction
{
    ast::Instruction {
        name: n,
        value: ast::InstructionExpression::Condition(c)
    }
}

pub fn __action6<
>(
    (_, __0, _): (TokenLocation, ::std::vec::Vec<ast::NumberExpression>, TokenLocation),
) -> ast::Value
{
    ast::Value::Numbers(__0)
}

pub fn __action7<
>(
    (_, __0, _): (TokenLocation, ::std::vec::Vec<String>, TokenLocation),
) -> ast::Value
{
    ast::Value::Names(__0)
}

pub fn __action8<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
) -> String
{
    String::from(__0)
}

pub fn __action9<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
) -> String
{
    String::from(__0)
}

pub fn __action10<
>(
    (_, op, _): (TokenLocation, ast::ComparisonOperator, TokenLocation),
    (_, v, _): (TokenLocation, ast::Value, TokenLocation),
) -> ast::Condition
{
    ast::Condition { value: v, operator: op }
}

pub fn __action11<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Gte
}

pub fn __action12<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Gt
}

pub fn __action13<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Lte
}

pub fn __action14<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Lt
}

pub fn __action15<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Eq
}

pub fn __action16<
>(
    (_, __0, _): (TokenLocation, i32, TokenLocation),
) -> ast::NumberExpression
{
    ast::NumberExpression::Number(__0)
}

pub fn __action17<
>(
    (_, r, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, g, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, b, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, a, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::Color
{
    ast::Color {
        r: r,
        g: g,
        b: b,
        a: a
    }
}

pub fn __action18<
>(
    (_, r, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, g, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, b, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::Color
{
    ast::Color {
        r: r,
        g: g,
        b: b,
        a: ast::NumberExpression::Number(255)
    }
}

pub fn __action19<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
) -> ::std::vec::Vec<String>
{
    vec![__0]
}

pub fn __action20<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<String>, TokenLocation),
    (_, e, _): (TokenLocation, String, TokenLocation),
) -> ::std::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action21<
>(
    (_, __0, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ::std::vec::Vec<ast::NumberExpression>
{
    vec![__0]
}

pub fn __action22<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::NumberExpression>, TokenLocation),
    (_, e, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ::std::vec::Vec<ast::NumberExpression>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action23<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> ::std::vec::Vec<ast::Instruction>
{
    vec![]
}

pub fn __action24<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation),
) -> ::std::vec::Vec<ast::Instruction>
{
    v
}

pub fn __action25<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> ::std::vec::Vec<ast::Block>
{
    vec![]
}

pub fn __action26<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> ::std::vec::Vec<ast::Block>
{
    v
}

pub fn __action27<
>(
    (_, __0, _): (TokenLocation, ast::Block, TokenLocation),
) -> ::std::vec::Vec<ast::Block>
{
    vec![__0]
}

pub fn __action28<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
    (_, e, _): (TokenLocation, ast::Block, TokenLocation),
) -> ::std::vec::Vec<ast::Block>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action29<
>(
    (_, __0, _): (TokenLocation, ast::Instruction, TokenLocation),
) -> ::std::vec::Vec<ast::Instruction>
{
    vec![__0]
}

pub fn __action30<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation),
    (_, e, _): (TokenLocation, ast::Instruction, TokenLocation),
) -> ::std::vec::Vec<ast::Instruction>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action31<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> Box<Vec<ast::Block>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action25(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

pub fn __action32<
>(
    __0: (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action26(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

pub fn __action33<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
) -> ast::Block
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action23(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        __0,
        __1,
        __temp0,
    )
}

pub fn __action34<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
    __2: (TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation),
) -> ast::Block
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action24(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        __0,
        __1,
        __temp0,
    )
}

pub fn __action35<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
) -> ast::Block
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action23(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        __0,
        __1,
        __temp0,
    )
}

pub fn __action36<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
    __2: (TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation),
) -> ast::Block
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action24(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        __0,
        __1,
        __temp0,
    )
}

pub trait __ToTriple<> {
    type Error;
    fn to_triple(value: Self) -> Result<(TokenLocation,Tok,TokenLocation),Self::Error>;
}

impl<> __ToTriple<> for (TokenLocation, Tok, TokenLocation) {
    type Error = char;
    fn to_triple(value: Self) -> Result<(TokenLocation,Tok,TokenLocation),char> {
        Ok(value)
    }
}
impl<> __ToTriple<> for Result<(TokenLocation, Tok, TokenLocation),char> {
    type Error = char;
    fn to_triple(value: Self) -> Result<(TokenLocation,Tok,TokenLocation),char> {
        value
    }
}
