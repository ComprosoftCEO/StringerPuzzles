use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::path::Path;

/// A program is a set of rules to run on the given input string
#[derive(Debug, Clone)]
pub struct Program {
  rules: Vec<Rule>,
}

/// Data type for the state of the program when executing
#[derive(Debug, Clone)]
pub struct ProgramState(HashSet<usize>);

/// Single rule to handle substitution
#[derive(Debug, Clone)]
pub struct Rule {
  left: String,
  right: String,
  once: bool,
}

#[allow(unused)]
impl Rule {
  pub fn new(left: impl Into<String>, right: impl Into<String>, once: bool) -> Self {
    Rule {
      left: left.into(),
      right: right.into(),
      once,
    }
  }

  pub fn left(&self) -> &str {
    &self.left
  }

  pub fn right(&self) -> &str {
    &self.right
  }

  pub fn once(&self) -> bool {
    self.once
  }
}

impl fmt::Display for Rule {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    if self.once {
      write!(f, "{}:={}", self.left, self.right)
    } else {
      write!(f, "{}={}", self.left, self.right)
    }
  }
}

impl ProgramState {
  /// Construct a new program state
  pub fn new() -> Self {
    Self(HashSet::new())
  }
}

impl Program {
  /// Load and parse a program from a file
  pub fn from_file<P: AsRef<Path>>(file: P) -> io::Result<Self> {
    let mut rules = Vec::new();

    let file = File::open(file)?;
    for (line, line_number) in BufReader::new(file).lines().zip(1..) {
      let line: String = line?;

      let rule = match Self::try_parse_line(&line) {
        Ok(None) => continue,
        Ok(Some(rule)) => rule,
        Err(_) => {
          return Err(io::Error::new(
            ErrorKind::InvalidData,
            format!("invalid rule '{line}' on line {line_number}"),
          ))
        },
      };

      rules.push(rule);
    }

    Ok(Self { rules })
  }

  /// Try to parse a single line in the program
  fn try_parse_line(line: &str) -> Result<Option<Rule>, ()> {
    // Search for an '=' equal sign
    let equal_sign = match line.find("=") {
      None => return Ok(None),
      Some(index) => index,
    };

    // Make sure there is only one equals sign
    //  Otherwise, we have a badly-formed substitution
    if line.rfind("=").unwrap() != equal_sign {
      return Err(());
    }

    // Check for the prescence of the once ':' operator
    //  Make sure there is only one operation right before '=' sign
    let is_once = match line.find(":") {
      None => false,
      Some(index) if line.rfind(":").unwrap() != index => return Err(()),
      Some(index) if index == equal_sign - 1 => true,
      _ => return Err(()),
    };

    // Extract the left and right sides of the rule
    let left_side = if is_once {
      String::from(&line[0..(equal_sign - 1)])
    } else {
      String::from(&line[0..equal_sign])
    };
    let right_side = String::from(&line[(equal_sign + 1)..]);

    Ok(Some(Rule::new(left_side, right_side, is_once)))
  }

  /// Print all rules in the program
  pub fn print_rules(&self) {
    for rule in self.rules.iter() {
      println!("{rule}");
    }
    println!("");
  }

  /// Execute the first matching rule found, returns None if no rules matched
  ///   This method requires a program state to be stored between invocations
  pub fn execute_rule(&self, input: &str, state: &mut ProgramState) -> Option<(String, &Rule)> {
    for (rule, rule_index) in self.rules.iter().zip(0..) {
      // See if input contains the string
      if !input.contains(rule.left()) {
        continue;
      }

      // Special case for "once" rules
      if rule.once() {
        if state.0.contains(&rule_index) {
          continue;
        } else {
          state.0.insert(rule_index);
        }
      }

      // Apply the rule to the first instance in the string
      let new_string = input.replacen(rule.left(), rule.right(), 1);
      return Some((new_string, rule));
    }

    None
  }
}
