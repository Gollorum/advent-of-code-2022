use std::collections::HashMap;
use crate::utils;
use crate::utils::ErrorMsg;
use std::str::FromStr;

pub fn run_sample() {
    ErrorMsg::print(run("input/day21_sample.txt"));
}

pub fn run_actual() {
    ErrorMsg::print(run("input/day21.txt"));
}

trait Expression {
    fn eval(&self, expressions: &HashMap<String, Box<dyn Expression>>) -> Result<i64, ErrorMsg>;
    fn inv_eval(&self, expressions: &HashMap<String, Box<dyn Expression>>, expected_res: i64, to_expand: Vec<String>) -> Result<i64, ErrorMsg>;
    fn children(&self) -> Vec<String>;
}
struct Lit { val: i64 }
impl Expression for Lit { 
    fn eval(&self, _: &HashMap<String, Box<dyn Expression>>) -> Result<i64, ErrorMsg> {
        Ok(self.val) 
    }

    fn inv_eval(&self, _: &HashMap<String, Box<dyn Expression>>, expected_res: i64, to_expand: Vec<String>) -> Result<i64, ErrorMsg> {
        assert!(to_expand.is_empty());
        Ok(expected_res)
    }

    fn children(&self) -> Vec<String> { Vec::new() }
}
struct Add { l: String, r: String }
impl Expression for Add { 
    fn eval(&self, expressions: &HashMap<String, Box<dyn Expression>>) -> Result<i64, ErrorMsg> { 
        Ok(expressions[&self.l].eval(expressions)? + expressions[&self.r].eval(expressions)?)
    }

    fn inv_eval(&self, expressions: &HashMap<String, Box<dyn Expression>>, expected_res: i64, mut to_expand: Vec<String>) -> Result<i64, ErrorMsg> {
        let e = to_expand.pop().ok_or(ErrorMsg::new("Unexpected end of expansion"))?;
        if self.l == e {
            expressions[&self.l].inv_eval(expressions, expected_res - expressions[&self.r].eval(expressions)?, to_expand)
        } else if self.r == e {
            expressions[&self.r].inv_eval(expressions, expected_res - expressions[&self.l].eval(expressions)?, to_expand)
        } else { Err(ErrorMsg::new("Failed to expand: Queried entry was not present")) }
    }

    fn children(&self) -> Vec<String> { vec![self.l.clone(), self.r.clone()] }
}
struct Mul { l: String, r: String }
impl Expression for Mul {
    fn eval(&self, expressions: &HashMap<String, Box<dyn Expression>>) -> Result<i64, ErrorMsg> {
        Ok(expressions[&self.l].eval(expressions)? * expressions[&self.r].eval(expressions)?)
    }

    fn inv_eval(&self, expressions: &HashMap<String, Box<dyn Expression>>, expected_res: i64, mut to_expand: Vec<String>) -> Result<i64, ErrorMsg> {
        let e = to_expand.pop().ok_or(ErrorMsg::new("Unexpected end of expansion"))?;
        if self.l == e {
            expressions[&self.l].inv_eval(expressions, expected_res / expressions[&self.r].eval(expressions)?, to_expand)
        } else if self.r == e {
            expressions[&self.r].inv_eval(expressions, expected_res / expressions[&self.l].eval(expressions)?, to_expand)
        } else { Err(ErrorMsg::new("Failed to expand: Queried entry was not present")) }
    }

    fn children(&self) -> Vec<String> { vec![self.l.clone(), self.r.clone()] }
}
struct Sub { l: String, r: String }
impl Expression for Sub {
    fn eval(&self, expressions: &HashMap<String, Box<dyn Expression>>) -> Result<i64, ErrorMsg> {
        Ok(expressions[&self.l].eval(expressions)? - expressions[&self.r].eval(expressions)?)
    }

    fn inv_eval(&self, expressions: &HashMap<String, Box<dyn Expression>>, expected_res: i64, mut to_expand: Vec<String>) -> Result<i64, ErrorMsg> {
        let e = to_expand.pop().ok_or(ErrorMsg::new("Unexpected end of expansion"))?;
        if self.l == e {
            expressions[&self.l].inv_eval(expressions, expected_res + expressions[&self.r].eval(expressions)?, to_expand)
        } else if self.r == e {
            expressions[&self.r].inv_eval(expressions, expressions[&self.l].eval(expressions)? - expected_res, to_expand)
        } else { Err(ErrorMsg::new("Failed to expand: Queried entry was not present")) }
    }

    fn children(&self) -> Vec<String> { vec![self.l.clone(), self.r.clone()] }
}
struct Div { l: String, r: String }
impl Expression for Div {
    fn eval(&self, expressions: &HashMap<String, Box<dyn Expression>>) -> Result<i64, ErrorMsg> {
        Ok(expressions[&self.l].eval(expressions)? / expressions[&self.r].eval(expressions)?)
    }

    fn inv_eval(&self, expressions: &HashMap<String, Box<dyn Expression>>, expected_res: i64, mut to_expand: Vec<String>) -> Result<i64, ErrorMsg> {
        let e = to_expand.pop().ok_or(ErrorMsg::new("Unexpected end of expansion"))?;
        if self.l == e {
            expressions[&self.l].inv_eval(expressions, expected_res * expressions[&self.r].eval(expressions)?, to_expand)
        } else if self.r == e {
            expressions[&self.r].inv_eval(expressions, expressions[&self.l].eval(expressions)? / expected_res, to_expand)
        } else { Err(ErrorMsg::new("Failed to expand: Queried entry was not present")) }
    }

    fn children(&self) -> Vec<String> { vec![self.l.clone(), self.r.clone()] }
}

impl FromStr for Box<dyn Expression> {
    type Err = ErrorMsg;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().nth(11) == Some('+') {
            Ok(Box::new(Add {
                l: s[6..10].to_string(),
                r: s[13..17].to_string()
            }))
        } else if s.chars().nth(11) == Some('-') {
            Ok(Box::new(Sub {
                l: s[6..10].to_string(),
                r: s[13..17].to_string()
            }))
        } else if s.chars().nth(11) == Some('*') {
            Ok(Box::new(Mul {
                l: s[6..10].to_string(),
                r: s[13..17].to_string()
            }))
        } else if s.chars().nth(11) == Some('/') {
            Ok(Box::new(Div {
                l: s[6..10].to_string(),
                r: s[13..17].to_string()
            }))
        } else {Ok(Box::new(Lit{val:s[6..].parse()?}))}
    }

}

fn parse_entry(res_s: std::io::Result<String>) -> Result<(String, Box<dyn Expression>), ErrorMsg> {
    let s = res_s?;
    Ok((s[..4].to_string(), s.parse::<Box<dyn Expression>>()?))
}

fn run(path: &str) -> Result<(), ErrorMsg> {
    let mut expressions: HashMap<String, Box<dyn Expression>> = HashMap::new();
    let mut parent_of : HashMap<String, String> = HashMap::new();
    for s in utils::read_lines(path)? {
        let e = parse_entry(s)?;
        for child in e.1.children() {
            parent_of.insert(child, e.0.clone());
        }
        expressions.insert(e.0, e.1);
    }
    let root = &expressions["root"];
    let result = root.eval(&expressions)?;

    let mut inv_search: Vec<String> = vec!["humn".to_string()];
    loop {
        let parent = &parent_of[inv_search.last().unwrap()];
        if parent == "root" {
            break;
        } else {
            inv_search.push(parent.clone());
        }
    }
    let expected = expressions[root.children().iter().find(|c| c != &inv_search.last().unwrap()).unwrap()].eval(&expressions)?;
    let inv_res = expressions[&inv_search.pop().unwrap()].inv_eval(&expressions, expected, inv_search)?;
    
    Ok(println!("Aaaand the result iiis ...: {result}!!!!! But I actually expected you to say {inv_res}"))
}