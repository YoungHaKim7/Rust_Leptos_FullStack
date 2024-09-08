project_name := `basename "$(pwd)"`

# cargo run
r:
    cargo r

# (optimization)cargo run --release
rr:
    cargo run --release

# cargo watch(check & test & run)
w:
    cargo watch -x check -x test -x run

# cargo watch(simple only run)
ws:
    cargo watch -x 'run'

# cargo check(Test Before Deployment)
c:
    cargo check --all-features --all-targets --all

# cargo test
t:
    cargo t

# cargo expand(test --lib)
tex:
    cargo expand --lib --tests

# cargo test -- --nocapture
tp:
    cargo t -- --nocapture

# nightly(cargo nextest run)
tn:
    cargo nextest run

# nightly(cargo nextest run --nocapture)
tnp:
    cargo nextest run --nocapture

# macro show(cargo expand)
ex:
    cargo expand

# emit mir file
mir:
    cargo rustc -- -Zunpretty=mir > target/{{project_name}}.mir

# emit asm file
es:
    cargo rustc -- --emit asm=target/{{project_name}}.s

# optimized assembly
eos:
    cargo rustc --release -- --emit asm > target/{{project_name}}.s

# emit llvm-ir file
llvm:
    cargo rustc -- --emit llvm-ir=target/{{project_name}}.ll

# emit hir file
hir:
    cargo rustc -- -Zunpretty=hir > target/{{project_name}}.hir

# cargo asm
asm METHOD:
    cargo asm {{project_name}}::{{METHOD}}

# clean file
cf:
    rm -rf target ./config rust-toolchain.toml *.lock

# nightly setting(faster compilation)
n:
    rm -rf .cargo rust-toolchain.toml
    mkdir .cargo
    echo "[toolchain]" >> rust-toolchain.toml
    echo "channel = \"nightly\"" >> rust-toolchain.toml
    echo "components = [\"rustfmt\", \"rust-src\"]" >> rust-toolchain.toml
    echo "[build]" >> .cargo/config.toml
    echo "rustflags = [\"-Z\", \"threads=8\"]" >> .cargo/config.toml

# .gitignore setting
gi:
    echo "# Result" >> README.md
    echo "" >> README.md
    echo "\`\`\`bash" >> README.md
    echo "" >> README.md
    echo "\`\`\`" >> README.md
    echo "" >> README.md
    echo "justfile" >> .gitignore
    echo "# Visual Studio 2015/2017 cache/options directory" >> .gitignore
    echo ".vs/" >> .gitignore
    echo "" >> .gitignore
    echo "# A collection of useful .gitignore templates " >> .gitignore
    echo "# https://github.com/github/gitignore" >> .gitignore
    echo "# General" >> .gitignore
    echo ".DS_Store" >> .gitignore
    echo "dir/otherdir/.DS_Store" >> .gitignore
    echo "" >> .gitignore
    echo "# VS Code files for those working on multiple tools" >> .gitignore
    echo ".vscode/" >> .gitignore
    echo "# Generated by Cargo" >> .gitignore
    echo "# will have compiled files and executables" >> .gitignore
    echo "debug/" >> .gitignore
    echo "target/" >> .gitignore
    echo "" >> .gitignore
    echo "# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries" >> .gitignore
    echo "# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html" >> .gitignore
    echo "Cargo.lock" >> .gitignore
    echo "" >> .gitignore
    echo "# These are backup files generated by rustfmt" >> .gitignore
    echo "**/*.rs.bk" >> .gitignore
    echo "" >> .gitignore
    echo "# MSVC Windows builds of rustc generate these, which store debugging information" >> .gitignore
    echo "*.pdb" >> .gitignore
    echo "" >> .gitignore
    echo "# WASM" >> .gitignore
    echo "pkg/" >> .gitignore
    echo "/wasm-pack.log" >> .gitignore
    echo "dist/" >> .gitignore