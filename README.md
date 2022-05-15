# Stringer Puzzles

Programming puzzles using string substitution

## About

Stringer puzzles require you to write simple computer programs to compute the correct results on various text inputs.
The programming language is incredibly simple and only has two types of rules:

```
<left>=<right>
<left>:=<right>
```

`<left>` and `<right>` can be any combination of characters (including no characters) except for the colon or equal sign. A rule is terminated by a new line.

A program consists of one or more lines of rules that will get executed on the input test string.
The program tries rules sequentially from top to bottom. For each rule, it scans the string from left to right.
If the `<left>` string is found, then it replaces it by the `<right>` string. Otherwise, it tries the next rule in the program.
If no rules match, then the program ends. As a special case, rules with a `:=` (colon-equals) will only match at most once per test case; subsequent matches will be ignored.

For example, when run on a test case, the following program replaces the first "a" with a "b" and every "ca" with a "b":

```
a:=b
ca=b
```

Remember that left and right can also be empty.
For example, the program below deletes every "b" from the string and appends "abc" to the start of the string:

```
b=
:=abc
```

Every level will run 100 test cases on the program. Each test case gives your program an input string, and your program should match the output string when run to completion.
If you successfully complete a level, it will give you the secret code for the next level.

## Compiling and Running

This program is written using the Rust programming language, so you will need to follow the [online directions](https://www.rust-lang.org/tools/install) to install Rust on your system.
After Rust is installed, you can compile the program using:

```
cargo build
```

Cargo will automatically download any dependencies needed to build the code.
The [rlua](https://github.com/amethyst/rlua) package also comes bundled with Lua 5.4 so you do **NOT** need to install Lua on your system.
(_However, if you intend to write custom levels, it might be helpful to install Lua for testing your levels_).

You can run the program using:

```
cargo run -- <parameters....>
```

Where any program parameters go after the `--`.

## Usage

Levels are grouped into collections called level packs.
Running the program with no arguments will print the list of all installed level packs along with the string code for the starting level in each pack.
To print details about a specific level, use the `-l <level-code>` flag with the **level code**.
You can also see more details about a specific level pack by instead passing the `-p <pack-code>` flag with the **level pack code**.
Although rare, it may be possible that the auto-generated codes assign the same level code to levels in different packs.
If this happens, pass both the `-p <pack-code> -l <level-code>` which selects the pack and only searches for the level code inside the pack.

If you provide the `code-file`, it will execute the file on the level instead of printing information about the level.
If all test cases pass successfully, then the program will print the next level code for the current level pack.
Otherwise, it will output execution debug information to help fix any bugs with your code.
The executor will automatically time and return an error if your code fails to finish after 100 thousand executions.

```
USAGE:
    stringer-puzzles [OPTIONS] [code-file]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -l, --level-code <level-code>    Special passcode for a given level
    -p, --pack-code <pack-code>      Code for a specific level pack

ARGS:
    <code-file>    Code file to run
```

## Custom Level Packs

You can write custom level packs by:

1. Adding a new folder to the [packs](packs/) directory (_The folder name should **NOT** contain any spaces_)
2. Creating a `pack.json` file inside the folder
3. Creating [Lua Code Files](https://www.lua.org/) for all of the levels

The `pack.json` file has the following fields:

- `id` - Unique identifier for the level pack (_Can be any string, but I usually use a random UUID_)
- `version` - Optional string field to identify the version (_Can be any string, but I use semantic version numbers_)
- `name` - Simple name for the level pack
- `description` - Longer description
- `levels` - Array of 1 or more levels in the pack
- `winMessage` - Optional message to show when the last pack level is completed (_If not provided, the program shows a default message instead_)

Each entry in the levels list has the following fields:

- `name` - Short name for the level
- `description` - Longer text description that describes the level goals along with any important constraints
- `luaFile` - Lua code file to generate the level

Each Lua file needs to define a global function named `generateTestCase()` that returns an input string and corresponding expected output string.
The Lua program can use `math.random()` but **should not** mess with `math.randomseed()`.
The engine automatically sets the random seed to create reproducible test cases.
The pack directory is automatically added to the Lua `package.path` so you can import other local files as needed.

## Credit

The stringer puzzles are based heavily on the Steam game [A=B](https://store.steampowered.com/app/1720850/AB/) as created by Artless Games.
However, this variant uses a slightly different syntax and does not support `(start)=`, `=(end)`, or `=(return)` rules.
It also uses [Lua Code](https://www.lua.org/) to generate the puzzles as opposed to the in-game code editor.
