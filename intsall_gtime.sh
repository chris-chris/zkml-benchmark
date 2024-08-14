#!/bin/bash

# Check gtime
if ! command -v gtime &> /dev/null
then
    echo "gtime not found, installing gtime..."
    brew install gnu-time
fi
