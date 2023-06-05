#!/usr/bin/env sh

# Function to add to .zshrc
write_function_zshrc() {
cat <<EOF >> $1

# This function is added by installer of ask.sh
function ask() {
    suggested_commands=\`echo "\$@" | ask-sh 2> >(cat 1>&2)\`
    if [ -z "\$suggested_commands" ]; then
        return
    else
        selected_command=\`echo "\$suggested_commands" | peco  --prompt "AI suggested commands (Enter to use / Ctrl+C to exit):"\`
        if [ -z "\$selected_command" ]; then
            return
        else
            print -z \$selected_command
        fi
    fi
}
EOF
}

write_function_others() {
cat <<EOF >> $1

# This function is added by installer of ask.sh
function ask() {
    suggested_commands=\`echo "\$@" | ask-sh 2> >(cat 1>&2)\`
    if [ -z "\$suggested_commands" ]; then
        return
    else
        selected_command=\`echo "\$suggested_commands" | peco  --prompt "AI suggested commands (Enter to use / Ctrl+C to exit):"\`
        if [ -z "\$selected_command" ]; then
            return
        else
            history -s \\\$selected_command
        fi
    fi
}
EOF
}

echo_bash_function() {
    echo "function ask() { "
    echo "    suggested_commands=\`echo \"\$@\" | ask-sh 2> >(cat 1>&2)\`"
    echo "    if [ -z \"\$suggested_commands\" ]; then"
    echo "        return"
    echo "    else"
    echo "        selected_command=\`echo \"\$suggested_commands\" | peco  --prompt \"AI suggested commands (Enter to use / Ctrl+C to exit):\"\`"
    echo "        if [ -z \"\$selected_command\" ]; then"
    echo "            return"
    echo "        else"
    echo "            history -s \$selected_command"
    echo "        fi"
    echo "    fi"
    echo "}"
}

echo_zsh_function() {
    echo "function ask() { "
    echo "    suggested_commands=\`echo \"\$@\" | ask-sh 2> >(cat 1>&2)\`"
    echo "    if [ -z \"\$suggested_commands\" ]; then"
    echo "        return"
    echo "    else"
    echo "        selected_command=\`echo \"\$suggested_commands\" | peco  --prompt \"AI suggested commands (Enter to use / Ctrl+C to exit):\"\`"
    echo "        if [ -z \"\$selected_command\" ]; then"
    echo "            return"
    echo "        else"
    echo "            print -z \$selected_command"
    echo "        fi"
    echo "    fi"
    echo "}"
}

write_ask_to_rc_file() {
  RC_FILE=$1
  if ! grep -q "function ask()" "$RC_FILE"; then
    case "$SHELL" in
    *'bash'*)
      write_function_others "$RC_FILE"
      ;;
    *'zsh'*)
      write_function_zshrc "$RC_FILE"
      ;;
    esac
  else
    echo "ask function is already defined in $RC_FILE. Skipping adding function to rc file."
    echo "If you want to manually add the function, if you are using bash, add the following to ~/.bashrc:"
    echo_bash_function
    echo "If you are using zsh, add the following to ~/.zshrc:"
    echo_zsh_function
  fi
}

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Cargo is not installed. See https://doc.rust-lang.org/cargo/getting-started/installation.html for installation instructions"
    exit 1
fi
echo "Cargo is installed. Proceeding with installation."

# Check if peco is installed
if ! command -v peco &> /dev/null; then
    echo "Peco is not installed. See https://github.com/peco/peco#installation for installation instructions"
    exit 1
fi
echo "Peco is installed. Proceeding with installation."

# Check if tmux is installed
if ! command -v tmux &> /dev/null; then
    echo "Tmux is not installed. ask.sh uses tmux to capture current terminal screen and send to API."
    echo "If you proceed without installation, you cannot have context-aware/multi-turn conversations with AI."
    echo "See https://github.com/tmux/tmux/wiki/Installing for installation instructions"
    read -p "Do you want to proceed without tmux? (y/n)" -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]
    then
        exit 1
    fi
fi
echo "Tmux is installed or user chose to proceed without it. Proceeding with installation."

# if ai-sh is installed by cargo, uninstall it
if cargo install --list | grep -q ai-sh; then
    # asking to uninstall ai-sh
    echo "Thank you for installing ai.sh. ai.sh is now renamed and upgraded to ask.sh."
    read -p "To continue, ai-sh must be uninstalled. Do you want to uninstall ai-sh? (y/n)" -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]
    then
        exit 1
    fi
    cargo uninstall ai-sh
    echo "Uninstalled ai-sh."
fi

# Install or upgrade asksh using cargo and if failed, exit
echo "Installing/Updating ask.sh..."
if ! cargo install --force --path .; then
    echo "Failed to install asksh. Please check the error message above."
    exit 1
fi
echo "ask.sh is installed/updated."



# Check if ask function is defined in the current shell
# Check the default shell and write to appropriate rc file
case "$SHELL" in
*'bash'*)
if [ -f "$HOME/.bashrc" ]; then
    echo "Checking .bashrc"
    write_ask_to_rc_file "$HOME/.bashrc"
fi
;;
*'zsh'*)
if [ -f "$HOME/.zshrc" ]; then
    echo "Checking .zshrc"
    write_ask_to_rc_file "$HOME/.zshrc"
fi
;;
*)
echo "Unsupported shell. The script currently supports bash and zsh."
exit 1
;;
esac

echo "Installation complete. Please restart your shell or run 'source ~/.bashrc' or 'source ~/.zshrc' for changes to take effect."
