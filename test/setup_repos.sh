#! /usr/bin/bash

# Exit on error
set -e

function delete_all_repos() {
    rm -rf repos
}

function commit(){
    local msg="$1"
    local date="$2" # An optional date string like "yesterday"

    # paste dev/urandom into dummy.txt
    echo $(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 32 | head -n 1) > "dummy.txt"
    git add "dummy.txt"
    if [ -z "$date" ]; then
        git commit -m "$msg"
    else
        # Set the author and committer date for the commit
        GIT_COMMITTER_DATE="$(date --date="$date")" git commit --date="$date" -m "$msg"
    fi
}

function change_author() {
    git config user.name "$1"
}

function create_repo() {
    # create directory
    mkdir "$1" && cd "$1"

    git init
    # set needed git configs
    git config user.name "Author1"
    git config user.email "Author1@example.com"

    # switch back out of the repository
    cd ..
}
########################
# TEST CASES
########################

function testcase_a_simple() {
    create_repo "a"
    cd "a"
    commit "feat: init repo"
    commit "bug: fixed something"
    commit "doc: documented something"

    cd ..
}

function testcase_b_branches() {
    create_repo "b"
    cd "b"
    commit "feat: init repo"
    commit "bug: fixed something"
    commit "doc: documented something"

    git checkout HEAD~2
    git checkout -b "branch2"
    commit "bug: something wrong"
    commit "doc: again"

    cd ..
}

function testcase_c_merge() {
    create_repo "c"
    cd "c"
    commit "feat: init repo"
    commit "bug: fixed something"
    commit "doc: documented something"

    git checkout HEAD~2
    git checkout -b "branch2"
    commit "bug: something wrong"
    commit "doc: again"

    # merge and force our changes
    git merge -X ours -m "merge feat" main

    cd ..
}

function testcase_d_authors() {
    create_repo "d"
    cd "d"
    commit "feat: init repo"
    change_author "Author2"
    commit "bug: fixed something"
    change_author "Author3"
    commit "doc: documented something"
    change_author "Author4"
    commit "feat: new author"

    cd ..
}

function testcase_e_dates() {
    create_repo "e"
    cd "e"
    commit "feat: init repo" "2 days ago"
    change_author "Author2"
    commit "bug: fixed something" "2 days ago"
    change_author "Author3"
    commit "doc: documented something"  "yesterday"
    change_author "Author4"
    commit "feat: new author"

    cd ..
}
########################
# MAIN
########################

delete_all_repos

# create a directory for all the automated repositories
mkdir -p repos

cd repos

testcase_a_simple
testcase_b_branches
testcase_c_merge
testcase_d_authors
testcase_e_dates
