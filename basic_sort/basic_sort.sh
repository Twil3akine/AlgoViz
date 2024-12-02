# underdeveloping now...

#!/bin/bash

# 選択肢のリスト
options=("Bubble" "Choose" "Insert" "Quit")
current=0  # 現在の選択位置

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

# 選択肢を表示する関数
draw_menu() {
    clear
    echo "Use ↑ or ↓ to move, Enter to select:"
    for i in "${!options[@]}"; do
        if [ "$i" -eq "$current" ]; then
            echo -e "> \033[32m${options[i]}\033[0m"  # 緑色で強調表示
        else
            echo "  ${options[i]}"
        fi
    done
}

while true; do
    draw_menu

    # キー入力を取得
    read -s -n3 key

    case "$key" in
        $'\e[A')  # 上矢印
            ((current--))
            if [ "$current" -lt 0 ]; then
                current=$((${#options[@]} - 1))
            fi
            ;;
        $'\e[B')  # 下矢印
            ((current++))
            if [ "$current" -ge "${#options[@]}" ]; then
                current=0
            fi
            ;;
        "")  # Enterキー
            echo
            echo "You selected: ${options[current]}"
            echo

            if [ "${options[current]}" == "Quit" ]; then
                exit 0
            else
                cargo run -- "${options[current]}" $step_by_step
                
                echo
            fi
            break
            ;;
        *)
            ;;
    esac
done