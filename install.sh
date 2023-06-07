#!/usr/bin/env sh

SHELL_SOURCE_LINE="eval \"\$(ask-sh --init)\""

if [ "$NO_ASK_SHELL_SETUP" = "1" ]; then
    echo "I will not automatically insert necessary shell function NO_ASK_SHELL_SETUP=1 is set"
    echo "Please add the following line to your shell config file manually:"
    echo "$SHELL_SOURCE_LINE"
    echo ""
fi

DEFAULT_SHELL=$(basename "$SHELL")

# Fail if $SHELL is not set
# Check if ask function is defined in the current shell
# Check the default shell and write to appropriate rc file
if [ -z "$DEFAULT_SHELL" ]; then
    echo "😭 Could not automatically determine your shell based on \$SHELL environment variable."
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

echo "** Installing ask.sh to your $DEFAULT_SHELL. **"
echo ""

# recommend bash users to use zsh
if [ "$DEFAULT_SHELL" = "bash" ]; then
    echo "** 😎 We recommend you to use zsh instead of bash. **"
    echo "zsh is a modern and powerful shell, and ask.sh works better with zsh."
    echo "Using zsh, you can use auto-typing of the AI suggested commands."
    echo "Nevertheless, let's proceed with installing ask.sh for bash."
    echo ""
fi

# From now on, we assume that the default shell is either zsh or bash

echo "** 🍭 Checking prerequisites... **"

# Check if cargo is installed
if ! command -v cargo >/dev/null 2>&1; then
    echo "Cargo is not installed. See https://doc.rust-lang.org/cargo/getting-started/installation.html for installation instructions"
    exit 1
fi
echo "📦 Cargo is installed. Proceeding with installation."

# Check if peco is installed
if ! command -v peco >/dev/null 2>&1; then
    echo "Peco is not installed. See https://github.com/peco/peco#installation for installation instructions"
    exit 1
fi
echo "🍬 Peco is installed. Proceeding with installation."

# Check if tmux is installed
if ! command -v tmux >/dev/null 2>&1; then
    echo "Tmux is not installed. ask.sh uses tmux to capture current terminal screen and send to API."
    echo "If you proceed without installation, you cannot have context-aware/multi-turn conversations with AI."
    echo "See https://github.com/tmux/tmux/wiki/Installing for installation instructions"
    printf "Do you want to proceed without tmux? (y/n): "
    read -r REPLY
    echo ""
    # shellcheck disable=SC3010
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Exiting..."
        exit 1
    fi
fi
echo "🖼️ Tmux is installed or user chose to proceed without it. Proceeding with installation."
echo "🤩 Prerequisites are satisfied!"
echo ""

# if ai-sh is installed by cargo, uninstall it
if cargo install --list | grep -q ai-sh; then
    # asking to uninstall ai-sh
    echo "Thank you for installing ai.sh. ai.sh is now renamed and upgraded to ask.sh."
    echo "To continue, ai-sh must be uninstalled."
    printf "Do you want to uninstall ai-sh? (y/n): "
    read -r REPLY
    echo ""
    # shellcheck disable=SC3010
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
    cargo uninstall ai-sh
    echo "Uninstalled ai-sh."
fi

# Install or upgrade ask.sh using cargo and if failed, exit
echo "** 🌎 Installing/Updating ask.sh... **"
if ! cargo install ask-sh; then
    echo "💥 Failed to install ask.sh. Please check the error message above."
    exit 1
fi
echo "✨ ask.sh is installed/updated."
echo ""

# Check the default shell and select appropriate rc file
if [ "$DEFAULT_SHELL" = "zsh" ]; then
    RC_FILE="$HOME/.zshrc"
else
    RC_FILE="$HOME/.bashrc"
fi

echo "** 🌏 Checking OpenAI api_key... **"

# Read ASK_SH_OPENAI_API_KEY from environment variable if not set, ask user to set it now
if [ -z "$ASK_SH_OPENAI_API_KEY" ]; then
    if [ -z "$NO_ASK_OPENAI_API_KEY" ]; then
        # use read
        echo "🤖 OpenAI API key is not set. Please set it now. This will be written to $RC_FILE. If you do not want me to setup ASK_SH_OPENAI_API_KEY variable, exit now and run the installer again with NO_ASK_OPENAI_API_KEY=1."
        echo "You can obtain your API key from https://platform.openai.com/account/api-keys"
        printf "Please enter your OpenAI API key: "
        read -r INPUT_OPENAI_API_KEY
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
        echo ""
    fi
else
    echo "🤖 OpenAI API key is already set!"
    echo ""
fi

# if INPUT_OPENAI_API_KEY is not empty write to RC_FILE
if [ -n "$INPUT_OPENAI_API_KEY" ]; then
    printf "
# This variable is automatically inserted by the installer of ask.sh
export ASK_SH_OPENAI_API_KEY=$INPUT_OPENAI_API_KEY
" >>"$RC_FILE"
    echo "🎂 OpenAI API key is written to $RC_FILE"
fi

# check ask function should be updated or not
# the output of this section is correct UPDATE_SHELL_FUNCTION value

echo "** 🌍 Checking shell setup... **"

# check if sourcing line is written in RC_FILE
if grep -q "$SHELL_SOURCE_LINE" "$RC_FILE"; then
    echo "🔎 Necessary lines are already available in your $RC_FILE!"
    echo "😆 Thank you for keep using ask.sh!"
else
    echo "🔎 Necessary lines are not available in your $RC_FILE."
    echo "😆 Don't worry! I will write the necessary lines to $RC_FILE."
    printf "
# This line is automatically inserted by the installer of ask.sh
$SHELL_SOURCE_LINE
" >>"$RC_FILE"
    echo "✨ Necessary lines are written to $RC_FILE"
    echo "If you don't like automatic writing to $RC_FILE, you can disable it by running the installer with NO_ASK_SHELL_SETUP=1 next time."
fi

echo ""
echo "🎉 Congratulations, installation is complete! 🎉"
echo ""
echo "❗️❗️❗️ Don't forget to \`source $RC_FILE\` or reload shell! ❗️❗️❗️"
echo ""
echo "Once everything's set up, you can chat with your AI by using the ask command. Why not start with \`ask hello\`? It's a great way to introduce yourself. The AI is looking forward to meeting you! 🤖"
echo "For more details and guidance, our helpful guide is ready for you at https://github.com/hmirin/ask.sh/blob/main/README.md#installation."
echo ""
echo "Thank you for installing and we hope you enjoy exploring with AI. Happy coding! 😄"
