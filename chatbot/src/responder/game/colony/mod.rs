use database::models::responders::TwitchResponder;

pub async fn dispatch(responder: &TwitchResponder, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("games::colony::tribute") {
        return cmd_tribute(command).await;
    } else {
        return "Unknown Function (tribute)".to_owned();
    }
}

async fn cmd_tribute(command: &str) -> String {
    match command {
        "!tribute" => {
            return "\"I volunteer as tribute!\" screams a wild-eyed {sender}".to_owned();
        }
        "!dwarfme" => {
            return "{sender} wants to be a short, sturdy creature, fond of drink and industry."
                .to_string();
        }
        "!volunteer" | _ => {
            return "{sender} says \"Put me in, coach!\"".to_string();
        }
    }
}
