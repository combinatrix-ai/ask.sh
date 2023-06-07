#!/usr/bin/env sh

LATEST_SHELL_FUNCTION_VERSION=v2
UPDATE_SHELL_FUNCTION=1
if [ -z "$DO_NOT_UPDATE_SHELL_FUNCTION" ]; then
    echo "Disabled shell function update with DO_NOT_UPDATE_SHELL_FUNCTION=1"
    UPDATE_SHELL_FUNCTION=0
fi

DEFAULT_SHELL=$(basename "$SHELL")

# Fail if $SHELL is not set
# Check if ask function is defined in the current shell
# Check the default shell and write to appropriate rc file
if [ -z "$DEFAULT_SHELL" ]; then
    echo "Could not automatically determine your shell based on \$SHELL environment variable."
    echo "Please set the \$SHELL environment variable and run this script again."
    echo "zsh -c \"\$(SHELL=zsh curl -fsSL https://raw.githubusercontent.com/hmirin/ask.sh/main/install.sh)\""
    echo "bash -c \"\$(SHELL=bash curl -fsSL https://raw.githubusercontent.com/hmirin/ask.sh/main/install.sh)\""
    echo "If you are using other than zsh or bash, follow the instructions on https://github.com/hmirin/ask.sh#installation"
    exit 1
else
#  Fail if $SHELL is not zsh or bash
    if [ "$DEFAULT_SHELL" != "zsh" ] && [ "$DEFAULT_SHELL" != "bash" ]; then
        echo "It seems you're using a shell other than bash or zsh, according to \$SHELL environment variable."
        echo "If you are using other than zsh or bash, follow the instructions on https://github.com/hmirin/ask.sh#installation"
        exit 1
    fi
fi

echo "Installing ask.sh to your $DEFAULT_SHELL."
# recommend bash users to use zsh
if [ "$DEFAULT_SHELL" = "bash" ]; then
    echo "We recommend you to use zsh instead of bash."
    echo "zsh is a modern and powerful shell, and ask.sh works better with zsh."
    echo "Using zsh, you can use auto-typing of the AI suggested commands."
    echo "Nevertheless, let's proceed with installing ask.sh for bash."
fi

# From now on, we assume that the default shell is either zsh or bash

# Function to add to .zshrc
write_function_zshrc() {
cat <<EOF >> $1


# This function is automatically added by the installer of ask.sh
# ask.sh v0.2.2 (Do not edit this line manually)
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

# This function is automatically added by the installer of ask.sh.
# ask.sh shell function v2 (Do not edit this line manually)
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
    echo "\# ask.sh shell function v2 (Do not edit this line manually)"
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
    echo "\# ask.sh shell function v2 (Do not edit this line manually)"
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
    if [ "$DEFAULT_SHELL" == "zsh" ]
        read "REPLY?Do you want to proceed without tmux? (y/n): "
    else
        read -p "Do you want to proceed without tmux? (y/n): " REPLY
    fi
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]
    then
        echo "Exiting..."
        exit 1
    fi
fi
echo "Tmux is installed or user chose to proceed without it. Proceeding with installation."

# if ai-sh is installed by cargo, uninstall it
if cargo install --list | grep -q ai-sh; then
    # asking to uninstall ai-sh
    echo "Thank you for installing ai.sh. ai.sh is now renamed and upgraded to ask.sh."
    echo "To continue, ai-sh must be uninstalled."
    if [ "$DEFAULT_SHELL" == "zsh" ]
        read "REPLY?Do you want to uninstall ai-sh? (y/n): "
    else
        read -p "Do you want to uninstall ai-sh? (y/n): " REPLY
    fi
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]
    then
        exit 1
    fi
    cargo uninstall ai-sh
    echo "Uninstalled ai-sh."
fi

# Install or upgrade ask.sh using cargo and if failed, exit
echo "Installing/Updating ask.sh..."
if ! cargo install ask-sh; then
    echo "Failed to install ask.sh. Please check the error message above."
    exit 1
fi
echo "ask.sh is installed/updated."

# Check the default shell and select appropriate rc file
if [  "$DEFAULT_SHELL" == "zsh" ]; then
    RC_FILE="$HOME/.zshrc"
else
    RC_FILE="$HOME/.bashrc"
fi

# Read ASK_SH_OPENAI_API_KEY from environment variable if not set, ask user to set it now
INPUT_OPENAI_API_KEY = ""
if [ -z "$ASK_SH_OPENAI_API_KEY" ]; then
    if [ -z "$NO_ASK_OPENAI_API_KEY" ]; then
        # use read
        echo "OpenAI API key is not set. Please set it now. This will be written to $RC_FILE. If you want to set it later, exit now and run the installer again with NO_ASK_OPENAI_API_KEY=1."
        echo "You can obtain your API key from https://platform.openai.com/account/api-keys"
        if [ "$DEFAULT_SHELL" == "zsh" ]
            read "INPUT_OPENAI_API_KEY?Please enter your OpenAI API key: "
        else
            read -p "Please enter your OpenAI API key: " INPUT_OPENAI_API_KEY
        fi
        echo ""
        # remove newline, spaces, and quotes
        INPUT_OPENAI_API_KEY=$(echo "$INPUT_OPENAI_API_KEY" | tr -d '\n' | tr -d ' ' | tr -d '"')
        # check if the input is empty
        if [ -z "$INPUT_OPENAI_API_KEY" ]; then
            echo "OpenAI API key is not appropriate. Try again."
            exit 1
        fi
        # show user the input
        echo "Your OpenAI API key is set to $INPUT_OPENAI_API_KEY"
    fi
fi

# if INPUT_OPENAI_API_KEY is not empty write to RC_FILE
if [ INPUT_OPENAI_API_KEY != "" ]; then
    echo "export ASK_SH_OPENAI_API_KEY=$INPUT_OPENAI_API_KEY" >> "$RC_FILE"
    echo "OpenAI API key is written to $RC_FILE"
fi

# check ask function should be updated or not
# the output of this section is correct UPDATE_SHELL_FUNCTION value

# check if ask function is defined in the current shell
if ! command -v ask >/dev/null 2>&1 ; then
    # ask function exists in the current shell
    # if not UPDATE_SHELL_FUNCTION is already turned off, keep going
    if [ -z "$UPDATE_SHELL_FUNCTION" ]; then
        echo "`ask` is already available in your shell."
        # capture the current installation of ask.sh shell function version
        CURRENT_SHELL_FUNCTION_VERSION=$(grep -oP '(?<=# ask.sh shell function v)\d' "$RC_FILE")
        if [ -z "$CURRENT_SHELL_FUNCTION_VERSION" ]; then
            echo "However, I could not find the version of the ask.sh shell function in $RC_FILE."
            echo "If you have manually installed ask.sh shell function, please delete the function and reload the shell and try again."
            echo "If this is first time you install ask.sh, the other programs might using the name `ask`. Please rename them and try again."
            echo "If you have manually installed ask.sh shell function, elsewhere than $RC_FILE, please manually update the shell function."
            echo "If you don't want to update the shell function automatically by this installer, please set environment variable DO_NOT_UPDATE_SHELL_FUNCTION=1 and run the installer again."
            exit 1
        fi
        # if the shell function version is same as the current version, do not update shell function
        if [ "$CURRENT_SHELL_FUNCTION_VERSION" = "$LATEST_SHELL_FUNCTION_VERSION" ]; then
            echo "The shell function version is latest. Skipping updating shell function."
            UPDATE_SHELL_FUNCTION=0
        else
            echo "The shell function version is outdated. Updating shell function."
        fi
   fi
fi

# if UPDATE_SHELL_FUNCTION is still 1, update the shell function
if [ "$UPDATE_SHELL_FUNCTION" = 1 ]; then
    echo "Updating shell function..."
    case "$SHELL" in
    *'bash'*)
    if [ -f "$HOME/.bashrc" ]; then
        write_ask_to_rc_file "$HOME/.bashrc"
    fi
    ;;
    *'zsh'*)
    if [ -f "$HOME/.zshrc" ]; then
        write_ask_to_rc_file "$HOME/.zshrc"
    fi
    ;;
    ;;
    esac

echo ""
echo "Congratulations, installation is complete! 🎉"
echo ""
echo "Don't forget to source ~$RC_FILE or reload shell!"
echo "Once everything's set up, you can chat with your AI by using the ask command. Why not start with \`ask hello\`? It's a great way to introduce yourself. The AI is looking forward to meeting you! 🤖"
echo "For more details and guidance, our helpful guide is ready for you at https://github.com/hmirin/ask.sh/blob/main/README.md#installation."
echo ""
echo "Thank you for installing and we hope you enjoy exploring with AI. Happy coding! 😄"
