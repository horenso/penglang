#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Int(i64),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
}

impl Expression {
    pub fn eval(self) -> Result<i64, String> {
        match self {
            Expression::Int(value) => Ok(value),
            Expression::Add(left, right) => Ok(left.eval()? + right.eval()?),
            Expression::Sub(left, right) => Ok(left.eval()? - right.eval()?),
            Expression::Mul(left, right) => Ok(left.eval()? * right.eval()?),
            Expression::Div(left, right) => {
                let right_val = right.eval()?;
                if right_val != 0 {
                    Ok(left.eval()? / right_val)
                } else {
                    Err(String::from("Division by zero"))
                }
            }
            Expression::Power(base, power) => Ok(base.eval()?.pow(power.eval()? as u32)),
        }
    }
}
