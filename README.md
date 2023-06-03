# ai.sh: The Future of Terminal Interfacing 🚀
(This section is entirely written by ChatGPT.)
Welcome to a revolutionary way of interacting with your terminal - the AI Terminal Assistant. Developed using Rust, this tool leverages the power of OpenAI's language model, providing you an interactive, context-aware dialog right in your console. It's easy to use, intuitive, and designed to supercharge your terminal sessions and command line productivity.

AI Terminal Assistant is your coding companion that provides AI insights at your command, simplifying your coding journey. Welcome to the future of coding!

# Commands

- `ask`: Query the AI for anything right from your terminal
- `fill`: Let the AI suggest commands, which you can directly input to the shell

## Query the AI from Your Terminal

No need to bother with a separate browser. Consult the AI about anything directly from your terminal. Here's an example:

```shell
❯ ask how to unzip tar gz
```

The AI responds:

```
❯ ai how to unzip tar gz
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
❯ ai what does this mean
```

The AI responds:

```
❯ ai what does this mean
The message "is a directory" means that you are trying to remove a directory (in this case, the .git directory). In order to remove a directory, you need to use the command "rm -r" instead of just "rm". The "-r" flag tells the "rm" command to remove directories recursively. However, be careful when using this command as it can delete multiple files and directories at once.
```

Great! Isn't it?

Note: You need to use tmux to use this feature. See [#tmux].

## Let the AI Write to Your Terminal Directly!

`fill` command let you type the command AI suggests directly to the shell.

Ask AI to make command like this:

````
❯ fill allocate 5GB file here
````

The AI responds:

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

Then, you get overlay selector (peco) to select the best command. 

```
QUERY>            IgnoreCase [1 (1/1)]
fallocate -l 5G filename
fallocate -l 5G -z filename
```

The selected command is typed into the shell directly. Just hit enter to execute.
```
❯ fallocate -l 5G filename
```

# Setup

## Presiquities

If you just want `ask` command:
- rust

If you also want `fill` command:
- `peco`: The `fill` command uses peco to let you select the command to execute from the AI suggested commands.

If you want `fill` command to work more nicely:
- `tmux`: If you run ai.sh in tmux, you can send the current terminal to the AI for context-aware input.
- `zsh`: If you run ai.sh in zhs, you can fill the next command directly. No copy-paste required.

## Installation

- Install using cargo: `cargo install ai_sh`
- Set `AI_SH_OPENAI_API_KEY` in your shell. Example: `export OPENAI_API_KEY=xxxx`
- If you don't want to use tmux or send your terminal outputs to the OpenAI server, set `AI_SH_NO_PANE=true`
  - If you don't set this variable, `ask` command will always recommend you to use tmux.
- Test the command with `ask hey whats up`

### Shell setup

If you just want `ask` command, you can safely skip here.

#### Zsh (recommended)

- If you use zsh, write this to the end of .zshrc
  - `fill` command which let you directly type the AI suggested commands.

```shell
function fill() {
    echo "$@" | ask --fill | peco | print -z 
}
```

#### Bash

- If you use bash, you can't directly type the AI suggested commands because bash doesn't allow.
- However, the code below insert the selected command to the end of history. Thus you can use the command if you push up key.

```shell
fill() {
    echo "$@" | ask --fill | peco | xargs -I {} history -s {}
}
```

# 


# License

This project is licensed under the terms of the MIT license.

# Disclaimer

- Use at Your Own Risk: This software is provided "as is" without warranty of any kind, either expressed or implied. The use of this software is at your own discretion and risk, and you will be solely responsible for any damage or loss that results from its use.

- Data Transmission to OpenAI: By using this software, the text you input, as well as certain terminal information, will be sent to OpenAI as part of the software's operation.

- Potential for Unintended Data Transmission: Please be aware that due to the possibility of software bugs or unexpected behaviour, unintended data may be sent to OpenAI or whatsoever. While we strive to ensure the security and privacy of your data, these risks can never be completely eliminated.


# Q&A

#### How ai.sh send the current output of terminal?

- ai.sh use `tmux capture-pane -p` to get the current terminal status. Therefore, if you run `ask` and `fill` in tmux pane, text on the pane will be sent to the OpenAI.
- This will give AI the context of your request and improve the result.
- If you don't want to use this feature, set `AI_SH_NO_PANE=true` in your shell.
