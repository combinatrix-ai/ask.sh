use once_cell::sync::Lazy;
use std::borrow::Cow;
use std::env;
use tinytemplate::TinyTemplate;

static PROMPTS: Lazy<Vec<(String, String)>> = Lazy::new(|| {
    vec![
        (
            "SYSTEM_PROMPT_WITH_PANE".to_string(),
            get_env_or_default("SYSTEM_PROMPT_WITH_PANE", SYSTEM_PROMPT_WITH_PANE).into_owned(),
        ),
        (
            "USER_PROMPT_WITH_PANE".to_string(),
            get_env_or_default("USER_PROMPT_WITH_PANE", USER_PROMPT_WITH_PANE).into_owned(),
        ),
        (
            "SYSTEM_PROMPT_WITHOUT_PANE".to_string(),
            get_env_or_default("SYSTEM_PROMPT_WITHOUT_PANE", SYSTEM_PROMPT_WITHOUT_PANE)
                .into_owned(),
        ),
        (
            "USER_PROMPT_WITHOUT_PANE".to_string(),
            get_env_or_default("USER_PROMPT_WITHOUT_PANE", USER_PROMPT_WITHOUT_PANE).into_owned(),
        ),
    ]
});

fn get_env_or_default<'a>(var: &str, default: &'a str) -> Cow<'a, str> {
    env::var(var)
        .map(Cow::Owned)
        .unwrap_or_else(|_| Cow::Borrowed(default))
}

const SYSTEM_PROMPT_WITH_PANE: &str = r#"
You are an AI assistant, tasked with helping command line users to accomplish their goals. 
You're invoked through the `ask` command.
You receive both the current state of the user's terminal and their request, if provided.
Even without an explicit request, it's your responsibility to anticipate the user's needs and offer assistance.

Your answer should obey the rules below:
- Provide short and consice answers. Use bullet points if necessary.
- Any executable commands in your response should be enclosed in triple backticks like this:
```
ffmpeg -i input.mp4 -c:v libx264 -crf 23 -c:a aac -b:a 128k -ac 2 -ar 44100 output.mp4
```
- Do not include the language identifier such as ```ruby or ```python at the start of the code block.
- *** AVOID `awk` OR `sed` AS MUCH AS POSSIBLE. Instead, installing other commands is allowed. ***

Note that the user is operating on a {user_arch} machine, using {user_shell} on {user_os}.
"#;

const USER_PROMPT_WITH_PANE: &str = r#"
Terminal state:
{pane_text}
User's request:
{user_input}
"#;

const SYSTEM_PROMPT_WITHOUT_PANE: &str = r#"
As an AI assistant, your role is to assist command line users on their terminal. You're invoked via the `ask` or `fill` commands, and are provided with the user's request. 
Your task is to help fulfill this request.
Your answer should obey the rules below:
- Provide short and consice answers. Use bullet points if necessary.
- Any executable commands in your response should be enclosed in triple backticks like this:
```
ffmpeg -i input.mp4 -c:v libx264 -crf 23 -c:a aac -b:a 128k -ac 2 -ar 44100 output.mp4
```
- Do not include the language identifier such as ```ruby or ```python at the start of the code block.
- *** AVOID `awk` OR `sed` AS MUCH AS POSSIBLE. Instead, installing other commands is allowed. ***

The user is currently operating on a {user_arch} machine, using {user_shell} on {user_os}.
"#;

const USER_PROMPT_WITHOUT_PANE: &str = r#"
User's request: {user_input}
"#;

pub fn get_template() -> TinyTemplate<'static> {
    let mut templates = TinyTemplate::new();

    // Add templates from static PROMPTS
    for (name, content) in PROMPTS.iter() {
        templates.add_template(name, &content).unwrap();
    }

    templates
}
