#![allow(unused_imports)]
use ast;
use std::str::FromStr;
use tok::Location as TokenLocation;
use tok::Tok;
extern crate lalrpop_util as __lalrpop_util;
use self::__lalrpop_util::ParseError as __ParseError;

mod __parse__Filter {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use ast;
    use std::str::FromStr;
    use tok::Location as TokenLocation;
    use tok::Tok;
    extern crate lalrpop_util as __lalrpop_util;
    use self::__lalrpop_util::ParseError as __ParseError;
    use super::__ToTriple;
    pub fn parse_Filter<
        __TOKEN: __ToTriple<Error=char>,
        __TOKENS: IntoIterator<Item=__TOKEN>,
    >(
        __tokens: __TOKENS,
    ) -> Result<Box<Vec<ast::Block>>, __ParseError<TokenLocation,Tok,char>>
    {
        let __tokens = __tokens.into_iter();
        let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match try!(__state0(&mut __tokens, __lookahead)) {
            (Some(__lookahead), _) => {
                Err(__ParseError::ExtraToken { token: __lookahead })
            }
            (None, __Nonterminal::____Filter((_, __nt, _))) => {
                Ok(__nt)
            }
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub enum __Nonterminal<> {
        Block((TokenLocation, ast::Block, TokenLocation)),
        Block_2a((TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation)),
        Block_2b((TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation)),
        Color((TokenLocation, ast::Color, TokenLocation)),
        ComparisonOperator((TokenLocation, ast::ComparisonOperator, TokenLocation)),
        Condition((TokenLocation, ast::Condition, TokenLocation)),
        Filter((TokenLocation, Box<Vec<ast::Block>>, TokenLocation)),
        Ident_2b((TokenLocation, ::std::vec::Vec<String>, TokenLocation)),
        Line((TokenLocation, ast::Instruction, TokenLocation)),
        Line_2a((TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation)),
        Line_2b((TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation)),
        NumExpression((TokenLocation, ast::NumberExpression, TokenLocation)),
        NumExpression_2b((TokenLocation, ::std::vec::Vec<ast::NumberExpression>, TokenLocation)),
        Value((TokenLocation, ast::Value, TokenLocation)),
        ____Filter((TokenLocation, Box<Vec<ast::Block>>, TokenLocation)),
    }

    pub fn __state0<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Hide, __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state4(__tokens, __sym0));
            }
            Some((__loc1, __tok @ Tok::Show, __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state5(__tokens, __sym0));
            }
            None => {
                let __start: TokenLocation = ::std::default::Default::default();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action29(&__start, &__end);
                let __nt = __Nonterminal::Filter((
                    __start,
                    __nt,
                    __end,
                ));
                __result = (__lookahead, __nt);
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        loop {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Block(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state1(__tokens, __lookahead, __sym0));
                }
                __Nonterminal::Block_2b(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state2(__tokens, __lookahead, __sym0));
                }
                __Nonterminal::Filter(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state3(__tokens, __lookahead, __sym0));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
    }

    pub fn __state1<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::Block, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25(__sym0);
                let __nt = __Nonterminal::Block_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state2<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Hide, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state4(__tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Show, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state5(__tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30(__sym0);
                let __nt = __Nonterminal::Filter((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Block(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state6(__tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state3<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, Box<Vec<ast::Block>>, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0(__sym0);
                let __nt = __Nonterminal::____Filter((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state4<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::NewLine, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state7(__tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state5<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::NewLine, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state8(__tokens, __sym0, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state6<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::Block, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action26(__sym0, __sym1);
                let __nt = __Nonterminal::Block_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state7<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, Tok::Ident(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state11(__tokens, __sym2));
            }
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action33(__sym0, __sym1);
                let __nt = __Nonterminal::Block((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Line(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::Line_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state8<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, Tok::Ident(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state11(__tokens, __sym2));
            }
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31(__sym0, __sym1);
                let __nt = __Nonterminal::Block((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Line(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state9(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::Line_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state12(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state9<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::Instruction, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) |
            Some((_, Tok::Ident(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27(__sym0);
                let __nt = __Nonterminal::Line_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state10<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, Tok::Ident(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state11(__tokens, __sym3));
            }
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action34(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Block((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Line(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state13(__tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state11<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((__loc1, __tok @ Tok::Lt, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state20(__tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Lte, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state21(__tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Eql, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state22(__tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Gt, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state23(__tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Gte, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state24(__tokens, __sym1));
            }
            Some((__loc1, Tok::Ident(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(__tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(__tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::ComparisonOperator(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state14(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::Condition(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state15(__tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::Ident_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state16(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::NumExpression(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::NumExpression_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state18(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state19(__tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state12<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, Tok::Ident(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state11(__tokens, __sym3));
            }
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Block((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym2.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Line(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state13(__tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state13<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::Instruction, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) |
            Some((_, Tok::Ident(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action28(__sym0, __sym1);
                let __nt = __Nonterminal::Line_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state14<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::ComparisonOperator, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, Tok::Ident(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state25(__tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(__tokens, __sym1));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Ident_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state16(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::NumExpression(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state17(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::NumExpression_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state18(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state27(__tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state15<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::Condition, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::NewLine, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state28(__tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state16<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ::std::vec::Vec<String>, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, Tok::Ident(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state29(__tokens, __sym0, __sym1));
            }
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7(__sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state17<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Num(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(__sym0);
                let __nt = __Nonterminal::NumExpression_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state18<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ::std::vec::Vec<ast::NumberExpression>, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state26(__tokens, __sym1));
            }
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6(__sym0);
                let __nt = __Nonterminal::Value((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        while __sym0.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::NumExpression(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state30(__tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state19<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::Value, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::NewLine, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state31(__tokens, __sym0, __sym1, __sym2));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
        return Ok(__result);
    }

    pub fn __state20<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::Ident(_), _)) |
            Some((_, Tok::Num(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(__sym0);
                let __nt = __Nonterminal::ComparisonOperator((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state21<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::Ident(_), _)) |
            Some((_, Tok::Num(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(__sym0);
                let __nt = __Nonterminal::ComparisonOperator((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state22<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::Ident(_), _)) |
            Some((_, Tok::Num(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(__sym0);
                let __nt = __Nonterminal::ComparisonOperator((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state23<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::Ident(_), _)) |
            Some((_, Tok::Num(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(__sym0);
                let __nt = __Nonterminal::ComparisonOperator((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state24<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::Ident(_), _)) |
            Some((_, Tok::Num(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(__sym0);
                let __nt = __Nonterminal::ComparisonOperator((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state25<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Ident(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(__sym0);
                let __nt = __Nonterminal::Ident_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state26<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, i32, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Num(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(__sym0);
                let __nt = __Nonterminal::NumExpression((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state27<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::ComparisonOperator, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::Value, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action8(__sym0, __sym1);
                let __nt = __Nonterminal::Condition((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state28<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::Condition, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, Tok, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) |
            Some((_, Tok::Ident(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action5(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Line((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state29<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, ::std::vec::Vec<String>, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, String, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Ident(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18(__sym0, __sym1);
                let __nt = __Nonterminal::Ident_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state30<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ::std::vec::Vec<ast::NumberExpression>, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Num(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action20(__sym0, __sym1);
                let __nt = __Nonterminal::NumExpression_2b((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
        }
    }

    pub fn __state31<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::Value, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, Tok, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        let __lookahead = match __tokens.next() {
            Some(Ok(v)) => Some(v),
            None => None,
            Some(Err(e)) => return Err(__ParseError::User { error: e }),
        };
        match __lookahead {
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) |
            Some((_, Tok::Ident(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::Line((
                    __start,
                    __nt,
                    __end,
                ));
                return Ok((__lookahead, __nt));
            }
            _ => {
                return Err(__ParseError::UnrecognizedToken {
                    token: __lookahead,
                    expected: vec![],
                });
            }
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
    (_, op, _): (TokenLocation, ast::ComparisonOperator, TokenLocation),
    (_, v, _): (TokenLocation, ast::Value, TokenLocation),
) -> ast::Condition
{
    ast::Condition { value: v, operator: op }
}

pub fn __action9<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Gte
}

pub fn __action10<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Gt
}

pub fn __action11<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Lte
}

pub fn __action12<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Lt
}

pub fn __action13<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Eq
}

pub fn __action14<
>(
    (_, __0, _): (TokenLocation, i32, TokenLocation),
) -> ast::NumberExpression
{
    ast::NumberExpression::Number(__0)
}

pub fn __action15<
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

pub fn __action16<
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

pub fn __action17<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
) -> ::std::vec::Vec<String>
{
    vec![__0]
}

pub fn __action18<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<String>, TokenLocation),
    (_, e, _): (TokenLocation, String, TokenLocation),
) -> ::std::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action19<
>(
    (_, __0, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ::std::vec::Vec<ast::NumberExpression>
{
    vec![__0]
}

pub fn __action20<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::NumberExpression>, TokenLocation),
    (_, e, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ::std::vec::Vec<ast::NumberExpression>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action21<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> ::std::vec::Vec<ast::Instruction>
{
    vec![]
}

pub fn __action22<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation),
) -> ::std::vec::Vec<ast::Instruction>
{
    v
}

pub fn __action23<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> ::std::vec::Vec<ast::Block>
{
    vec![]
}

pub fn __action24<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> ::std::vec::Vec<ast::Block>
{
    v
}

pub fn __action25<
>(
    (_, __0, _): (TokenLocation, ast::Block, TokenLocation),
) -> ::std::vec::Vec<ast::Block>
{
    vec![__0]
}

pub fn __action26<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
    (_, e, _): (TokenLocation, ast::Block, TokenLocation),
) -> ::std::vec::Vec<ast::Block>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action27<
>(
    (_, __0, _): (TokenLocation, ast::Instruction, TokenLocation),
) -> ::std::vec::Vec<ast::Instruction>
{
    vec![__0]
}

pub fn __action28<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation),
    (_, e, _): (TokenLocation, ast::Instruction, TokenLocation),
) -> ::std::vec::Vec<ast::Instruction>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action29<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> Box<Vec<ast::Block>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action23(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

pub fn __action30<
>(
    __0: (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action24(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

pub fn __action31<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
) -> ast::Block
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action21(
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

pub fn __action32<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
    __2: (TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation),
) -> ast::Block
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action22(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        __0,
        __1,
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
    let __temp0 = __action21(
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

pub fn __action34<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
    __2: (TokenLocation, ::std::vec::Vec<ast::Instruction>, TokenLocation),
) -> ast::Block
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action22(
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
