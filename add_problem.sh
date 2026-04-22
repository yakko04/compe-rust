#!/bin/bash

if [ $# -ne 2 ]; then
    echo "Usage: ./add_problem.sh <contest> <problem>"
    echo "Example: ./add_problem.sh tenkei90 070"
    exit 1
fi

CONTEST=$1
PROBLEM=$2
DIR="src/$CONTEST"
FILE="$DIR/$PROBLEM.rs"
BIN_NAME="${CONTEST}_${PROBLEM}"

mkdir -p "$DIR"

if [ ! -f "$FILE" ]; then
    cat > "$FILE" << 'EOF'
use proconio::input;

fn main() {
    input!{
        
    }
}
EOF
    echo "✓ Created $FILE"
else
    echo "⏭ $FILE already exists"
fi

if ! grep -q "name = \"$BIN_NAME\"" Cargo.toml; then
    cat >> Cargo.toml << EOF

[[bin]]
name = "$BIN_NAME"
path = "$FILE"
EOF
    echo "✓ Added $BIN_NAME to Cargo.toml"
fi

echo "Done!"
