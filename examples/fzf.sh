#!/bin/sh

list_lists() {
  options=($(./target/debug/lm list) "[add]" "[remove]")
  selected_option=$(printf '%s\n' "${options[@]}" | fzf --prompt="select an option: ")

  if [ -n "$selected_option" ]; then
    case "$selected_option" in
      "[add]")
        add_list
        ;;
      "[remove]")
        delete_list
        ;;
      *)
        list_items "$selected_option"
        ;;
    esac
  fi
}

delete_list() {
  list_name=$(./target/debug/lm list | fzf --prompt="select a list to delete: ")
  if [ -n "$list_name" ]; then
    remove_list "$list_name"
  fi
}

remove_list() {
  list_name="$1"
  ./target/debug/lm remove "$list_name"
}

add_list() {
  new_list=$(read -p "[enter a name for the list]: " && echo "$REPLY")
  if [ -n "$new_list" ]; then
    ./target/debug/lm add "$new_list"
    list_items "$new_list"
  fi
}

list_items() {
  list_name="$1"
  while true; do
    options=($(./target/debug/lm list "$list_name") "[remove]" "[add]" "[back]")
    selected_option=$(printf '%s\n' "${options[@]}" | fzf --prompt="select an option: ")

    if [ -z "$selected_option" ]; then
      break
    elif [ "$selected_option" == "[add]" ]; then
      add_item "$list_name"
    elif [ "$selected_option" == "[remove]" ]; then
      delete_item "$list_name"
    elif [ "$selected_option" == "[back]" ]; then
      list_lists && break
    else
      edit_item "$list_name" "$selected_option"
    fi
  done
}

delete_item() {
  list_name="$1"
  item_content=$(./target/debug/lm list "$list_name" | fzf --prompt="select an item to delete: ")
  if [ -n "$item_content" ]; then
    remove_item "$list_name" "$item_content"
  fi
}

add_item() {
  list_name="$1"
  new_item=$(read -p "[enter a name for the item]: " && echo "$REPLY")
  if [ -n "$new_item" ]; then
    ./target/debug/lm add "$list_name" "$new_item"
    list_items "$list_name"
  fi
}

remove_item() {
  list_name="$1"
  item_content="$2"
  ./target/debug/lm remove "$list_name" "$item_content"
}

edit_item() {
  list_name="$1"
  item_content="$2"
  if [[ $item_content == *http*://* || $item_content == *https*://* ]]; then
    xdg-open "$item_content"
    edit_item "$list_name" "$item_content" "done"
  else
    new_status=$(printf "todo\ndoing\ndone" | fzf --prompt="select a status for the item: ")
    if [ -n "$new_status" ]; then
      ./target/debug/lm edit "$list_name" "$item_content" "$new_status"
    fi
  fi
}

list_lists
