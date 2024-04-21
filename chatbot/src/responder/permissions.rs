use database::models::responders::TwitchResponder;
use twitch_irc::message::PrivmsgMessage;

#[derive(Debug, PartialEq)]
enum Permissions {
    BROADCASTER = 1,
    MODERATOR = 2,
    VIP = 3,
    SUBSCRIBER = 4,
    // FOLLOWER = 5,
    ALL = 6,
    // TURBO = 7,
}

/// Checks if the sender has valid permissions for the responder
pub fn check(msg: &PrivmsgMessage, responder: &TwitchResponder) -> bool {
    check_permissions(get_permissions_level(msg), responder)
}

/// The permissions level of the message sender
fn get_permissions_level(msg: &PrivmsgMessage) -> Permissions {
    if msg.channel_login == msg.sender.login {
        return Permissions::BROADCASTER;
    }

    let tags = &msg.source.tags.0;

    if tags.get("mod").unwrap().clone().unwrap() == "1" {
        return Permissions::MODERATOR;
    }

    // if is_vip{ return Permissions::VIP; }

    if tags.get("subscriber").unwrap().clone().unwrap() == "1" {
        return Permissions::SUBSCRIBER;
    }

    // if is_turbo { return Permissions::TURBO; }

    // if is_follower { return Permissions::FOLLOWER; }

    return Permissions::ALL;
}

/// Whether the permissions level is sufficient for the responder permissions
fn check_permissions(auth_level: Permissions, responder: &TwitchResponder) -> bool {
    // This is sorted by complexity in terms of permissions, but might be better understood if organized by role rather than permission
    // e.g. instead of "if responder.requires_vip what are valid permissions", do "if Permissions::VIP what are valid responder.requires_x values"
    auth_level == Permissions::ALL
    // Broadcaster-only
    || responder.requires_broadcaster && auth_level == Permissions::BROADCASTER
    // Moderator+
    || (responder.requires_moderator
        && (auth_level == Permissions::BROADCASTER || auth_level == Permissions::MODERATOR))
    // VIP+
    || (responder.requires_vip
        && (auth_level == Permissions::BROADCASTER
            || auth_level == Permissions::MODERATOR
            || auth_level == Permissions::VIP))
    // Subscriber+
    || (responder.requires_subscriber
        && (auth_level == Permissions::BROADCASTER
            || auth_level == Permissions::MODERATOR
            || auth_level == Permissions::SUBSCRIBER))
}
