#!/bin/bash

for file in src/*/*.rs; do
    [ -e "$file" ] || continue
    
    CONTEST=$(basename $(dirname "$file"))
    PROBLEM=$(basename "$file" .rs)
    BIN_NAME="${CONTEST}_${PROBLEM}"
    
    if ! grep -q "name = \"$BIN_NAME\"" Cargo.toml; then
        cat >> Cargo.toml << EOF

[[bin]]
name = "$BIN_NAME"
path = "$file"
EOF
        echo "✓ Added $BIN_NAME ($file)"
    fi
done

echo "Done!"
