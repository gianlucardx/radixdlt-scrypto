use crate::ast::{Instruction, Transaction};
use crate::lexer::{Token, TokenKind};
use sbor::any::{Fields, Value};
use scrypto::types::{Address, Bid, Decimal, Rid};
use std::str::FromStr;

pub enum ParserError {
    UnexpectedEof,
    UnexpectedToken(Token),
    InvalidDecimal(String),
    InvalidAddress(String),
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[macro_export]
macro_rules! advance_return {
    ( $self:expr, $v:expr ) => {{
        $self.advance()?;
        Ok($v)
    }};
}

#[macro_export]
macro_rules! advance_expect {
    ( $self:expr, $v:path ) => {{
        let token = $self.advance()?;
        if !matches!(token.kind, $v) {
            return Err(ParserError::UnexpectedToken(token));
        }
        token
    }};
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn is_eof(&self) -> bool {
        self.current == self.tokens.len()
    }

    pub fn peek(&mut self) -> Result<Token, ParserError> {
        self.tokens
            .get(self.current)
            .cloned()
            .ok_or(ParserError::UnexpectedEof)
    }

    pub fn advance(&mut self) -> Result<Token, ParserError> {
        let token = self.peek()?;
        self.current += 1;
        Ok(token)
    }

    pub fn parse_transaction(&mut self) -> Result<Transaction, ParserError> {
        let mut instructions = Vec::<Instruction>::new();

        while !self.is_eof() {
            let token = self.advance()?;
            match token.kind {
                TokenKind::DeclareTempBucket => {
                    let name = self.parse_string()?;
                    instructions.push(Instruction::DeclareTempBucket { name });
                }
                TokenKind::DeclareTempBucketRef => {
                    let name = self.parse_string()?;
                    instructions.push(Instruction::DeclareTempBucketRef { name });
                }
                TokenKind::TakeFromContext => {
                    let amount = self.parse_decimal()?;
                    let resource_address = self.parse_address()?;
                    let to = self.parse_bucket()?;
                    instructions.push(Instruction::TakeFromContext {
                        amount,
                        resource_address,
                        to,
                    });
                }
                TokenKind::BorrowFromContext => {
                    let amount = self.parse_decimal()?;
                    let resource_address = self.parse_address()?;
                    let to = self.parse_bucket_ref()?;
                    instructions.push(Instruction::BorrowFromContext {
                        amount,
                        resource_address,
                        to,
                    });
                }
                TokenKind::CallFunction => {
                    let package_address = self.parse_address()?;
                    let blueprint_name = self.parse_string()?;
                    let function = self.parse_string()?;
                    let mut args = vec![];
                    while self.peek()?.kind != TokenKind::Semicolon {
                        args.push(self.parse_value()?);
                    }
                    instructions.push(Instruction::CallFunction {
                        package_address,
                        blueprint_name,
                        function,
                        args,
                    });
                }
                TokenKind::CallMethod => {
                    let component_address = self.parse_address()?;
                    let method = self.parse_string()?;
                    let mut args = vec![];
                    while self.peek()?.kind != TokenKind::Semicolon {
                        args.push(self.parse_value()?);
                    }
                    instructions.push(Instruction::CallMethod {
                        component_address,
                        method,
                        args,
                    });
                }
                TokenKind::DropAllBucketRefs => {
                    instructions.push(Instruction::DropAllBucketRefs);
                }
                TokenKind::DepositAllBuckets => {
                    let account = self.parse_address()?;
                    instructions.push(Instruction::DepositAllBuckets { account });
                }
                _ => {
                    return Err(ParserError::UnexpectedToken(token));
                }
            }
        }

        Ok(Transaction { instructions })
    }

    pub fn parse_value(&mut self) -> Result<Value, ParserError> {
        let token = self.peek()?;

        match token.kind {
            TokenKind::Unit => advance_return!(self, Value::Unit),
            TokenKind::True => advance_return!(self, Value::Bool(true)),
            TokenKind::False => advance_return!(self, Value::Bool(false)),
            TokenKind::U8(value) => advance_return!(self, Value::U8(value)),
            TokenKind::U16(value) => advance_return!(self, Value::U16(value)),
            TokenKind::U32(value) => advance_return!(self, Value::U32(value)),
            TokenKind::U64(value) => advance_return!(self, Value::U64(value)),
            TokenKind::I8(value) => advance_return!(self, Value::I8(value)),
            TokenKind::I16(value) => advance_return!(self, Value::I16(value)),
            TokenKind::I32(value) => advance_return!(self, Value::I32(value)),
            TokenKind::I64(value) => advance_return!(self, Value::I64(value)),
            TokenKind::String(value) => advance_return!(self, Value::String(value)),
            TokenKind::Struct => self.parse_struct(),
            TokenKind::Enum => self.parse_enum(),
            _ => Err(ParserError::UnexpectedToken(token)),
        }
    }

    /// Grammar:
    /// ```
    /// values = '(' ')' | '(' value (',' value)* ')'
    /// ```
    pub fn parse_values(&mut self) -> Result<Vec<Value>, ParserError> {
        let mut values = Vec::new();

        advance_expect!(self, TokenKind::OpenParenthesis);
        while self.peek()?.kind != TokenKind::CloseParenthesis {
            values.push(self.parse_value()?);
            if self.peek()?.kind != TokenKind::CloseParenthesis {
                advance_expect!(self, TokenKind::Comma);
            }
        }
        advance_expect!(self, TokenKind::CloseParenthesis);

        Ok(values)
    }

    /// Grammar:
    /// ```
    /// struct = 'struct' values
    /// ```
    pub fn parse_struct(&mut self) -> Result<Value, ParserError> {
        // TODO tuple struct and unit struct
        advance_expect!(self, TokenKind::Struct);
        let values = self.parse_values()?;
        Ok(Value::Struct(Fields::Named(values)))
    }

    pub fn parse_enum(&mut self) -> Result<Value, ParserError> {
        // TODO tuple struct and unit struct
        advance_expect!(self, TokenKind::Enum);
        let values = self.parse_values()?;
        Ok(Value::Struct(Fields::Named(values)))
    }

    pub fn parse_string(&mut self) -> Result<String, ParserError> {
        let token = self.advance()?;
        match token.kind {
            TokenKind::String(value) => Ok(value),
            _ => Err(ParserError::UnexpectedToken(token)),
        }
    }

    pub fn parse_decimal(&mut self) -> Result<Decimal, ParserError> {
        advance_expect!(self, TokenKind::Decimal);
        advance_expect!(self, TokenKind::OpenParenthesis);
        let s = self.parse_string()?;
        advance_expect!(self, TokenKind::CloseParenthesis);
        Decimal::from_str(&s).map_err(|_| ParserError::InvalidDecimal(s))
    }

    pub fn parse_address(&mut self) -> Result<Address, ParserError> {
        advance_expect!(self, TokenKind::Address);
        advance_expect!(self, TokenKind::OpenParenthesis);
        let s = self.parse_string()?;
        advance_expect!(self, TokenKind::CloseParenthesis);
        Address::from_str(&s).map_err(|_| ParserError::InvalidAddress(s))
    }

    pub fn parse_bucket(&mut self) -> Result<Bid, ParserError> {
        advance_expect!(self, TokenKind::Bucket);
        advance_expect!(self, TokenKind::OpenParenthesis);
        let token = self.advance()?;
        let bid = match &token.kind {
            TokenKind::String(_value) => {
                todo!()
            }
            TokenKind::U32(value) => Bid(*value),
            _ => return Err(ParserError::UnexpectedToken(token)),
        };
        advance_expect!(self, TokenKind::CloseParenthesis);
        Ok(bid)
    }

    pub fn parse_bucket_ref(&mut self) -> Result<Rid, ParserError> {
        advance_expect!(self, TokenKind::BucketRef);
        advance_expect!(self, TokenKind::OpenParenthesis);
        let token = self.advance()?;
        let rid = match &token.kind {
            TokenKind::String(_value) => {
                todo!()
            }
            TokenKind::U32(value) => Rid(*value),
            _ => return Err(ParserError::UnexpectedToken(token)),
        };
        advance_expect!(self, TokenKind::CloseParenthesis);
        Ok(rid)
    }
}
