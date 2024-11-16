arrows=(
    "↪"
    "→"
)

arrow() {
    local common_arrow=${arrows[0]}
    local flech_arrow=${arrows[1]}

    echo -e "Variable name: common_arrow, Value: \033[33m$common_arrow\033[0m"
    echo -e "Variable name: common_arrow, Value: \033[33m$flech_arrow\033[0m"

    
}

arrow
