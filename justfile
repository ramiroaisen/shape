set windows-powershell := true

# limit the numbers of parallel jobs run by cargo
# this (tries to) prevent a random freeze in some local development environments at high cpu usage
j := env_var_or_default("J", "8")

# this is a justfile, just is a task runner written in Rust. see https://github.com/casey/just

# list all just recipes
default:
  just --list --unsorted

ci-coverage:
  mkdir -p github-pages/coverage

  mkdir -p target/llvm-cov-html
  cargo llvm-cov --html --output-dir target/llvm-cov-html --package shape --ignore-filename-regex "patches/"
  mv target/llvm-cov-html/html/* github-pages/coverage

  mkdir -p target/llvm-cov-lcov
  cargo llvm-cov --lcov --output-path target/llvm-cov-lcov/lcov.info --package shape --ignore-filename-regex "patches/"
  
  npm i -g lcov-badge2
  mkdir -p target/llvm-cov-badge
  lcov-badge2 target/llvm-cov-lcov/lcov.info -o target/llvm-cov-badge/badge.svg
  
  mv target/llvm-cov-badge/badge.svg github-pages/coverage/badge.svg
  mv target/llvm-cov-lcov/lcov.info github-pages/coverage/lcov.info

# build shape bin in release mode with default features
build *args:
  cargo build --release {{args}}

# build the default binary for windows
build-windows:
  cross build -j {{j}} --release --target=x86_64-pc-windows-msvc

# build the default binary for darwin
build-darwin:
  cross build -j {{j}} --release --target=x86_64-apple-darwin

# run cargo unit tests with default features
unit:
  cargo test -j {{j}} --lib

# run cargo integration tests with default features
integration:
  cargo test -j {{j}} --test "*"

# run cargo unit and integration tests with default features
test:
  cargo test -j {{j}}

# run cargo build for all features combinations
build-all-feat:
  cargo build-all-features --release

# run cargo check for all features combinations
check-all-feat:
  cargo check-all-features

# run cargo test for all features combinations
test-all-feat:
  cargo test-all-features

# generate tarpaulin based coverage report
tarpaulin:
  cargo tarpaulin -j {{j}} --out html --out lcov --engine llvm --output-dir target/tarpaulin/html

# open the tarpaulin report
tarpaulin-open:
  xdg-open target/tarpaulin/html/shape/index.html

# generate grcov based coverage report (collect stage)
grcov-collect:
  CARGO_INCREMENTAL=0 \
  RUSTFLAGS='-Cinstrument-coverage' \
  LLVM_PROFILE_FILE='target/coverage/profraw/cargo-test-%p-%m.profraw' \
  cargo test -j {{j}} -p shape # ignore patches
  
# generate grcov based coverage report (report stage)
grcov-report:
  grcov \
    "." \
    --source-dir "./src" \
    --log-level "INFO" \
    --binary-path "./target/debug/" \
    --branch \
    --ignore-not-existing \
    --output-types "html,lcov" \
    --output-path "./target/coverage"

# generate grcov based coverage report (collect and report stages)
grcov:
  just grcov-collect
  just grcov-report

# open the grcov report
grcov-open:
  xdg-open target/coverage/html/index.html

# generage llvm-cov based coverage report, the default coverage system for shape
coverage:
  cargo llvm-cov -j {{j}} --html --output-dir target/llvm-cov --package shape --ignore-filename-regex "patches/"

# open the llvm-cov report
coverage-open:
  xdg-open target/llvm-cov/html/index.html

# generate a coverage lcov report
coverage-lcov:
  cargo llvm-cov -j {{j}} --lcov --output-path target/llvm-cov/lcov.info --package shape --ignore-filename-regex "patches/"

# generate a coverage badge
coverage-badge: coverage-lcov
  # npm install -g lcov-badge2
  lcov-badge2 target/llvm-cov/lcov.info -o target/llvm-cov/badge.svg

# open the coverage badge
coverage-badge-open:
  xdg-open target/llvm-cov/badge.svg

# generate docs
doc:
  cargo doc

# open the generated docs
doc-open:
  xdg-open target/doc/shape/index.html

# build in debug mode
debug:
  cargo build --debug -j {{j}}