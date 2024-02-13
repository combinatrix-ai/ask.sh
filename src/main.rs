use async_openai::config::OpenAIConfig;
use async_openai::types::ChatCompletionRequestUserMessageArgs;
use async_openai::types::CreateChatCompletionRequestArgs;
use async_openai::{types::ChatCompletionRequestSystemMessageArgs, Client};
use dotenv::dotenv;
use futures::stream::StreamExt;
use regex::Regex;
use std::env;
use std::error::Error;
use std::io::{self, BufRead};
use std::io::{stdout, Write};
use std::process;
mod prompts;
use serde::Serialize;
use std::env::consts::{ARCH, OS};

// args
const ARG_DEBUG: &'static str = "--debug_ask_sh";
const ARG_NO_PANE: &'static str = "--no_pane";
const ARG_NO_SUGGEST: &'static str = "--no_suggest";
const ARG_VERSION: &'static str = "--version";

const ARG_STRINGS: &'static [&'static str] = &[ARG_DEBUG, ARG_NO_PANE, ARG_NO_SUGGEST];

// special arg
const ARG_INIT: &'static str = "--init";

// env
const ENV_DEBUG: &'static str = "ASK_SH_DEBUG";
const ENV_NO_PANE: &'static str = "ASK_SH_NO_PANE";
const ENV_NO_SUGGEST: &'static str = "ASK_SH_NO_SUGGEST";
const ENV_OPENAI_API_KEY: &'static str = "ASK_SH_OPENAI_API_KEY";
const ENV_OPENAI_MODEL: &'static str = "ASK_SH_OPENAI_MODEL";

fn get_openai_api_key() -> Option<String> {
    dotenv().ok();
    env::var(ENV_OPENAI_API_KEY).ok()
}

fn get_openai_model_name() -> String {
    dotenv().ok();
    match env::var(ENV_OPENAI_MODEL) {
        Ok(val) => val,
        Err(_e) => "gpt-3.5-turbo".to_string(),
    }
}

fn get_env_flag(key: &str) -> bool {
    dotenv().ok();
    match env::var(key) {
        Ok(val) => val.parse::<bool>().unwrap_or(false),
        Err(_e) => false,
    }
}

// json structure
#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct Payload {
    model: String,
    messages: Vec<Message>,
}

struct UserInfo {
    arch: String,
    os: String,
    shell: String,
    // TODO: add distro info if linux
}

/// Interactive chat with OpenAI API.
///
/// # Examples
///
/// ```
/// println!(chat("api-key", "gpt-3.5-turbo", "You're an AI assistant.", "Tell me how to unarchive tar.gz." ))
/// ```
/// Taken from https://github.com/64bit/async-openai/blob/main/examples/chat/src/main.rs under MIT License
#[tokio::main]
async fn chat(
    api_key: &str,
    model_name: &str,
    user_input: &str,
    system_message: &str,
    debug_mode: &bool,
) -> Result<String, Box<dyn Error>> {
    let config = OpenAIConfig::new().with_api_key(api_key);
    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .model(model_name)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(system_message)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(user_input)
                .build()?
                .into(),
        ])
        .build()?;

    let mut stream = client.chat().create_stream(request).await?;

    let mut lock = stdout().lock();
    let mut response_to_return = String::new();
    while let Some(result) = stream.next().await {
        match result {
            Ok(response) => {
                response.choices.iter().for_each(|chat_choice| {
                    if let Some(ref content) = chat_choice.delta.content {
                        response_to_return = response_to_return.clone() + content;
                        write!(lock, "{}", content).unwrap();
                    }
                });
            }
            Err(err) => {
                writeln!(lock, "error: {err}").unwrap();
            }
        }
        stdout().flush()?;
    }
    Ok(response_to_return)
}

fn post_process(text: &str) -> Vec<String> {
    let mut commands = Vec::new();
    // extract all commands enclosed in ``` ```
    let re = Regex::new(r#"```(.+?)```"#).unwrap();
    re.captures_iter(&text.replace("\n", ";")).for_each(|cap| {
        commands.push(
            cap[1]
                .to_string()
                .replace("\n", " ")
                .trim_start_matches(";")
                .trim_end_matches(";")
                .trim()
                .to_owned(),
        );
    });
    // if command start from bash; or sh; remove it
    commands = commands
        .iter()
        .map(|command| {
            if command.starts_with("bash;") {
                command.trim_start_matches("bash;").trim().to_owned()
            } else if command.starts_with("sh;") {
                command.trim_start_matches("sh;").trim().to_owned()
            } else {
                command.to_owned()
            }
        })
        .collect();
    // deduplicate with keeping the order
    // count the number of occurences of each command
    let mut counts = std::collections::HashMap::new();
    for command in &commands {
        let count = counts.entry(command).or_insert(0);
        *count += 1;
    }
    // add only the first occurence of each command to deduped_commands
    // TODO: not elegant
    let mut deduped_commands: Vec<String> = Vec::new();
    for command in &commands {
        if deduped_commands.contains(&command) {
        } else {
            deduped_commands.push(command.to_string());
        }
    }
    deduped_commands
}

fn print_init_script() {
    print!(
        r#"# This function is automatically generated by ask-sh --init
# ask.sh shell function v2
ask() {{
    suggested_commands=`echo "$@" | ask-sh 2> >(cat 1>&2)`
    if [ -n "$suggested_commands" ]; then
        printf "\n"
        printf "👋 Hey, AI has suggested some commands that can be typed into your terminal.\n"
        printf "🔍 Press Enter to view and select the commands, or type any other key to exit:"
        if [ -n "$ZSH_VERSION" ]; then # read a single char
            read -r -k 1 REPLY # zsh
        else
            read -r -n 1 REPLY # bash
        fi
        REPLY="${{REPLY#"${{REPLY%%[![:space:]]*}}"}}"  # trim whitespaces
        printf "\033[3A" # go back three lines
        printf "\033[2K\n\033[2K\n\033[2K\n\033[2K\n" # delete uninformative lines
        printf "\033[4A" # go back again
        if [ -z "$REPLY" ] ; then
            selected_command=`echo "$suggested_commands" | peco  --prompt "AI suggested commands (Enter to use / Ctrl+C to exit):"`
            if [ -n "$selected_command" ]; then
                if ! print -z $selected_command 2>/dev/null; then
                    history -s $selected_command
                fi
            fi
        fi
    fi
    if [ -z "$ASK_SH_NO_UPDATE" ]; then
        latest_version=`cargo search ask-sh | grep ask-sh | awk '{{print $3}}' | cut -d '"' -f2`
        current_version=`ask-sh --version`
        if [ "$latest_version" != "$current_version" ]; then
            # clear line
            printf "\n"
            printf "🎉 New version of ask-sh is available! (Current: $current_version vs New: $latest_version) Set \$ASK_SH_NO_UPDATE=1 to suppress this notice.\n"
            printf "🆙 Press Enter to run update now, or type any other key to exit:"
            if [ -n "$ZSH_VERSION" ]; then # read a single char
                read -r -k 1 REPLY # zsh
            else
                read -r -n 1 REPLY # bash
            fi
            REPLY="${{REPLY#"${{REPLY%%[![:space:]]*}}"}}"  # trim whitespaces
            if [ -z "$REPLY" ] ; then
                cargo install --force ask-sh
                printf "\nDone! Please restart your shell or source ~/.zshrc or ~/.bashrc etc... to use the new version.\n"
            else
                printf "\nOk, you can update ask-sh later by running 'cargo install --force ask-sh'.\n"
            fi
        fi
    fi
}}
"#
    );
}

fn main() {
    // if called with only --init, the command emits a shell script to be sourced
    if env::args().len() == 2 && env::args().nth(1).unwrap() == ARG_INIT {
        print_init_script();
        return;
    }

    // if called with only --version, print version and exit
    if env::args().len() == 2 && env::args().nth(1).unwrap() == ARG_VERSION {
        println!("{}", env!("CARGO_PKG_VERSION"));
        return;
    }
    // check input from users

    // arg without the first executable name
    let args: Vec<String> = env::args().skip(1).collect();
    // check if args are all predefined args
    let is_using_stdin = args.iter().all(|arg| ARG_STRINGS.contains(&arg.as_str()));

    let user_input = if is_using_stdin {
        io::stdin().lock().lines().next().unwrap().unwrap()
    } else {
        args.join(" ")
    };

    // filter out predefined args
    let user_input_without_flags = user_input
        .split_whitespace()
        .filter(|arg| !ARG_STRINGS.contains(&arg))
        .collect::<Vec<&str>>()
        .join(" ");

    // debug_mode is true if args contains --debug_ASK_SH or stdin text contains "--debug_ASK_SH" or env var ASK_SH_DEBUG is defined
    let debug_mode = env::args()
        .any(|arg| arg == ARG_DEBUG || user_input.contains(ARG_DEBUG) || get_env_flag(ENV_DEBUG));

    // send_pane is false if args contains --no_pane or stdin text contains "--no_pane" or env var ASK_SH_NO_PANE is defined
    // send_pane is immutable in case tmux capture-pane -p fails
    let mut send_pane = !env::args().any(|arg| arg == ARG_NO_PANE)
        && !user_input.contains(ARG_NO_PANE)
        && !get_env_flag(ENV_NO_PANE);

    // no_suggest is true if args contains --no_suggest or stdin text contains "--no_suggest" or env var ASK_SH_NO_SUGGEST is defined
    let no_suggest = env::args().any(|arg| arg == ARG_NO_SUGGEST)
        || user_input.contains(ARG_NO_SUGGEST)
        || get_env_flag(ENV_NO_SUGGEST);

    // run tmux capture-pane -p before anything is printed.
    // if run with no_pane, pane_text is empty string.
    // if run without no_pane, execute shell command tmux capture-pane -p, when the command fail, pane_text return empty string
    // when fail, print error message to stderr
    let mut pane_text: String = "".to_string();
    if send_pane {
        {
            // check if in tmux session
            let mut in_tmux = false;
            match env::var("TMUX") {
                Ok(_value) => in_tmux = true,
                Err(_e) => {
                    eprintln!("*** Note: Terminal output is not sent to AI. Run this command inside tmux to enable the feature. See https://github.com/hmirin/ask.sh/blob/master/README.md#qa for more information. If you no longer want to see this message, run `ask` with --no_pane option or set ASK_SH_NO_PANE=true. ***\n")
                }
            }
            if in_tmux {
                match std::process::Command::new("tmux")
                    .arg("capture-pane")
                    .arg("-p")
                    .output()
                {
                    Ok(output) => pane_text = String::from_utf8_lossy(&output.stdout).to_string(),
                    Err(e) => {
                        eprintln!("Somehow tmux capture-pane -p failed: {}", e);
                    }
                }
            } else {
            }
        }
    };
    // remove last empty lines from pane_text
    let mut pane_text = pane_text.trim_end().to_string();
    // remove last line of pane_text
    if pane_text != "" {
        let pane_text_lines: Vec<&str> = pane_text.split("\n").collect();
        let mut pane_text_lines = pane_text_lines;
        pane_text_lines.pop();
        pane_text = pane_text_lines.join("\n");
    }

    // get user's shell name
    // when env::var("SHELL") is not set, use BASH_VERSION or ZSH_VERSION to guess the shell
    let shell = match env::var("SHELL") {
        Ok(value) => value,
        Err(_e) => {
            if env::var("BASH_VERSION").is_ok() {
                "Bash".to_string()
            } else if env::var("ZSH_VERSION").is_ok() {
                "zsh".to_string()
            } else {
                "Unknown".to_string()
            }
        }
    };

    // print user info
    if debug_mode {
        eprintln!("OS: {}", OS);
        eprintln!("osArch: {}", ARCH);
        eprintln!("shell: {}", shell);
    }

    let user_info: UserInfo = UserInfo {
        arch: ARCH.to_string(),
        os: OS.to_string(),
        shell: shell,
    };

    // disable send_pane if pane_text is not empty
    if pane_text == "" && send_pane == true {
        if debug_mode {
            eprintln!("pane_text is empty, so I set no_pane to true");
        }
        send_pane = false;
    }
    if debug_mode {
        eprintln!("args: {}", args.join(" "));
        eprintln!("is_using_stdin: {}", is_using_stdin);
        eprintln!("user_input: {}", user_input);
        eprintln!("user_input_without_flags: {}", user_input_without_flags);
        eprintln!("debug_mode: {}", debug_mode);
        eprintln!("no_suggest: {}", no_suggest);
        eprintln!("pane_text: {}", pane_text);
    }

    let api_key = match get_openai_api_key() {
        Some(val) => val,
        None => {
            eprintln!(
                "Please set your {} environment variable.",
                ENV_OPENAI_API_KEY
            );
            process::exit(1);
        }
    };

    let templates = prompts::get_template();
    let mut vars = std::collections::HashMap::new();
    vars.insert("pane_text".to_owned(), pane_text.to_owned());
    vars.insert("user_input".to_owned(), user_input_without_flags.to_owned());
    vars.insert("user_os".to_owned(), user_info.os.to_owned());
    vars.insert("user_arch".to_owned(), user_info.arch.to_owned());
    vars.insert("user_shell".to_owned(), user_info.shell.to_owned());
    let system_message = if send_pane {
        templates.render("tell_system_with_pane", &vars).unwrap()
    } else {
        templates.render("tell_system_without_pane", &vars).unwrap()
    };
    let user_input = if send_pane {
        // templates.render("fill_user_with_pane", &vars).unwrap()
        templates.render("tell_user_with_pane", &vars).unwrap()
    } else {
        // templates.render("fill_user_without_pane", &vars).unwrap()
        templates.render("tell_user_without_pane", &vars).unwrap()
    };

    let model_name = get_openai_model_name();

    let response = chat(
        &api_key,
        &model_name,
        &user_input,
        &system_message,
        &debug_mode,
    );
    let response = match response {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Communication with OpenAI API failed: {}", e);
            process::exit(1);
        }
    };

    let commands = post_process(&response);

    // print suggested commands to stdout to further process
    if !no_suggest {
        for command in commands {
            println!("{}", command);
        }
    }
}
