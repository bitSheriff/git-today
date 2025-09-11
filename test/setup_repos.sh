#! /usr/bin/bash

function delte_all_repos() {
    rm -rf repos
}

function commit(){
    local msg="$1"

    # paste dev/urandom into dummy.txt
    echo $(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | fold -w 32 | head -n 1) > "dummy.txt"
    git add "dummy.txt" && git commit -m "$msg"
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
    git merge -X ours -m "merge" main

    cd ..
}
########################
# MAIN
########################

# create a directory for all the automated repositories
mkdir -p repos

cd repos

testcase_a_simple
testcase_b_branches
testcase_c_merge
