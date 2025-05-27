the function of this repo is to hold the source files of me learning rust.

Structured list of 3 projects for each of the 11 main areas of a programming language:

🔧 1. File System Basics
file_logger: Complete
file_organizer: Organize files in a directory based on extensions.
Tasks:

List directory contents

Move .txt, .jpg, etc. into their respective folders

Handle missing folders (create if needed)

Use relative and absolute paths

backup_tool: Create a simple backup tool for a given folder.
Tasks:

Copy all files from source to backup directory

Rename or skip if file exists

Delete backup folder if too old

Use std::fs::metadata for age checking

📦 2. Data Structures & Algorithms
frequency_counter: Count word frequency in a text file.
Tasks:

Read file line by line

Use HashMap to count each word

Sort results by frequency

Display top N words

custom_stack_queue: Implement your own stack and queue structures.
Tasks:

Use vectors to simulate stack/queue behavior

Add push, pop, peek methods

Demonstrate usage with a small scenario

sorting_benchmark: Compare sorting algorithms.
Tasks:

Implement bubble sort, merge sort

Generate large random array

Measure and compare time complexity

Use std::time::Instant

🧠 3. Variables, Types, and Control Flow
temperature_converter: Console app that converts between Celsius and Fahrenheit.
Tasks:

Take user input

Convert and display result

Use match statements and loops

type_classifier: Determine data type properties.
Tasks:

Create different primitive and reference types

Demonstrate immutability, shadowing, and type inference

Use typeof equivalent via traits or macros

guessing_game: Number guessing game with limited attempts.
Tasks:

Generate random number

Loop for guesses

Use match for result: too high/low/correct

🧰 4. Functions / Procedures
calculator_lib: Make a reusable calculator module.
Tasks:

Write functions for add, subtract, multiply, divide

Accept arguments, return results

Use scope management

Build and test a main interface

string_utils: Create a utility lib for string operations.
Tasks:

Functions: capitalize, reverse, slugify

Use slice manipulation and ownership principles

Package as a crate

pure_vs_impure_demo: Show difference between pure and impure functions.
Tasks:

Write pure functions (e.g., add, double)

Write impure ones (e.g., logging to file)

Compare for testability and predictability

🏗️ 5. Project Structure & Modules
modular_todo_app: Basic To-Do app using modules.
Tasks:

Separate input, logic, and storage into modules

Use mod, pub, use correctly

Follow proper directory structure

Include a config file

weather_fetcher: Use modules to fetch and show mock weather data.
Tasks:

network, parser, display modules

Simulate API response

Organize code cleanly

simple_blog_engine: Simulate a tiny markdown blog parser.
Tasks:

Parse .md files

Split responsibilities: file read, parse, render

Add folder for test data

🧪 6. Testing and Debugging
math_test_suite: Write tests for a math utility library.
Tasks:

Add unit tests with #[test]

Test edge cases

Use assert_eq! and should_panic

bug_hunter_game: Intentionally buggy game logic for debugging practice.
Tasks:

Find and fix logical bugs

Use println!, dbg!, and cargo test

Optional: Use debugger (e.g., LLDB with VSCode)

test_coverage_tracker: Track test coverage manually.
Tasks:

Annotate which functions are tested

Write integration tests

Use Rust’s cargo test with custom output

💬 7. Error Handling
json_config_loader: Load and validate a JSON config file.
Tasks:

Parse JSON using serde_json

Handle file not found, malformed JSON

Return meaningful error messages

retry_on_fail: Retry HTTP request or file access on failure.
Tasks:

Use Result and pattern matching

Retry a few times with delay

Return a custom error type

error_chain_demo: Build your own error types with thiserror.
Tasks:

Define enums for different error kinds

Implement From for conversions

Chain errors with context

🧵 8. Concurrency / Async Basics
parallel_downloader: Simulate downloading multiple files in parallel.
Tasks:

Spawn threads for each download

Use thread::spawn, join

Print progress of each

async_timer: Create an async task scheduler.
Tasks:

Use tokio::time::sleep or similar

Await multiple timers

Print results asynchronously

counter_with_mutex: Shared counter across threads.
Tasks:

Use Arc<Mutex<_>>

Spawn threads to increment counter

Avoid race conditions

🌐 9. Networking & APIs (Basics)
http_status_checker: Check if a website is up.
Tasks:

Use reqwest to send GET requests

Print status code and reason

Handle timeouts

weather_api_client: Call real or mocked weather API.
Tasks:

Send request with query parameters

Parse JSON response

Print formatted data

http_file_uploader: Simulate a file upload API client.
Tasks:

Read file from disk

Send as multipart/form-data

Handle response

🔐 10. Security Awareness
input_sanitizer: Prevent command injection in user input.
Tasks:

Take string input

Strip unsafe characters

Demonstrate injection risk and safe handling

hash_my_password: Store password hashes securely.
Tasks:

Use argon2 or bcrypt crate

Hash and verify password

Demonstrate salt usage

file_permission_checker: Check file access rights.
Tasks:

Get file metadata

Report owner, permissions

Warn if sensitive file is world-readable

📁 BONUS: Environment & Tooling
rust_env_toolkit: Set up a Rust project with Cargo features.
Tasks:

Use .env file for config

Use dotenv crate

Use cargo features to toggle functionality

lint_formatter_runner: Set up linter and formatter in CI-like fashion.
Tasks:

Use rustfmt, clippy

Create shell script to run checks

Enforce lint rules

error_log_reader: Parse and pretty-print compiler error logs.
Tasks:

Run failing program

Capture stderr

Highlight errors and suggest fixes


