use dotenv::dotenv;
use futures::stream::StreamExt;
use openai_api_stream_rs::OpenAIStream;
use regex::Regex;
use std::env;
use std::io::{self, BufRead};
use std::process;
mod messages;
use serde::Serialize;

// args
const ARG_DEBUG: &'static str = "--debug_ai_sh";
const ARG_NO_PANE: &'static str = "--no_pane";
const ARG_FILL: &'static str = "--fill";
// env
const ENV_DEBUG: &'static str = "AI_SH_DEBUG";
const ENV_NO_PANE: &'static str = "AI_SH_NO_PANE";
const ENV_OPENAI_API_KEY: &'static str = "AI_SH_OPENAI_API_KEY";
const ENV_OPENAI_MODEL: &'static str = "AI_SH_OPENAI_MODEL";

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

fn get_debug_mode() -> bool {
    dotenv().ok();
    match env::var(ENV_DEBUG) {
        Ok(val) => val.parse::<bool>().unwrap_or(false),
        Err(_e) => false,
    }
}

fn get_ai_sh_no_pane() -> bool {
    dotenv().ok();
    match env::var(ENV_NO_PANE) {
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


/// Interactive chat with OpenAI API.
///
/// # Examples
///
/// ```
/// println!(chat("api-key", "gpt-3.5-turbo", "You're an AI assistant.", "Tell me how to unarchive tar.gz." ))
/// ```
#[tokio::main]
async fn chat(
    api_key: &str,
    model_name: &str,
    user_input: &str,
    system_message: &str,
    debug_mode: &bool,
) -> String {
    let openai_stream = OpenAIStream::new(api_key.to_string());

    // build json payload
    let messages = vec![
        Message { role: "system".to_string(), content: system_message.to_string() },
        Message { role: "user".to_string(), content: user_input.to_string() },
    ];

    let payload = Payload { model: model_name.to_string(), messages: messages };

    let serialized_payload = serde_json::to_string(&payload).unwrap();

    // print input
    if *debug_mode {
        eprintln!("API Call:\n{}\n", serialized_payload);
    }
    let gpt_stream = openai_stream.gpt_stream(&serialized_payload).await.unwrap();
    let mut gpt_stream = Box::pin(gpt_stream);

    let mut n_char = 0;
    let mut response_to_return = String::new();
    while let Some(response) = gpt_stream.next().await {
        // response is incremental, so we need to print only the new part
        // print only after n_char to avoid printing already printed text
        let response_to_show = &response[n_char..];
        eprint!("{}", response_to_show);
        n_char = response.len();
        response_to_return = response.to_string();
    }
    eprintln!();
    return response_to_return;
}

fn post_process(text: &str) -> Vec<String> {
    let mut commands = Vec::new();
    // extract all commands enclosed in ``` ```
    let re = Regex::new(r#"```(.+?)```"#).unwrap();
    re.captures_iter(&text.replace("\n", ";")).for_each(|cap| {
        commands.push(cap[1].to_string().replace("\n", " ").trim_start_matches(";").trim_end_matches(";").trim().to_owned());
    });
    // extract all commands enclosed in ` ` if no commands are found in ``` ```
    if commands.len() == 0 {
        let re = Regex::new(r#"`(.+?)`"#).unwrap();
        re.captures_iter(&text.replace("\n", ";")).for_each(|cap| {
            commands.push(cap[1].to_string().replace("\n", " ").trim_start_matches(";").trim_end_matches(";").trim().to_owned());
        });
    }
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

// Examples:
// echo "Hey what's up?" | ai
// ai Hey what's up?

fn main() {
    // if run with --debug_ai_sh, then set debug_mode to true
    let debug_mode = if env::args().any(|arg| arg == ARG_DEBUG) {
        true
    } else {
        false
    };
    // if run with --no_pane or -n, then set not_send_pane to true
    let no_pane_arg_var = if env::args().any(|arg| arg == ARG_NO_PANE) {
        true
    } else {
        false
    };
    let no_pane_env_var = get_ai_sh_no_pane();
    // if no_pane is defined by any of the two variables. Do not send pane.
    let mut send_pane: bool = !no_pane_env_var && !no_pane_arg_var;

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
                    eprintln!("*** Note: If you run this command in tmux, I can send the current session log to AI. See https://github.com/hmirin/ai.sh/blob/master/README.md#tmux for more information. If you no longer want to see this message, run ai.sh with --no_pane option or set AI_SH_NO_PANE=true. ***\n")
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


    let text: String;
    // if ai is given with other arguments than --debug_ai_sh or --no_pane or --fill, then use that args
    let mut stdin_mode = false;
    if env::args()
        .skip(1)
        .any(|arg| arg != ARG_DEBUG && arg != ARG_NO_PANE && arg != ARG_FILL)
    {
        if debug_mode {
            eprintln!(
                "Some arguments are given. I send them to AI: {:?}",
                env::args().skip(1)
            );
        }
        text = env::args()
            .skip(1)
            .filter(|arg| arg != ARG_DEBUG && arg != ARG_NO_PANE && arg != ARG_FILL)
            .collect::<Vec<String>>()
            .join(" ");
    } else {
        // read from stdin
        text = io::stdin().lock().lines().next().unwrap().unwrap();
        stdin_mode = true;
        // TODO: if stdin is empty, timeout and exit
    }

    if debug_mode {
        eprintln!("stdin_mode: {}", stdin_mode);
        eprintln!("text: {}", text);
    }

    // if false, read global mode
    let debug_mode = debug_mode || get_debug_mode();

    if debug_mode {
        eprintln!("debug_mode: {}", debug_mode);
    }

    // if stdin_mode and run with --fill, then set fill_mode to true
    let fill_mode = if stdin_mode && env::args().any(|arg| arg == ARG_FILL) {
        true
    } else {
        false
    };
    if debug_mode {
        eprintln!("fill_mode: {}", fill_mode);
    }

    if debug_mode {
        eprintln!("pane_text: {}", pane_text);
    }

    // disable send_pane if pane_text is not empty
    if pane_text == "" && send_pane == true {
        if debug_mode {
            eprintln!("pane_text is empty, so I set no_pane to true");
        }
        send_pane = false;
    } else {
    }

    let api_key = match get_openai_api_key() {
        Some(val) => val,
        None => {
            eprintln!(
                    "Please set your {} environment variable.", ENV_OPENAI_API_KEY
            );
            process::exit(1);
        }
    };

    let templates = messages::get_template();
    let mut vars = std::collections::HashMap::new();
    vars.insert("pane_text".to_owned(), pane_text.to_owned());
    vars.insert("user_input".to_owned(), text.to_owned());
    let system_message = if fill_mode {
        if send_pane {
            // templates.render("fill_system_with_pane", &vars).unwrap()
            templates.render("tell_system_with_pane", &vars).unwrap()
        } else {
            // templates.render("fill_system_without_pane", &vars).unwrap()
            templates.render("tell_system_without_pane", &vars).unwrap()
        }
    } else {
        if send_pane {
            templates.render("tell_system_with_pane", &vars).unwrap()
        } else {
            templates.render("tell_system_without_pane", &vars).unwrap()
        }
    };
    let user_input = if fill_mode {
        if send_pane {
            // templates.render("fill_user_with_pane", &vars).unwrap()
            templates.render("tell_user_with_pane", &vars).unwrap()
        } else {
            // templates.render("fill_user_without_pane", &vars).unwrap()
            templates.render("tell_user_without_pane", &vars).unwrap()
        }
    } else {
        if send_pane {
            templates.render("tell_user_with_pane", &vars).unwrap()
        } else {
            templates.render("tell_user_without_pane", &vars).unwrap()
        }
    };

    let model_name = get_openai_model_name();

    let response = chat(
        &api_key,
        &model_name,
        &user_input,
        &system_message,
        &debug_mode,
    );
    let commands = post_process(&response);

    // if fill_mode is true, then print the possible commands
    if fill_mode {
        for command in commands {
            println!("{}", command);
        }
    }
}
