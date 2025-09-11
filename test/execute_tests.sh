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
echo "Generating expected output..."
mkdir -p expected

TEST_CASES="a b c d e"
#for test in $TEST_CASES; do
#    $BINARY repos/$test > expected/$test
#done

# 4. Run tests
echo "Running tests..."
for test in $TEST_CASES; do
    printf "%-20s" "Testing repository $test..." # Print test name

    # Run the command and capture output
    output=$($BINARY repos/$test)

    # Compare with expected output
    if diff -q -w <(echo "$output") "expected/${test}.out" > /dev/null; then
        echo "✅"
    else
        echo "❌"
        echo "Error: Output for test '$test' does not match expected output."
        echo "-------------------------------------------------"
        # Use colordiff if available, otherwise regular diff
        if command -v colordiff &> /dev/null; then
            colordiff <(echo "$output") "expected/$test.out"
        else
            diff <(echo "$output") "expected/$test.out"
        fi
        echo "-------------------------------------------------"
        exit 1
    fi
done

echo "All tests passed! 🎉"
