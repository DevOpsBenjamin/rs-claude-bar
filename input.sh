#!/bin/bash

# Claude Code Status Line
# Shows context usage, session limits, and time until reset

# Read JSON input from Claude Code
if [ -t 0 ]; then
    # If running standalone (for testing), use mock data
    printf "NO PARAM\n"
else
    # Read from stdin when called by Claude Code
    input=$(cat)    
    printf "PARAM:${input}\n"
fi