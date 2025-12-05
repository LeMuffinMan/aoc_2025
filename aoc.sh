#!/bin/bash

YEAR=$(date "+%Y")
MOUNTH=$(date "+%m")
DAY=$(date "+%-d")
HOUR=$(date "+%H")

source generate_day.sh

if [ -z "$1" ]; then 
  if [[ $YEAR -eq 2025 && $DAY -lt 13 ]]; then
    if [ $HOUR -lt 6 ]; then
      DAY=$(($DAY - 1))
    fi
    if [ ! -d "src/day_$DAY" ]; then
      generate_day
    fi
  else
    echo "Usage: ./aoc <day>"
    exit 1
  fi
elif ! expr "$1" + 0 >/dev/null 2>&1; then
  echo "Usage: ./aoc <day>"
  exit 1
elif [[ "$DAY" -gt 12 ]]; then
  echo "12 puzzles available in 2025"
  exit 1
else
  DAY=$1
fi

cargo run -- $DAY

