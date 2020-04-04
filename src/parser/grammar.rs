// auto-generated: "lalrpop 0.18.1"
// sha256: 905560c5a85e6e3bd418733f55949296bbaf1e8e7af61e3bdae9bbb4d7243
use crate::lexer::tokens::*;
use crate::lexer::LexicalError;
use crate::executor::ast::*;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__SourceUnit {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use crate::lexer::tokens::*;
    use crate::lexer::LexicalError;
    use crate::executor::ast::*;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(TokenType<'input>),
        Variant1(&'input str),
        Variant2(Expression<'input>),
        Variant3(SourceUnit<'input>),
        Variant4(SourceUnitPart<'input>),
        Variant5(::std::vec::Vec<SourceUnitPart<'input>>),
        Variant6(Statement<'input>),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18, 19, 20, 21,
        // State 1
        0, 0, -9, 0, 22, 23, 0, -9, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, -3, 25, -3, -3, 26, -3, 0, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, -15, 0, 0, 0, -15, 0, 0, 0, 0, 0, 0, 0, -15, -15, 0, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 6
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, -13, 13, 0, 14, 15, 16, 17, 18, 19, 20, 21,
        // State 7
        0, -14, 0, 0, 0, -14, 0, 0, 0, 0, 0, 0, 0, -14, -14, 0, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 8
        0, 0, -30, -30, -30, -30, -30, -30, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, -12, -12, -12, -12, -12, -12, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 11
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 12
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 13
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, 0, 0, -24, 0, -24, -24, -24, 0, -24, 0,
        // State 14
        0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 15
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, 0, 0, -26, 0, -26, -26, -26, 0, -26, 0,
        // State 16
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, 0, 0, -27, 0, -27, -27, -27, 0, -27, 0,
        // State 17
        -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, 0, 0, -23, 0, -23, -23, -23, 0, -23, 0,
        // State 18
        0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 19
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, 0, 0, -25, 0, -25, -25, -25, 0, -25, 0,
        // State 20
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 21
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 22
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 23
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 24
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 25
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 26
        0, -16, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, -16, -16, 0, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 27
        0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, -29, -29, -29, -29, -29, -29, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 31
        -8, -8, 0, 0, 0, 0, 0, 0, -8, 0, -8, -8, -8, 0, 0, 0, -8, 0, -8, -8, -8, 0, -8, 0,
        // State 32
        0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, -1, 25, -1, -1, 26, -1, 0, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, -2, 25, -2, -2, 26, -2, 0, -2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, -10, -10, -10, -10, -10, -10, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, -11, -11, -11, -11, -11, -11, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, 0, 0, 0, -28, 0, -28, -28, -28, 0, -28, 0,
        // State 40
        0, -17, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, 0, -17, -17, 0, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 41
        47, 11, 0, 0, 0, 0, 0, 0, 48, 0, 49, 50, 0, 0, 0, 0, 14, 0, 16, 17, 18, 0, 20, 0,
        // State 42
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18, 19, 20, 21,
        // State 43
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18, 19, 20, 21,
        // State 44
        0, -19, 0, 0, 0, -19, 0, 0, 0, 0, 0, 0, 0, -19, -19, 0, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 45
        0, -18, 0, 0, 0, -18, 0, 0, 0, 0, 0, 0, 0, -18, -18, 0, -18, -18, -18, -18, -18, -18, -18, -18,
        // State 46
        -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, -5, -5, -5, 0, 0, 0, -5, 0, -5, -5, -5, 0, -5, 0,
        // State 47
        -7, -7, 0, 0, 0, 0, 0, 0, -7, 0, -7, -7, -7, 0, 0, 0, -7, 0, -7, -7, -7, 0, -7, 0,
        // State 48
        -4, -4, 0, 0, 0, 0, 0, 0, -4, 0, -4, -4, -4, 0, 0, 0, -4, 0, -4, -4, -4, 0, -4, 0,
        // State 49
        -6, -6, 0, 0, 0, 0, 0, 0, -6, 0, -6, -6, -6, 0, 0, 0, -6, 0, -6, -6, -6, 0, -6, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, -20, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0, 0, -20, -20, 55, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 53
        0, -22, 0, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, -22, -22, 0, -22, -22, -22, -22, -22, -22, -22, -22,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 11, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 14, 15, 16, 17, 18, 19, 20, 21,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, -21, 0, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21, -21, 0, -21, -21, -21, -21, -21, -21, -21, -21,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        0,
        // State 3
        0,
        // State 4
        -31,
        // State 5
        -15,
        // State 6
        -13,
        // State 7
        -14,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        -16,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        -17,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        -19,
        // State 45
        -18,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        -20,
        // State 53
        -22,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        -21,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        2, 0, 3, 4, 5, 6, 7, 8, 9, 10, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        2, 0, 3, 4, 0, 27, 0, 8, 9, 10, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        2, 0, 28, 4, 0, 0, 0, 0, 9, 10, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 9, 29, 0,
        // State 12
        2, 0, 30, 4, 0, 0, 0, 0, 9, 10, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 31, 0, 0, 0, 0, 0, 0, 32, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 33, 0, 0, 0, 0, 0, 0, 32, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        2, 0, 34, 4, 0, 0, 0, 0, 9, 10, 0,
        // State 21
        0, 0, 0, 35, 0, 0, 0, 0, 9, 10, 0,
        // State 22
        0, 0, 0, 36, 0, 0, 0, 0, 9, 10, 0,
        // State 23
        2, 0, 37, 4, 0, 0, 0, 0, 9, 10, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 9, 38, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 9, 39, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 42, 0, 0, 0, 0, 0, 0, 32, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 42, 0, 0, 0, 0, 0, 0, 32, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 42, 0, 0, 0, 0, 0, 0, 32, 0, 0,
        // State 42
        2, 0, 3, 4, 51, 6, 7, 8, 9, 10, 0,
        // State 43
        2, 0, 3, 4, 52, 6, 7, 8, 9, 10, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        2, 0, 3, 4, 57, 6, 7, 8, 9, 10, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###""/""###,
            r###"";""###,
            r###""<""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###""{""###,
            r###""}""###,
            r###"Declaration"###,
            r###"Else"###,
            r###"Identifier"###,
            r###"If"###,
            r###"InputNumber"###,
            r###"InputString"###,
            r###"Integer"###,
            r###"Loop"###,
            r###"StringLiteral"###,
            r###"Write"###,
        ];
        __ACTION[(__state * 24)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct __StateMachine<'input>
    where 
    {
        input: &'input str,
        __phantom: ::std::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = usize;
        type Error = LexicalError;
        type Token = TokenType<'input>;
        type TokenIndex = usize;
        type Symbol = __Symbol<'input>;
        type Success = SourceUnit<'input>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, ::std::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __ACTION[(state as usize) * 24 + integer]
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __ACTION[(state as usize) * 24 + (24 - 1)]
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __GOTO[(state as usize) * 11 + nt] - 1
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, ::std::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> Vec<String> {
            __expected_tokens(state as usize)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut Vec<i8>,
            symbols: &mut Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                self.input,
                action,
                start_location,
                states,
                symbols,
                ::std::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &TokenType<'input>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            TokenType::NotEqual if true => Some(0),
            TokenType::OpenParantheses if true => Some(1),
            TokenType::CloseParantheses if true => Some(2),
            TokenType::Product if true => Some(3),
            TokenType::Plus if true => Some(4),
            TokenType::Minus if true => Some(5),
            TokenType::Divide if true => Some(6),
            TokenType::SemiColon if true => Some(7),
            TokenType::LessThan if true => Some(8),
            TokenType::Assignment if true => Some(9),
            TokenType::EqualTo if true => Some(10),
            TokenType::GreaterThan if true => Some(11),
            TokenType::LeftBrace if true => Some(12),
            TokenType::RightBrace if true => Some(13),
            TokenType::Declaration if true => Some(14),
            TokenType::Else if true => Some(15),
            TokenType::Symbol(_) if true => Some(16),
            TokenType::If if true => Some(17),
            TokenType::InputNumber if true => Some(18),
            TokenType::InputString if true => Some(19),
            TokenType::Number(i64) if true => Some(20),
            TokenType::Loop if true => Some(21),
            TokenType::Literal(_) if true => Some(22),
            TokenType::Write if true => Some(23),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: TokenType<'input>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<'input>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 17 | 18 | 19 | 20 | 21 | 23 => __Symbol::Variant0(__token),
            16 | 22 => match __token {
                TokenType::Symbol(__tok0) | TokenType::Literal(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct SourceUnitParser {
        _priv: (),
    }

    impl SourceUnitParser {
        pub fn new() -> SourceUnitParser {
            SourceUnitParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            __TOKEN: __ToTriple<'input, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            input: &'input str,
            __tokens0: __TOKENS,
        ) -> Result<SourceUnit<'input>, __lalrpop_util::ParseError<usize, TokenType<'input>, LexicalError>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            let __r = __state_machine::Parser::drive(
                __StateMachine {
                    input,
                    __phantom: ::std::marker::PhantomData::<(&())>,
                },
                __tokens,
            );
            __r
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<SourceUnit<'input>,__lalrpop_util::ParseError<usize, TokenType<'input>, LexicalError>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            2 => {
                __reduce2(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            3 => {
                __reduce3(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            8 => {
                __reduce8(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            9 => {
                __reduce9(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            10 => {
                __reduce10(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            11 => {
                __reduce11(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            12 => {
                __reduce12(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            13 => {
                __reduce13(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            14 => {
                __reduce14(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            15 => {
                __reduce15(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            16 => {
                __reduce16(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            17 => {
                __reduce17(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            18 => {
                __reduce18(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            19 => {
                __reduce19(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            20 => {
                __reduce20(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            21 => {
                __reduce21(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            22 => {
                __reduce22(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            23 => {
                __reduce23(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            24 => {
                __reduce24(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            25 => {
                __reduce25(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            26 => {
                __reduce26(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            27 => {
                __reduce27(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            28 => {
                __reduce28(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            29 => {
                __reduce29(input, __lookahead_start, __symbols, ::std::marker::PhantomData::<(&())>)
            }
            30 => {
                // __SourceUnit = SourceUnit => ActionFn(0);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 11 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expression<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SourceUnit<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, SourceUnitPart<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, TokenType<'input>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<SourceUnitPart<'input>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ArithExpression = ArithExpression, "+", Factor => ActionFn(15);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action15::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ArithExpression = ArithExpression, "-", Factor => ActionFn(16);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action16::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 0)
    }
    pub(crate) fn __reduce2<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // ArithExpression = Factor => ActionFn(17);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action17::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Conditional = Conditional, Conditional, "==" => ActionFn(10);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action10::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 1)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Conditional = Conditional, Conditional, "!=" => ActionFn(11);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action11::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 1)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Conditional = Conditional, Conditional, ">" => ActionFn(12);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action12::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 1)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Conditional = Conditional, Conditional, "<" => ActionFn(13);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 1)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Conditional = Term => ActionFn(14);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action14::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 1)
    }
    pub(crate) fn __reduce8<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Expression = ArithExpression => ActionFn(9);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce9<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "*", Unary => ActionFn(18);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action18::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce10<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Factor = Factor, "/", Unary => ActionFn(19);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action19::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 3)
    }
    pub(crate) fn __reduce11<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Factor = Unary => ActionFn(20);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action20::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce12<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SourceUnit = SourceUnitPart+ => ActionFn(1);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action1::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce13<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SourceUnitPart = Statement => ActionFn(2);
        let __sym0 = __pop_Variant6(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 5)
    }
    pub(crate) fn __reduce14<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SourceUnitPart+ = SourceUnitPart => ActionFn(29);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action29::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (1, 6)
    }
    pub(crate) fn __reduce15<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // SourceUnitPart+ = SourceUnitPart+, SourceUnitPart => ActionFn(30);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action30::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant5(__nt), __end));
        (2, 6)
    }
    pub(crate) fn __reduce16<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = Declaration, Expression, ";" => ActionFn(3);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action3::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 7)
    }
    pub(crate) fn __reduce17<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = Expression, "=", Expression, ";" => ActionFn(4);
        assert!(__symbols.len() >= 4);
        let __sym3 = __pop_Variant0(__symbols);
        let __sym2 = __pop_Variant2(__symbols);
        let __sym1 = __pop_Variant0(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2, __sym3);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (4, 7)
    }
    pub(crate) fn __reduce18<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = Write, Expression, ";" => ActionFn(5);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (3, 7)
    }
    pub(crate) fn __reduce19<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = If, Conditional, "{", SourceUnit, "}" => ActionFn(6);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action6::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (5, 7)
    }
    pub(crate) fn __reduce20<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = If, Conditional, "{", SourceUnit, "}", Else, "{", SourceUnit, "}" => ActionFn(7);
        assert!(__symbols.len() >= 9);
        let __sym8 = __pop_Variant0(__symbols);
        let __sym7 = __pop_Variant3(__symbols);
        let __sym6 = __pop_Variant0(__symbols);
        let __sym5 = __pop_Variant0(__symbols);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym8.2.clone();
        let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (9, 7)
    }
    pub(crate) fn __reduce21<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Statement = Loop, Conditional, "{", SourceUnit, "}" => ActionFn(8);
        assert!(__symbols.len() >= 5);
        let __sym4 = __pop_Variant0(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action8::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
        __symbols.push((__start, __Symbol::Variant6(__nt), __end));
        (5, 7)
    }
    pub(crate) fn __reduce22<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = Integer => ActionFn(23);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action23::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce23<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = Identifier => ActionFn(24);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action24::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce24<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = StringLiteral => ActionFn(25);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action25::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce25<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = InputNumber => ActionFn(26);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action26::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce26<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = InputString => ActionFn(27);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action27::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 8)
    }
    pub(crate) fn __reduce27<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Term = "(", Expression, ")" => ActionFn(28);
        assert!(__symbols.len() >= 3);
        let __sym2 = __pop_Variant0(__symbols);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action28::<>(input, __sym0, __sym1, __sym2);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (3, 8)
    }
    pub(crate) fn __reduce28<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Unary = "-", Unary => ActionFn(21);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant2(__symbols);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action21::<>(input, __sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (2, 9)
    }
    pub(crate) fn __reduce29<
        'input,
    >(
        input: &'input str,
        __lookahead_start: Option<&usize>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Unary = Term => ActionFn(22);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action22::<>(input, __sym0);
        __symbols.push((__start, __Symbol::Variant2(__nt), __end));
        (1, 9)
    }
}
pub use self::__parse__SourceUnit::SourceUnitParser;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SourceUnit<'input>, usize),
) -> SourceUnit<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<SourceUnitPart<'input>>, usize),
) -> SourceUnit<'input>
{
    SourceUnit(__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Statement<'input>, usize),
) -> SourceUnitPart<'input>
{
    SourceUnitPart::Statement(__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, id, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
) -> Statement<'input>
{
    Statement::Declaration(id)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, r, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
) -> Statement<'input>
{
    Statement::Assignment(l,r)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, e, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
) -> Statement<'input>
{
    Statement::Write(e)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, condition, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, s, _): (usize, SourceUnit<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
) -> Statement<'input>
{
    Statement::Conditional(condition,s,None)
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, condition, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, s, _): (usize, SourceUnit<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, f, _): (usize, SourceUnit<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
) -> Statement<'input>
{
    Statement::Conditional(condition,s,Some(f))
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, condition, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, s, _): (usize, SourceUnit<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
) -> Statement<'input>
{
    Statement::Loop(condition,s)
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expression<'input>, usize),
    (_, r, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
) -> Expression<'input>
{
    Expression::Equals(Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expression<'input>, usize),
    (_, r, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
) -> Expression<'input>
{
    Expression::NotEquals(Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expression<'input>, usize),
    (_, r, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
) -> Expression<'input>
{
    Expression::GreaterThan(Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expression<'input>, usize),
    (_, r, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
) -> Expression<'input>
{
    Expression::LessThan(Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, r, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    Expression::Add(Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, r, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    Expression::Subtract(Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, r, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    Expression::Multiply(Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, l, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, r, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    Expression::Divide(Box::new(l),Box::new(r))
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, r, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    Expression::UnaryMinus(Box::new(r))
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression<'input>, usize),
) -> Expression<'input>
{
    __0
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, TokenType<'input>, usize),
) -> Expression<'input>
{
    Expression::Integer(v)
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, id, _): (usize, &'input str, usize),
) -> Expression<'input>
{
    Expression::Symbol(id)
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, string, _): (usize, &'input str, usize),
) -> Expression<'input>
{
    Expression::StringLiteral(string)
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, TokenType<'input>, usize),
) -> Expression<'input>
{
    Expression::InputNumber
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, TokenType<'input>, usize),
) -> Expression<'input>
{
    Expression::InputString
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, TokenType<'input>, usize),
    (_, e, _): (usize, Expression<'input>, usize),
    (_, _, _): (usize, TokenType<'input>, usize),
) -> Expression<'input>
{
    e
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, SourceUnitPart<'input>, usize),
) -> ::std::vec::Vec<SourceUnitPart<'input>>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<SourceUnitPart<'input>>, usize),
    (_, e, _): (usize, SourceUnitPart<'input>, usize),
) -> ::std::vec::Vec<SourceUnitPart<'input>>
{
    { let mut v = v; v.push(e); v }
}

pub trait __ToTriple<'input, > {
    fn to_triple(value: Self) -> Result<(usize,TokenType<'input>,usize), __lalrpop_util::ParseError<usize, TokenType<'input>, LexicalError>>;
}

impl<'input, > __ToTriple<'input, > for (usize, TokenType<'input>, usize) {
    fn to_triple(value: Self) -> Result<(usize,TokenType<'input>,usize), __lalrpop_util::ParseError<usize, TokenType<'input>, LexicalError>> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, TokenType<'input>, usize), LexicalError> {
    fn to_triple(value: Self) -> Result<(usize,TokenType<'input>,usize), __lalrpop_util::ParseError<usize, TokenType<'input>, LexicalError>> {
        match value {
            Ok(v) => Ok(v),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}