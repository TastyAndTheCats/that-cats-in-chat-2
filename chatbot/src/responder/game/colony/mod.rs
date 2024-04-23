use database::{models::responders::TwitchResponder, responder};
use twitch_irc::message::PrivmsgMessage;
use utils::{message::first_word_after_command_as_number, num::ordinal, rand::random_number_1_to};

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn.starts_with("games::colony::tribute") {
        cmd_tribute(responder, msg, command).await
    } else {
        "Unknown Function (tribute)".to_owned()
    }
}

async fn cmd_tribute(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let user_id = msg.sender.id.parse::<i32>().unwrap_or(0);
    let responder_id = responder.id;
    if msg.sender.login == msg.channel_login {
        let update_count_to: i32 = first_word_after_command_as_number(msg, command).unwrap_or(-1);
        if update_count_to > -1 {
            let db_response = responder::update_count(user_id, responder_id, update_count_to);
            if update_count_to > 0 {
                match db_response {
                    Ok(_) => match command {
                        "!tribute" => {
                            return format!(
                                "The call goes out: {} more tributes are needed for {{sender}}'s army! Use !tribute to make your name known.", 
                                update_count_to
                            );
                        }
                        "!dwarfme" => {
                            return format!(
                                "Migrants have arrived! There are {} unanointed Dwarfs in {{sender}}'s fortress. Use !dwarfme to strike the earth.", 
                                update_count_to
                            );
                        }
                        "!volunteer" | _ => {
                            return format!(
                                "Volunteer or be voluntold! {} more sacrifices are needed! Use !volunteer to get ahead of the draft.", 
                                update_count_to
                            );
                        }
                    },
                    Err(_) => {
                        return format!(
                            "Sorry {{sender}} we were unable to set count to {} for {}",
                            update_count_to, command
                        )
                    }
                }
            } else {
                match db_response {
                    Ok(_) => return format!("{{sender}}, {} has been reset.", command),
                    Err(_) => {
                        return format!("Sorry {{sender}} we were unable to reset {}", command)
                    }
                }
            }
        }
    }

    let remaining_volunteers = responder::get_count(user_id, responder_id).unwrap_or(0);
    if remaining_volunteers > 0 {
        responder::update_count(user_id, responder_id, remaining_volunteers - 1).expect(&format!(
            "Unable to step down the count for {}:{}",
            user_id, responder_id
        ));
        let which_one = random_number_1_to(remaining_volunteers);
        return match command {
            "!tribute" => format!("\"I volunteer as tribute!\" screams a wild-eyed {{sender}}. \"Applicant {} is accepted,\" replies the clown with the clipboard", which_one),
            "!dwarfme" => format!(
                "{{sender}} wants to be a short, sturdy creature, fond of drink and industry. The {} dwarf must suffer the name of {{sender}}.",
                ordinal(which_one)
            ),
            "!volunteer" | _ => format!("{{sender}} says \"As {} in line for the throne, it may as well be me\"", ordinal(which_one)),
        };
    } else {
        return match command {
            "!tribute" => "\"I volunteer as tribute!\" screams a wild-eyed {sender}".to_owned(),
            "!dwarfme" => format!(
                "{{sender}} wants to be a short, sturdy creature, fond of drink and industry."
            ),
            "!volunteer" | _ => "{sender} says \"Put me in, coach!\"".to_owned(),
        };
    }
}
