// auto-generated: "lalrpop 0.15.2"
// sha256: 684b76e8af83f7b1bdfe46fc23bfc5c88f1aff7f4733ac96ea15b4bf11b6d
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

trait S { fn s(&self) -> usize; }
impl S for usize { fn s(&self) -> usize { *self } }
impl S for Vec<usize> { fn s(&self) -> usize { self.iter().cloned().sum() } }
impl S for Option<usize> { fn s(&self) -> usize { self.into_iter().sum() } }
impl S for Vec<(usize, usize)> { fn s(&self) -> usize { self.iter().map(|&(x, y)| x + y).sum() } }

macro_rules! s {
    () => { 0 };
    ($e:expr) => { $e.s() };
    ($e:expr, $($es:expr),*) => { $e.s() + s!($($es),*)}
}
    
pub mod symbols {
    pub const file: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(0);
    pub const tokens_def: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(1);
    pub const token_def: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(2);
    pub const rule_def: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(3);
    pub const expr: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(4);
    pub const symbol: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(5);
    pub const op: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(6);
    pub const paren_expr: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(7);
    pub const token_kw_t: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(8);
    pub const rule_kw_t: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(9);
    pub const l_curly_t: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(10);
    pub const r_curly_t: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(11);
    pub const l_paren_t: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(12);
    pub const r_paren_t: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(13);
    pub const eq_t: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(14);
    pub const pipe_t: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(15);
    pub const star_t: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(16);
    pub const qmark_t: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(17);
    pub const ident_t: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(18);
    pub const word_t: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(19);
    pub const regex_t: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(20);
    pub const TOKEN: super::__lalrpop_util::Symbol = super::__lalrpop_util::Symbol(21);
}

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__file {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1((usize, usize)),
        Variant2(::std::vec::Vec<(usize, usize)>),
        Variant3(usize),
        Variant4(::std::option::Option<usize>),
        Variant5(::std::vec::Vec<usize>),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 4
        0, 0, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 16, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0,
        // State 6
        0, -20, 0, 0, 0, 0, -20, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, -36, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0,
        // State 9
        0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, -52, 0, 0, 0,
        // State 13
        0, -49, 0, 0, 0, 0, -49, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 16, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0,
        // State 15
        -19, -19, 0, -19, -19, -19, 0, -19, 0, -19, -19, 0, -19,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, -37, 0, 0, 0,
        // State 18
        21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 28, 0, 0, 0, 29,
        // State 20
        0, -14, 0, -14, -14, 0, 0, 0, -14, -14, 0, 0, -14,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, -53, 0, 0, 0,
        // State 22
        0, -50, 0, 0, 0, 0, -50, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 16, 0, 38, -15, 0, 0, 0, 0, -15, 0, 0, 29,
        // State 24
        0, -46, 0, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0,
        // State 25
        0, -13, 0, 0, 0, 0, -13, 0, 0, 0, 0, 0, 0,
        // State 26
        0, -12, 0, 0, 0, 0, -12, 0, 0, 0, 0, 0, 0,
        // State 27
        0, -31, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0,
        // State 28
        0, -54, 0, -54, -54, -54, -54, -54, 0, -54, -54, 0, -54,
        // State 29
        0, -41, 0, -41, -41, 42, 0, -41, 0, -41, 43, 0, -41,
        // State 30
        0, 0, 0, 0, 46, 0, 0, 0, 0, -32, 0, 0, 0,
        // State 31
        0, -10, 0, -10, -10, -10, 0, -10, 0, -10, -10, 0, -10,
        // State 32
        0, 16, 0, 38, 0, 0, 0, -15, 0, 0, 0, 0, 29,
        // State 33
        0, -11, 0, -11, -11, -11, 0, -11, 0, -11, -11, 0, -11,
        // State 34
        0, -44, 0, -44, -44, 0, 0, -44, 0, -44, 0, 0, -44,
        // State 35
        0, 16, 0, 38, -16, 0, 0, -16, 0, -16, 0, 0, 29,
        // State 36
        0, -9, 0, -9, -9, -9, 0, -9, 0, -9, -9, 0, -9,
        // State 37
        0, -21, 0, -21, 0, 0, 0, -21, 0, 0, 0, 0, -21,
        // State 38
        0, -40, 0, -40, -40, 0, 0, -40, 0, -40, 0, 0, -40,
        // State 39
        0, -22, 0, -22, -22, 0, 0, -22, 0, -22, 0, 0, -22,
        // State 40
        0, -23, 0, -23, -23, 0, 0, -23, 0, -23, 0, 0, -23,
        // State 41
        0, -28, 0, -28, -28, 0, 0, -28, 0, -28, 0, 0, -28,
        // State 42
        0, -39, 0, -39, -39, 0, 0, -39, 0, -39, 0, 0, -39,
        // State 43
        0, 0, 0, 0, 46, 0, 0, 0, 0, -33, 0, 0, 0,
        // State 44
        0, 16, 0, 38, -15, 0, 0, 0, 0, -15, 0, 0, 29,
        // State 45
        0, -27, 0, -27, -27, 0, 0, 0, 0, -27, 0, 0, -27,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0,
        // State 47
        0, -45, 0, -45, -45, 0, 0, -45, 0, -45, 0, 0, -45,
        // State 48
        0, 16, 0, 38, -15, 0, 0, 0, 0, -15, 0, 0, 29,
        // State 49
        0, 0, 0, 0, -4, 0, 0, 0, 0, -4, 0, 0, 0,
        // State 50
        0, -26, 0, -26, -26, -26, 0, -26, 0, -26, -26, 0, -26,
        // State 51
        0, -30, 0, -30, -30, -30, 0, -30, 0, -30, -30, 0, -30,
        // State 52
        0, 0, 0, 0, -5, 0, 0, 0, 0, -5, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        0,
        // State 1
        -8,
        // State 2
        0,
        // State 3
        -17,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        -36,
        // State 8
        -18,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        -52,
        // State 13
        0,
        // State 14
        0,
        // State 15
        -19,
        // State 16
        -29,
        // State 17
        -37,
        // State 18
        0,
        // State 19
        0,
        // State 20
        -14,
        // State 21
        -53,
        // State 22
        0,
        // State 23
        -15,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        -54,
        // State 29
        -41,
        // State 30
        -32,
        // State 31
        -10,
        // State 32
        0,
        // State 33
        -11,
        // State 34
        -44,
        // State 35
        -16,
        // State 36
        -9,
        // State 37
        0,
        // State 38
        -40,
        // State 39
        -22,
        // State 40
        -23,
        // State 41
        -28,
        // State 42
        -39,
        // State 43
        -33,
        // State 44
        -15,
        // State 45
        -27,
        // State 46
        0,
        // State 47
        -45,
        // State 48
        -15,
        // State 49
        -4,
        // State 50
        -26,
        // State 51
        -30,
        // State 52
        -5,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 4, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 0, 15, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 27,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 30, 0, 0, 31, 0, 32, 0, 33, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 36, 0, 0, 0, 0, 0, 37,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 30, 0, 0, 47, 0, 32, 0, 33, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 36, 0, 0, 0, 0, 0, 37,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 32, 0, 33, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 37,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 30, 0, 0, 50, 0, 32, 0, 33, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 36, 0, 0, 0, 0, 0, 37,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 30, 0, 0, 53, 0, 32, 0, 33, 0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 36, 0, 0, 0, 0, 0, 37,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"eq"###,
            r###"ident"###,
            r###"l_curly"###,
            r###"l_paren"###,
            r###"pipe"###,
            r###"qmark"###,
            r###"r_curly"###,
            r###"r_paren"###,
            r###"regex"###,
            r###"rule_kw"###,
            r###"star"###,
            r###"token_kw"###,
            r###"word"###,
        ];
        __ACTION[(__state * 13)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct fileParser {
        builder: super::__intern_token::__MatcherBuilder,
        _priv: (),
    }

    impl fileParser {
        pub fn new() -> fileParser {
            let __builder = super::__intern_token::__MatcherBuilder::new();
            fileParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            E,
        >(
            &self,
            events: &mut E,
            input: &'input str,
        ) -> Result<usize, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>> where
          E:  __lalrpop_util::LrEvents,
        {
            let mut __tokens = self.builder.matcher(input);
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(e),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token(6, _) if true => 0,
                    Token(2, _) if true => 1,
                    Token(10, _) if true => 2,
                    Token(3, _) if true => 3,
                    Token(11, _) if true => 4,
                    Token(7, _) if true => 5,
                    Token(12, _) if true => 6,
                    Token(4, _) if true => 7,
                    Token(1, _) if true => 8,
                    Token(8, _) if true => 9,
                    Token(5, _) if true => 10,
                    Token(9, _) if true => 11,
                    Token(0, _) if true => 12,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 13 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                Token(6, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                Token(10, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                Token(11, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                Token(7, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                Token(12, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                Token(8, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                Token(5, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                Token(9, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(events, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<(E)>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(events, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<(E)>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> Option<Result<usize,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>> where
      E:  __lalrpop_util::LrEvents,
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                __reduce1(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            2 => {
                __reduce2(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            3 => {
                __reduce3(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            4 => {
                __reduce4(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            5 => {
                __reduce5(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            6 => {
                __reduce6(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            7 => {
                __reduce7(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            8 => {
                // __file = file => ActionFn(0);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<E>(events, input, __sym0);
                return Some(Ok(__nt));
            }
            9 => {
                __reduce9(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            10 => {
                __reduce10(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            11 => {
                __reduce11(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            12 => {
                __reduce12(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            13 => {
                __reduce13(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            14 => {
                __reduce14(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            15 => {
                __reduce15(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            16 => {
                __reduce16(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            17 => {
                __reduce17(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            18 => {
                __reduce18(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            19 => {
                __reduce19(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            20 => {
                __reduce20(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            21 => {
                __reduce21(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            22 => {
                __reduce22(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            23 => {
                __reduce23(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            24 => {
                __reduce24(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            25 => {
                __reduce25(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            26 => {
                __reduce26(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            27 => {
                __reduce27(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            28 => {
                __reduce28(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            29 => {
                __reduce29(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            30 => {
                __reduce30(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            31 => {
                __reduce31(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            32 => {
                __reduce32(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            33 => {
                __reduce33(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            34 => {
                __reduce34(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            35 => {
                __reduce35(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            36 => {
                __reduce36(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            37 => {
                __reduce37(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            38 => {
                __reduce38(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            39 => {
                __reduce39(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            40 => {
                __reduce40(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            41 => {
                __reduce41(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            42 => {
                __reduce42(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            43 => {
                __reduce43(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            44 => {
                __reduce44(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            45 => {
                __reduce45(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            46 => {
                __reduce46(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            47 => {
                __reduce47(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            48 => {
                __reduce48(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            49 => {
                __reduce49(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            50 => {
                __reduce50(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            51 => {
                __reduce51(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            52 => {
                __reduce52(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            53 => {
                __reduce53(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            54 => {
                __reduce54(events, input, __action, __lookahead_start, __states, __symbols, ::std::marker::PhantomData::<(E)>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 36 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, (usize, usize), usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
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
    ) -> (usize, ::std::option::Option<usize>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<(usize, usize)>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<usize>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    pub(crate) fn __reduce1<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // (pipe_t expr) = pipe_t, expr => ActionFn(36);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action36::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant1(__nt), __end);
        (2, __symbol, 0)
    }
    pub(crate) fn __reduce2<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // (pipe_t expr)* =  => ActionFn(34);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action34::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (0, __symbol, 1)
    }
    pub(crate) fn __reduce3<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // (pipe_t expr)* = (pipe_t expr)+ => ActionFn(35);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action35::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (1, __symbol, 1)
    }
    pub(crate) fn __reduce4<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // (pipe_t expr)+ = pipe_t, expr => ActionFn(49);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action49::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (2, __symbol, 2)
    }
    pub(crate) fn __reduce5<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // (pipe_t expr)+ = (pipe_t expr)+, pipe_t, expr => ActionFn(50);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant2(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action50::<E>(events, input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant2(__nt), __end);
        (3, __symbol, 2)
    }
    pub(crate) fn __reduce6<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // @L =  => ActionFn(28);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action28::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 3)
    }
    pub(crate) fn __reduce7<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // @R =  => ActionFn(29);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action29::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 4)
    }
    pub(crate) fn __reduce9<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // _atom = word_t => ActionFn(11);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action11::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce10<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // _atom = ident_t => ActionFn(12);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action12::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce11<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // _atom = paren_expr => ActionFn(13);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action13::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 6)
    }
    pub(crate) fn __reduce12<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // _token_re = word_t => ActionFn(4);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce13<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // _token_re = regex_t => ActionFn(5);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 7)
    }
    pub(crate) fn __reduce14<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // eq_t = eq => ActionFn(66);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action66::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 8)
    }
    pub(crate) fn __reduce15<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // expr =  => ActionFn(83);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action83::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (0, __symbol, 9)
    }
    pub(crate) fn __reduce16<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // expr = symbol+ => ActionFn(84);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action84::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 9)
    }
    pub(crate) fn __reduce17<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // file = tokens_def => ActionFn(81);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action81::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 10)
    }
    pub(crate) fn __reduce18<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // file = tokens_def, rule_def+ => ActionFn(82);
        let __sym1 = __pop_Variant5(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action82::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 10)
    }
    pub(crate) fn __reduce19<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // ident_t = ident => ActionFn(67);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action67::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 11)
    }
    pub(crate) fn __reduce20<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // l_curly_t = l_curly => ActionFn(68);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action68::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 12)
    }
    pub(crate) fn __reduce21<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // l_paren_t = l_paren => ActionFn(69);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action69::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 13)
    }
    pub(crate) fn __reduce22<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // op = qmark_t => ActionFn(9);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action9::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce23<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // op = star_t => ActionFn(10);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action10::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 14)
    }
    pub(crate) fn __reduce24<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // op? = op => ActionFn(30);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action30::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (1, __symbol, 15)
    }
    pub(crate) fn __reduce25<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // op? =  => ActionFn(31);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action31::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant4(__nt), __end);
        (0, __symbol, 15)
    }
    pub(crate) fn __reduce26<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // paren_expr = l_paren_t, expr, r_paren_t => ActionFn(14);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action14::<E>(events, input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 16)
    }
    pub(crate) fn __reduce27<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // pipe_t = pipe => ActionFn(70);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action70::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 17)
    }
    pub(crate) fn __reduce28<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // qmark_t = qmark => ActionFn(71);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action71::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 18)
    }
    pub(crate) fn __reduce29<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // r_curly_t = r_curly => ActionFn(72);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action72::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 19)
    }
    pub(crate) fn __reduce30<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // r_paren_t = r_paren => ActionFn(73);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action73::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 20)
    }
    pub(crate) fn __reduce31<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // regex_t = regex => ActionFn(74);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action74::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 21)
    }
    pub(crate) fn __reduce32<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // rule_def = rule_kw_t, ident_t, eq_t, expr => ActionFn(51);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action51::<E>(events, input, __sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (4, __symbol, 22)
    }
    pub(crate) fn __reduce33<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // rule_def = rule_kw_t, ident_t, eq_t, expr, (pipe_t expr)+ => ActionFn(52);
        let __sym4 = __pop_Variant2(__symbols);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym4.2.clone();
        let __nt = super::__action52::<E>(events, input, __sym0, __sym1, __sym2, __sym3, __sym4);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (5, __symbol, 22)
    }
    pub(crate) fn __reduce34<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // rule_def* =  => ActionFn(39);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action39::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 23)
    }
    pub(crate) fn __reduce35<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // rule_def* = rule_def+ => ActionFn(40);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action40::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 23)
    }
    pub(crate) fn __reduce36<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // rule_def+ = rule_def => ActionFn(41);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action41::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 24)
    }
    pub(crate) fn __reduce37<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // rule_def+ = rule_def+, rule_def => ActionFn(42);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action42::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 24)
    }
    pub(crate) fn __reduce38<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // rule_kw_t = rule_kw => ActionFn(75);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action75::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 25)
    }
    pub(crate) fn __reduce39<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // star_t = star => ActionFn(76);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action76::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 26)
    }
    pub(crate) fn __reduce40<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // symbol = _atom, op => ActionFn(79);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action79::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (2, __symbol, 27)
    }
    pub(crate) fn __reduce41<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // symbol = _atom => ActionFn(80);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action80::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 27)
    }
    pub(crate) fn __reduce42<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // symbol* =  => ActionFn(32);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action32::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 28)
    }
    pub(crate) fn __reduce43<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // symbol* = symbol+ => ActionFn(33);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action33::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 28)
    }
    pub(crate) fn __reduce44<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // symbol+ = symbol => ActionFn(47);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action47::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 29)
    }
    pub(crate) fn __reduce45<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // symbol+ = symbol+, symbol => ActionFn(48);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action48::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 29)
    }
    pub(crate) fn __reduce46<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // token_def = ident_t, eq_t, _token_re => ActionFn(3);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action3::<E>(events, input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 30)
    }
    pub(crate) fn __reduce47<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // token_def* =  => ActionFn(37);
        let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
        let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
        let __nt = super::__action37::<E>(events, input, &__start, &__end);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (0, __symbol, 31)
    }
    pub(crate) fn __reduce48<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // token_def* = token_def+ => ActionFn(38);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action38::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 31)
    }
    pub(crate) fn __reduce49<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // token_def+ = token_def => ActionFn(43);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action43::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (1, __symbol, 32)
    }
    pub(crate) fn __reduce50<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // token_def+ = token_def+, token_def => ActionFn(44);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant5(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action44::<E>(events, input, __sym0, __sym1);
        let __symbol = (__start, __Symbol::Variant5(__nt), __end);
        (2, __symbol, 32)
    }
    pub(crate) fn __reduce51<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // token_kw_t = token_kw => ActionFn(77);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action77::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 33)
    }
    pub(crate) fn __reduce52<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // tokens_def = token_kw_t, l_curly_t, r_curly_t => ActionFn(85);
        let __sym2 = __pop_Variant3(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym2.2.clone();
        let __nt = super::__action85::<E>(events, input, __sym0, __sym1, __sym2);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (3, __symbol, 34)
    }
    pub(crate) fn __reduce53<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // tokens_def = token_kw_t, l_curly_t, token_def+, r_curly_t => ActionFn(86);
        let __sym3 = __pop_Variant3(__symbols);
        let __sym2 = __pop_Variant5(__symbols);
        let __sym1 = __pop_Variant3(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym3.2.clone();
        let __nt = super::__action86::<E>(events, input, __sym0, __sym1, __sym2, __sym3);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (4, __symbol, 34)
    }
    pub(crate) fn __reduce54<
        'input,
        E,
    >(
        events: &mut E,
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<(E)>,
    ) -> (usize, (usize,__Symbol<'input>,usize), usize) where
      E:  __lalrpop_util::LrEvents,
    {
        // word_t = word => ActionFn(78);
        let __sym0 = __pop_Variant0(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action78::<E>(events, input, __sym0);
        let __symbol = (__start, __Symbol::Variant3(__nt), __end);
        (1, __symbol, 35)
    }
}
pub use self::__parse__file::fileParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt(&self, formatter: &mut __fmt::Formatter) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __MatcherBuilder {
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl __MatcherBuilder {
        pub fn new() -> __MatcherBuilder {
            let __strs: &[&str] = &[
                "^((?u:\')(?u:[\u{0}-\\&\\(-\u{10ffff}])+(?u:\'))",
                "^((((?u:r\")(?u:[\u{0}-!\\#-\u{10ffff}])+(?u:\"))|((?u:r\\#\")((?u:\")(?u:[\u{0}-!\\$-\u{10ffff}])|(?u:[\u{0}-!\\#-\u{10ffff}]))+(?u:\"\\#))|((?u:r)(?u:\\#){2}(?u:\")((?u:\"\\#)(?u:[\u{0}-!\\$-\u{10ffff}])|(?u:\")(?u:[\u{0}-!\\$-\u{10ffff}])|(?u:[\u{0}-!\\#-\u{10ffff}]))+(?u:\")(?u:\\#){2})))",
                "^((?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࡠ-ࡪࢠ-ࢴࢶ-ࢽࣔ-ࣣ࣡-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱৼ-ৼਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-૿ଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಀ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഀ-ഃഅ-ഌഎ-ഐഒ-ൄെ-ൈൊ-ൎൔ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽᲀ-ᲈ᳐-᳔᳒-᳹ᴀ-᷹᷻-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄮㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿪ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞮꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-ꣅ꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌭-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑈾-𑈾𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑐀-𑑊𑑐-𑑙𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑨀-𑨾𑩇-𑩇𑩐-𑪃𑪆-𑪙𑫀-𑫸𑰀-𑰈𑰊-𑰶𑰸-𑱀𑱐-𑱙𑱲-𑲏𑲒-𑲧𑲩-𑲶𑴀-𑴆𑴈-𑴉𑴋-𑴶𑴺-𑴺𑴼-𑴽𑴿-𑵇𑵐-𑵙𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𖿠-𖿡𗀀-𘟬𘠀-𘫲𛀀-𛄞𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞀀-𞀆𞀈-𞀘𞀛-𞀡𞀣-𞀤𞀦-𞀪𞠀-𞣄𞣐-𞣖𞤀-𞥊𞥐-𞥙𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀󠄀-󠇯])+)",
                "^((?u:\\())",
                "^((?u:\\)))",
                "^((?u:\\*))",
                "^((?u:=))",
                "^((?u:\\?))",
                "^((?u:rule))",
                "^((?u:tokens))",
                "^((?u:\\{))",
                "^((?u:\\|))",
                "^((?u:\\}))",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^((?u:\')(?u:[\u{0}-\\&\\(-\u{10ffff}])+(?u:\'))").unwrap(),
                __regex::Regex::new("^((((?u:r\")(?u:[\u{0}-!\\#-\u{10ffff}])+(?u:\"))|((?u:r\\#\")((?u:\")(?u:[\u{0}-!\\$-\u{10ffff}])|(?u:[\u{0}-!\\#-\u{10ffff}]))+(?u:\"\\#))|((?u:r)(?u:\\#){2}(?u:\")((?u:\"\\#)(?u:[\u{0}-!\\$-\u{10ffff}])|(?u:\")(?u:[\u{0}-!\\$-\u{10ffff}])|(?u:[\u{0}-!\\#-\u{10ffff}]))+(?u:\")(?u:\\#){2})))").unwrap(),
                __regex::Regex::new("^((?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࡠ-ࡪࢠ-ࢴࢶ-ࢽࣔ-ࣣ࣡-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱৼ-ৼਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-૿ଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಀ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഀ-ഃഅ-ഌഎ-ഐഒ-ൄെ-ൈൊ-ൎൔ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽᲀ-ᲈ᳐-᳔᳒-᳹ᴀ-᷹᷻-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄮㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿪ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞮꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-ꣅ꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌭-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐒰-𐓓𐓘-𐓻𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑈾-𑈾𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑐀-𑑊𑑐-𑑙𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑨀-𑨾𑩇-𑩇𑩐-𑪃𑪆-𑪙𑫀-𑫸𑰀-𑰈𑰊-𑰶𑰸-𑱀𑱐-𑱙𑱲-𑲏𑲒-𑲧𑲩-𑲶𑴀-𑴆𑴈-𑴉𑴋-𑴶𑴺-𑴺𑴼-𑴽𑴿-𑵇𑵐-𑵙𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𖿠-𖿡𗀀-𘟬𘠀-𘫲𛀀-𛄞𛅰-𛋻𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞀀-𞀆𞀈-𞀘𞀛-𞀡𞀣-𞀤𞀦-𞀪𞠀-𞣄𞣐-𞣖𞤀-𞥊𞥐-𞥙𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡𬺰-𮯠丽-𪘀󠄀-󠇯])+)").unwrap(),
                __regex::Regex::new("^((?u:\\())").unwrap(),
                __regex::Regex::new("^((?u:\\)))").unwrap(),
                __regex::Regex::new("^((?u:\\*))").unwrap(),
                __regex::Regex::new("^((?u:=))").unwrap(),
                __regex::Regex::new("^((?u:\\?))").unwrap(),
                __regex::Regex::new("^((?u:rule))").unwrap(),
                __regex::Regex::new("^((?u:tokens))").unwrap(),
                __regex::Regex::new("^((?u:\\{))").unwrap(),
                __regex::Regex::new("^((?u:\\|))").unwrap(),
                __regex::Regex::new("^((?u:\\}))").unwrap(),
            ];
            __MatcherBuilder { regex_set: __regex_set, regex_vec: __regex_vec }
        }
        pub fn matcher<'input, 'builder>(&'builder self, s: &'input str) -> __Matcher<'input, 'builder> {
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: &self.regex_set,
                regex_vec: &self.regex_vec,
            }
        }
    }

    pub struct __Matcher<'input, 'builder> {
        text: &'input str,
        consumed: usize,
        regex_set: &'builder __regex::RegexSet,
        regex_vec: &'builder Vec<__regex::Regex>,
    }

    impl<'input, 'builder> Iterator for __Matcher<'input, 'builder> {
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 13 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, ::std::vec::Vec<usize>, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    events.reduce(symbols::file, s!(__0, __1)); 1
}

#[allow(unused_variables)]
fn __action2<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, usize, usize),
    (_, __2, _): (usize, ::std::vec::Vec<usize>, usize),
    (_, __3, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    events.reduce(symbols::tokens_def, s!(__0, __1, __2, __3)); 1
}

#[allow(unused_variables)]
fn __action3<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, usize, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    events.reduce(symbols::token_def, s!(__0, __1, __2)); 1
}

#[allow(unused_variables)]
fn __action4<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    s!(__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    s!(__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, usize, usize),
    (_, __2, _): (usize, usize, usize),
    (_, __3, _): (usize, usize, usize),
    (_, __4, _): (usize, ::std::vec::Vec<(usize, usize)>, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    events.reduce(symbols::rule_def, s!(__0, __1, __2, __3, __4)); 1
}

#[allow(unused_variables)]
fn __action7<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<usize>, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    events.reduce(symbols::expr, s!(__0)); 1
}

#[allow(unused_variables)]
fn __action8<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, ::std::option::Option<usize>, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    events.reduce(symbols::symbol, s!(__0, __1)); 1
}

#[allow(unused_variables)]
fn __action9<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    events.reduce(symbols::op, s!(__0)); 1
}

#[allow(unused_variables)]
fn __action10<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    events.reduce(symbols::op, s!(__0)); 1
}

#[allow(unused_variables)]
fn __action11<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    s!(__0)
}

#[allow(unused_variables)]
fn __action12<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    s!(__0)
}

#[allow(unused_variables)]
fn __action13<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    s!(__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, usize, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    events.reduce(symbols::paren_expr, s!(__0, __1, __2)); 1
}

#[allow(unused_variables)]
fn __action15<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let (l, _, r) = (__0, __1, __2); events.shift(symbols::token_kw_t, l, r); 1
}

#[allow(unused_variables)]
fn __action16<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let (l, _, r) = (__0, __1, __2); events.shift(symbols::rule_kw_t, l, r); 1
}

#[allow(unused_variables)]
fn __action17<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let (l, _, r) = (__0, __1, __2); events.shift(symbols::l_curly_t, l, r); 1
}

#[allow(unused_variables)]
fn __action18<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let (l, _, r) = (__0, __1, __2); events.shift(symbols::r_curly_t, l, r); 1
}

#[allow(unused_variables)]
fn __action19<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let (l, _, r) = (__0, __1, __2); events.shift(symbols::l_paren_t, l, r); 1
}

#[allow(unused_variables)]
fn __action20<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let (l, _, r) = (__0, __1, __2); events.shift(symbols::r_paren_t, l, r); 1
}

#[allow(unused_variables)]
fn __action21<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let (l, _, r) = (__0, __1, __2); events.shift(symbols::eq_t, l, r); 1
}

#[allow(unused_variables)]
fn __action22<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let (l, _, r) = (__0, __1, __2); events.shift(symbols::pipe_t, l, r); 1
}

#[allow(unused_variables)]
fn __action23<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let (l, _, r) = (__0, __1, __2); events.shift(symbols::star_t, l, r); 1
}

#[allow(unused_variables)]
fn __action24<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let (l, _, r) = (__0, __1, __2); events.shift(symbols::qmark_t, l, r); 1
}

#[allow(unused_variables)]
fn __action25<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let (l, _, r) = (__0, __1, __2); events.shift(symbols::ident_t, l, r); 1
}

#[allow(unused_variables)]
fn __action26<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let (l, _, r) = (__0, __1, __2); events.shift(symbols::word_t, l, r); 1
}

#[allow(unused_variables)]
fn __action27<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, &'input str, usize),
    (_, __2, _): (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let (l, _, r) = (__0, __1, __2); events.shift(symbols::regex_t, l, r); 1
}

#[allow(unused_variables)]
fn __action28<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    __lookahead.clone()
}

#[allow(unused_variables)]
fn __action29<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    __lookbehind.clone()
}

#[allow(unused_variables)]
fn __action30<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> ::std::option::Option<usize> where
  E:  __lalrpop_util::LrEvents,
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action31<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<usize> where
  E:  __lalrpop_util::LrEvents,
{
    None
}

#[allow(unused_variables)]
fn __action32<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<usize> where
  E:  __lalrpop_util::LrEvents,
{
    vec![]
}

#[allow(unused_variables)]
fn __action33<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<usize>, usize),
) -> ::std::vec::Vec<usize> where
  E:  __lalrpop_util::LrEvents,
{
    v
}

#[allow(unused_variables)]
fn __action34<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<(usize, usize)> where
  E:  __lalrpop_util::LrEvents,
{
    vec![]
}

#[allow(unused_variables)]
fn __action35<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(usize, usize)>, usize),
) -> ::std::vec::Vec<(usize, usize)> where
  E:  __lalrpop_util::LrEvents,
{
    v
}

#[allow(unused_variables)]
fn __action36<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
    (_, __1, _): (usize, usize, usize),
) -> (usize, usize) where
  E:  __lalrpop_util::LrEvents,
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action37<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<usize> where
  E:  __lalrpop_util::LrEvents,
{
    vec![]
}

#[allow(unused_variables)]
fn __action38<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<usize>, usize),
) -> ::std::vec::Vec<usize> where
  E:  __lalrpop_util::LrEvents,
{
    v
}

#[allow(unused_variables)]
fn __action39<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<usize> where
  E:  __lalrpop_util::LrEvents,
{
    vec![]
}

#[allow(unused_variables)]
fn __action40<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<usize>, usize),
) -> ::std::vec::Vec<usize> where
  E:  __lalrpop_util::LrEvents,
{
    v
}

#[allow(unused_variables)]
fn __action41<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> ::std::vec::Vec<usize> where
  E:  __lalrpop_util::LrEvents,
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action42<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<usize>, usize),
    (_, e, _): (usize, usize, usize),
) -> ::std::vec::Vec<usize> where
  E:  __lalrpop_util::LrEvents,
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action43<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> ::std::vec::Vec<usize> where
  E:  __lalrpop_util::LrEvents,
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action44<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<usize>, usize),
    (_, e, _): (usize, usize, usize),
) -> ::std::vec::Vec<usize> where
  E:  __lalrpop_util::LrEvents,
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action45<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, (usize, usize), usize),
) -> ::std::vec::Vec<(usize, usize)> where
  E:  __lalrpop_util::LrEvents,
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action46<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<(usize, usize)>, usize),
    (_, e, _): (usize, (usize, usize), usize),
) -> ::std::vec::Vec<(usize, usize)> where
  E:  __lalrpop_util::LrEvents,
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action47<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> ::std::vec::Vec<usize> where
  E:  __lalrpop_util::LrEvents,
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action48<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<usize>, usize),
    (_, e, _): (usize, usize, usize),
) -> ::std::vec::Vec<usize> where
  E:  __lalrpop_util::LrEvents,
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action49<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, usize, usize),
) -> ::std::vec::Vec<(usize, usize)> where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action36(
        events,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action50<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<(usize, usize)>, usize),
    __1: (usize, usize, usize),
    __2: (usize, usize, usize),
) -> ::std::vec::Vec<(usize, usize)> where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action36(
        events,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action51<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, usize, usize),
    __2: (usize, usize, usize),
    __3: (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __3.2.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action34(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        events,
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action52<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, usize, usize),
    __2: (usize, usize, usize),
    __3: (usize, usize, usize),
    __4: (usize, ::std::vec::Vec<(usize, usize)>, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __4.0.clone();
    let __end0 = __4.2.clone();
    let __temp0 = __action35(
        events,
        input,
        __4,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        events,
        input,
        __0,
        __1,
        __2,
        __3,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action53<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action21(
        events,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action54<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action25(
        events,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action55<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action17(
        events,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action56<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action19(
        events,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action57<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action22(
        events,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action58<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action24(
        events,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action59<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action18(
        events,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action60<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action20(
        events,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action61<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action27(
        events,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action62<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action16(
        events,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action63<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action23(
        events,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action64<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        events,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action65<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action28(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action26(
        events,
        input,
        __0,
        __1,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action66<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action67<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action54(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action68<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action55(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action69<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action56(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action70<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action57(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action71<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action58(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action72<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action59(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action73<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action60(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action74<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action61(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action75<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action62(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action76<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action63(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action77<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action64(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action78<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, &'input str, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action29(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action65(
        events,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action79<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action30(
        events,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action80<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action31(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action81<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action39(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action82<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, ::std::vec::Vec<usize>, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action40(
        events,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        events,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action83<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action32(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action84<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<usize>, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action33(
        events,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        events,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action85<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, usize, usize),
    __2: (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action37(
        events,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        events,
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action86<
    'input,
    E,
>(
    events: &mut E,
    input: &'input str,
    __0: (usize, usize, usize),
    __1: (usize, usize, usize),
    __2: (usize, ::std::vec::Vec<usize>, usize),
    __3: (usize, usize, usize),
) -> usize where
  E:  __lalrpop_util::LrEvents,
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action38(
        events,
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        events,
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

pub trait __ToTriple<'input, E, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),Self::Error>;
}

impl<'input, E, > __ToTriple<'input, E, > for (usize, Token<'input>, usize) {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        Ok(value)
    }
}
impl<'input, E, > __ToTriple<'input, E, > for Result<(usize, Token<'input>, usize),&'static str> {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        value
    }
}
