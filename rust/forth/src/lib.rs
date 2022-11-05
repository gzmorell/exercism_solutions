pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    values : std::vec::Vec<Value>
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub enum Asociativity {
    Left,
    Right,
}

pub trait Operator {
    fn arity(&self) -> i8;
    fn priority(&self) -> i8;
    fn association(&self) -> Asociativity;
}

pub enum Token<'a> {
    Number(i32),
    Operator(&'a dyn Operator),
    Word(String),
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            values : std::vec::Vec::new()
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.values
    }

    pub fn eval(&mut self, input: &str) -> Result {
        input.split(' ').for_each(|x| self.values.push(x.parse::<i32>().unwrap()));
        Ok(())
    }
}
