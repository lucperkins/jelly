jelly := "cargo run --quiet --color always --"
jellyTpl := "cargo run --quiet --color always --features dev-handlebars-templates --"

alias b := build
alias p := preview
alias t := tpl

default: dev

@ci:
    set -e

    cargo fmt --check
    cargo clippy --all --all-targets --all-features --  -Dwarnings
    cargo build --release
    cargo test --all --all-targets --all-features
    cargo machete

    echo "SUCCESS"

@dev:
    bacon

@tpl:
    {{ jellyTpl }} serve --open --source ./tests/full/medium

@build:
    {{ jelly }} build --source ./tests/full/medium

@preview: build
    static-web-server \
        --root ./dist \
        --log-level error \
        --port 3000
