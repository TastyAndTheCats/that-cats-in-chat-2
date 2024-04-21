use twitch_irc::message::PrivmsgMessage;

#[derive(Debug, PartialEq)]
pub enum Permissions {
    BROADCASTER = 1,
    MODERATOR = 2,
    VIP = 3,
    SUBSCRIBER = 4,
    // FOLLOWER = 5,
    ALL = 6,
    // TURBO = 7,
}

pub fn check(msg: &PrivmsgMessage) -> Permissions {
    let tags = &msg.source.tags.0;
    let is_broadcaster = msg.channel_login == msg.sender.login;
    let is_moderator = tags.get("mod").unwrap().clone().unwrap() == "1";
    let is_subscriber = tags.get("subscriber").unwrap().clone().unwrap() == "1";

    if is_broadcaster {
        return Permissions::BROADCASTER;
    }
    if is_moderator {
        return Permissions::MODERATOR;
    }
    // if is_vip{ return Permissions::VIP; }
    if is_subscriber {
        return Permissions::SUBSCRIBER;
    }
    // if is_turbo { return Permissions::TURBO; }
    // if is_follower { return Permissions::FOLLOWER; }

    return Permissions::ALL;
}
