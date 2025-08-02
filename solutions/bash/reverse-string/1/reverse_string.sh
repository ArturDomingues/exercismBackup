#!/usr/bin/env bash
main () {
    local str="$1"
    echo "$str" | rev
}

# call main with all of the positional arguments
main "$@"

