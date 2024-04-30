#!/bin/bash

# Check if input file is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <input_file>"
  exit 1
fi

input_file="$1"
output_file="${input_file}-example"

# Remove output file if it exists
if [ -e "$output_file" ]; then
  rm "$output_file"
fi

# Read input file line by line
while IFS= read -r line; do
  # Check if line contains a key-value pair
  if [[ "$line" =~ ^[[:space:]]*([^#=]+)=([^#]*) ]]; then
    # Extract the key and value
    key="${BASH_REMATCH[1]}"
    value="${BASH_REMATCH[2]}"
    # Preserve indentation
    indent="${line%%$key*}"
    if [[ "$key" == "RUST_LOG" || "$key" == "RUST_BACKTRACE" || "$key" == "DATABASE_URL" ]]; then
      echo "${indent}${key}=${value}" >> "$output_file"
    else
      echo "${indent}${key}=" >> "$output_file"
    fi
  elif [[ "$line" =~ ^[[:space:]]*# ]]; then
    # Preserve comments
    echo "$line" >> "$output_file"
  fi
done < "$input_file"
