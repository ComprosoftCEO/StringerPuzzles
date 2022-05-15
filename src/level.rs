use rand::prelude::*;
use rlua::prelude::*;
use serde::Deserialize;
use std::error::Error;
use std::fs;

use crate::level_pack::PACKS_FOLDER;
use crate::program::{Program, ProgramState};

const NUM_EXAMPLES: usize = 5;
const NUM_TEST_CASES: usize = 100;
const MAX_EXECUTIONS: usize = 100_000; /* 100 Thousand */
const TEST_CASE_SEED: u32 = 12345;

/// Single entry in the levels.json file
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Level {
  name: String,
  description: String,
  lua_file: String,
}

#[allow(unused)]
impl Level {
  /// Construct a new level data entry
  pub fn new(name: impl Into<String>, description: impl Into<String>, lua_file: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      description: description.into(),
      lua_file: lua_file.into(),
    }
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn description(&self) -> &str {
    &self.description
  }

  pub fn lua_file(&self) -> &str {
    &self.lua_file
  }

  ///
  /// Print the full level details along with some examples
  ///
  pub fn print_level_details(&self, level_number: usize, level_code: &str, parent_folder: &str) {
    println!("Level {}: {}", level_number, self.name);
    println!("  Code: {}\n", level_code);
    println!("{}\n", self.description);

    println!("Examples:\n");

    let test_cases = match self.generate_test_cases(thread_rng().gen(), NUM_EXAMPLES, parent_folder) {
      Ok(t) => t,
      Err(e) => {
        println!("Failed to load and run Lua file: {}", e);
        return;
      },
    };

    for (input, output) in test_cases {
      println!("Input:  {input}");
      println!("Output: {output}\n");
    }
  }

  ///
  /// See if the given rules passes all of the test cases
  ///
  pub fn validate_code(&self, code: &Program, parent_folder: &str) -> bool {
    let test_cases = match self.generate_test_cases(TEST_CASE_SEED, NUM_TEST_CASES, parent_folder) {
      Ok(t) => t,
      Err(e) => {
        println!("Failed to load and run Lua file: {}", e);
        return false;
      },
    };

    // Run through the test cases one-by-one
    for ((mut input, output), test_case_number) in test_cases.into_iter().zip(1..) {
      println!("===== Test case {test_case_number}: =====\n  Input:  {input}\n  Output: {output}\n");

      // Keep applying executions until no more to apply or we time out
      let mut execution = 0;
      let mut state = ProgramState::new();
      while execution < MAX_EXECUTIONS {
        input = match code.execute_rule(&input, &mut state) {
          None => break,
          Some((new_string, rule)) => {
            println!("Rule: {rule}\n{new_string}\n");
            new_string
          },
        };

        execution += 1;
      }

      // Print error if the execution timed out
      if execution >= MAX_EXECUTIONS {
        println!("Error! Program exceeded maximum number of executions ({MAX_EXECUTIONS})");
        return false;
      }

      println!("Finished");
      if input != output {
        println!("Error! String does not match expected output");
        println!("  Given:    {input}");
        println!("  Expected: {output}\n");
        return false;
      }
      println!("Passed test case {test_case_number}\n");
    }

    // All test cases passed
    true
  }

  ///
  /// Load and run the Lua code to generate the test cases
  ///
  fn generate_test_cases(
    &self,
    seed: u32,
    n: usize,
    parent_folder: &str,
  ) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    // Try to load the Lua code file into memory
    let lua_code = fs::read_to_string(format!("{PACKS_FOLDER}/{parent_folder}/{}", self.lua_file))?;

    // Generate and run the code within the Lua context
    let test_cases = Lua::new().context::<_, LuaResult<Vec<(String, String)>>>(|ctx| {
      let globals = ctx.globals();

      // Add the levels folder to the path
      ctx
        .load(&format!(
          r#"package.path = "./{PACKS_FOLDER}/{parent_folder}/?.lua;" .. package.path"#
        ))
        .exec()?;

      // Seed the random number generator
      globals
        .get::<_, LuaTable>("math")?
        .get::<_, LuaFunction>("randomseed")?
        .call::<_, ()>(seed)?;

      // Load the script code
      //  This should define a global function named "generateTestCase"
      ctx.load(&lua_code).exec()?;

      // Generate the test cases one-by-one
      let generate_test_case: LuaFunction = globals.get("generateTestCase")?;
      let test_cases = (0..n)
        .map(|_| generate_test_case.call(()))
        .collect::<Result<Vec<(String, String)>, _>>()?;

      Ok(test_cases)
    })?;

    Ok(test_cases)
  }
}
