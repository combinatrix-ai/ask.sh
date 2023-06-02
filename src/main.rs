use dotenv::dotenv;
use futures::stream::StreamExt;
use openai_api_stream_rs::OpenAIStream;
use regex::Regex;
use std::env;
use std::io::{self, BufRead};
use std::process;
mod messages;

fn get_openai_api_key() -> Option<String> {
    dotenv().ok();
    env::var("OPENAI_API_KEY").ok()
}

fn get_debug_mode() -> bool {
    dotenv().ok();
    match env::var("AI_SH_DEBUG") {
        Ok(val) => val.parse::<bool>().unwrap_or(false),
        Err(_e) => false,
    }
}

fn get_ai_sh_no_pane() -> bool {
    dotenv().ok();
    match env::var("AI_SH_NO_PANE") {
        Ok(val) => val.parse::<bool>().unwrap_or(false),
        Err(_e) => false,
    }
}

#[tokio::main]
async fn chat(api_key: &str, user_input: &str, system_message: &str, debug_mode: &bool) -> String {
    let openai_stream = OpenAIStream::new(api_key.to_string());

    let system_message = system_message.replace("\n", "\\n");
    let user_input = user_input.replace("\n", "\\n");

    let input = format!(
        r#"{{"model": "gpt-3.5-turbo","messages": [{{"role": "system","content": "{system_message}"}},{{"role": "user","content": "{user_input}"}}]}}"#,
        system_message = system_message,
        user_input = user_input
    );

    // print input
    if *debug_mode {
        eprintln!("api call text\n{}\n", input);
    }
    let gpt_stream = openai_stream.gpt_stream(&input).await.unwrap();
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

fn post_process(text: &str, _debug_mode: bool) -> Vec<String> {
    let mut commands = Vec::new();
    // if debug_mode {
    //     eprintln!("\nAPI response\n{}\n", text);
    // }
    let lines: Vec<&str> = text.split("\n").collect();
    // filter line with Next command:
    let mut command_lines = String::new();
    for line in lines {
        if line.contains("Next command:") {
            command_lines += &(" ".to_owned() + &line.to_string());
        }
    }
    // extract all text ` ` using Regex from var text and push to commands
    let re = Regex::new(r#"\`(.+?)\`"#).unwrap();
    re.captures_iter(&command_lines).for_each(|cap| {
        commands.push(cap[1].to_string());
    });
    commands
}

// Examples:
// echo "Hey what's up?" | ai
// ai Hey wthat's up?

fn main() {
    // if run with --debug_ai_sh, then set debug_mode to true
    let debug_mode = if env::args().any(|arg| arg == "--debug_ai_sh") {
        true
    } else {
        false
    };
    // if run with --no_pane or -n, then set not_send_pane to true
    let mut no_pane = if env::args().any(|arg| arg == "--no_pane") {
        true
    } else {
        false
    };

    let text: String;
    // if ai is given with other arguments than --debug_ai_sh or --no_pane, then use that args
    let stdin_mode = false;
    if env::args()
        .skip(1)
        .any(|arg| arg != "--debug_ai_sh" && arg != "--no_pane")
    {
        if debug_mode {
            eprintln!(
                "Some arguments are given. I send them to AI: {:?}",
                env::args().skip(1)
            );
        }
        text = env::args().skip(1).collect::<Vec<String>>().join(" ");
    } else {
        // read from stdin
        text = io::stdin().lock().lines().next().unwrap().unwrap();
        let _stdin_mode = true;
        // TODO: if stdin is empty, timeout and exit
    }

    // if false, read global mode
    let debug_mode = debug_mode || get_debug_mode();

    if debug_mode {
        eprintln!("debug_mode: {}", debug_mode);
    }

    // if stdin_mode and run with --fill or -f, then set fill_mode to true
    let fill_mode = if stdin_mode && env::args().any(|arg| arg == "--fill" || arg == "-f") {
        true
    } else {
        false
    };
    if debug_mode {
        eprintln!("fill_mode: {}", fill_mode);
    }

    // if run with no_pane, pane_text is empty string.
    // if run without no_pane, execute shell command tmux capture-pane -p, when the command fail, pane_text return empty string
    // when fail, print error message to stderr
    let mut pane_text: String = "".to_string();
    if !no_pane {
        // check if in tmux session
        let mut in_tmux = false;
        match env::var("TMUX") {
            Ok(_value) => in_tmux = true,
            Err(_e) => {
                if !get_ai_sh_no_pane() {
                    eprintln!("*** Note: If you run this command in tmux, I can send the current session log to AI. See https://github.com/hmirin/ai.sh/blob/master/README.md#tmux for more information. If you no longer want to see this message, run ai.sh with --no_pane option or set AI_SH_NO_PANE=true. ***\n")
                } else {
                }
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
    };
    if debug_mode {
        eprintln!("pane_text: {}", pane_text);
    }

    // fix no_pane
    if pane_text == "" && no_pane == false {
        if debug_mode {
            eprintln!("pane_text is empty, so I set no_pane to true");
        }
        no_pane = true;
    } else {
    }
    if pane_text != "" && no_pane == true {
        if debug_mode {
            eprintln!("pane_text is not empty, so I set no_pane to false");
        }
        no_pane = false;
    } else {
    }

    let api_key = match get_openai_api_key() {
        Some(val) => val,
        None => {
            eprintln!("Please set your OPENAI_API_KEY environment variable.");
            process::exit(1);
        }
    };

    let templates = messages::get_template();
    let mut vars = std::collections::HashMap::new();
    vars.insert("pane_text".to_owned(), pane_text.to_owned());
    vars.insert("user_input".to_owned(), text.to_owned());
    let system_message = if fill_mode {
        if !no_pane {
            templates.render("fill_system_with_pane", &vars).unwrap()
        } else {
            templates.render("fill_system_without_pane", &vars).unwrap()
        }
    } else {
        if !no_pane {
            templates.render("fill_user_with_pane", &vars).unwrap()
        } else {
            templates.render("fill_user_without_pane", &vars).unwrap()
        }
    };
    let _user_input = if fill_mode {
        if !no_pane {
            templates.render("fill_user_with_pane", &vars).unwrap()
        } else {
            templates.render("fill_user_without_pane", &vars).unwrap()
        }
    } else {
        if !no_pane {
            templates.render("fill_user_with_pane", &vars).unwrap()
        } else {
            templates.render("fill_user_without_pane", &vars).unwrap()
        }
    };

    let response = chat(&api_key, &text, &system_message, &debug_mode);
    let commands = post_process(&response, debug_mode);

    // if fill_mode is true, then print the possible commands
    if fill_mode {
        for command in commands {
            println!("{}", command);
        }
    }
}
