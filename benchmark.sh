#!/usr/bin/env zsh

test_file=$1
hyperfine --warmup 3 --export-markdown table.md \
          "cat '$test_file' | ./target/release/00-orig" \
          "cat '$test_file' | ./target/release/01-drobnosti" \
          "cat '$test_file' | ./target/release/02-min_alokaci" \
          "cat '$test_file' | ./target/release/03-jen_main" \
          "cat '$test_file' | ./target/release/04-vetveni" \
          "cat '$test_file' | ./target/release/05-vetveni_bez_zbytecnych_alokaci" \
          "cat '$test_file' | ./target/release/06-bez_parsovani_utf8" \
          "cat '$test_file' | ./simplify_tags.pl"
