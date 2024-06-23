jelly := "cargo run --quiet --"

alias b := build
alias p := preview

ci:
    set -e

    cargo fmt --check
    cargo clippy --all --all-targets --all-features --  -Dwarnings
    cargo build --release
    cargo test --all --all-targets --all-features
    cargo machete

    echo "SUCCESS"

dev:
    bacon

tpl:
    bacon tpl

build:
    {{ jelly }} build --source ./tests/full/medium

preview: build
    static-web-server \
        --root ./dist \
        --port 3000

default:
    dev
