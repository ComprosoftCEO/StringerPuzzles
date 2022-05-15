use rand::prelude::*;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::fs::{self, File};
use std::io::{self, BufReader, ErrorKind};
use std::path::Path;

use crate::level::Level;

pub type LevelNumber = usize;

pub const PACKS_FOLDER: &str = "packs";
pub const PACK_JSON_FILE: &str = "pack.json";

const CODE_LENGTH: usize = 6;
static CODE_CHARS: &[char] = &[
  'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z', '1', '2',
  '3', '4', '5', '6', '7', '8', '9', '0',
];

static DEFAULT_WIN_MESSAGE: &str = "Congratulations! You solved all puzzles in the level pack. Good job!";

/// Helper function to generate a random code given the pseudo-random number generator
#[inline]
fn generate_single_code<R: Rng + ?Sized>(rng: &mut R) -> String {
  (0..CODE_LENGTH)
    .map(|_| CODE_CHARS.choose(rng).cloned().unwrap_or('0'))
    .collect()
}

/// Store all loaded level packs inside a single data structure
#[derive(Debug, Clone, Default)]
pub struct AllLevelPacks {
  level_packs: BTreeMap<String, LevelPack>,
}

/// Stores all details about a single level package
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LevelPack {
  id: String,
  name: String,
  #[serde(default = "default_version")]
  version: String,
  description: String,
  levels: Vec<Level>,
  win_message: Option<String>,

  // Name of the parent folder
  #[serde(skip)]
  parent_folder: String,

  // Internal fields for storing the level codes
  #[serde(skip)]
  starting_code: String,
  #[serde(skip)]
  codes: HashMap<String, usize>,
  #[serde(skip)]
  next_level_code: HashMap<String, String>,
}

#[inline]
fn default_version() -> String {
  "1.0.0".into()
}

#[allow(unused)]
impl LevelPack {
  ///
  /// Load a level pack from a folder
  ///   Returns an error if there are no levels inside the pack file
  ///
  pub fn from_file<P: AsRef<Path>>(json_pack_file: P) -> io::Result<Self> {
    // Parse the level as a JSON file
    let file = File::open(json_pack_file)?;
    let reader = BufReader::new(file);
    let mut me: Self = serde_json::from_reader(reader)?;

    // Make sure there is at least one level
    if me.levels.len() == 0 {
      Err(io::Error::new(
        ErrorKind::InvalidData,
        format!("No levels provided in pack file"),
      ))?;
    }

    // Generate the level codes
    let mut rng: Pcg64 = Seeder::from(format!("{}-{}", me.id, me.version)).make_rng();
    let mut last_level_code = String::new();

    for index in 0..me.levels.len() {
      let mut code = generate_single_code(&mut rng);
      while me.codes.get(&code).is_some() {
        code = generate_single_code(&mut rng);
      }

      me.codes.insert(code.clone(), index);

      if index == 0 {
        me.starting_code = code.clone();
      } else if index > 0 {
        me.next_level_code.insert(last_level_code, code.clone());
      }

      last_level_code = code;
    }

    Ok(me)
  }

  pub fn id(&self) -> &str {
    &self.id
  }

  pub fn version(&self) -> &str {
    &self.version
  }

  pub fn name(&self) -> &str {
    &self.name
  }

  pub fn description(&self) -> &str {
    &self.description
  }

  pub fn win_message(&self) -> &str {
    self
      .win_message
      .as_ref()
      .map(String::as_str)
      .unwrap_or(DEFAULT_WIN_MESSAGE)
  }

  pub fn parent_folder(&self) -> &str {
    &self.parent_folder
  }

  /// Get the code for the first level in the pack
  pub fn get_starting_code(&self) -> &str {
    &self.starting_code
  }

  /// Get a level given the level code, or None if the level does not exist
  pub fn get_level_from_code(&self, code: &str) -> Option<(LevelNumber, &Level)> {
    self
      .codes
      .get(code)
      .and_then(|index| self.levels.get(*index).map(|level| (*index, level)))
  }

  /// Get the next level code given the code, or None if there are no more levels
  pub fn get_next_level_code(&self, code: &str) -> Option<&str> {
    self.next_level_code.get(code).map(String::as_str)
  }

  /// Print out details about the level pack
  pub fn print(&self, pack_code: &str) {
    println!("Level Pack: {}", self.name);
    println!("  Version: {}", self.version);
    println!("  Code: {}", pack_code);
    println!("\n{}", self.description);
    println!("\nLevel 1 Code: {}", self.get_starting_code());
  }

  /// Print the list of all level codes
  pub fn print_level_codes(&self) {
    let mut level_code = self.starting_code.as_str();
    for (level, level_number) in self.levels.iter().zip(1..) {
      println!("{} = Level {}: {}", level_code, level_number, level.name());
      level_code = self.get_next_level_code(level_code).unwrap_or(&"");
    }
  }
}

impl AllLevelPacks {
  /// Load any level packs
  ///   Returns an empty list of packs on errors
  pub fn load() -> Self {
    let result: io::Result<_> = (|| {
      let mut level_packs = Vec::new();
      for entry in fs::read_dir(PACKS_FOLDER)? {
        let entry = entry?;

        // Only directories are level packs
        let mut path = entry.path();
        if path.is_dir() {
          // Make sure the path has a pack.json file
          path.push(PACK_JSON_FILE);
          if !Path::new(&path).exists() {
            continue;
          }

          // A badly formed level pack is not an error, just a warning
          let mut level_pack = match LevelPack::from_file(path) {
            Ok(pack) => pack,
            Err(e) => {
              println!("Warning: failed to load level pack: {}", e);
              continue;
            },
          };

          // Also set the parent folder
          level_pack.parent_folder = entry.file_name().into_string().expect("Invalid path string");
          level_packs.push(level_pack);
        }
      }

      Ok(level_packs)
    })();

    // Return an empty level pack on a file system error
    let packs = match result {
      Ok(packs) => packs,
      Err(e) => {
        println!("Failed to load level packs: {}", e);
        return Self { ..Default::default() };
      },
    };

    // Generate the level pack codes (for disambiguation if needed)
    let mut rng: Pcg64 = Seeder::from(PACK_JSON_FILE).make_rng();
    let mut level_packs = BTreeMap::new();

    for pack in packs {
      let mut code = generate_single_code(&mut rng);
      while level_packs.get(&code).is_some() {
        code = generate_single_code(&mut rng);
      }
      level_packs.insert(code, pack);
    }

    Self { level_packs }
  }

  /// Print the list of loaded packs
  pub fn print_loaded_packs(&self) {
    println!("--- Loaded Level Packs: ---");
    for (code, level_pack) in &self.level_packs {
      println!("{} = {}", code, level_pack.name());
      println!("  -> Level 1 Code: {}\n", level_pack.get_starting_code());
    }
  }

  /// Print the generated codes for every level pack
  pub fn print_level_codes(&self) {
    for (pack_code, level_pack) in &self.level_packs {
      println!("{} = {}", pack_code, level_pack.name);

      let mut level_code = level_pack.starting_code.as_str();
      for (level, level_number) in level_pack.levels.iter().zip(1..) {
        println!("  {} = Level {}: {}", level_code, level_number, level.name());
        level_code = level_pack.get_next_level_code(level_code).unwrap_or(&"");
      }
      println!("");
    }
  }

  /// Get a level pack given the code
  pub fn get_level_pack(&self, code: &str) -> Option<&LevelPack> {
    self.level_packs.get(code)
  }

  /// Get a level(s) given the level code
  ///   This could return 0, 1, or more levels
  pub fn get_level(&self, level_code: &str, pack_code: Option<&str>) -> Vec<(&str, LevelNumber, &Level)> {
    // Search only a single pack to get a specific level
    if let Some(code) = pack_code {
      return self
        .level_packs
        .get_key_value(code)
        .and_then(|(code, pack)| {
          pack
            .get_level_from_code(level_code)
            .map(|(num, level)| vec![(code.as_str(), num + 1, level)])
        })
        .unwrap_or_default();
    }

    // Search all packs
    self
      .level_packs
      .iter()
      .filter_map(|(code, pack)| {
        pack
          .get_level_from_code(level_code)
          .map(|(num, lvl)| (code.as_str(), num + 1, lvl))
      })
      .collect()
  }
}
