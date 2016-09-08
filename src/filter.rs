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
        FilterInstruction((TokenLocation, ast::FilterInstruction, TokenLocation)),
        FilterInstruction_2a((TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation)),
        FilterInstruction_2b((TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation)),
        NumExpression((TokenLocation, ast::NumberExpression, TokenLocation)),
        NumFactor((TokenLocation, ast::NumberExpression, TokenLocation)),
        NumTerm((TokenLocation, ast::NumberExpression, TokenLocation)),
        StrLiteral((TokenLocation, String, TokenLocation)),
        StringExpression((TokenLocation, ast::StringBox, TokenLocation)),
        Value((TokenLocation, ast::Value, TokenLocation)),
        Value_2b((TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation)),
        VarDefinition((TokenLocation, ast::VarDefinition, TokenLocation)),
        VarDefinition_2b((TokenLocation, ::std::vec::Vec<ast::VarDefinition>, TokenLocation)),
        VarDefinitionBlock((TokenLocation, ast::Block, TokenLocation)),
        VarDefinitionBlock_3f((TokenLocation, ::std::option::Option<ast::Block>, TokenLocation)),
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
                __result = try!(__state7(__tokens, __sym0));
            }
            Some((__loc1, __tok @ Tok::Show, __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state8(__tokens, __sym0));
            }
            Some((__loc1, Tok::VarIdentifier(__tok0), __loc2)) => {
                let mut __sym0 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state9(__tokens, __sym0));
            }
            None => {
                let __start: TokenLocation = ::std::default::Default::default();
                let __end = __lookahead.as_ref().map(|o| o.0.clone()).unwrap_or_else(|| __start.clone());
                let __nt = super::__action52(&__start, &__end);
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
                __Nonterminal::VarDefinition(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state4(__tokens, __lookahead, __sym0));
                }
                __Nonterminal::VarDefinition_2b(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state5(__tokens, __lookahead, __sym0));
                }
                __Nonterminal::VarDefinitionBlock(__nt) => {
                    let __sym0 = &mut Some(__nt);
                    __result = try!(__state6(__tokens, __lookahead, __sym0));
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
                let __nt = super::__action41(__sym0);
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
                __result = try!(__state7(__tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Show, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state8(__tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action54(__sym0);
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
                    __result = try!(__state10(__tokens, __lookahead, __sym0, __sym1));
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
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::VarDefinition, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35(__sym0);
                let __nt = __Nonterminal::VarDefinition_2b((
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

    pub fn __state5<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ::std::vec::Vec<ast::VarDefinition>, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, Tok::VarIdentifier(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state9(__tokens, __sym1));
            }
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2(__sym0);
                let __nt = __Nonterminal::VarDefinitionBlock((
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
                __Nonterminal::VarDefinition(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state11(__tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state6<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::Block, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Hide, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state7(__tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Show, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state8(__tokens, __sym1));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action51(__sym0);
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
                    __result = try!(__state1(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::Block_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state12(__tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state7<
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
                __result = try!(__state13(__tokens, __sym0, __sym1));
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

    pub fn __state8<
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
                __result = try!(__state14(__tokens, __sym0, __sym1));
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

    pub fn __state9<
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
            Some((__loc1, __tok @ Tok::Eql, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state15(__tokens, __sym0, __sym1));
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

    pub fn __state10<
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
                let __nt = super::__action42(__sym0, __sym1);
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

    pub fn __state11<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ::std::vec::Vec<ast::VarDefinition>, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::VarDefinition, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36(__sym0, __sym1);
                let __nt = __Nonterminal::VarDefinition_2b((
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

    pub fn __state12<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::Block, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Hide, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state7(__tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::Show, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state8(__tokens, __sym2));
            }
            None => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action53(__sym0, __sym1);
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
        while __sym1.is_some() {
            let (__lookahead, __nt) = __result;
            match __nt {
                __Nonterminal::Block(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state10(__tokens, __lookahead, __sym1, __sym2));
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
            Some((__loc1, Tok::Constant(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(__tokens, __sym2));
            }
            Some((__loc1, Tok::VarIdentifier(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(__tokens, __sym2));
            }
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action49(__sym0, __sym1);
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
                __Nonterminal::FilterInstruction(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::FilterInstruction_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state17(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::VarDefinition(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(__tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state14<
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
            Some((__loc1, Tok::Constant(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(__tokens, __sym2));
            }
            Some((__loc1, Tok::VarIdentifier(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(__tokens, __sym2));
            }
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action47(__sym0, __sym1);
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
                __Nonterminal::FilterInstruction(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state16(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::FilterInstruction_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state21(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::VarDefinition(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state18(__tokens, __lookahead, __sym2));
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
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state29(__tokens, __sym2));
            }
            Some((__loc1, Tok::Constant(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(__tokens, __sym2));
            }
            Some((__loc1, Tok::StrLiteral(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(__tokens, __sym2));
            }
            Some((__loc1, Tok::VarIdentifier(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(__tokens, __sym2));
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
                __Nonterminal::NumExpression(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::NumFactor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state23(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::StrLiteral(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::StringExpression(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state27(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state28(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state16<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::FilterInstruction, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action43(__sym0);
                let __nt = __Nonterminal::FilterInstruction_2b((
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

    pub fn __state17<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, Tok::Constant(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(__tokens, __sym3));
            }
            Some((__loc1, Tok::VarIdentifier(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(__tokens, __sym3));
            }
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action50(__sym0, __sym1, __sym2);
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
                __Nonterminal::FilterInstruction(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state34(__tokens, __lookahead, __sym2, __sym3));
                }
                __Nonterminal::VarDefinition(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state18(__tokens, __lookahead, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state18<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::VarDefinition, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8(__sym0);
                let __nt = __Nonterminal::FilterInstruction((
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

    pub fn __state19<
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state29(__tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Lt, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state38(__tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Lte, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state39(__tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Eql, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state40(__tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Gt, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state41(__tokens, __sym1));
            }
            Some((__loc1, __tok @ Tok::Gte, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state42(__tokens, __sym1));
            }
            Some((__loc1, Tok::Constant(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(__tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(__tokens, __sym1));
            }
            Some((__loc1, Tok::StrLiteral(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(__tokens, __sym1));
            }
            Some((__loc1, Tok::VarIdentifier(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(__tokens, __sym1));
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
                    __result = try!(__state35(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::Condition(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state36(__tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::NumExpression(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state22(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::NumFactor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state23(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state24(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::StrLiteral(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state25(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::StringExpression(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state26(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state27(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value_2b(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state37(__tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state20<
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
            Some((__loc1, __tok @ Tok::Eql, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state43(__tokens, __sym0, __sym1));
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

    pub fn __state21<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, Tok::Constant(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state19(__tokens, __sym3));
            }
            Some((__loc1, Tok::VarIdentifier(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state20(__tokens, __sym3));
            }
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action48(__sym0, __sym1, __sym2);
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
                __Nonterminal::FilterInstruction(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state34(__tokens, __lookahead, __sym2, __sym3));
                }
                __Nonterminal::VarDefinition(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state18(__tokens, __lookahead, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state22<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state44(__tokens, __sym0, __sym1));
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state45(__tokens, __sym0, __sym1));
            }
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(__sym0);
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

    pub fn __state23<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state46(__tokens, __sym0, __sym1));
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state47(__tokens, __sym0, __sym1));
            }
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23(__sym0);
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
        return Ok(__result);
    }

    pub fn __state24<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26(__sym0);
                let __nt = __Nonterminal::NumFactor((
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
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(__sym0);
                let __nt = __Nonterminal::StringExpression((
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
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::StringBox, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(__sym0);
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
    }

    pub fn __state27<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::Value, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31(__sym0);
                let __nt = __Nonterminal::Value_2b((
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
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state29(__tokens, __sym3));
            }
            Some((__loc1, __tok @ Tok::NewLine, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state49(__tokens, __sym0, __sym1, __sym2, __sym3));
            }
            Some((__loc1, Tok::Constant(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(__tokens, __sym3));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(__tokens, __sym3));
            }
            Some((__loc1, Tok::StrLiteral(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(__tokens, __sym3));
            }
            Some((__loc1, Tok::VarIdentifier(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(__tokens, __sym3));
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
                __Nonterminal::NumExpression(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state22(__tokens, __lookahead, __sym3));
                }
                __Nonterminal::NumFactor(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state23(__tokens, __lookahead, __sym3));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state24(__tokens, __lookahead, __sym3));
                }
                __Nonterminal::StrLiteral(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state25(__tokens, __lookahead, __sym3));
                }
                __Nonterminal::StringExpression(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state26(__tokens, __lookahead, __sym3));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state48(__tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state29<
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state53(__tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state54(__tokens, __sym1));
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
                    __result = try!(__state50(__tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::NumFactor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state51(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state52(__tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state30<
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
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(__sym0);
                let __nt = __Nonterminal::StrLiteral((
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
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27(__sym0);
                let __nt = __Nonterminal::NumTerm((
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

    pub fn __state32<
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
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(__sym0);
                let __nt = __Nonterminal::StrLiteral((
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

    pub fn __state33<
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
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(__sym0);
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
    }

    pub fn __state34<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::FilterInstruction, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            None |
            Some((_, Tok::Hide, _)) |
            Some((_, Tok::Show, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action44(__sym0, __sym1);
                let __nt = __Nonterminal::FilterInstruction_2b((
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

    pub fn __state35<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::ComparisonOperator, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state61(__tokens, __sym1));
            }
            Some((__loc1, Tok::Constant(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state62(__tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state63(__tokens, __sym1));
            }
            Some((__loc1, Tok::StrLiteral(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state64(__tokens, __sym1));
            }
            Some((__loc1, Tok::VarIdentifier(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state65(__tokens, __sym1));
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
                    __result = try!(__state55(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::NumFactor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state56(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state57(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::StrLiteral(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state58(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::StringExpression(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state59(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state60(__tokens, __lookahead, __sym0, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state36<
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
                __result = try!(__state66(__tokens, __sym0, __sym1, __sym2));
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

    pub fn __state37<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state29(__tokens, __sym2));
            }
            Some((__loc1, __tok @ Tok::NewLine, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state67(__tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, Tok::Constant(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(__tokens, __sym2));
            }
            Some((__loc1, Tok::StrLiteral(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(__tokens, __sym2));
            }
            Some((__loc1, Tok::VarIdentifier(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(__tokens, __sym2));
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
                __Nonterminal::NumExpression(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::NumFactor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state23(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::StrLiteral(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::StringExpression(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state48(__tokens, __lookahead, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state38<
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
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19(__sym0);
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

    pub fn __state39<
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
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18(__sym0);
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

    pub fn __state40<
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
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20(__sym0);
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

    pub fn __state41<
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
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17(__sym0);
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

    pub fn __state42<
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
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16(__sym0);
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

    pub fn __state43<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state29(__tokens, __sym2));
            }
            Some((__loc1, Tok::Constant(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(__tokens, __sym2));
            }
            Some((__loc1, Tok::StrLiteral(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(__tokens, __sym2));
            }
            Some((__loc1, Tok::VarIdentifier(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(__tokens, __sym2));
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
                __Nonterminal::NumExpression(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state22(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::NumFactor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state23(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::StrLiteral(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state25(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::StringExpression(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state26(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state27(__tokens, __lookahead, __sym2));
                }
                __Nonterminal::Value_2b(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state68(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state44<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state29(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(__tokens, __sym2));
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
                __Nonterminal::NumFactor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state69(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(__tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state45<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state29(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(__tokens, __sym2));
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
                __Nonterminal::NumFactor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state70(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state24(__tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state46<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state29(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(__tokens, __sym2));
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
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state71(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state47<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state29(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(__tokens, __sym2));
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
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state72(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state48<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::Value, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action32(__sym0, __sym1);
                let __nt = __Nonterminal::Value_2b((
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

    pub fn __state49<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation)>,
        __sym3: &mut Option<(TokenLocation, Tok, TokenLocation)>,
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
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5(__sym0, __sym1, __sym2, __sym3);
                let __nt = __Nonterminal::VarDefinition((
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

    pub fn __state50<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state73(__tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state74(__tokens, __sym1, __sym2));
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state75(__tokens, __sym1, __sym2));
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

    pub fn __state51<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state76(__tokens, __sym0, __sym1));
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state77(__tokens, __sym0, __sym1));
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23(__sym0);
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
        return Ok(__result);
    }

    pub fn __state52<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26(__sym0);
                let __nt = __Nonterminal::NumFactor((
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

    pub fn __state53<
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state53(__tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state54(__tokens, __sym1));
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
                    __result = try!(__state78(__tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::NumFactor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state51(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state52(__tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state54<
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
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27(__sym0);
                let __nt = __Nonterminal::NumTerm((
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

    pub fn __state55<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state79(__tokens, __sym0, __sym1));
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state80(__tokens, __sym0, __sym1));
            }
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9(__sym0);
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

    pub fn __state56<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state81(__tokens, __sym0, __sym1));
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state82(__tokens, __sym0, __sym1));
            }
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23(__sym0);
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
        return Ok(__result);
    }

    pub fn __state57<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26(__sym0);
                let __nt = __Nonterminal::NumFactor((
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

    pub fn __state58<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12(__sym0);
                let __nt = __Nonterminal::StringExpression((
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

    pub fn __state59<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::StringBox, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10(__sym0);
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
    }

    pub fn __state60<
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
                let __nt = super::__action15(__sym0, __sym1);
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

    pub fn __state61<
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state53(__tokens, __sym1));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym1 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state54(__tokens, __sym1));
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
                    __result = try!(__state83(__tokens, __lookahead, __sym0, __sym1));
                }
                __Nonterminal::NumFactor(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state51(__tokens, __lookahead, __sym1));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym1 = &mut Some(__nt);
                    __result = try!(__state52(__tokens, __lookahead, __sym1));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state62<
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
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14(__sym0);
                let __nt = __Nonterminal::StrLiteral((
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

    pub fn __state63<
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
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27(__sym0);
                let __nt = __Nonterminal::NumTerm((
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

    pub fn __state64<
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
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13(__sym0);
                let __nt = __Nonterminal::StrLiteral((
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

    pub fn __state65<
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
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11(__sym0);
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
    }

    pub fn __state66<
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
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action7(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::FilterInstruction((
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

    pub fn __state67<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation)>,
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
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action6(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::FilterInstruction((
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

    pub fn __state68<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state29(__tokens, __sym3));
            }
            Some((__loc1, __tok @ Tok::NewLine, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state84(__tokens, __sym0, __sym1, __sym2, __sym3));
            }
            Some((__loc1, Tok::Constant(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state30(__tokens, __sym3));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state31(__tokens, __sym3));
            }
            Some((__loc1, Tok::StrLiteral(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state32(__tokens, __sym3));
            }
            Some((__loc1, Tok::VarIdentifier(__tok0), __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state33(__tokens, __sym3));
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
                __Nonterminal::NumExpression(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state22(__tokens, __lookahead, __sym3));
                }
                __Nonterminal::NumFactor(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state23(__tokens, __lookahead, __sym3));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state24(__tokens, __lookahead, __sym3));
                }
                __Nonterminal::StrLiteral(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state25(__tokens, __lookahead, __sym3));
                }
                __Nonterminal::StringExpression(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state26(__tokens, __lookahead, __sym3));
                }
                __Nonterminal::Value(__nt) => {
                    let __sym3 = &mut Some(__nt);
                    __result = try!(__state48(__tokens, __lookahead, __sym2, __sym3));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state69<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state46(__tokens, __sym2, __sym3));
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state47(__tokens, __sym2, __sym3));
            }
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21(__sym0, __sym1, __sym2);
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
        return Ok(__result);
    }

    pub fn __state70<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state46(__tokens, __sym2, __sym3));
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state47(__tokens, __sym2, __sym3));
            }
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22(__sym0, __sym1, __sym2);
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
        return Ok(__result);
    }

    pub fn __state71<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::NumFactor((
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

    pub fn __state72<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::NumFactor((
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

    pub fn __state73<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((_, Tok::LParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            Some((_, Tok::NewLine, _)) |
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::Num(_), _)) |
            Some((_, Tok::StrLiteral(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action28(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::NumTerm((
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

    pub fn __state74<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state53(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state54(__tokens, __sym2));
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
                __Nonterminal::NumFactor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state85(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state52(__tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state75<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state53(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state54(__tokens, __sym2));
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
                __Nonterminal::NumFactor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state86(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state52(__tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state76<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state53(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state54(__tokens, __sym2));
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
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state87(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state77<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state53(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state54(__tokens, __sym2));
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
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state88(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state78<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state89(__tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state74(__tokens, __sym1, __sym2));
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state75(__tokens, __sym1, __sym2));
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

    pub fn __state79<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state61(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state63(__tokens, __sym2));
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
                __Nonterminal::NumFactor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state90(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state57(__tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state80<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state61(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state63(__tokens, __sym2));
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
                __Nonterminal::NumFactor(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state91(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state57(__tokens, __lookahead, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state81<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state61(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state63(__tokens, __sym2));
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
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state92(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state82<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((__loc1, __tok @ Tok::LParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state61(__tokens, __sym2));
            }
            Some((__loc1, Tok::Num(__tok0), __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok0), __loc2));
                __result = try!(__state63(__tokens, __sym2));
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
                __Nonterminal::NumTerm(__nt) => {
                    let __sym2 = &mut Some(__nt);
                    __result = try!(__state93(__tokens, __lookahead, __sym0, __sym1, __sym2));
                }
                _ => {
                    return Ok((__lookahead, __nt));
                }
            }
        }
        return Ok(__result);
    }

    pub fn __state83<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::RParen, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state94(__tokens, __sym0, __sym1, __sym2));
            }
            Some((__loc1, __tok @ Tok::Plus, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state74(__tokens, __sym1, __sym2));
            }
            Some((__loc1, __tok @ Tok::Minus, __loc2)) => {
                let mut __sym2 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state75(__tokens, __sym1, __sym2));
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

    pub fn __state84<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, String, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation)>,
        __sym3: &mut Option<(TokenLocation, Tok, TokenLocation)>,
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
            Some((_, Tok::Constant(_), _)) |
            Some((_, Tok::VarIdentifier(_), _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __sym3 = __sym3.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5(__sym0, __sym1, __sym2, __sym3);
                let __nt = __Nonterminal::VarDefinition((
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

    pub fn __state85<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state76(__tokens, __sym2, __sym3));
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state77(__tokens, __sym2, __sym3));
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21(__sym0, __sym1, __sym2);
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
        return Ok(__result);
    }

    pub fn __state86<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state76(__tokens, __sym2, __sym3));
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state77(__tokens, __sym2, __sym3));
            }
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22(__sym0, __sym1, __sym2);
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
        return Ok(__result);
    }

    pub fn __state87<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::NumFactor((
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

    pub fn __state88<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::NumFactor((
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

    pub fn __state89<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((_, Tok::RParen, _)) |
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action28(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::NumTerm((
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

    pub fn __state90<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state81(__tokens, __sym2, __sym3));
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state82(__tokens, __sym2, __sym3));
            }
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action21(__sym0, __sym1, __sym2);
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
        return Ok(__result);
    }

    pub fn __state91<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((__loc1, __tok @ Tok::Times, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state81(__tokens, __sym2, __sym3));
            }
            Some((__loc1, __tok @ Tok::Div, __loc2)) => {
                let mut __sym3 = &mut Some((__loc1, (__tok), __loc2));
                __result = try!(__state82(__tokens, __sym2, __sym3));
            }
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action22(__sym0, __sym1, __sym2);
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
        return Ok(__result);
    }

    pub fn __state92<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action24(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::NumFactor((
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

    pub fn __state93<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __lookahead: Option<(TokenLocation, Tok, TokenLocation)>,
        __sym0: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym2: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
    ) -> Result<(Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>), __ParseError<TokenLocation,Tok,char>>
    {
        let mut __result: (Option<(TokenLocation, Tok, TokenLocation)>, __Nonterminal<>);
        match __lookahead {
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::NumFactor((
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

    pub fn __state94<
        __TOKENS: Iterator<Item=Result<(TokenLocation, Tok, TokenLocation),char>>,
    >(
        __tokens: &mut __TOKENS,
        __sym0: &mut Option<(TokenLocation, Tok, TokenLocation)>,
        __sym1: &mut Option<(TokenLocation, ast::NumberExpression, TokenLocation)>,
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
            Some((_, Tok::Times, _)) |
            Some((_, Tok::Plus, _)) |
            Some((_, Tok::Minus, _)) |
            Some((_, Tok::Div, _)) |
            Some((_, Tok::NewLine, _)) => {
                let __sym0 = __sym0.take().unwrap();
                let __sym1 = __sym1.take().unwrap();
                let __sym2 = __sym2.take().unwrap();
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action28(__sym0, __sym1, __sym2);
                let __nt = __Nonterminal::NumTerm((
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
    ast::Block::Definitions(Box::new(__0))
}

pub fn __action3<
>(
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, l, _): (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ast::Block
{
    ast::Block::Show(Box::new(l))
}

pub fn __action4<
>(
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, l, _): (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ast::Block
{
    ast::Block::Hide(Box::new(l))
}

pub fn __action5<
>(
    (_, id, _): (TokenLocation, String, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
) -> ast::VarDefinition
{
    ast::VarDefinition { identifier: id, values: v }
}

pub fn __action6<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
    (_, __1, _): (TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
) -> ast::FilterInstruction
{
    ast::FilterInstruction::SetValue(__0, __1)
}

pub fn __action7<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
    (_, __1, _): (TokenLocation, ast::Condition, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
) -> ast::FilterInstruction
{
    ast::FilterInstruction::Condition(__0, __1)
}

pub fn __action8<
>(
    (_, __0, _): (TokenLocation, ast::VarDefinition, TokenLocation),
) -> ast::FilterInstruction
{
    ast::FilterInstruction::Var(__0)
}

pub fn __action9<
>(
    (_, __0, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::Value
{
    ast::Value::Num(__0)
}

pub fn __action10<
>(
    (_, __0, _): (TokenLocation, ast::StringBox, TokenLocation),
) -> ast::Value
{
    ast::Value::Str(__0)
}

pub fn __action11<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
) -> ast::Value
{
    ast::Value::Var(__0)
}

pub fn __action12<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
) -> ast::StringBox
{
    ast::StringBox::Value(__0)
}

pub fn __action13<
>(
    (_, __0, _): (TokenLocation, String, TokenLocation),
) -> String
{
    String::from(__0)
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
    (_, op, _): (TokenLocation, ast::ComparisonOperator, TokenLocation),
    (_, v, _): (TokenLocation, ast::Value, TokenLocation),
) -> ast::Condition
{
    ast::Condition { value: v, operator: op }
}

pub fn __action16<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Gte
}

pub fn __action17<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Gt
}

pub fn __action18<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Lte
}

pub fn __action19<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Lt
}

pub fn __action20<
>(
    (_, __0, _): (TokenLocation, Tok, TokenLocation),
) -> ast::ComparisonOperator
{
    ast::ComparisonOperator::Eq
}

pub fn __action21<
>(
    (_, l, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, r, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::NumberExpression
{
    ast::NumberExpression::Op(Box::new(l), ast::NumberOperation::Add, Box::new(r))
}

pub fn __action22<
>(
    (_, l, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, r, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::NumberExpression
{
    ast::NumberExpression::Op(Box::new(l), ast::NumberOperation::Sub, Box::new(r))
}

pub fn __action23<
>(
    (_, __0, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::NumberExpression
{
    (__0)
}

pub fn __action24<
>(
    (_, l, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, r, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::NumberExpression
{
    ast::NumberExpression::Op(Box::new(l), ast::NumberOperation::Mul, Box::new(r))
}

pub fn __action25<
>(
    (_, l, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, r, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::NumberExpression
{
    ast::NumberExpression::Op(Box::new(l), ast::NumberOperation::Div, Box::new(r))
}

pub fn __action26<
>(
    (_, __0, _): (TokenLocation, ast::NumberExpression, TokenLocation),
) -> ast::NumberExpression
{
    (__0)
}

pub fn __action27<
>(
    (_, __0, _): (TokenLocation, i32, TokenLocation),
) -> ast::NumberExpression
{
    ast::NumberExpression::Number(ast::NumberBox::IntValue(__0))
}

pub fn __action28<
>(
    (_, _, _): (TokenLocation, Tok, TokenLocation),
    (_, __0, _): (TokenLocation, ast::NumberExpression, TokenLocation),
    (_, _, _): (TokenLocation, Tok, TokenLocation),
) -> ast::NumberExpression
{
    __0
}

pub fn __action29<
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

pub fn __action30<
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

pub fn __action31<
>(
    (_, __0, _): (TokenLocation, ast::Value, TokenLocation),
) -> ::std::vec::Vec<ast::Value>
{
    vec![__0]
}

pub fn __action32<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Value>, TokenLocation),
    (_, e, _): (TokenLocation, ast::Value, TokenLocation),
) -> ::std::vec::Vec<ast::Value>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action33<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> ::std::vec::Vec<ast::FilterInstruction>
{
    vec![]
}

pub fn __action34<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ::std::vec::Vec<ast::FilterInstruction>
{
    v
}

pub fn __action35<
>(
    (_, __0, _): (TokenLocation, ast::VarDefinition, TokenLocation),
) -> ::std::vec::Vec<ast::VarDefinition>
{
    vec![__0]
}

pub fn __action36<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::VarDefinition>, TokenLocation),
    (_, e, _): (TokenLocation, ast::VarDefinition, TokenLocation),
) -> ::std::vec::Vec<ast::VarDefinition>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action37<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> ::std::vec::Vec<ast::Block>
{
    vec![]
}

pub fn __action38<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> ::std::vec::Vec<ast::Block>
{
    v
}

pub fn __action39<
>(
    (_, __0, _): (TokenLocation, ast::Block, TokenLocation),
) -> ::std::option::Option<ast::Block>
{
    Some(__0)
}

pub fn __action40<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> ::std::option::Option<ast::Block>
{
    None
}

pub fn __action41<
>(
    (_, __0, _): (TokenLocation, ast::Block, TokenLocation),
) -> ::std::vec::Vec<ast::Block>
{
    vec![__0]
}

pub fn __action42<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
    (_, e, _): (TokenLocation, ast::Block, TokenLocation),
) -> ::std::vec::Vec<ast::Block>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action43<
>(
    (_, __0, _): (TokenLocation, ast::FilterInstruction, TokenLocation),
) -> ::std::vec::Vec<ast::FilterInstruction>
{
    vec![__0]
}

pub fn __action44<
>(
    (_, v, _): (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
    (_, e, _): (TokenLocation, ast::FilterInstruction, TokenLocation),
) -> ::std::vec::Vec<ast::FilterInstruction>
{
    { let mut v = v; v.push(e); v }
}

pub fn __action45<
>(
    __0: (TokenLocation, ::std::option::Option<ast::Block>, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action37(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __0,
        __temp0,
    )
}

pub fn __action46<
>(
    __0: (TokenLocation, ::std::option::Option<ast::Block>, TokenLocation),
    __1: (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action38(
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __0,
        __temp0,
    )
}

pub fn __action47<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
) -> ast::Block
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action33(
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

pub fn __action48<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
    __2: (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ast::Block
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action34(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action3(
        __0,
        __1,
        __temp0,
    )
}

pub fn __action49<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
) -> ast::Block
{
    let __start0 = __1.2.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action33(
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

pub fn __action50<
>(
    __0: (TokenLocation, Tok, TokenLocation),
    __1: (TokenLocation, Tok, TokenLocation),
    __2: (TokenLocation, ::std::vec::Vec<ast::FilterInstruction>, TokenLocation),
) -> ast::Block
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action34(
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        __0,
        __1,
        __temp0,
    )
}

pub fn __action51<
>(
    __0: (TokenLocation, ast::Block, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action39(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        __temp0,
    )
}

pub fn __action52<
>(
    __lookbehind: &TokenLocation,
    __lookahead: &TokenLocation,
) -> Box<Vec<ast::Block>>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        __temp0,
    )
}

pub fn __action53<
>(
    __0: (TokenLocation, ast::Block, TokenLocation),
    __1: (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action39(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        __temp0,
        __1,
    )
}

pub fn __action54<
>(
    __0: (TokenLocation, ::std::vec::Vec<ast::Block>, TokenLocation),
) -> Box<Vec<ast::Block>>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action40(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        __temp0,
        __0,
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
