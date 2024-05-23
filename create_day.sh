#!/bin/bash

YEAR=$1
DAY=$2
DAY_PADDED=$(printf "%02d" $DAY)
SESSION_TOKEN="53616c7465645f5fe3e6a120a6261ae43e1a8337131aa149199d36444158e57bb76fcb6bd11f2785679f0f0d57bc61fb0457a31484dd2bc18af2b8cb5732959a"  # Replace this with your session token

if [ -z "$YEAR" ] || [ -z "$DAY" ]; then
  echo "Usage: $0 <year> <day>"
  exit 1
fi

TARGET_DIR="src/year${YEAR}"
TARGET_FILE="${TARGET_DIR}/day${DAY_PADDED}.rs"
INPUT_DIR="inputs/${YEAR}"
INPUT_FILE="${INPUT_DIR}/day${DAY_PADDED}.txt"
MAIN_FILE="src/main.rs"

# Ensure the input directory exists
mkdir -p "$INPUT_DIR"

# Check if the input file already exists
if [ -f "$INPUT_FILE" ]; then
  echo "Input file ${INPUT_FILE} already exists."
else
  # Download input file
  curl -s "https://adventofcode.com/${YEAR}/day/${DAY}/input" -H "Cookie: session=${SESSION_TOKEN}" -o "$INPUT_FILE"
  echo "Downloaded input file to ${INPUT_FILE}"
fi

# Create target directory if it does not exist
mkdir -p "$TARGET_DIR"

# Create the day module file from the template if it does not exist
if [ -f "$TARGET_FILE" ]; then
  echo "File ${TARGET_FILE} already exists"
else
  cp src/template.rs "$TARGET_FILE"
  sed -i "s/YEAR/${YEAR}/g; s/DAY/${DAY_PADDED}/g" "$TARGET_FILE"
  echo "Created ${TARGET_FILE}"
fi

# Update the mod.rs file to include the new day module
MOD_FILE="${TARGET_DIR}/mod.rs"
if ! grep -q "pub mod day${DAY_PADDED};" "$MOD_FILE"; then
  echo "pub mod day${DAY_PADDED};" >> "$MOD_FILE"
  echo "Updated ${MOD_FILE}"
fi

# Update the main.rs file to include the year module if not present
if ! grep -q "mod year${YEAR};" "$MAIN_FILE"; then
    sed -i "/mod utils;/a \mod year${YEAR};" "$MAIN_FILE"
    echo "Added mod year${YEAR} to ${MAIN_FILE}"
fi

# Update the main.rs file to include the new day in the match statement before the "// Add new days here" comment
if ! grep -q "year${YEAR}::day${DAY_PADDED}::run()" "$MAIN_FILE"; then
  sed -i "/\/\/ Add new days here/i\\
  (\"${YEAR}\", \"day${DAY_PADDED}\") => year${YEAR}::day${DAY_PADDED}::run()," "$MAIN_FILE"
  echo "Updated ${MAIN_FILE}"
fi
