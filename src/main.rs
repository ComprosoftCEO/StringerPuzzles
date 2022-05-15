mod level;
mod level_pack;
mod program;

use level_pack::AllLevelPacks;
use program::Program;
use std::path::PathBuf;
use structopt::StructOpt;

/// Fun string substitution puzzles
#[derive(StructOpt)]
struct Opt {
  /// Code file to run
  #[structopt(parse(from_os_str))]
  code_file: Option<PathBuf>,

  /// Special passcode for a given level
  #[structopt(short, long)]
  level_code: Option<String>,

  /// Code for a specific level pack
  #[structopt(short = "p", long)]
  pack_code: Option<String>,

  /// List all of the level codes (cheat flag)
  #[structopt(long, hidden = true)]
  show_codes: bool,
}

fn main() {
  let opt: Opt = Opt::from_args();

  // Try to load the levels
  let all_packs = AllLevelPacks::load();

  // Cheat option to show the codes
  if opt.show_codes {
    return match opt.pack_code {
      Some(code) => match all_packs.get_level_pack(&code) {
        None => println!("Unknown level pack ID '{}'", code),
        Some(pack) => pack.print_level_codes(),
      },
      None => all_packs.print_level_codes(),
    };
  }

  let level_code = match opt.level_code {
    None => match opt.pack_code {
      None => return all_packs.print_loaded_packs(),
      Some(code) => match all_packs.get_level_pack(&code) {
        None => return println!("Unknown level pack ID '{}'", code),
        Some(pack) => return pack.print(&code),
      },
    },
    Some(code) => code,
  };
  let code_file = opt.code_file;

  // Search for the level
  let (pack_code, level_number, level) =
    match all_packs.get_level(&level_code, opt.pack_code.as_ref().map(|s| s.as_str())) {
      levels if levels.len() == 0 => {
        println!("Error! Unknown level code '{level_code}'");
        return;
      },
      levels if levels.len() > 1 => {
        println!("Ambiguous level code '{level_code}'.");
        println!("Please specify one of the following level packs:");

        for (code, _, _) in levels {
          let pack = all_packs.get_level_pack(code).unwrap(); // Will not fail
          println!("  {} = {}", code, pack.name());
        }

        return;
      },

      levels => levels[0],
    };
  let level_pack = all_packs.get_level_pack(pack_code).unwrap(); // Will not fail

  // Show the level description if no code file provided
  if code_file.is_none() {
    level.print_level_details(level_number, &level_code, level_pack.parent_folder());
    return;
  }

  // Parse the code filProgram
  let program = match Program::from_file(code_file.unwrap()) {
    Ok(e) => e,
    Err(e) => {
      println!("Error loading code file: {}", e);
      return;
    },
  };

  // Always show a shortened level description
  println!("Level {}: {}", level_number, level.name());
  println!("  Code: {}\n", level_code);

  // Show the list of loaded rules
  println!("----- Loaded Rules: -----");
  program.print_rules();

  // Try the test cases on the level
  if !level.validate_code(&program, level_pack.parent_folder()) {
    return;
  }

  println!("Success! All test cases passed!\n");

  // Show the next level code
  match level_pack.get_next_level_code(&level_code) {
    Some(next_code) => {
      println!("Level {} code: {}", level_number + 1, next_code);
    },

    None => {
      println!("{}", level_pack.win_message());
    },
  }
}
