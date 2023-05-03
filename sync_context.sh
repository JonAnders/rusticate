#!/bin/bash

print_files() {
  local dir="$1"
  for file in "$dir"/*; do
    if [[ -d "$file" ]]; then
      print_files "$file"
    elif [[ -f "$file" ]]; then
      echo -e "\nFile: \`$file\`"
      echo '```'
      cat "$file"
      echo '```'
    fi
  done
}

echo "You and I are both expert software developers with 15 years of experience."
echo "We are developing a RESTful API for managing a TODO-list using Rust and Actix. The name of the project is Rusticate."
echo "Here is all of the code we have written so far:"

echo -e "\nFile: \`Cargo.toml\`"
echo '```'
cat Cargo.toml
echo '```'

print_files src
