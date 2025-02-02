# ask.sh: AI terminal assistant that read from & write to your terminal

- `ask.sh` is an AI terminal assistant that supports multiple LLM providers (OpenAI and Anthropic)!
- What's unique?
    - `ask.sh` can *read from and write to your terminal*!
        - No need to copy and paste error texts to a browser window and then bring solutions back to the terminal!
        - It maintains some memory and can handle multi-turn conversations!
- `ask.sh` provides `ask` command to your terminal:

<img src="https://github.com/hmirin/ask.sh/assets/1284876/60f4f432-0306-4284-a1ed-5bf87aba6b04" style="max-width: 100%;" />

- Key Features
  - [Query the AI from Your Terminal](#query-the-ai-from-your-terminal)
  - [The AI Understands Your Context!](#the-ai-understands-your-context)
  - [Multiturn Conversations with AI in Your Terminal](#multiturn-conversations-with-ai-in-your-terminal)
  - [Let the AI Write to Your Terminal Directly!](#let-the-ai-write-to-your-terminal-directly)
  - [OS / CPU arch / Shell Aware Conversation!](#os--cpu-arch--shell-aware-conversations)

# Demo

From downloading the Titanic dataset using the curl command to calculate the survival rate for all combinations of sex and room class using the `awk` command, without leaving the terminal. (Played in 2x. Using `gpt-3.5-turbo`)

https://github.com/hmirin/ask.sh/assets/1284876/4311db79-c56e-46c9-9cfe-66fce8f800df

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

Then, follow the instructions shown by the installer. 
See [Setup](#setup) for manual installation.

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

Note: You need to use tmux to use this feature. See [Q&A](#how-asksh-send-the-current-output-of-terminal).

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

If the AI has suggested commands to execute, it will ask if you want to use those commands.

```
👋 Hey, AI has suggested some commands that can be typed into your terminal.
🔍 Press Enter to view and select the commands, or type any other key to exit:
```

After you press Enter, an overlay selector (`peco`) will appear, allowing you to select the most suitable command.

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

- If you use Bash or zsh, [install script](#quick-start) should work.
    - If you find any problem, please file an issue!
- If you want to install `ask.sh` manually, follow the steps below:
    1. Install [prerequisites](#prerequisites)
    2. Install `ask.sh` using cargo: `cargo install ask-sh`
    3. Choose and configure your LLM provider:
       - For OpenAI and its compatible APIs (default):
         - Set `ASK_SH_OPENAI_API_KEY` in your shell
         - You can get your API key from [OpenAI](https://platform.openai.com/account/api-keys)
         - Optional: Set `ASK_SH_OPENAI_BASE_URL` for custom OpenAI-compatible endpoints
           - For Ollama: `ASK_SH_OPENAI_BASE_URL="http://localhost:11434/v1"`
           - For Deepseek: `ASK_SH_OPENAI_BASE_URL="https://api.deepseek.com"`
           - See [here](#which-llm-providers-are-supported) for details.
       - For Anthropic:
         - Set `ASK_SH_ANTHROPIC_API_KEY` in your shell
         - You can get your API key from [Anthropic](https://console.anthropic.com/account/keys)
         - Set `ASK_SH_LLM_PROVIDER=anthropic`
    4. Optional: Configure model settings
       - OpenAI: Set `ASK_SH_OPENAI_MODEL` (default: gpt-4o)
       - Anthropic: Set `ASK_SH_ANTHROPIC_MODEL` (default: claude-3-opus-20240229)
    5. If you don't want to use tmux or send your terminal outputs to the LLM provider, set `ASK_SH_NO_PANE=true`
        - If you don't set this variable when you query to `ask`, `ask` command will always recommend you to use tmux.
    6. Set up your shell environment
        - Add `eval "$(ask-sh --init)"` to your rc file (e.g., `~/.bashrc`, `~/.zshrc`)
        - Do not forget to source your shell config file or restart your shell.
    6. Test the command with `ask hey whats up`
        - If AI responds with phrases like "As an AI assistant, I can't experience emotions blah blah blah", it means that the setup is done correctly.

# Extras!

- We will soon release a Chrome Extension that will let you open chat.openai.com with the terminal output.
- If you want more, please let us know by creating an issue! (But note that I'm a little busy these days.)


# License

This project is licensed under the terms of the MIT license.

# Disclaimer

- Use at Your Own Risk: This software is provided "as is" without warranty of any kind, either expressed or implied. The use of this software is at your own discretion and risk, and you will be solely responsible for any damage or loss that results from its use.

- Data Transmission to LLM Providers: By using this software, the text you input, as well as certain terminal information, will be sent to the configured LLM provider (OpenAI or Anthropic) as part of the software's operation.

- Potential for Unintended Data Transmission: Please be aware that due to the possibility of software bugs or unexpected behaviour, unintended data may be sent to LLM providers or whatsoever. While we strive to ensure the security and privacy of your data, these risks can never be completely eliminated.


# Q&A

#### Why Another Terminal AI Assistant?

Sure, there are plenty of [great projects](https://github.com/sindresorhus/awesome-chatgpt#cli-tools) for terminal AI assistants already out there. But, in my experience, none of these tools completely meet the criteria I consider essential:

- Simple API: Who wants to remember complex commands and options? Isn't that why we need AI in the first place?
- Multi-turn capability: Most tools cater to single-turn queries because they lack memory, an important feature in my book.
- Terminal Reading: Copying and pasting from terminal? No, thank you!
- Direct Command Execution on zsh/bash: Running commands in a wrapped environment or REPLs isn't my cup of tea.

This is why I created `ask.sh`.

Similar projects:
- [Github Copilot for CLI](https://githubnext.com/projects/copilot-cli)
  - It's fantastic, but it's primarily focused on a single-turn conversation for command generation.
- [ShellGPT](https://github.com/TheR1D/shell_gpt)
  - A pioneering work with great features! However, I find the API overly complex and I'm not a fan of the way it runs commands in REPL.


#### How ask.sh send the current output of terminal?

- ask.sh use `tmux capture-pane -p` to get the current terminal status. Therefore, if you run `ask` in tmux pane, text on the pane will be sent to the OpenAI.
- This will give AI the context of your request and improve the result.
- If you don't want to use this feature, set `ASK_SH_NO_PANE=true` in your shell.

#### Privacy concerns?

- Data usage policies:
  - OpenAI [states](https://openai.com/policies/api-data-usage-policies) that they will not use data submitted via their API to train or improve their models, unless you explicitly opt-in to do so.
  - Anthropic [states](https://console.anthropic.com/legal/terms) that they may use API data to improve their services, but you can request data deletion.
- You can use ask.sh without sending terminal output to any LLM providers by setting `ASK_SH_NO_PANE=true` in your shell.

#### Which LLM providers are supported?

- OpenAI and its compatible APIs (default)
  - Models: GPT-3.5, GPT-4, and any other models OpenAI serves
  - Configure with `ASK_SH_OPENAI_MODEL` (default: gpt-4o)
    - Example: `ASK_SH_OPENAI_MODEL=gpt-4`
  - Custom Endpoints: You can use OpenAI-compatible APIs by setting `ASK_SH_OPENAI_BASE_URL`
    - Ollama Example: `ASK_SH_OPENAI_BASE_URL="http://localhost:11434/v1" ASK_SH_OPENAI_MODEL="deepseek-r1:8b" ask who are you`
    - DeepSeek Example: `ASK_SH_OPENAI_BASE_URL="https://api.deepseek.com" ASK_SH_OPENAI_MODEL="deepseek-chat" ASK_SH_OPENAI_API_KEY=xxx ask who are you`
- Anthropic
  - Models: Claude-3 and other Claude models
  - Configure with `ASK_SH_ANTHROPIC_MODEL` (default: claude-3-5-opus-latest)
  - Example: `ASK_SH_LLM_PROVIDER=anthropic ASK_SH_ANTHROPIC_MODEL=claude-3-opus-20240229`

To switch providers, set `ASK_SH_LLM_PROVIDER` to either `openai` or `anthropic`. Don't forget to set the corresponding API key:
- OpenAI: `ASK_SH_OPENAI_API_KEY`
- Anthropic: `ASK_SH_ANTHROPIC_API_KEY`

#### Why Rust?

- It's just because shell tools should have less dependencies!
  - To my knowledge, there's no standard way in Python to make a command available to everywhere.

#### Wanna change prompts?

You can customize the prompts used by ask.sh by setting the following environment variables:

- `ASK_SH_SYSTEM_PROMPT_WITH_PANE`: System prompt used when terminal context is available (in tmux)
- `ASK_SH_USER_PROMPT_WITH_PANE`: User prompt format used when terminal context is available
- `ASK_SH_SYSTEM_PROMPT_WITHOUT_PANE`: System prompt used when terminal context is not available
- `ASK_SH_USER_PROMPT_WITHOUT_PANE`: User prompt format used when terminal context is not available

The prompts support the following variables that will be replaced with actual values:
- `{user_arch}`: CPU architecture
- `{user_os}`: Operating system
- `{user_shell}`: Current shell
- `{pane_text}`: Terminal context (only in WITH_PANE prompts)
- `{user_input}`: User's input/question

See the default prompts in [src/prompt.rs](src/prompts.rs) for examples.

# Contributing
- Of course, we welcome contributions! Please feel free to open an issue or submit a pull request.

# Acknowledgements

A special thanks to @xarsh for their effort in debugging, testing, and providing valuable feedback, contributing significantly to the refinement of this project.
Our sincere gratitude to @matsurih for their outstanding contribution to our work, marking the project's first-ever PR.
