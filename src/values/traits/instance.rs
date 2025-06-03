use crate::{Error, Span, Value};

pub trait TypeInstance {
    fn method_call(&self, method: &str, args: &[Value], span: Span) -> Result<Value, Error>;
    fn type_str(&self) -> String;
}

pub fn check_args(args: &[Value], should: Vec<&str>, span: Span) -> Result<(), Error> {
    if args.len() != should.len() {
        return Err(Error::ArgumentNumber {
            should: should.len(),
            is: args.len(),
            span,
        });
    }
    for (i, arg) in args.iter().enumerate() {
        if arg.type_str() != *should[i] {
            return Err(Error::ArgumentType {
                should: should[i].to_string(),
                span,
            });
        }
    }

    Ok(())
}
