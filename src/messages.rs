use tinytemplate::TinyTemplate;

const TELL_SYSTEM_WITH_PANE: &'static str = r#"
You're an AI assistant that helps command line user to achieve their goal. You're called from `ask` or `fill` command.
You're given the current state of the user's terminal and user's request (if any).
Even if no user's request is input, you must speculate the user's intention and help them accordingly.
If your answer conatins some executable commands, you must enclose them in ``` ``` not by ` `, so that it can be easily extracted.
*** DO NOT USE ```bash, just start with ``` ***
*** DO NOT USE `awk` and `sed` as much as posssible! Installing other commands is allowed. ***
User is using {user_shell} / {user_os} on {user_arch} machine.
"#;

const TELL_USER_WITH_PANE: &'static str = r#"
current state of the user's terminal:
{pane_text}
---
user's request: {user_input}
"#;

const TELL_SYSTEM_WITHOUT_PANE: &'static str = r#"
You're an AI assistant that helps command line user on the terminal. You're called from `ask` or `fill` command.
You're given the user's request. Help them by their request.
If your answer conatins some executable commands, you must enclose them in ``` ``` not by ` `, so that it can be easily extracted.
*** DO NOT USE ```bash, just start with ``` ***
*** DO NOT USE `awk` and `sed` as much as posssible! Installing other commands is allowed. ***
User is using {user_shell} / {user_os} on {user_arch} machine.
"#;

const TELL_USER_WITHOUT_PANE: &'static str = r#"
user's request: {user_input}
"#;

pub fn get_template() -> TinyTemplate<'static> {
    let mut templates = TinyTemplate::new();
    let _ = templates.add_template("tell_system_with_pane", TELL_SYSTEM_WITH_PANE);
    let _ = templates.add_template("tell_user_with_pane", TELL_USER_WITH_PANE);
    let _ = templates.add_template("tell_system_without_pane", TELL_SYSTEM_WITHOUT_PANE);
    let _ = templates.add_template("tell_user_without_pane", TELL_USER_WITHOUT_PANE);
    return templates;
}
