# Project Setup
- This project is developing a library called `ask-sh` and its companion command `ask`, written in Rust and shell

# How ask works
- In zsh or bash, `eval "$(ask-sh --init)"` is executed in .zshrc/.bashrc and you can use the 'ask' shell command
- Use like `ask how can I undo git commit` 

# Testing
- `cargo build`: Build and see errors
- `cargo install --path ./ && source ~/.zshrc`: Update command and shell. You can call updated `ask` command in terminal.
- `ask-sh -v` will show you the current installed version. If you're unsure whether the command is updated, change the version number in Cargo.toml, reinstall, and verify that the new version is displayed.
- `ask hey there` will return something
- `SYSTEM_PROMPT_WITHOUT_PANE='' USER_PROMPT_WITHOUT_PANE='{user_input}' ASK_SH_DEBUG=true ask "who are you."` will return a raw response without applying templates, allowing you to verify responses directly from OpenAI or Anthropic
- Use `--debug` flag to see detailed debug information during execution
