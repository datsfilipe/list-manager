# list-manager

The basic idea here is to manage todo lists with zero overhead.

The rust package offers commands to manage the lists and items stored in an sqlite database.

So, with scripts, from your Neovim, from your WM, from wherever you want, you will be able to manipulate those lists.

## Examples

- **Using FZF inside a bash script to manipulate lists**

<table>
<tr>
<td><b>fzf.sh</b></td>
</tr>

<td>

```bash
#!/bin/sh

list_lists() {
  del_cmd="list-manager remove {}"
  add_cmd="read 'REPLY?name: ' && echo \$REPLY | list-manager add \$REPLY"
  selected_option=$(list-manager list | fzf \
    --bind "ctrl-o:execute($del_cmd)" \
    --bind="ctrl-n:execute($add_cmd)")

  if [ -n "$selected_option" ]; then
    list_items "$selected_option"
  fi
}

list_items() {
  list_name="$1"
  del_cmd="list-manager remove $list_name {}"
  add_cmd="read 'REPLY?content: ' && echo \$REPLY | list-manager add $list_name \$REPLY"
  while true; do
    options=("[back]" $(list-manager list "$list_name"))
    selected_option=$(printf '%s\n' "${options[@]}" | fzf \
      --bind "ctrl-o:execute($del_cmd)" \
      --bind="ctrl-n:execute($add_cmd)")

    if [ -z "$selected_option" ]; then
      break
    elif [ "$selected_option" == "[back]" ]; then
      list_lists && break
    else
      open "$selected_option" "$list_name" && break
    fi
  done
}

open() {
  item_content="$1"
  list_name="$2"
  if [[ $item_content == *http*://* || $item_content == *https*://* ]]; then
    xdg-open "$item_content"
  else
    read -p "copy to clipboard/edit [c/e]: " -n 1 -r
    echo ""
    if [[ $REPLY =~ ^[Ee]$ ]]; then
      read -p "content/status: " -r
      list-manager edit "$list_name" "$item_content" "$REPLY"
    else
      echo "$item_content" | wl-copy
    fi
  fi
}

list_lists
```
</td>
</table>
