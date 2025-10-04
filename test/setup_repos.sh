#! /usr/bin/bash
set -e

function delete_all_repos() {
    rm -rf repos
}

function commit() {
    local msg="$1"
    local date="$2" # optional date argument, e.g., "yesterday" or "2 days ago"

    # paste dev/urandom into dummy.txt
    echo $(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 32 | head -n 1) >"dummy.txt"
    git add "dummy.txt"
    if [ -z "$date" ]; then
        git commit -m "$msg"
    else
        GIT_COMMITTER_DATE="$(LC_ALL=C date --date="$date")" git commit --date="$(LC_ALL=C date --date="$date")" -m "$msg"
    fi
}

function change_author() {
    git config user.name "$1"
    git config user.email "$1@example.com"
}

function create_repo() {
    # create directory
    mkdir -p "$1" && cd "$1"

    git init --initial-branch=main
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
    echo "----- Creating Repository a -----"
    create_repo "a"
    cd "a"
    commit "feat: init repo"
    commit "bug: fixed something"
    commit "doc: documented something"

    cd ..
}

function testcase_b_branches() {
    echo "----- Creating Repository b -----"
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
    echo "----- Creating Repository c -----"
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
    echo "----- Creating Repository d -----"
    create_repo "d"
    cd "d"
    commit "feat: init repo"
    change_author "Author2"
    commit "bug: fixed something"
    change_author "Author3"
    commit "doc: documented something"
    change_author "Author4"
    commit "feat: new author"
    commit "feat: new author"
    commit "feat: new author"
    commit "feat: new author"

    cd ..
}

function testcase_e_time_based() {
    echo "----- Creating Repository e -----"
    create_repo "e"
    cd "e"
    commit "docs: docs from 2 days ago" "2 days ago"
    commit "fix: a bug from yesterday" "yesterday"
    commit "feat: something from today"
    cd ..
}

function testcase_f_issue_numbers() {
    echo "----- Creating Repository f -----"
    create_repo "f"
    cd "f"
    commit "[issue-123] docs: docs"
    commit "[issue-1] docs: another doc"
    commit "[issue-1] fix: a bug from today"
    commit "#issue-1 feat: something from today"
    commit "#issue-1: feat: something from today"
    commit "[issue-1] feat: something from today"
    commit "[issue-a] feat: something from today"
    commit "[issue-a] feat: something from today"
    commit "[issue-a] feat: something from today"
    commit "[issue-a] feat: something from today"
    cd ..
}

########################
# MAIN
########################
function main() {
    delete_all_repos

    # create a directory for all the automated repositories
    mkdir -p repos

    cd repos

    testcase_a_simple
    testcase_b_branches
    testcase_c_merge
    testcase_d_authors
    testcase_e_time_based
    testcase_f_issue_numbers
}

main
