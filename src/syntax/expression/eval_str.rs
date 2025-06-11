use crate::{Error, Expression, Member, PartNamespace, tokenize};

/// Evaluate an `Expression` from a &str and return the result.
///
/// This method is not used directly but only for testing and demonstration purposes.
pub fn eval_str(input: &str) -> Result<Member, Error> {
    Expression::from_tokens(&tokenize(input, &"".into())?)?.evaluate(&PartNamespace::new())
}
