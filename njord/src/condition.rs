pub enum Condition {
    Eq(String, String),
    Ne(String, String),
    Lt(String, String),
    Gt(String, String),
    Le(String, String),
    Ge(String, String),
    And(Box<Condition>, Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
}

impl Condition {
    fn is_numeric(value: &str) -> bool {
        value.parse::<f64>().is_ok() || value.parse::<i64>().is_ok()
    }

    pub fn build(&self) -> String {
        match self {
            Condition::Eq(column, value) => {
                if Condition::is_numeric(value) {
                    format!("{} = {}", column, value)
                } else {
                    format!("{} = '{}'", column, value)
                }
            }
            Condition::Ne(column, value) => {
                if Condition::is_numeric(value) {
                    format!("{} <> {}", column, value)
                } else {
                    format!("{} <> '{}'", column, value)
                }
            }
            Condition::Lt(column, value) => {
                if Condition::is_numeric(value) {
                    format!("{} < {}", column, value)
                } else {
                    format!("{} < '{}'", column, value)
                }
            }
            Condition::Gt(column, value) => {
                if Condition::is_numeric(value) {
                    format!("{} > {}", column, value)
                } else {
                    format!("{} > '{}'", column, value)
                }
            }
            Condition::Le(column, value) => {
                if Condition::is_numeric(value) {
                    format!("{} <= {}", column, value)
                } else {
                    format!("{} <= '{}'", column, value)
                }
            }
            Condition::Ge(column, value) => {
                if Condition::is_numeric(value) {
                    format!("{} >= {}", column, value)
                } else {
                    format!("{} >= '{}'", column, value)
                }
            }
            Condition::And(left, right) => format!("({}) AND ({})", left.build(), right.build()),
            Condition::Or(left, right) => format!("({}) OR ({})", left.build(), right.build()),
        }
    }
}
