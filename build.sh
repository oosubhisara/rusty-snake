#!/usr/bin/env bash

game_dir=dist/snake-linux/game/
res_dir=dist/snake-linux/res/

mkdir -p $game_dir 
mkdir -p $res_dir 
cargo build --release
cp target/release/snake $game_dir 
cp data.pak $game_dir
cp linux/icon.png $res_dir
cp linux/snake.desktop $res_dir
strip --strip-all ${game_dir}/snake


