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
        Term_22_2c_22(Tok),
        Term_22_2d_22(Tok),
        Term_22_2f_22(Tok),
        Term_22_3c_22(Tok),
        Term_22_3c_3d_22(Tok),
        Term_22_3d_22(Tok),
        Term_22_3e_22(Tok),
        Term_22_3e_3d_22(Tok),
        Term_22Hide_22(Tok),
        Term_22Import_22(Tok),
        Term_22Mixin_22(Tok),
        Term_22Show_22(Tok),
        Term_22_5c_5cn_22(Tok),
        TermConstant(String),
        TermNum(i32),
        TermQuotedStrLiteral(String),
        TermVarIdentifier(String),
        Nt_28_22_28_22_20_3cComma_3cVarIdentifier_3e_3e_20_22_29_22_29(Vec<String>),
        Nt_28_22_28_22_20_3cComma_3cVarIdentifier_3e_3e_20_22_29_22_29_3f(::std::option::Option<Vec<String>>),
        Nt_28_3cVarIdentifier_3e_20_22_2c_22_29(String),
        Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2a(::std::vec::Vec<String>),
        Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2b(::std::vec::Vec<String>),
        NtBlock(ast::Block),
        NtBlock_2a(::std::vec::Vec<ast::Block>),
        NtBlock_2b(::std::vec::Vec<ast::Block>),
        NtColor(ast::Color),
        NtComma_3cVarIdentifier_3e(Vec<String>),
        NtComparisonOperator(ast::ComparisonOperator),
        NtCondition(ast::Condition),
        NtFilter(Box<Vec<ast::Block>>),
        NtFilterInstruction(ast::FilterInstruction),
        NtFilterInstruction_2a(::std::vec::Vec<ast::FilterInstruction>),
        NtFilterInstruction_2b(::std::vec::Vec<ast::FilterInstruction>),
        NtNumExpression(ast::NumberExpression),
        NtNumFactor(ast::NumberExpression),
        NtNumTerm(ast::NumberExpression),
        NtStrLiteral(String),
        NtStringExpression(ast::StringBox),
        NtValue(ast::Value),
        NtValue_2b(::std::vec::Vec<ast::Value>),
        NtVarDefinition(ast::VarDefinition),
        NtVarDefinition_2b(::std::vec::Vec<ast::VarDefinition>),
        NtVarDefinitionBlock(ast::Block),
        NtVarDefinitionBlock_3f(::std::option::Option<ast::Block>),
        NtVarIdentifier_3f(::std::option::Option<String>),
        Nt____Filter(Box<Vec<ast::Block>>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        8, // on "Hide", goto 7
        0, // on "Import", error
        9, // on "Mixin", goto 8
        10, // on "Show", goto 9
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        11, // on VarIdentifier, goto 10
        // State 1
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -19, // on "Hide", reduce `Block+ = Block => ActionFn(46);`
        0, // on "Import", error
        -19, // on "Mixin", reduce `Block+ = Block => ActionFn(46);`
        -19, // on "Show", reduce `Block+ = Block => ActionFn(46);`
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 2
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        8, // on "Hide", goto 7
        0, // on "Import", error
        9, // on "Mixin", goto 8
        10, // on "Show", goto 9
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 3
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 4
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -61, // on "Hide", reduce `VarDefinition+ = VarDefinition => ActionFn(40);`
        0, // on "Import", error
        -61, // on "Mixin", reduce `VarDefinition+ = VarDefinition => ActionFn(40);`
        -61, // on "Show", reduce `VarDefinition+ = VarDefinition => ActionFn(40);`
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        -61, // on VarIdentifier, reduce `VarDefinition+ = VarDefinition => ActionFn(40);`
        // State 5
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -63, // on "Hide", reduce `VarDefinitionBlock = VarDefinition+ => ActionFn(2);`
        0, // on "Import", error
        -63, // on "Mixin", reduce `VarDefinitionBlock = VarDefinition+ => ActionFn(2);`
        -63, // on "Show", reduce `VarDefinitionBlock = VarDefinition+ => ActionFn(2);`
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        11, // on VarIdentifier, goto 10
        // State 6
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        8, // on "Hide", goto 7
        0, // on "Import", error
        9, // on "Mixin", goto 8
        10, // on "Show", goto 9
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 7
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        15, // on "\\n", goto 14
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 8
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        16, // on Constant, goto 15
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 9
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        17, // on "\\n", goto 16
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 10
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        18, // on "=", goto 17
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 11
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -20, // on "Hide", reduce `Block+ = Block+, Block => ActionFn(47);`
        0, // on "Import", error
        -20, // on "Mixin", reduce `Block+ = Block+, Block => ActionFn(47);`
        -20, // on "Show", reduce `Block+ = Block+, Block => ActionFn(47);`
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 12
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -62, // on "Hide", reduce `VarDefinition+ = VarDefinition+, VarDefinition => ActionFn(41);`
        0, // on "Import", error
        -62, // on "Mixin", reduce `VarDefinition+ = VarDefinition+, VarDefinition => ActionFn(41);`
        -62, // on "Show", reduce `VarDefinition+ = VarDefinition+, VarDefinition => ActionFn(41);`
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        -62, // on VarIdentifier, reduce `VarDefinition+ = VarDefinition+, VarDefinition => ActionFn(41);`
        // State 13
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        8, // on "Hide", goto 7
        0, // on "Import", error
        9, // on "Mixin", goto 8
        10, // on "Show", goto 9
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 14
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -11, // on "Hide", reduce `Block = "Hide", "\\n" => ActionFn(68);`
        0, // on "Import", error
        -11, // on "Mixin", reduce `Block = "Hide", "\\n" => ActionFn(68);`
        -11, // on "Show", reduce `Block = "Hide", "\\n" => ActionFn(68);`
        0, // on "\\n", error
        22, // on Constant, goto 21
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        23, // on VarIdentifier, goto 22
        // State 15
        24, // on "(", goto 23
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        25, // on "\\n", goto 24
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 16
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -9, // on "Hide", reduce `Block = "Show", "\\n" => ActionFn(66);`
        0, // on "Import", error
        -9, // on "Mixin", reduce `Block = "Show", "\\n" => ActionFn(66);`
        -9, // on "Show", reduce `Block = "Show", "\\n" => ActionFn(66);`
        0, // on "\\n", error
        22, // on Constant, goto 21
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        23, // on VarIdentifier, goto 22
        // State 17
        34, // on "(", goto 33
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        35, // on Constant, goto 34
        36, // on Num, goto 35
        37, // on QuotedStrLiteral, goto 36
        38, // on VarIdentifier, goto 37
        // State 18
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -42, // on "Hide", reduce `FilterInstruction+ = FilterInstruction => ActionFn(48);`
        0, // on "Import", error
        -42, // on "Mixin", reduce `FilterInstruction+ = FilterInstruction => ActionFn(48);`
        -42, // on "Show", reduce `FilterInstruction+ = FilterInstruction => ActionFn(48);`
        0, // on "\\n", error
        -42, // on Constant, reduce `FilterInstruction+ = FilterInstruction => ActionFn(48);`
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        -42, // on VarIdentifier, reduce `FilterInstruction+ = FilterInstruction => ActionFn(48);`
        // State 19
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -12, // on "Hide", reduce `Block = "Hide", "\\n", FilterInstruction+ => ActionFn(69);`
        0, // on "Import", error
        -12, // on "Mixin", reduce `Block = "Hide", "\\n", FilterInstruction+ => ActionFn(69);`
        -12, // on "Show", reduce `Block = "Hide", "\\n", FilterInstruction+ => ActionFn(69);`
        0, // on "\\n", error
        22, // on Constant, goto 21
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        23, // on VarIdentifier, goto 22
        // State 20
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -39, // on "Hide", reduce `FilterInstruction = VarDefinition => ActionFn(9);`
        0, // on "Import", error
        -39, // on "Mixin", reduce `FilterInstruction = VarDefinition => ActionFn(9);`
        -39, // on "Show", reduce `FilterInstruction = VarDefinition => ActionFn(9);`
        0, // on "\\n", error
        -39, // on Constant, reduce `FilterInstruction = VarDefinition => ActionFn(9);`
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        -39, // on VarIdentifier, reduce `FilterInstruction = VarDefinition => ActionFn(9);`
        // State 21
        34, // on "(", goto 33
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        43, // on "<", goto 42
        44, // on "<=", goto 43
        45, // on "=", goto 44
        46, // on ">", goto 45
        47, // on ">=", goto 46
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        35, // on Constant, goto 34
        36, // on Num, goto 35
        37, // on QuotedStrLiteral, goto 36
        38, // on VarIdentifier, goto 37
        // State 22
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        48, // on "=", goto 47
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 23
        0, // on "(", error
        -24, // on ")", reduce `Comma<VarIdentifier> =  => ActionFn(79);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        51, // on VarIdentifier, goto 50
        // State 24
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -15, // on "Hide", reduce `Block = "Mixin", Constant, "\\n" => ActionFn(72);`
        0, // on "Import", error
        -15, // on "Mixin", reduce `Block = "Mixin", Constant, "\\n" => ActionFn(72);`
        -15, // on "Show", reduce `Block = "Mixin", Constant, "\\n" => ActionFn(72);`
        0, // on "\\n", error
        22, // on Constant, goto 21
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        23, // on VarIdentifier, goto 22
        // State 25
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -10, // on "Hide", reduce `Block = "Show", "\\n", FilterInstruction+ => ActionFn(67);`
        0, // on "Import", error
        -10, // on "Mixin", reduce `Block = "Show", "\\n", FilterInstruction+ => ActionFn(67);`
        -10, // on "Show", reduce `Block = "Show", "\\n", FilterInstruction+ => ActionFn(67);`
        0, // on "\\n", error
        22, // on Constant, goto 21
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        23, // on VarIdentifier, goto 22
        // State 26
        -55, // on "(", reduce `Value = NumExpression => ActionFn(10);`
        0, // on ")", error
        0, // on "*", error
        53, // on "+", goto 52
        0, // on ",", error
        54, // on "-", goto 53
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -55, // on "\\n", reduce `Value = NumExpression => ActionFn(10);`
        -55, // on Constant, reduce `Value = NumExpression => ActionFn(10);`
        -55, // on Num, reduce `Value = NumExpression => ActionFn(10);`
        -55, // on QuotedStrLiteral, reduce `Value = NumExpression => ActionFn(10);`
        -55, // on VarIdentifier, reduce `Value = NumExpression => ActionFn(10);`
        // State 27
        -46, // on "(", reduce `NumExpression = NumFactor => ActionFn(24);`
        0, // on ")", error
        55, // on "*", goto 54
        -46, // on "+", reduce `NumExpression = NumFactor => ActionFn(24);`
        0, // on ",", error
        -46, // on "-", reduce `NumExpression = NumFactor => ActionFn(24);`
        56, // on "/", goto 55
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -46, // on "\\n", reduce `NumExpression = NumFactor => ActionFn(24);`
        -46, // on Constant, reduce `NumExpression = NumFactor => ActionFn(24);`
        -46, // on Num, reduce `NumExpression = NumFactor => ActionFn(24);`
        -46, // on QuotedStrLiteral, reduce `NumExpression = NumFactor => ActionFn(24);`
        -46, // on VarIdentifier, reduce `NumExpression = NumFactor => ActionFn(24);`
        // State 28
        -49, // on "(", reduce `NumFactor = NumTerm => ActionFn(27);`
        0, // on ")", error
        -49, // on "*", reduce `NumFactor = NumTerm => ActionFn(27);`
        -49, // on "+", reduce `NumFactor = NumTerm => ActionFn(27);`
        0, // on ",", error
        -49, // on "-", reduce `NumFactor = NumTerm => ActionFn(27);`
        -49, // on "/", reduce `NumFactor = NumTerm => ActionFn(27);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -49, // on "\\n", reduce `NumFactor = NumTerm => ActionFn(27);`
        -49, // on Constant, reduce `NumFactor = NumTerm => ActionFn(27);`
        -49, // on Num, reduce `NumFactor = NumTerm => ActionFn(27);`
        -49, // on QuotedStrLiteral, reduce `NumFactor = NumTerm => ActionFn(27);`
        -49, // on VarIdentifier, reduce `NumFactor = NumTerm => ActionFn(27);`
        // State 29
        -54, // on "(", reduce `StringExpression = StrLiteral => ActionFn(13);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -54, // on "\\n", reduce `StringExpression = StrLiteral => ActionFn(13);`
        -54, // on Constant, reduce `StringExpression = StrLiteral => ActionFn(13);`
        -54, // on Num, reduce `StringExpression = StrLiteral => ActionFn(13);`
        -54, // on QuotedStrLiteral, reduce `StringExpression = StrLiteral => ActionFn(13);`
        -54, // on VarIdentifier, reduce `StringExpression = StrLiteral => ActionFn(13);`
        // State 30
        -56, // on "(", reduce `Value = StringExpression => ActionFn(11);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -56, // on "\\n", reduce `Value = StringExpression => ActionFn(11);`
        -56, // on Constant, reduce `Value = StringExpression => ActionFn(11);`
        -56, // on Num, reduce `Value = StringExpression => ActionFn(11);`
        -56, // on QuotedStrLiteral, reduce `Value = StringExpression => ActionFn(11);`
        -56, // on VarIdentifier, reduce `Value = StringExpression => ActionFn(11);`
        // State 31
        -58, // on "(", reduce `Value+ = Value => ActionFn(32);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -58, // on "\\n", reduce `Value+ = Value => ActionFn(32);`
        -58, // on Constant, reduce `Value+ = Value => ActionFn(32);`
        -58, // on Num, reduce `Value+ = Value => ActionFn(32);`
        -58, // on QuotedStrLiteral, reduce `Value+ = Value => ActionFn(32);`
        -58, // on VarIdentifier, reduce `Value+ = Value => ActionFn(32);`
        // State 32
        34, // on "(", goto 33
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        58, // on "\\n", goto 57
        35, // on Constant, goto 34
        36, // on Num, goto 35
        37, // on QuotedStrLiteral, goto 36
        38, // on VarIdentifier, goto 37
        // State 33
        62, // on "(", goto 61
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        63, // on Num, goto 62
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 34
        -53, // on "(", reduce `StrLiteral = Constant => ActionFn(15);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -53, // on "\\n", reduce `StrLiteral = Constant => ActionFn(15);`
        -53, // on Constant, reduce `StrLiteral = Constant => ActionFn(15);`
        -53, // on Num, reduce `StrLiteral = Constant => ActionFn(15);`
        -53, // on QuotedStrLiteral, reduce `StrLiteral = Constant => ActionFn(15);`
        -53, // on VarIdentifier, reduce `StrLiteral = Constant => ActionFn(15);`
        // State 35
        -50, // on "(", reduce `NumTerm = Num => ActionFn(28);`
        0, // on ")", error
        -50, // on "*", reduce `NumTerm = Num => ActionFn(28);`
        -50, // on "+", reduce `NumTerm = Num => ActionFn(28);`
        0, // on ",", error
        -50, // on "-", reduce `NumTerm = Num => ActionFn(28);`
        -50, // on "/", reduce `NumTerm = Num => ActionFn(28);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -50, // on "\\n", reduce `NumTerm = Num => ActionFn(28);`
        -50, // on Constant, reduce `NumTerm = Num => ActionFn(28);`
        -50, // on Num, reduce `NumTerm = Num => ActionFn(28);`
        -50, // on QuotedStrLiteral, reduce `NumTerm = Num => ActionFn(28);`
        -50, // on VarIdentifier, reduce `NumTerm = Num => ActionFn(28);`
        // State 36
        -52, // on "(", reduce `StrLiteral = QuotedStrLiteral => ActionFn(14);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -52, // on "\\n", reduce `StrLiteral = QuotedStrLiteral => ActionFn(14);`
        -52, // on Constant, reduce `StrLiteral = QuotedStrLiteral => ActionFn(14);`
        -52, // on Num, reduce `StrLiteral = QuotedStrLiteral => ActionFn(14);`
        -52, // on QuotedStrLiteral, reduce `StrLiteral = QuotedStrLiteral => ActionFn(14);`
        -52, // on VarIdentifier, reduce `StrLiteral = QuotedStrLiteral => ActionFn(14);`
        // State 37
        -57, // on "(", reduce `Value = VarIdentifier => ActionFn(12);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -57, // on "\\n", reduce `Value = VarIdentifier => ActionFn(12);`
        -57, // on Constant, reduce `Value = VarIdentifier => ActionFn(12);`
        -57, // on Num, reduce `Value = VarIdentifier => ActionFn(12);`
        -57, // on QuotedStrLiteral, reduce `Value = VarIdentifier => ActionFn(12);`
        -57, // on VarIdentifier, reduce `Value = VarIdentifier => ActionFn(12);`
        // State 38
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -43, // on "Hide", reduce `FilterInstruction+ = FilterInstruction+, FilterInstruction => ActionFn(49);`
        0, // on "Import", error
        -43, // on "Mixin", reduce `FilterInstruction+ = FilterInstruction+, FilterInstruction => ActionFn(49);`
        -43, // on "Show", reduce `FilterInstruction+ = FilterInstruction+, FilterInstruction => ActionFn(49);`
        0, // on "\\n", error
        -43, // on Constant, reduce `FilterInstruction+ = FilterInstruction+, FilterInstruction => ActionFn(49);`
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        -43, // on VarIdentifier, reduce `FilterInstruction+ = FilterInstruction+, FilterInstruction => ActionFn(49);`
        // State 39
        70, // on "(", goto 69
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        71, // on Constant, goto 70
        72, // on Num, goto 71
        73, // on QuotedStrLiteral, goto 72
        74, // on VarIdentifier, goto 73
        // State 40
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        75, // on "\\n", goto 74
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 41
        34, // on "(", goto 33
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        76, // on "\\n", goto 75
        35, // on Constant, goto 34
        36, // on Num, goto 35
        37, // on QuotedStrLiteral, goto 36
        38, // on VarIdentifier, goto 37
        // State 42
        -30, // on "(", reduce `ComparisonOperator = "<" => ActionFn(20);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        -30, // on Constant, reduce `ComparisonOperator = "<" => ActionFn(20);`
        -30, // on Num, reduce `ComparisonOperator = "<" => ActionFn(20);`
        -30, // on QuotedStrLiteral, reduce `ComparisonOperator = "<" => ActionFn(20);`
        -30, // on VarIdentifier, reduce `ComparisonOperator = "<" => ActionFn(20);`
        // State 43
        -29, // on "(", reduce `ComparisonOperator = "<=" => ActionFn(19);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        -29, // on Constant, reduce `ComparisonOperator = "<=" => ActionFn(19);`
        -29, // on Num, reduce `ComparisonOperator = "<=" => ActionFn(19);`
        -29, // on QuotedStrLiteral, reduce `ComparisonOperator = "<=" => ActionFn(19);`
        -29, // on VarIdentifier, reduce `ComparisonOperator = "<=" => ActionFn(19);`
        // State 44
        -31, // on "(", reduce `ComparisonOperator = "=" => ActionFn(21);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        -31, // on Constant, reduce `ComparisonOperator = "=" => ActionFn(21);`
        -31, // on Num, reduce `ComparisonOperator = "=" => ActionFn(21);`
        -31, // on QuotedStrLiteral, reduce `ComparisonOperator = "=" => ActionFn(21);`
        -31, // on VarIdentifier, reduce `ComparisonOperator = "=" => ActionFn(21);`
        // State 45
        -28, // on "(", reduce `ComparisonOperator = ">" => ActionFn(18);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        -28, // on Constant, reduce `ComparisonOperator = ">" => ActionFn(18);`
        -28, // on Num, reduce `ComparisonOperator = ">" => ActionFn(18);`
        -28, // on QuotedStrLiteral, reduce `ComparisonOperator = ">" => ActionFn(18);`
        -28, // on VarIdentifier, reduce `ComparisonOperator = ">" => ActionFn(18);`
        // State 46
        -27, // on "(", reduce `ComparisonOperator = ">=" => ActionFn(17);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        -27, // on Constant, reduce `ComparisonOperator = ">=" => ActionFn(17);`
        -27, // on Num, reduce `ComparisonOperator = ">=" => ActionFn(17);`
        -27, // on QuotedStrLiteral, reduce `ComparisonOperator = ">=" => ActionFn(17);`
        -27, // on VarIdentifier, reduce `ComparisonOperator = ">=" => ActionFn(17);`
        // State 47
        34, // on "(", goto 33
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        35, // on Constant, goto 34
        36, // on Num, goto 35
        37, // on QuotedStrLiteral, goto 36
        38, // on VarIdentifier, goto 37
        // State 48
        0, // on "(", error
        -26, // on ")", reduce `Comma<VarIdentifier> = (<VarIdentifier> ",")+ => ActionFn(81);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        78, // on VarIdentifier, goto 77
        // State 49
        0, // on "(", error
        79, // on ")", goto 78
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 50
        0, // on "(", error
        -23, // on ")", reduce `Comma<VarIdentifier> = VarIdentifier => ActionFn(78);`
        0, // on "*", error
        0, // on "+", error
        80, // on ",", goto 79
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 51
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -16, // on "Hide", reduce `Block = "Mixin", Constant, "\\n", FilterInstruction+ => ActionFn(73);`
        0, // on "Import", error
        -16, // on "Mixin", reduce `Block = "Mixin", Constant, "\\n", FilterInstruction+ => ActionFn(73);`
        -16, // on "Show", reduce `Block = "Mixin", Constant, "\\n", FilterInstruction+ => ActionFn(73);`
        0, // on "\\n", error
        22, // on Constant, goto 21
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        23, // on VarIdentifier, goto 22
        // State 52
        34, // on "(", goto 33
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        36, // on Num, goto 35
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 53
        34, // on "(", goto 33
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        36, // on Num, goto 35
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 54
        34, // on "(", goto 33
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        36, // on Num, goto 35
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 55
        34, // on "(", goto 33
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        36, // on Num, goto 35
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 56
        -59, // on "(", reduce `Value+ = Value+, Value => ActionFn(33);`
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -59, // on "\\n", reduce `Value+ = Value+, Value => ActionFn(33);`
        -59, // on Constant, reduce `Value+ = Value+, Value => ActionFn(33);`
        -59, // on Num, reduce `Value+ = Value+, Value => ActionFn(33);`
        -59, // on QuotedStrLiteral, reduce `Value+ = Value+, Value => ActionFn(33);`
        -59, // on VarIdentifier, reduce `Value+ = Value+, Value => ActionFn(33);`
        // State 57
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -60, // on "Hide", reduce `VarDefinition = VarIdentifier, "=", Value+, "\\n" => ActionFn(6);`
        0, // on "Import", error
        -60, // on "Mixin", reduce `VarDefinition = VarIdentifier, "=", Value+, "\\n" => ActionFn(6);`
        -60, // on "Show", reduce `VarDefinition = VarIdentifier, "=", Value+, "\\n" => ActionFn(6);`
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        -60, // on VarIdentifier, reduce `VarDefinition = VarIdentifier, "=", Value+, "\\n" => ActionFn(6);`
        // State 58
        0, // on "(", error
        85, // on ")", goto 84
        0, // on "*", error
        86, // on "+", goto 85
        0, // on ",", error
        87, // on "-", goto 86
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 59
        0, // on "(", error
        -46, // on ")", reduce `NumExpression = NumFactor => ActionFn(24);`
        88, // on "*", goto 87
        -46, // on "+", reduce `NumExpression = NumFactor => ActionFn(24);`
        0, // on ",", error
        -46, // on "-", reduce `NumExpression = NumFactor => ActionFn(24);`
        89, // on "/", goto 88
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 60
        0, // on "(", error
        -49, // on ")", reduce `NumFactor = NumTerm => ActionFn(27);`
        -49, // on "*", reduce `NumFactor = NumTerm => ActionFn(27);`
        -49, // on "+", reduce `NumFactor = NumTerm => ActionFn(27);`
        0, // on ",", error
        -49, // on "-", reduce `NumFactor = NumTerm => ActionFn(27);`
        -49, // on "/", reduce `NumFactor = NumTerm => ActionFn(27);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 61
        62, // on "(", goto 61
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        63, // on Num, goto 62
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 62
        0, // on "(", error
        -50, // on ")", reduce `NumTerm = Num => ActionFn(28);`
        -50, // on "*", reduce `NumTerm = Num => ActionFn(28);`
        -50, // on "+", reduce `NumTerm = Num => ActionFn(28);`
        0, // on ",", error
        -50, // on "-", reduce `NumTerm = Num => ActionFn(28);`
        -50, // on "/", reduce `NumTerm = Num => ActionFn(28);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 63
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        91, // on "+", goto 90
        0, // on ",", error
        92, // on "-", goto 91
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -55, // on "\\n", reduce `Value = NumExpression => ActionFn(10);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 64
        0, // on "(", error
        0, // on ")", error
        93, // on "*", goto 92
        -46, // on "+", reduce `NumExpression = NumFactor => ActionFn(24);`
        0, // on ",", error
        -46, // on "-", reduce `NumExpression = NumFactor => ActionFn(24);`
        94, // on "/", goto 93
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -46, // on "\\n", reduce `NumExpression = NumFactor => ActionFn(24);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 65
        0, // on "(", error
        0, // on ")", error
        -49, // on "*", reduce `NumFactor = NumTerm => ActionFn(27);`
        -49, // on "+", reduce `NumFactor = NumTerm => ActionFn(27);`
        0, // on ",", error
        -49, // on "-", reduce `NumFactor = NumTerm => ActionFn(27);`
        -49, // on "/", reduce `NumFactor = NumTerm => ActionFn(27);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -49, // on "\\n", reduce `NumFactor = NumTerm => ActionFn(27);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 66
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -54, // on "\\n", reduce `StringExpression = StrLiteral => ActionFn(13);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 67
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -56, // on "\\n", reduce `Value = StringExpression => ActionFn(11);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 68
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -32, // on "\\n", reduce `Condition = ComparisonOperator, Value => ActionFn(16);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 69
        62, // on "(", goto 61
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        63, // on Num, goto 62
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 70
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -53, // on "\\n", reduce `StrLiteral = Constant => ActionFn(15);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 71
        0, // on "(", error
        0, // on ")", error
        -50, // on "*", reduce `NumTerm = Num => ActionFn(28);`
        -50, // on "+", reduce `NumTerm = Num => ActionFn(28);`
        0, // on ",", error
        -50, // on "-", reduce `NumTerm = Num => ActionFn(28);`
        -50, // on "/", reduce `NumTerm = Num => ActionFn(28);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -50, // on "\\n", reduce `NumTerm = Num => ActionFn(28);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 72
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -52, // on "\\n", reduce `StrLiteral = QuotedStrLiteral => ActionFn(14);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 73
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -57, // on "\\n", reduce `Value = VarIdentifier => ActionFn(12);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 74
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -38, // on "Hide", reduce `FilterInstruction = Constant, Condition, "\\n" => ActionFn(8);`
        0, // on "Import", error
        -38, // on "Mixin", reduce `FilterInstruction = Constant, Condition, "\\n" => ActionFn(8);`
        -38, // on "Show", reduce `FilterInstruction = Constant, Condition, "\\n" => ActionFn(8);`
        0, // on "\\n", error
        -38, // on Constant, reduce `FilterInstruction = Constant, Condition, "\\n" => ActionFn(8);`
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        -38, // on VarIdentifier, reduce `FilterInstruction = Constant, Condition, "\\n" => ActionFn(8);`
        // State 75
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -37, // on "Hide", reduce `FilterInstruction = Constant, Value+, "\\n" => ActionFn(7);`
        0, // on "Import", error
        -37, // on "Mixin", reduce `FilterInstruction = Constant, Value+, "\\n" => ActionFn(7);`
        -37, // on "Show", reduce `FilterInstruction = Constant, Value+, "\\n" => ActionFn(7);`
        0, // on "\\n", error
        -37, // on Constant, reduce `FilterInstruction = Constant, Value+, "\\n" => ActionFn(7);`
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        -37, // on VarIdentifier, reduce `FilterInstruction = Constant, Value+, "\\n" => ActionFn(7);`
        // State 76
        34, // on "(", goto 33
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        96, // on "\\n", goto 95
        35, // on Constant, goto 34
        36, // on Num, goto 35
        37, // on QuotedStrLiteral, goto 36
        38, // on VarIdentifier, goto 37
        // State 77
        0, // on "(", error
        -25, // on ")", reduce `Comma<VarIdentifier> = (<VarIdentifier> ",")+, VarIdentifier => ActionFn(80);`
        0, // on "*", error
        0, // on "+", error
        97, // on ",", goto 96
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 78
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        98, // on "\\n", goto 97
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 79
        0, // on "(", error
        -7, // on ")", reduce `(<VarIdentifier> ",")+ = VarIdentifier, "," => ActionFn(60);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        -7, // on VarIdentifier, reduce `(<VarIdentifier> ",")+ = VarIdentifier, "," => ActionFn(60);`
        // State 80
        -44, // on "(", reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        0, // on ")", error
        55, // on "*", goto 54
        -44, // on "+", reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        0, // on ",", error
        -44, // on "-", reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        56, // on "/", goto 55
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -44, // on "\\n", reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        -44, // on Constant, reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        -44, // on Num, reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        -44, // on QuotedStrLiteral, reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        -44, // on VarIdentifier, reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        // State 81
        -45, // on "(", reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        0, // on ")", error
        55, // on "*", goto 54
        -45, // on "+", reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        0, // on ",", error
        -45, // on "-", reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        56, // on "/", goto 55
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -45, // on "\\n", reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        -45, // on Constant, reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        -45, // on Num, reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        -45, // on QuotedStrLiteral, reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        -45, // on VarIdentifier, reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        // State 82
        -47, // on "(", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        0, // on ")", error
        -47, // on "*", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        -47, // on "+", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        0, // on ",", error
        -47, // on "-", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        -47, // on "/", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -47, // on "\\n", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        -47, // on Constant, reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        -47, // on Num, reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        -47, // on QuotedStrLiteral, reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        -47, // on VarIdentifier, reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        // State 83
        -48, // on "(", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        0, // on ")", error
        -48, // on "*", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        -48, // on "+", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        0, // on ",", error
        -48, // on "-", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        -48, // on "/", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -48, // on "\\n", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        -48, // on Constant, reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        -48, // on Num, reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        -48, // on QuotedStrLiteral, reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        -48, // on VarIdentifier, reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        // State 84
        -51, // on "(", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        0, // on ")", error
        -51, // on "*", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        -51, // on "+", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        0, // on ",", error
        -51, // on "-", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        -51, // on "/", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -51, // on "\\n", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        -51, // on Constant, reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        -51, // on Num, reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        -51, // on QuotedStrLiteral, reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        -51, // on VarIdentifier, reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        // State 85
        62, // on "(", goto 61
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        63, // on Num, goto 62
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 86
        62, // on "(", goto 61
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        63, // on Num, goto 62
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 87
        62, // on "(", goto 61
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        63, // on Num, goto 62
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 88
        62, // on "(", goto 61
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        63, // on Num, goto 62
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 89
        0, // on "(", error
        103, // on ")", goto 102
        0, // on "*", error
        86, // on "+", goto 85
        0, // on ",", error
        87, // on "-", goto 86
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 90
        70, // on "(", goto 69
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        72, // on Num, goto 71
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 91
        70, // on "(", goto 69
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        72, // on Num, goto 71
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 92
        70, // on "(", goto 69
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        72, // on Num, goto 71
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 93
        70, // on "(", goto 69
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        72, // on Num, goto 71
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 94
        0, // on "(", error
        108, // on ")", goto 107
        0, // on "*", error
        86, // on "+", goto 85
        0, // on ",", error
        87, // on "-", goto 86
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 95
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -60, // on "Hide", reduce `VarDefinition = VarIdentifier, "=", Value+, "\\n" => ActionFn(6);`
        0, // on "Import", error
        -60, // on "Mixin", reduce `VarDefinition = VarIdentifier, "=", Value+, "\\n" => ActionFn(6);`
        -60, // on "Show", reduce `VarDefinition = VarIdentifier, "=", Value+, "\\n" => ActionFn(6);`
        0, // on "\\n", error
        -60, // on Constant, reduce `VarDefinition = VarIdentifier, "=", Value+, "\\n" => ActionFn(6);`
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        -60, // on VarIdentifier, reduce `VarDefinition = VarIdentifier, "=", Value+, "\\n" => ActionFn(6);`
        // State 96
        0, // on "(", error
        -8, // on ")", reduce `(<VarIdentifier> ",")+ = (<VarIdentifier> ",")+, VarIdentifier, "," => ActionFn(61);`
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        -8, // on VarIdentifier, reduce `(<VarIdentifier> ",")+ = (<VarIdentifier> ",")+, VarIdentifier, "," => ActionFn(61);`
        // State 97
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -13, // on "Hide", reduce `Block = "Mixin", Constant, "(", Comma<VarIdentifier>, ")", "\\n" => ActionFn(70);`
        0, // on "Import", error
        -13, // on "Mixin", reduce `Block = "Mixin", Constant, "(", Comma<VarIdentifier>, ")", "\\n" => ActionFn(70);`
        -13, // on "Show", reduce `Block = "Mixin", Constant, "(", Comma<VarIdentifier>, ")", "\\n" => ActionFn(70);`
        0, // on "\\n", error
        22, // on Constant, goto 21
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        23, // on VarIdentifier, goto 22
        // State 98
        0, // on "(", error
        -44, // on ")", reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        88, // on "*", goto 87
        -44, // on "+", reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        0, // on ",", error
        -44, // on "-", reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        89, // on "/", goto 88
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 99
        0, // on "(", error
        -45, // on ")", reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        88, // on "*", goto 87
        -45, // on "+", reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        0, // on ",", error
        -45, // on "-", reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        89, // on "/", goto 88
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 100
        0, // on "(", error
        -47, // on ")", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        -47, // on "*", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        -47, // on "+", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        0, // on ",", error
        -47, // on "-", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        -47, // on "/", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 101
        0, // on "(", error
        -48, // on ")", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        -48, // on "*", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        -48, // on "+", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        0, // on ",", error
        -48, // on "-", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        -48, // on "/", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 102
        0, // on "(", error
        -51, // on ")", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        -51, // on "*", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        -51, // on "+", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        0, // on ",", error
        -51, // on "-", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        -51, // on "/", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        0, // on "\\n", error
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 103
        0, // on "(", error
        0, // on ")", error
        93, // on "*", goto 92
        -44, // on "+", reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        0, // on ",", error
        -44, // on "-", reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        94, // on "/", goto 93
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -44, // on "\\n", reduce `NumExpression = NumExpression, "+", NumFactor => ActionFn(22);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 104
        0, // on "(", error
        0, // on ")", error
        93, // on "*", goto 92
        -45, // on "+", reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        0, // on ",", error
        -45, // on "-", reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        94, // on "/", goto 93
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -45, // on "\\n", reduce `NumExpression = NumExpression, "-", NumFactor => ActionFn(23);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 105
        0, // on "(", error
        0, // on ")", error
        -47, // on "*", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        -47, // on "+", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        0, // on ",", error
        -47, // on "-", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        -47, // on "/", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -47, // on "\\n", reduce `NumFactor = NumFactor, "*", NumTerm => ActionFn(25);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 106
        0, // on "(", error
        0, // on ")", error
        -48, // on "*", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        -48, // on "+", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        0, // on ",", error
        -48, // on "-", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        -48, // on "/", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -48, // on "\\n", reduce `NumFactor = NumFactor, "/", NumTerm => ActionFn(26);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 107
        0, // on "(", error
        0, // on ")", error
        -51, // on "*", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        -51, // on "+", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        0, // on ",", error
        -51, // on "-", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        -51, // on "/", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        0, // on "Hide", error
        0, // on "Import", error
        0, // on "Mixin", error
        0, // on "Show", error
        -51, // on "\\n", reduce `NumTerm = "(", NumExpression, ")" => ActionFn(29);`
        0, // on Constant, error
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        0, // on VarIdentifier, error
        // State 108
        0, // on "(", error
        0, // on ")", error
        0, // on "*", error
        0, // on "+", error
        0, // on ",", error
        0, // on "-", error
        0, // on "/", error
        0, // on "<", error
        0, // on "<=", error
        0, // on "=", error
        0, // on ">", error
        0, // on ">=", error
        -14, // on "Hide", reduce `Block = "Mixin", Constant, "(", Comma<VarIdentifier>, ")", "\\n", FilterInstruction+ => ActionFn(71);`
        0, // on "Import", error
        -14, // on "Mixin", reduce `Block = "Mixin", Constant, "(", Comma<VarIdentifier>, ")", "\\n", FilterInstruction+ => ActionFn(71);`
        -14, // on "Show", reduce `Block = "Mixin", Constant, "(", Comma<VarIdentifier>, ")", "\\n", FilterInstruction+ => ActionFn(71);`
        0, // on "\\n", error
        22, // on Constant, goto 21
        0, // on Num, error
        0, // on QuotedStrLiteral, error
        23, // on VarIdentifier, goto 22
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -34, // on EOF, reduce `Filter =  => ActionFn(75);`
        -19, // on EOF, reduce `Block+ = Block => ActionFn(46);`
        -36, // on EOF, reduce `Filter = Block+ => ActionFn(77);`
        -68, // on EOF, reduce `__Filter = Filter => ActionFn(0);`
        -61, // on EOF, reduce `VarDefinition+ = VarDefinition => ActionFn(40);`
        -63, // on EOF, reduce `VarDefinitionBlock = VarDefinition+ => ActionFn(2);`
        -33, // on EOF, reduce `Filter = VarDefinitionBlock => ActionFn(74);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -20, // on EOF, reduce `Block+ = Block+, Block => ActionFn(47);`
        -62, // on EOF, reduce `VarDefinition+ = VarDefinition+, VarDefinition => ActionFn(41);`
        -35, // on EOF, reduce `Filter = VarDefinitionBlock, Block+ => ActionFn(76);`
        -11, // on EOF, reduce `Block = "Hide", "\\n" => ActionFn(68);`
        0, // on EOF, error
        -9, // on EOF, reduce `Block = "Show", "\\n" => ActionFn(66);`
        0, // on EOF, error
        -42, // on EOF, reduce `FilterInstruction+ = FilterInstruction => ActionFn(48);`
        -12, // on EOF, reduce `Block = "Hide", "\\n", FilterInstruction+ => ActionFn(69);`
        -39, // on EOF, reduce `FilterInstruction = VarDefinition => ActionFn(9);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -15, // on EOF, reduce `Block = "Mixin", Constant, "\\n" => ActionFn(72);`
        -10, // on EOF, reduce `Block = "Show", "\\n", FilterInstruction+ => ActionFn(67);`
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
        -43, // on EOF, reduce `FilterInstruction+ = FilterInstruction+, FilterInstruction => ActionFn(49);`
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
        -16, // on EOF, reduce `Block = "Mixin", Constant, "\\n", FilterInstruction+ => ActionFn(73);`
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -60, // on EOF, reduce `VarDefinition = VarIdentifier, "=", Value+, "\\n" => ActionFn(6);`
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
        -38, // on EOF, reduce `FilterInstruction = Constant, Condition, "\\n" => ActionFn(8);`
        -37, // on EOF, reduce `FilterInstruction = Constant, Value+, "\\n" => ActionFn(7);`
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
        0, // on EOF, error
        0, // on EOF, error
        0, // on EOF, error
        -60, // on EOF, reduce `VarDefinition = VarIdentifier, "=", Value+, "\\n" => ActionFn(6);`
        0, // on EOF, error
        -13, // on EOF, reduce `Block = "Mixin", Constant, "(", Comma<VarIdentifier>, ")", "\\n" => ActionFn(70);`
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
        -14, // on EOF, reduce `Block = "Mixin", Constant, "(", Comma<VarIdentifier>, ")", "\\n", FilterInstruction+ => ActionFn(71);`
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        2, // on Block, goto 1
        0, // on Block*, error
        3, // on Block+, goto 2
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        4, // on Filter, goto 3
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        5, // on VarDefinition, goto 4
        6, // on VarDefinition+, goto 5
        7, // on VarDefinitionBlock, goto 6
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 1
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 2
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        12, // on Block, goto 11
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 3
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 4
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 5
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        13, // on VarDefinition, goto 12
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 6
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        2, // on Block, goto 1
        0, // on Block*, error
        14, // on Block+, goto 13
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 7
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 8
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 9
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 10
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 11
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 12
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 13
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        12, // on Block, goto 11
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 14
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        19, // on FilterInstruction, goto 18
        0, // on FilterInstruction*, error
        20, // on FilterInstruction+, goto 19
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        21, // on VarDefinition, goto 20
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 15
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 16
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        19, // on FilterInstruction, goto 18
        0, // on FilterInstruction*, error
        26, // on FilterInstruction+, goto 25
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        21, // on VarDefinition, goto 20
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 17
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        27, // on NumExpression, goto 26
        28, // on NumFactor, goto 27
        29, // on NumTerm, goto 28
        30, // on StrLiteral, goto 29
        31, // on StringExpression, goto 30
        32, // on Value, goto 31
        33, // on Value+, goto 32
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 18
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 19
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        39, // on FilterInstruction, goto 38
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        21, // on VarDefinition, goto 20
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 20
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 21
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        40, // on ComparisonOperator, goto 39
        41, // on Condition, goto 40
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        27, // on NumExpression, goto 26
        28, // on NumFactor, goto 27
        29, // on NumTerm, goto 28
        30, // on StrLiteral, goto 29
        31, // on StringExpression, goto 30
        32, // on Value, goto 31
        42, // on Value+, goto 41
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 22
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 23
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        49, // on (<VarIdentifier> ",")+, goto 48
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        50, // on Comma<VarIdentifier>, goto 49
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 24
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        19, // on FilterInstruction, goto 18
        0, // on FilterInstruction*, error
        52, // on FilterInstruction+, goto 51
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        21, // on VarDefinition, goto 20
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 25
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        39, // on FilterInstruction, goto 38
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        21, // on VarDefinition, goto 20
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 26
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 27
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 28
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 29
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 30
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 31
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 32
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        27, // on NumExpression, goto 26
        28, // on NumFactor, goto 27
        29, // on NumTerm, goto 28
        30, // on StrLiteral, goto 29
        31, // on StringExpression, goto 30
        57, // on Value, goto 56
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 33
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        59, // on NumExpression, goto 58
        60, // on NumFactor, goto 59
        61, // on NumTerm, goto 60
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 34
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 35
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 36
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 37
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 38
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 39
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        64, // on NumExpression, goto 63
        65, // on NumFactor, goto 64
        66, // on NumTerm, goto 65
        67, // on StrLiteral, goto 66
        68, // on StringExpression, goto 67
        69, // on Value, goto 68
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 40
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 41
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        27, // on NumExpression, goto 26
        28, // on NumFactor, goto 27
        29, // on NumTerm, goto 28
        30, // on StrLiteral, goto 29
        31, // on StringExpression, goto 30
        57, // on Value, goto 56
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 42
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 43
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 44
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 45
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 46
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 47
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        27, // on NumExpression, goto 26
        28, // on NumFactor, goto 27
        29, // on NumTerm, goto 28
        30, // on StrLiteral, goto 29
        31, // on StringExpression, goto 30
        32, // on Value, goto 31
        77, // on Value+, goto 76
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 48
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 49
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 50
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 51
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        39, // on FilterInstruction, goto 38
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        21, // on VarDefinition, goto 20
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 52
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        81, // on NumFactor, goto 80
        29, // on NumTerm, goto 28
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 53
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        82, // on NumFactor, goto 81
        29, // on NumTerm, goto 28
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 54
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        83, // on NumTerm, goto 82
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 55
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        84, // on NumTerm, goto 83
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 56
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 57
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 58
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 59
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 60
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 61
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        90, // on NumExpression, goto 89
        60, // on NumFactor, goto 59
        61, // on NumTerm, goto 60
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 62
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 63
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 64
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 65
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 66
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 67
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 68
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 69
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        95, // on NumExpression, goto 94
        60, // on NumFactor, goto 59
        61, // on NumTerm, goto 60
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 70
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 71
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 72
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 73
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 74
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 75
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 76
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        27, // on NumExpression, goto 26
        28, // on NumFactor, goto 27
        29, // on NumTerm, goto 28
        30, // on StrLiteral, goto 29
        31, // on StringExpression, goto 30
        57, // on Value, goto 56
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 77
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 78
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 79
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 80
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 81
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 82
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 83
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 84
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 85
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        99, // on NumFactor, goto 98
        61, // on NumTerm, goto 60
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 86
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        100, // on NumFactor, goto 99
        61, // on NumTerm, goto 60
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 87
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        101, // on NumTerm, goto 100
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 88
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        102, // on NumTerm, goto 101
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 89
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 90
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        104, // on NumFactor, goto 103
        66, // on NumTerm, goto 65
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 91
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        105, // on NumFactor, goto 104
        66, // on NumTerm, goto 65
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 92
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        106, // on NumTerm, goto 105
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 93
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        107, // on NumTerm, goto 106
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 94
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 95
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 96
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 97
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        19, // on FilterInstruction, goto 18
        0, // on FilterInstruction*, error
        109, // on FilterInstruction+, goto 108
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        21, // on VarDefinition, goto 20
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 98
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 99
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 100
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 101
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 102
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 103
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 104
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 105
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 106
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 107
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        0, // on FilterInstruction, error
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        0, // on VarDefinition, error
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
        0, // on __Filter, error
        // State 108
        0, // on ("(" <Comma<VarIdentifier>> ")"), error
        0, // on ("(" <Comma<VarIdentifier>> ")")?, error
        0, // on (<VarIdentifier> ","), error
        0, // on (<VarIdentifier> ",")*, error
        0, // on (<VarIdentifier> ",")+, error
        0, // on Block, error
        0, // on Block*, error
        0, // on Block+, error
        0, // on Color, error
        0, // on Comma<VarIdentifier>, error
        0, // on ComparisonOperator, error
        0, // on Condition, error
        0, // on Filter, error
        39, // on FilterInstruction, goto 38
        0, // on FilterInstruction*, error
        0, // on FilterInstruction+, error
        0, // on NumExpression, error
        0, // on NumFactor, error
        0, // on NumTerm, error
        0, // on StrLiteral, error
        0, // on StringExpression, error
        0, // on Value, error
        0, // on Value+, error
        21, // on VarDefinition, goto 20
        0, // on VarDefinition+, error
        0, // on VarDefinitionBlock, error
        0, // on VarDefinitionBlock?, error
        0, // on VarIdentifier?, error
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
                (_, Tok::Comma, _) if true => 4,
                (_, Tok::Minus, _) if true => 5,
                (_, Tok::Div, _) if true => 6,
                (_, Tok::Lt, _) if true => 7,
                (_, Tok::Lte, _) if true => 8,
                (_, Tok::Eql, _) if true => 9,
                (_, Tok::Gt, _) if true => 10,
                (_, Tok::Gte, _) if true => 11,
                (_, Tok::Hide, _) if true => 12,
                (_, Tok::Import, _) if true => 13,
                (_, Tok::Mixin, _) if true => 14,
                (_, Tok::Show, _) if true => 15,
                (_, Tok::NewLine, _) if true => 16,
                (_, Tok::Constant(_), _) if true => 17,
                (_, Tok::Num(_), _) if true => 18,
                (_, Tok::StrLiteral(_), _) if true => 19,
                (_, Tok::VarIdentifier(_), _) if true => 20,
                _ => {
                    return Err(__lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: vec![],
                    });
                }
            };
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 21 + __integer];
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
                            __tok @ Tok::Comma => __Symbol::Term_22_2c_22(__tok),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            __tok @ Tok::Minus => __Symbol::Term_22_2d_22(__tok),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            __tok @ Tok::Div => __Symbol::Term_22_2f_22(__tok),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            __tok @ Tok::Lt => __Symbol::Term_22_3c_22(__tok),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            __tok @ Tok::Lte => __Symbol::Term_22_3c_3d_22(__tok),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            __tok @ Tok::Eql => __Symbol::Term_22_3d_22(__tok),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            __tok @ Tok::Gt => __Symbol::Term_22_3e_22(__tok),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            __tok @ Tok::Gte => __Symbol::Term_22_3e_3d_22(__tok),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            __tok @ Tok::Hide => __Symbol::Term_22Hide_22(__tok),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            __tok @ Tok::Import => __Symbol::Term_22Import_22(__tok),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            __tok @ Tok::Mixin => __Symbol::Term_22Mixin_22(__tok),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            __tok @ Tok::Show => __Symbol::Term_22Show_22(__tok),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            __tok @ Tok::NewLine => __Symbol::Term_22_5c_5cn_22(__tok),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            Tok::Constant(__tok0) => __Symbol::TermConstant(__tok0),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            Tok::Num(__tok0) => __Symbol::TermNum(__tok0),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            Tok::StrLiteral(__tok0) => __Symbol::TermQuotedStrLiteral(__tok0),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            Tok::VarIdentifier(__tok0) => __Symbol::TermVarIdentifier(__tok0),
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
                // ("(" <Comma<VarIdentifier>> ")") = "(", Comma<VarIdentifier>, ")" => ActionFn(36);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtComma_3cVarIdentifier_3e(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action36(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22_28_22_20_3cComma_3cVarIdentifier_3e_3e_20_22_29_22_29(__nt), __end));
                0
            }
            2 => {
                // ("(" <Comma<VarIdentifier>> ")")? = "(", Comma<VarIdentifier>, ")" => ActionFn(57);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtComma_3cVarIdentifier_3e(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action57(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_22_28_22_20_3cComma_3cVarIdentifier_3e_3e_20_22_29_22_29_3f(__nt), __end));
                1
            }
            3 => {
                // ("(" <Comma<VarIdentifier>> ")")? =  => ActionFn(35);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action35(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22_28_22_20_3cComma_3cVarIdentifier_3e_3e_20_22_29_22_29_3f(__nt), __end));
                1
            }
            4 => {
                // (<VarIdentifier> ",") = VarIdentifier, "," => ActionFn(54);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_TermVarIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cVarIdentifier_3e_20_22_2c_22_29(__nt), __end));
                2
            }
            5 => {
                // (<VarIdentifier> ",")* =  => ActionFn(52);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action52(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2a(__nt), __end));
                3
            }
            6 => {
                // (<VarIdentifier> ",")* = (<VarIdentifier> ",")+ => ActionFn(53);
                let __sym0 = __pop_Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2a(__nt), __end));
                3
            }
            7 => {
                // (<VarIdentifier> ",")+ = VarIdentifier, "," => ActionFn(60);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_TermVarIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action60(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2b(__nt), __end));
                4
            }
            8 => {
                // (<VarIdentifier> ",")+ = (<VarIdentifier> ",")+, VarIdentifier, "," => ActionFn(61);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_TermVarIdentifier(__symbols);
                let __sym0 = __pop_Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action61(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2b(__nt), __end));
                4
            }
            9 => {
                // Block = "Show", "\\n" => ActionFn(66);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Term_22Show_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action66(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                5
            }
            10 => {
                // Block = "Show", "\\n", FilterInstruction+ => ActionFn(67);
                let __sym2 = __pop_NtFilterInstruction_2b(__symbols);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Term_22Show_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action67(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                5
            }
            11 => {
                // Block = "Hide", "\\n" => ActionFn(68);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Term_22Hide_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action68(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                5
            }
            12 => {
                // Block = "Hide", "\\n", FilterInstruction+ => ActionFn(69);
                let __sym2 = __pop_NtFilterInstruction_2b(__symbols);
                let __sym1 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym0 = __pop_Term_22Hide_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action69(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                5
            }
            13 => {
                // Block = "Mixin", Constant, "(", Comma<VarIdentifier>, ")", "\\n" => ActionFn(70);
                let __sym5 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtComma_3cVarIdentifier_3e(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_TermConstant(__symbols);
                let __sym0 = __pop_Term_22Mixin_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action70(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                5
            }
            14 => {
                // Block = "Mixin", Constant, "(", Comma<VarIdentifier>, ")", "\\n", FilterInstruction+ => ActionFn(71);
                let __sym6 = __pop_NtFilterInstruction_2b(__symbols);
                let __sym5 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtComma_3cVarIdentifier_3e(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_TermConstant(__symbols);
                let __sym0 = __pop_Term_22Mixin_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym6.2.clone();
                let __nt = super::__action71(__sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6);
                let __states_len = __states.len();
                __states.truncate(__states_len - 7);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                5
            }
            15 => {
                // Block = "Mixin", Constant, "\\n" => ActionFn(72);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_TermConstant(__symbols);
                let __sym0 = __pop_Term_22Mixin_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action72(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                5
            }
            16 => {
                // Block = "Mixin", Constant, "\\n", FilterInstruction+ => ActionFn(73);
                let __sym3 = __pop_NtFilterInstruction_2b(__symbols);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_TermConstant(__symbols);
                let __sym0 = __pop_Term_22Mixin_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action73(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                5
            }
            17 => {
                // Block* =  => ActionFn(42);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action42(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtBlock_2a(__nt), __end));
                6
            }
            18 => {
                // Block* = Block+ => ActionFn(43);
                let __sym0 = __pop_NtBlock_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action43(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBlock_2a(__nt), __end));
                6
            }
            19 => {
                // Block+ = Block => ActionFn(46);
                let __sym0 = __pop_NtBlock(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action46(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBlock_2b(__nt), __end));
                7
            }
            20 => {
                // Block+ = Block+, Block => ActionFn(47);
                let __sym1 = __pop_NtBlock(__symbols);
                let __sym0 = __pop_NtBlock_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action47(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtBlock_2b(__nt), __end));
                7
            }
            21 => {
                // Color = NumExpression, NumExpression, NumExpression, NumExpression => ActionFn(30);
                let __sym3 = __pop_NtNumExpression(__symbols);
                let __sym2 = __pop_NtNumExpression(__symbols);
                let __sym1 = __pop_NtNumExpression(__symbols);
                let __sym0 = __pop_NtNumExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action30(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtColor(__nt), __end));
                8
            }
            22 => {
                // Color = NumExpression, NumExpression, NumExpression => ActionFn(31);
                let __sym2 = __pop_NtNumExpression(__symbols);
                let __sym1 = __pop_NtNumExpression(__symbols);
                let __sym0 = __pop_NtNumExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action31(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtColor(__nt), __end));
                8
            }
            23 => {
                // Comma<VarIdentifier> = VarIdentifier => ActionFn(78);
                let __sym0 = __pop_TermVarIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action78(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cVarIdentifier_3e(__nt), __end));
                9
            }
            24 => {
                // Comma<VarIdentifier> =  => ActionFn(79);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action79(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cVarIdentifier_3e(__nt), __end));
                9
            }
            25 => {
                // Comma<VarIdentifier> = (<VarIdentifier> ",")+, VarIdentifier => ActionFn(80);
                let __sym1 = __pop_TermVarIdentifier(__symbols);
                let __sym0 = __pop_Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action80(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cVarIdentifier_3e(__nt), __end));
                9
            }
            26 => {
                // Comma<VarIdentifier> = (<VarIdentifier> ",")+ => ActionFn(81);
                let __sym0 = __pop_Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action81(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cVarIdentifier_3e(__nt), __end));
                9
            }
            27 => {
                // ComparisonOperator = ">=" => ActionFn(17);
                let __sym0 = __pop_Term_22_3e_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                10
            }
            28 => {
                // ComparisonOperator = ">" => ActionFn(18);
                let __sym0 = __pop_Term_22_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                10
            }
            29 => {
                // ComparisonOperator = "<=" => ActionFn(19);
                let __sym0 = __pop_Term_22_3c_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                10
            }
            30 => {
                // ComparisonOperator = "<" => ActionFn(20);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                10
            }
            31 => {
                // ComparisonOperator = "=" => ActionFn(21);
                let __sym0 = __pop_Term_22_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOperator(__nt), __end));
                10
            }
            32 => {
                // Condition = ComparisonOperator, Value => ActionFn(16);
                let __sym1 = __pop_NtValue(__symbols);
                let __sym0 = __pop_NtComparisonOperator(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action16(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtCondition(__nt), __end));
                11
            }
            33 => {
                // Filter = VarDefinitionBlock => ActionFn(74);
                let __sym0 = __pop_NtVarDefinitionBlock(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action74(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFilter(__nt), __end));
                12
            }
            34 => {
                // Filter =  => ActionFn(75);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action75(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtFilter(__nt), __end));
                12
            }
            35 => {
                // Filter = VarDefinitionBlock, Block+ => ActionFn(76);
                let __sym1 = __pop_NtBlock_2b(__symbols);
                let __sym0 = __pop_NtVarDefinitionBlock(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action76(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtFilter(__nt), __end));
                12
            }
            36 => {
                // Filter = Block+ => ActionFn(77);
                let __sym0 = __pop_NtBlock_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action77(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFilter(__nt), __end));
                12
            }
            37 => {
                // FilterInstruction = Constant, Value+, "\\n" => ActionFn(7);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_NtValue_2b(__symbols);
                let __sym0 = __pop_TermConstant(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFilterInstruction(__nt), __end));
                13
            }
            38 => {
                // FilterInstruction = Constant, Condition, "\\n" => ActionFn(8);
                let __sym2 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym1 = __pop_NtCondition(__symbols);
                let __sym0 = __pop_TermConstant(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtFilterInstruction(__nt), __end));
                13
            }
            39 => {
                // FilterInstruction = VarDefinition => ActionFn(9);
                let __sym0 = __pop_NtVarDefinition(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFilterInstruction(__nt), __end));
                13
            }
            40 => {
                // FilterInstruction* =  => ActionFn(38);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action38(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtFilterInstruction_2a(__nt), __end));
                14
            }
            41 => {
                // FilterInstruction* = FilterInstruction+ => ActionFn(39);
                let __sym0 = __pop_NtFilterInstruction_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action39(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFilterInstruction_2a(__nt), __end));
                14
            }
            42 => {
                // FilterInstruction+ = FilterInstruction => ActionFn(48);
                let __sym0 = __pop_NtFilterInstruction(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action48(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFilterInstruction_2b(__nt), __end));
                15
            }
            43 => {
                // FilterInstruction+ = FilterInstruction+, FilterInstruction => ActionFn(49);
                let __sym1 = __pop_NtFilterInstruction(__symbols);
                let __sym0 = __pop_NtFilterInstruction_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action49(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtFilterInstruction_2b(__nt), __end));
                15
            }
            44 => {
                // NumExpression = NumExpression, "+", NumFactor => ActionFn(22);
                let __sym2 = __pop_NtNumFactor(__symbols);
                let __sym1 = __pop_Term_22_2b_22(__symbols);
                let __sym0 = __pop_NtNumExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtNumExpression(__nt), __end));
                16
            }
            45 => {
                // NumExpression = NumExpression, "-", NumFactor => ActionFn(23);
                let __sym2 = __pop_NtNumFactor(__symbols);
                let __sym1 = __pop_Term_22_2d_22(__symbols);
                let __sym0 = __pop_NtNumExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action23(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtNumExpression(__nt), __end));
                16
            }
            46 => {
                // NumExpression = NumFactor => ActionFn(24);
                let __sym0 = __pop_NtNumFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumExpression(__nt), __end));
                16
            }
            47 => {
                // NumFactor = NumFactor, "*", NumTerm => ActionFn(25);
                let __sym2 = __pop_NtNumTerm(__symbols);
                let __sym1 = __pop_Term_22_2a_22(__symbols);
                let __sym0 = __pop_NtNumFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtNumFactor(__nt), __end));
                17
            }
            48 => {
                // NumFactor = NumFactor, "/", NumTerm => ActionFn(26);
                let __sym2 = __pop_NtNumTerm(__symbols);
                let __sym1 = __pop_Term_22_2f_22(__symbols);
                let __sym0 = __pop_NtNumFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action26(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtNumFactor(__nt), __end));
                17
            }
            49 => {
                // NumFactor = NumTerm => ActionFn(27);
                let __sym0 = __pop_NtNumTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumFactor(__nt), __end));
                17
            }
            50 => {
                // NumTerm = Num => ActionFn(28);
                let __sym0 = __pop_TermNum(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNumTerm(__nt), __end));
                18
            }
            51 => {
                // NumTerm = "(", NumExpression, ")" => ActionFn(29);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtNumExpression(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action29(__sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtNumTerm(__nt), __end));
                18
            }
            52 => {
                // StrLiteral = QuotedStrLiteral => ActionFn(14);
                let __sym0 = __pop_TermQuotedStrLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStrLiteral(__nt), __end));
                19
            }
            53 => {
                // StrLiteral = Constant => ActionFn(15);
                let __sym0 = __pop_TermConstant(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStrLiteral(__nt), __end));
                19
            }
            54 => {
                // StringExpression = StrLiteral => ActionFn(13);
                let __sym0 = __pop_NtStrLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStringExpression(__nt), __end));
                20
            }
            55 => {
                // Value = NumExpression => ActionFn(10);
                let __sym0 = __pop_NtNumExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValue(__nt), __end));
                21
            }
            56 => {
                // Value = StringExpression => ActionFn(11);
                let __sym0 = __pop_NtStringExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValue(__nt), __end));
                21
            }
            57 => {
                // Value = VarIdentifier => ActionFn(12);
                let __sym0 = __pop_TermVarIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValue(__nt), __end));
                21
            }
            58 => {
                // Value+ = Value => ActionFn(32);
                let __sym0 = __pop_NtValue(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtValue_2b(__nt), __end));
                22
            }
            59 => {
                // Value+ = Value+, Value => ActionFn(33);
                let __sym1 = __pop_NtValue(__symbols);
                let __sym0 = __pop_NtValue_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtValue_2b(__nt), __end));
                22
            }
            60 => {
                // VarDefinition = VarIdentifier, "=", Value+, "\\n" => ActionFn(6);
                let __sym3 = __pop_Term_22_5c_5cn_22(__symbols);
                let __sym2 = __pop_NtValue_2b(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_TermVarIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action6(__sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtVarDefinition(__nt), __end));
                23
            }
            61 => {
                // VarDefinition+ = VarDefinition => ActionFn(40);
                let __sym0 = __pop_NtVarDefinition(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action40(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVarDefinition_2b(__nt), __end));
                24
            }
            62 => {
                // VarDefinition+ = VarDefinition+, VarDefinition => ActionFn(41);
                let __sym1 = __pop_NtVarDefinition(__symbols);
                let __sym0 = __pop_NtVarDefinition_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action41(__sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtVarDefinition_2b(__nt), __end));
                24
            }
            63 => {
                // VarDefinitionBlock = VarDefinition+ => ActionFn(2);
                let __sym0 = __pop_NtVarDefinition_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVarDefinitionBlock(__nt), __end));
                25
            }
            64 => {
                // VarDefinitionBlock? = VarDefinitionBlock => ActionFn(44);
                let __sym0 = __pop_NtVarDefinitionBlock(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVarDefinitionBlock_3f(__nt), __end));
                26
            }
            65 => {
                // VarDefinitionBlock? =  => ActionFn(45);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action45(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtVarDefinitionBlock_3f(__nt), __end));
                26
            }
            66 => {
                // VarIdentifier? = VarIdentifier => ActionFn(50);
                let __sym0 = __pop_TermVarIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action50(__sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVarIdentifier_3f(__nt), __end));
                27
            }
            67 => {
                // VarIdentifier? =  => ActionFn(51);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action51(&__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtVarIdentifier_3f(__nt), __end));
                27
            }
            68 => {
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
        let __next_state = __GOTO[__state * 29 + __nonterminal] - 1;
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
    fn __pop_Term_22_2c_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_Term_22Import_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Import_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22Mixin_22<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Tok, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22Mixin_22(__v), __r) => (__l, __v, __r),
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
    fn __pop_TermVarIdentifier<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, String, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermVarIdentifier(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_28_22_20_3cComma_3cVarIdentifier_3e_3e_20_22_29_22_29<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Vec<String>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_28_22_20_3cComma_3cVarIdentifier_3e_3e_20_22_29_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_28_22_20_3cComma_3cVarIdentifier_3e_3e_20_22_29_22_29_3f<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::option::Option<Vec<String>>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_28_22_20_3cComma_3cVarIdentifier_3e_3e_20_22_29_22_29_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cVarIdentifier_3e_20_22_2c_22_29<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, String, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cVarIdentifier_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::vec::Vec<String>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::vec::Vec<String>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cVarIdentifier_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtComma_3cVarIdentifier_3e<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, Vec<String>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cVarIdentifier_3e(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtFilterInstruction<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ast::FilterInstruction, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFilterInstruction(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFilterInstruction_2a<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFilterInstruction_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFilterInstruction_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFilterInstruction_2b(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtNumFactor<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ast::NumberExpression, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNumTerm<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ast::NumberExpression, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNumTerm(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtStringExpression<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ast::StringBox, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStringExpression(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtValue_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtValue_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVarDefinition<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ast::VarDefinition, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVarDefinition(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVarDefinition_2b<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::vec::Vec<ast::VarDefinition>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVarDefinition_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVarDefinitionBlock<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ast::Block, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVarDefinitionBlock(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVarDefinitionBlock_3f<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::option::Option<ast::Block>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVarDefinitionBlock_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVarIdentifier_3f<
    >(
        __symbols: &mut ::std::vec::Vec<(TokenLocation,__Symbol<>,TokenLocation)>
    ) -> (TokenLocation, ::std::option::Option<String>, TokenLocation) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVarIdentifier_3f(__v), __r) => (__l, __v, __r),
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
    (_, def, _): (TokenLocation, ::std::option::Option<ast::Block>, TokenLocation),
    (_, blocks, _): (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    {
        let mut blocks = blocks;
        if let Some(b) = def {
            blocks.insert(0, b);
        }
        Box::new(blocks)
    }
}

pub fn __action2<
>(
    (_, __0, _): (TokenLocation, ::std::vec::Vec<ast::VarDefinition>, TokenLocation),
) -> ast::Block
{
    ast::Block::Definitions(__0)
}

pub fn __action3<
>(
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, __0, _): (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ast::Block
{
    ast::Block::Show(__0)
}

pub fn __action4<
>(
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, __0, _): (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ast::Block
{
    ast::Block::Hide(__0)
}

pub fn __action5<
>(
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, name, _): (TokenLocation, String, TokenLocation),
    (_, args, _): (TokenLocation, ::std::option::Option<Vec<String>>, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, instructions, _): (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ast::Block
{
    {
        ast::Block::Mixin {
            name: name,
            parameters: args.unwrap_or(vec![]),
            instructions: instructions,
        }
    }
}

pub fn __action6<
>(
    (_, id, _): (TokenLocation, String, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
) -> ast::VarDefinition
{
    ast::VarDefinition { identifier: id, values: v }
}

pub fn __action7<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
    (_, __1, _): (TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
) -> ast::FilterInstruction
{
    ast::FilterInstruction::SetValue(__0, __1)
}

pub fn __action8<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
    (_, __1, _): (TokenLocation, ast::Condition, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
) -> ast::FilterInstruction
{
    ast::FilterInstruction::Condition(__0, __1)
}

pub fn __action9<
>(
    (_, __0, _): (TokenLocation, ast::VarDefinition, TokenLocation),
) -> ast::FilterInstruction
{
    ast::FilterInstruction::Var(__0)
}

pub fn __action10<
>(
    (_, __0, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::Value
{
    ast::Value::Num(__0)
}

pub fn __action11<
>(
    (_, __0, _): (TokenLocation, ast::StringBox, TokenLocation),
) -> ast::Value
{
    ast::Value::Str(__0)
}

pub fn __action12<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
) -> ast::Value
{
    ast::Value::Var(__0)
}

pub fn __action13<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
) -> ast::StringBox
{
    ast::StringBox::Value(__0)
}

pub fn __action14<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
) -> String
{
    String::from(__0)
}

pub fn __action15<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
) -> String
{
    String::from(__0)
}

pub fn __action16<
>(
    (_, op, _): (TokenLocation, ast::ComparisonOperator, TokenLocation),
    (_, v, _): (TokenLocation, ast::Value, TokenLocation),
) -> ast::Condition
{
    ast::Condition { value: v, operator: op }
}

pub fn __action17<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Gte
}

pub fn __action18<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Gt
}

pub fn __action19<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Lte
}

pub fn __action20<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Lt
}

pub fn __action21<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Eq
}

pub fn __action22<
>(
    (_, l, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, r, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::NumberExpression
{
    ast::NumberExpression::Op(Box::new(l), ast::NumberOperation::Add, Box::new(r))
}

pub fn __action23<
>(
    (_, l, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, r, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::NumberExpression
{
    ast::NumberExpression::Op(Box::new(l), ast::NumberOperation::Sub, Box::new(r))
}

pub fn __action24<
>(
    (_, __0, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::NumberExpression
{
    (__0)
}

pub fn __action25<
>(
    (_, l, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, r, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::NumberExpression
{
    ast::NumberExpression::Op(Box::new(l), ast::NumberOperation::Mul, Box::new(r))
}

pub fn __action26<
>(
    (_, l, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, r, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::NumberExpression
{
    ast::NumberExpression::Op(Box::new(l), ast::NumberOperation::Div, Box::new(r))
}

pub fn __action27<
>(
    (_, __0, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::NumberExpression
{
    (__0)
}

pub fn __action28<
>(
    (_, __0, _): (TokenLocation, i32, TokenLocation),
) -> ast::NumberExpression
{
    ast::NumberExpression::Number(ast::NumberBox::IntValue(__0))
}

pub fn __action29<
>(
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, __0, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
) -> ast::NumberExpression
{
    __0
}

pub fn __action30<
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

pub fn __action31<
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
        a: ast::NumberExpression::Number(ast::NumberBox::IntValue(255))
    }
}

pub fn __action32<
>(
    (_, __0, _): (TokenLocation, ast::Value, TokenLocation),
) -> ::std::vec::Vec<ast::Value>
{
    vec![__0]
}

pub fn __action33<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation),
    (_, e, _): (TokenLocation, ast::Value, TokenLocation),
) -> ::std::vec::Vec<ast::Value>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action34<
>(
    (_, __0, _): (TokenLocation, Vec<String>, TokenLocation),
) -> ::std::option::Option<Vec<String>>
{
    Some(__0)
}

pub fn __action35<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> ::std::option::Option<Vec<String>>
{
    None
}

pub fn __action36<
>(
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, __0, _): (TokenLocation, Vec<String>, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
) -> Vec<String>
{
    (__0)
}

pub fn __action37<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<String>, TokenLocation),
    (_, e, _): (TokenLocation, ::std::option::Option<String>, TokenLocation),
) -> Vec<String>
{
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

pub fn __action38<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> ::std::vec::Vec<ast::FilterInstruction>
{
    vec![]
}

pub fn __action39<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ::std::vec::Vec<ast::FilterInstruction>
{
    v
}

pub fn __action40<
>(
    (_, __0, _): (TokenLocation, ast::VarDefinition, TokenLocation),
) -> ::std::vec::Vec<ast::VarDefinition>
{
    vec![__0]
}

pub fn __action41<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::VarDefinition>, TokenLocation),
    (_, e, _): (TokenLocation, ast::VarDefinition, TokenLocation),
) -> ::std::vec::Vec<ast::VarDefinition>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action42<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> ::std::vec::Vec<ast::Block>
{
    vec![]
}

pub fn __action43<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> ::std::vec::Vec<ast::Block>
{
    v
}

pub fn __action44<
>(
    (_, __0, _): (TokenLocation, ast::Block, TokenLocation),
) -> ::std::option::Option<ast::Block>
{
    Some(__0)
}

pub fn __action45<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> ::std::option::Option<ast::Block>
{
    None
}

pub fn __action46<
>(
    (_, __0, _): (TokenLocation, ast::Block, TokenLocation),
) -> ::std::vec::Vec<ast::Block>
{
    vec![__0]
}

pub fn __action47<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
    (_, e, _): (TokenLocation, ast::Block, TokenLocation),
) -> ::std::vec::Vec<ast::Block>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action48<
>(
    (_, __0, _): (TokenLocation, ast::FilterInstruction, TokenLocation),
) -> ::std::vec::Vec<ast::FilterInstruction>
{
    vec![__0]
}

pub fn __action49<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
    (_, e, _): (TokenLocation, ast::FilterInstruction, TokenLocation),
) -> ::std::vec::Vec<ast::FilterInstruction>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action50<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
) -> ::std::option::Option<String>
{
    Some(__0)
}

pub fn __action51<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> ::std::option::Option<String>
{
    None
}

pub fn __action52<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> ::std::vec::Vec<String>
{
    vec![]
}

pub fn __action53<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<String>, TokenLocation),
) -> ::std::vec::Vec<String>
{
    v
}

pub fn __action54<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
) -> String
{
    (__0)
}

pub fn __action55<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
) -> ::std::vec::Vec<String>
{
    vec![__0]
}

pub fn __action56<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<String>, TokenLocation),
    (_, e, _): (TokenLocation, String, TokenLocation),
) -> ::std::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action57<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Vec<String>, TokenLocation),
    __2: (TokenLocation, Tok, TokenLocation),
) -> ::std::option::Option<Vec<String>>
{
    let __start0 = __0.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action36(
        __0,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        __temp0,
    )
}

pub fn __action58<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, String, TokenLocation),
    __2: (TokenLocation, Tok, TokenLocation),
    __3: (TokenLocation, Vec<String>, TokenLocation),
    __4: (TokenLocation, Tok, TokenLocation),
    __5: (TokenLocation, Tok, TokenLocation),
    __6: (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ast::Block
{
    let __start0 = __2.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action57(
        __2,
        __3,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        __0,
        __1,
        __temp0,
        __5,
        __6,
    )
}

pub fn __action59<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, String, TokenLocation),
    __2: (TokenLocation, Tok, TokenLocation),
    __3: (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ast::Block
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action35(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        __0,
        __1,
        __temp0,
        __2,
        __3,
    )
}

pub fn __action60<
>(
    __0: (TokenLocation, String, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
) -> ::std::vec::Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action54(
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        __temp0,
    )
}

pub fn __action61<
>(
    __0: (TokenLocation, ::std::vec::Vec<String>, TokenLocation),
    __1: (TokenLocation, String, TokenLocation),
    __2: (TokenLocation, Tok, TokenLocation),
) -> ::std::vec::Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action54(
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        __0,
        __temp0,
    )
}

pub fn __action62<
>(
    __0: (TokenLocation, ::std::option::Option<String>, TokenLocation),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action52(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        __temp0,
        __0,
    )
}

pub fn __action63<
>(
    __0: (TokenLocation, ::std::vec::Vec<String>, TokenLocation),
    __1: (TokenLocation, ::std::option::Option<String>, TokenLocation),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action53(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action37(
        __temp0,
        __1,
    )
}

pub fn __action64<
>(
    __0: (TokenLocation, ::std::option::Option<ast::Block>, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action42(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __0,
        __temp0,
    )
}

pub fn __action65<
>(
    __0: (TokenLocation, ::std::option::Option<ast::Block>, TokenLocation),
    __1: (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action43(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __0,
        __temp0,
    )
}

pub fn __action66<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
) -> ast::Block
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
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

pub fn __action67<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
    __2: (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ast::Block
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action39(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        __0,
        __1,
        __temp0,
    )
}

pub fn __action68<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
) -> ast::Block
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        __0,
        __1,
        __temp0,
    )
}

pub fn __action69<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
    __2: (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ast::Block
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action39(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        __0,
        __1,
        __temp0,
    )
}

pub fn __action70<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, String, TokenLocation),
    __2: (TokenLocation, Tok, TokenLocation),
    __3: (TokenLocation, Vec<String>, TokenLocation),
    __4: (TokenLocation, Tok, TokenLocation),
    __5: (TokenLocation, Tok, TokenLocation),
) -> ast::Block
{
    let __start0 = __5.2.clone();
    let __end0 = __5.2.clone();
    let __temp0 = __action38(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

pub fn __action71<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, String, TokenLocation),
    __2: (TokenLocation, Tok, TokenLocation),
    __3: (TokenLocation, Vec<String>, TokenLocation),
    __4: (TokenLocation, Tok, TokenLocation),
    __5: (TokenLocation, Tok, TokenLocation),
    __6: (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ast::Block
{
    let __start0 = __6.0.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action39(
        __6,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        __0,
        __1,
        __2,
        __3,
        __4,
        __5,
        __temp0,
    )
}

pub fn __action72<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, String, TokenLocation),
    __2: (TokenLocation, Tok, TokenLocation),
) -> ast::Block
{
    let __start0 = __2.2.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action38(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        __0,
        __1,
        __2,
        __temp0,
    )
}

pub fn __action73<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, String, TokenLocation),
    __2: (TokenLocation, Tok, TokenLocation),
    __3: (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ast::Block
{
    let __start0 = __3.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action39(
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        __0,
        __1,
        __2,
        __temp0,
    )
}

pub fn __action74<
>(
    __0: (TokenLocation, ast::Block, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        __temp0,
    )
}

pub fn __action75<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> Box<Vec<ast::Block>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action45(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        __temp0,
    )
}

pub fn __action76<
>(
    __0: (TokenLocation, ast::Block, TokenLocation),
    __1: (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action44(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        __temp0,
        __1,
    )
}

pub fn __action77<
>(
    __0: (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action45(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        __temp0,
        __0,
    )
}

pub fn __action78<
>(
    __0: (TokenLocation, String, TokenLocation),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action50(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        __temp0,
    )
}

pub fn __action79<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> Vec<String>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action51(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        __temp0,
    )
}

pub fn __action80<
>(
    __0: (TokenLocation, ::std::vec::Vec<String>, TokenLocation),
    __1: (TokenLocation, String, TokenLocation),
) -> Vec<String>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action50(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        __0,
        __temp0,
    )
}

pub fn __action81<
>(
    __0: (TokenLocation, ::std::vec::Vec<String>, TokenLocation),
) -> Vec<String>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action51(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        __0,
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
