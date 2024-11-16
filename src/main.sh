#!/bin/bash
ls --color=auto "$@" | GREP_COLORS='mt=01;31' grep --color=always '\.txt$' || true
