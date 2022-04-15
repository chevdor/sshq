VERSION := `toml get Cargo.toml package.version | jq -r`
export TAG:=`toml get Cargo.toml "package.version" | jq -r .`

# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

# Test / watch
test:
	cargo watch -x "test -- --no-capture"

# Test including ignored tests
test_all:
	cargo test -- --include-ignored

# Generate usage samples
_usage:
  cargo run -q -- --help > doc/main.adoc
  cargo run -q -- search --help > doc/search.adoc
  cargo run -q -- list --help > doc/list.adoc

# Generate documentation
doc: _usage
	cargo doc --all-features --no-deps

# Run rustfmt
_fmt:
	cargo +nightly fmt --all

# Run clippy
_clippy:
	cargo +nightly clippy --all-features --all-targets

_deny:
	cargo deny check

# Run checks such as clippy, rustfmt, etc...
check: _clippy _fmt _deny

# Generate the readme as .md
md:
    #!/usr/bin/env bash
    asciidoctor -b docbook -a leveloffset=+1 -o - README_src.adoc | pandoc  --markdown-headings=atx --wrap=preserve -t markdown_strict -f docbook - > README.md

tag:
    #!/bin/sh
    echo Tagging version v$TAG
    git tag "v$TAG" -f
    git tag | sort -Vr | head
