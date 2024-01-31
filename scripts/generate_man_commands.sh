#!/bin/bash

export LC_ALL=C
export LANG=C

man -k . | while read -r entry; do 
    # Extract the command names (before the dash) and remove section numbers in parentheses
    commands=$(echo "$entry" | sed -e 's/ - .*$//' -e 's/([^)]*)//g')

    # Split the commands and process each one
    IFS=',' read -ra CMD_ARRAY <<< "$commands"
    for cmd in "${CMD_ARRAY[@]}"; do
        # Trim leading and trailing whitespace
        cmd=$(echo "$cmd" | xargs)

        # Check if cmd is a valid command
        if ! type "$cmd" &> /dev/null; then
            echo "Skipping invalid command: $cmd"
            continue
        fi

        # Now run man -w
        man_output=$(man -w "$cmd" 2>&1)
        exit_status=$?

        echo "Command: $cmd"
        echo "Man Output: $man_output"
        echo "Exit Status: $exit_status"

        if [ $exit_status -eq 0 ]; then
            echo "Command succeeded"
        else
            echo "Command failed"
        fi
    done
done