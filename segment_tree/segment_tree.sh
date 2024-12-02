# underdeveloping now...

#!/bin/bash

# ヘルプの表示
show_help() {
    echo 
    echo "Usage: $(basename "$0") [OPTIONS]"
    echo
    echo "OPTIONS:"
    echo "  -h, --help           Show this help message and exit"
    echo "  -s, --step_by_step   Enable step-by-step mode for sorting visualization"
    echo
    exit 0
}

# step_by_stepの確認
step_by_step=""
for arg in "$@"; do
    case "$arg" in 
        -h|--help)
            show_help
            ;;
        -s|--step_by_step)
            step_by_step="-s"
            ;;
        *)
            echo
            echo "Error: Unknown  ortion `$arg`"
            show_help
            ;;
    esac
done

cargo run -- $step_by_step