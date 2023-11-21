pub enum Condition {
    Eq(String, String),
    Ne(String, String),
    Lt(String, String),
    Gt(String, String),
    Le(String, String),
    Ge(String, String),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    Comparison(String, String, String),
}

impl Condition {
    pub fn build(&self) -> String {
        match self {
            Condition::Eq(column, value) => format!("{} = '{}'", column, value),
            Condition::Ne(column, value) => format!("{} <> '{}'", column, value),
            Condition::Lt(column, value) => format!("{} < '{}'", column, value),
            Condition::Gt(column, value) => format!("{} > '{}'", column, value),
            Condition::Le(column, value) => format!("{} <= '{}'", column, value),
            Condition::Ge(column, value) => format!("{} >= '{}'", column, value),
            Condition::And(left, right) => format!("({}) AND ({})", left.build(), right.build()),
            Condition::Or(left, right) => format!("({}) OR ({})", left.build(), right.build()),
            Condition::Comparison(column, op, value) => format!("{} {} '{}'", column, op, value),
        }
    }
}
