# ask.sh: The Future of Terminal Interfacing 🚀

`ask.sh`: chat with AI in your terminal.

![example](https://github.com/hmirin/ask.sh/assets/1284876/8f920268-3a87-4d05-8499-9171df7905bc)

(This section is entirely written by ChatGPT.)

Welcome to a revolutionary way of interacting with your terminal - meet `ask.sh`. Developed using Rust, this tool leverages the power of OpenAI's language model, providing you with an interactive, context-aware dialogue right in your console. It's easy to use, intuitive, and designed to supercharge your terminal sessions and command-line productivity.

ask.sh is your coding companion that offers AI insights at your command, simplifying your coding journey. Welcome to the future of coding!

- Key Features
  - [Query the AI from Your Terminal](#query-the-ai-from-your-terminal)
  - [The AI Understands Your Context!](#the-ai-understands-your-context)
  - [Multiturn Conversations with AI in Your Terminal](#multiturn-conversations-with-ai-in-your-terminal)
  - [Let the AI Write to Your Terminal Directly!](#let-the-ai-write-to-your-terminal-directly)
  - [OS / CPU arch / Shell Aware Conversation!](#os--cpu-arch--shell-aware-conversations)

# Demo
Download iris dataset, do some analysis, all by shell commands without leaving the terminal. (Played in 3x. Using GPT-4)
![Demo GIF](https://github.com/hmirin/ask.sh/assets/1284876/93f54d0f-3ae8-4178-8e8e-201da7314441)

You might be interested in [other examples](examples.md)

# Quick Start

If you're on Bash:
```
bash -c "$(curl -fsSL https://raw.githubusercontent.com/hmirin/ask.sh/main/install.sh)"
```

If you're on Zsh:
```
zsh -c "$(curl -fsSL https://raw.githubusercontent.com/hmirin/ask.sh/main/install.sh)"
```

Then, follow the instructions. See [Setup](#setup) for manual installation.

# Key Features

## Query the AI from Your Terminal

No need to bother with a separate browser. Consult the AI about anything directly from your terminal. Here's an example:

```shell
❯ ask how to unzip tar gz
```

The AI responds:

```
❯ ask how to unzip tar gz
To unzip a tar.gz file, you can use the following command in a terminal:

`tar -xzf file.tar.gz`

where "file.tar.gz" is the name of your compressed file. This command will extract the contents of the file into a directory with the same name as the file. If you want to extract the contents into a different directory, you can specify the path with the -C option:

`tar -xzf file.tar.gz -C /path/to/directory`

Once the extraction is complete, you should have access to the uncompressed files in the directory where you extracted them.
```

Answers are printed incrementally, ensuring you don't have to wait until the complete response is available!


## The AI Understands Your Context!

The AI Terminal Assistant can interpret the context of your actions, providing more meaningful suggestions and explanations. Check this out:

You have typed a command and got error.

```shell
❯ rm .git
rm: .git: is a directory
```

Just ask AI:
```
❯ ask what does this mean
```

The AI responds:

```
❯ ask what does this mean
The message "is a directory" means that you are trying to remove a directory (in this case, the .git directory). In order to remove a directory, you need to use the command "rm -r" instead of just "rm". The "-r" flag tells the "rm" command to remove directories recursively. However, be careful when using this command as it can delete multiple files and directories at once.
```

Great! Isn't it?

Note: You need to use tmux to use this feature. See [Q&A](#how-aish-send-the-current-output-of-terminal).

## Multiturn Conversations with AI in Your Terminal

By understanding the context of your terminal, `ask.sh` is not just about asking questions and receiving answers; it's about engaging in an insightful dialogue that spans multiple turns. Here's a peek into what that might look like:

```
❯ git init
Reinitialized existing Git repository in /path/to/ask.sh/.git/
```

Do you know what exactly happen when you `git init` again? Let's ask:
```
❯ ask what does reinitialized mean
"Reinitialized" means that Git has reset the repository's initialization parameters. When you run `git init` in an existing Git repository, Git will re-run the repository's initialization process, but with the existing repository metadata in place. This allows you to reset the repository configuration without losing the existing commit history and other Git metadata.
```

Hmm, still have question? Ask again:

```
❯ ask what configuration is reset
When you run init` with the `--reinitialize` or `-o` option, Git will re-run the repository's initialization process, but with the existing repository metadata in place. This allows you to reset the repository configuration without losing the existing commit history and other Git metadata. The specific configuration that is reset depends on what options you passed to `git init`. If you did not pass any options, then Git will reset all initialization parameters to their default values. Is there anything else I can help you with?
```

## Let the AI Write to Your Terminal Directly!

`ask` command let you type the command AI suggests directly to the shell.

Ask AI to make command like this:

````
❯ ask allocate 5GB file here
````

The AI responds with some suggestions of the command:

````
To allocate a 5GB file here, you can use the `fallocate` command. `fallocate` can be used to allocate space for a file using either the preallocated or the zeroed method.

To allocate space for a 5GB file with the preallocated method, run the following command:

```
fallocate -l 5G filename
```

Remember to replace `filename` with the desired name for your file. This will create a file of size 5GB without actually allocating the space until it is actually used.

To allocate space for a 5GB file with the zeroed method, run the following command:

```
fallocate -l 5G -z filename
```

This will create a file of size 5GB with all bits set to 0.
````

If AI suggested commands to execute, you get an overlay selector (peco) to select the best command.

```
AI suggested commands (Enter to use / Ctrl+C to exit):
fallocate -l 5G filename
fallocate -l 5G -z filename
```

The selected command is typed into the shell directly. Just hit enter to execute.
```
❯ fallocate -l 5G filename
```

You won't struggle text-selection on terminal!

## OS / CPU arch / Shell Aware Conversations!

`ask` command is aware of your shell, OS, and CPU arch. So you can ask shell, OS, and CPU arch specific questions.

```
❯ ask how to install tmux
```

The AI responds:
````
❯ ask how to install tmux
To install tmux on your macOS machine, you can use Homebrew package manager. Here are the steps:

1. Open the terminal.

2. Install Homebrew package manager by running the following command:
```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
```

3. Once Homebrew is installed, install tmux:
```
brew install tmux
```

4. Wait for the installation to complete.

That's it! Now you can use tmux on your macOS machine.
````

Perfect! Isn't it? (Some may not like homebrew though...)

# Want to see more?

See [examples](https://github.com/hmirin/ask.sh/blob/main/examples.md)!

# Setup

## Prerequisites

- rust
- `peco`: The `ask` command uses peco to let you select the command to execute from the AI suggested commands.

Optional, but highly recommended if you want `ask` command to work more nicely:
- `tmux`: If you run `ask` command in tmux, you can send the current terminal to the AI for context-aware input.
- `zsh`: If you run `ask` command in zsh (not Bash), you can let AI write the next command directly to your terminal. No copy-paste is required.

## Installation

- In most cases, install script should work
```
curl -sSL https://raw.githubusercontent.com/hmirin/ask.sh/main/install.sh | bash
```

- If you want to install manually, follow the steps below: 

1. Install crate using cargo: `cargo install ask-sh`
2. Set `AI_SH_OPENAI_API_KEY` in your shell
3. If you don't want to use tmux or send your terminal outputs to the OpenAI server, set `AI_SH_NO_PANE=true`
  - If you don't set this variable when you query to `ask`, `ask` command will always recommend you to use tmux.
4. Set up your shell environment (see [Shell setup](#shell-setup))
  - Do not forget to source your shell config file or restart your shell.
5. Test the command with `ask hey whats up`
  - If AI responds with phrases like "As an AI assistant, I can't experience emotions blah blah blah", it means that the setup is done correctly.

### Shell setup

Add the following to your shell config file.

#### zsh (recommended)

```shell
function ask() {
    suggested_commands=`echo "$@" | ask-sh 2> >(cat 1>&2)`
    if [ -z "$suggested_commands" ]; then
        return
    else
        selected_command=`echo "$suggested_commands" | peco  --prompt "AI suggested commands (Enter to use / Ctrl+C to exit):"`
        if [ -z "$selected_command" ]; then
            return
        else
            print -z $selected_command
        fi
    fi
}
```

#### Bash and others

- If you use Bash, you can't let the `ask` command to type the AI suggested commands because Bash doesn't have necessary APIs.
- However, the code below insert the selected command to the end of history. Thus you can use the command if you push up key.

```shell
function ask() {
    suggested_commands=`echo "$@" | ask-sh 2> >(cat 1>&2)`
    if [ -z "$suggested_commands" ]; then
        return
    else
        selected_command=`echo "$suggested_commands" | peco  --prompt "AI suggested commands (Enter to use / Ctrl+C to exit):"`
        if [ -z "$selected_command" ]; then
            return
        else
            history -s $selected_command
        fi
    fi
}
```

# Extras!

- We will soon release a Chrome Extension that will let you open chat.openai.com with the terminal output.
- If you want more, please let us know by creating an issue! (But note that I'm a little busy these days.)


# License

This project is licensed under the terms of the MIT license.

# Disclaimer

- Use at Your Own Risk: This software is provided "as is" without warranty of any kind, either expressed or implied. The use of this software is at your own discretion and risk, and you will be solely responsible for any damage or loss that results from its use.

- Data Transmission to OpenAI: By using this software, the text you input, as well as certain terminal information, will be sent to OpenAI as part of the software's operation.

- Potential for Unintended Data Transmission: Please be aware that due to the possibility of software bugs or unexpected behaviour, unintended data may be sent to OpenAI or whatsoever. While we strive to ensure the security and privacy of your data, these risks can never be completely eliminated.


# Q&A

#### How ask.sh send the current output of terminal?

- ask.sh use `tmux capture-pane -p` to get the current terminal status. Therefore, if you run `ask` in tmux pane, text on the pane will be sent to the OpenAI.
- This will give AI the context of your request and improve the result.
- If you don't want to use this feature, set `AI_SH_NO_PANE=true` in your shell.

#### Privacy concerns?

- As of 5th July 2023, OpenAI [states](https://openai.com/policies/api-data-usage-policies) that they will not use data submitted by customers via their API to train or improve their models, unless you explicitly opt-in to do so.
- And of course, you can use ask.sh without sending the current terminal output to the OpenAI server. Just set `AI_SH_NO_PANE=true` in your shell.

#### Can I use GPT-4?

- Yes! You can use GPT-4 by setting the environmanet_variable `AI_SH_OPENAI_MODEL=gpt-4`.
  - This environment variable is just passed to OpenAI API. So you can use whatever model OpenAI serves.
- Currently, default model is set to `gpt-3.5-turbo`.

#### Why Rust?

- It's just because shell tools should have less dependencies!
  - To my knowledge, there's no standard way in Python to make a command available to everywhere.

# Contributing
- Of course, we welcome contributions! Please feel free to open an issue or submit a pull request.