#! /usr/bin/bash
set -e

# 1. Build the application
echo "Building the application..."
# The --quiet flag is used to make the output less verbose
cargo build --quiet

BINARY="../target/debug/git-today"

# 2. Setup the test repositories
echo "Setting up test repositories..."
./setup_repos.sh > /dev/null 2>&1

# 3. Generate expected output files
# This step creates the "golden files" for our tests.
# In a real-world scenario, you would commit these files to your repository.
# echo "Generating expected output..."
# mkdir -p expected

# Generate .call and .out files for regular tests
# for test in a b c d e; do
#     CMD="$BINARY repos/$test"
#     echo "$CMD" > "expected/${test}.call"
#     $CMD > "expected/${test}.out"
# done

# Generate .call and .out files for --full tests
# for test in a b c d e; do
#     CMD="$BINARY --full repos/$test"
#     # echo "$CMD" > "expected/${test}.call"
#     $CMD > "expected/${test}a.out"
# done

# 4. Run tests
echo "Running tests..."
for call_file in expected/*.call; do
    test_name=$(basename "$call_file" .call)
    printf "%-20s" "Testing repository $test_name..." # Print test name

    # Run the command and capture output
    output=$(bash "$call_file")

    # Compare with expected output
    if diff -q -w <(echo "$output") "expected/${test_name}.out" > /dev/null; then
        echo "✅"
    else
        echo "❌"
        echo "Error: Output for test '$test_name' does not match expected output."
        echo "-------------------------------------------------"
        # Use colordiff if available, otherwise regular diff
        if command -v colordiff &> /dev/null; then
            colordiff <(echo "$output") "expected/${test_name}.out"
        else
            diff <(echo "$output") "expected/${test_name}.out"
        fi
        echo "-------------------------------------------------"
        exit 1
    fi
done

echo "All tests passed! 🎉"
