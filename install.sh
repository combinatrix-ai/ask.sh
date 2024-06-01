#!/usr/bin/env sh

SHELL_SOURCE_LINE="eval \"\$(ask-sh --init)\""

if [ "$NO_ASK_SHELL_SETUP" = "1" ]; then
    echo "I will not automatically insert necessary shell function NO_ASK_SHELL_SETUP=1 is set"
    echo "Please add the following line to your shell config file manually:"
    echo "$SHELL_SOURCE_LINE"
    echo ""
fi

echo "** 🎁 Installing ask.sh to your shell. **"

# check current shell using variables.
if [ -n "$BASH_VERSION" ]; then
    echo "You're installing ask.sh in Bash."
    DEFAULT_SHELL="bash"
elif [ -n "$ZSH_VERSION" ]; then
    echo "You're installing ask.sh in zsh."
    DEFAULT_SHELL="zsh"
else
    echo "😭 Could not automatically determine your shell."
    echo "If you're using zsh or bash, please run the following command to install ask.sh:"
    echo "zsh -c \"\$(curl -fsSL https://raw.githubusercontent.com/hmirin/ask.sh/main/install.sh)\""
    echo "bash -c \"\$(curl -fsSL https://raw.githubusercontent.com/hmirin/ask.sh/main/install.sh)\""
    echo "If you are using a shell other than zsh or bash, this installer does not support it. However, ask.sh may work with manual install. Follow the instructions on https://github.com/hmirin/ask.sh#installation"
    exit 1
fi

# Check the default shell and select appropriate rc file
if [ "$DEFAULT_SHELL" = "zsh" ]; then
    RC_FILE="$HOME/.zshrc"
else
    RC_FILE="$HOME/.bashrc"
fi

if [ -z "$NO_ASK_SHELL_SETUP" ]; then
    echo "I will automatically insert necessary shell function to $RC_FILE"
fi
# ask user to continue with installation
printf "❓ Enter to continue, Ctrl-C to exit: "
read -r REPLY
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
    echo "Cargo is not installed. See https://www.rust-lang.org/tools/install for installation instructions"
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
    printf "❓ Do you want to proceed without tmux? (y/n): "
    read -r REPLY
    echo ""
    # shellcheck disable=SC3010
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Exiting..."
        exit 1
    fi
fi
echo "👍 Tmux is installed or user chose to proceed without it. Proceeding with installation."
echo "🤩 Prerequisites are satisfied!"
echo ""

# if ai-sh is installed by cargo, uninstall it
if cargo install --list | grep -q ai-sh; then
    # asking to uninstall ai-sh
    echo "Thank you for installing ai.sh. ai.sh is now renamed and upgraded to ask.sh."
    echo "To continue, ai-sh must be uninstalled."
    printf "❓ Do you want to uninstall ai-sh? (y/n): "
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

echo "** 🌏 Checking OpenAI api_key... **"

# Read ASK_SH_OPENAI_API_KEY from environment variable if not set, ask user to set it now
if [ -z "$ASK_SH_OPENAI_API_KEY" ]; then
    if [ -z "$NO_ASK_OPENAI_API_KEY" ]; then
        # use read
        echo "🤖 OpenAI API key is not set. Please set it now. This will be written to $RC_FILE. If you do not want me to setup ASK_SH_OPENAI_API_KEY variable, exit now and run the installer again with NO_ASK_OPENAI_API_KEY=1."
        echo "You can obtain your API key from https://platform.openai.com/account/api-keys."
        echo "Note: For those who want to restrict the API permissions, the minimum required permission for the API key is the `write` permission for `Model` capabilities."
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
    # shellcheck disable=SC2059
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
    # shellcheck disable=SC2059
    printf "
# This line is automatically inserted by the installer of ask.sh
$SHELL_SOURCE_LINE
" >>"$RC_FILE"
    echo "✨ Necessary lines are written to $RC_FILE"
    echo "If you don't like automatic writing to $RC_FILE, you can disable it by running the installer with NO_ASK_SHELL_SETUP=1 next time."
fi

# check if ask-sh is available in PATH
if ! command -v ask-sh >/dev/null 2>&1; then
echo "❌ Necessary rust package ask-sh is installed but cannot be accessed. Rust's bin path may not be added to your PATH."
echo "👉 It's usually under ~/.cargo/bin/"
echo "👀 Please add it to your PATH and restart your shell."
exit 1
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
