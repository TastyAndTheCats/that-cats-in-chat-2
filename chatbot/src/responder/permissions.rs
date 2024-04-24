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

/// Whether the user's permissions level is sufficient for the responder permissions
fn check_permissions(auth_level: Permissions, responder: &TwitchResponder) -> bool {
    // auth_level describes the maximum available permission to an account, so we check if that's valid for the responder here

    // If anyone can use the fn we shortcut the rest of the checks
    Permissions::ALL == auth_level
    // Broadcaster
        || Permissions::BROADCASTER == auth_level
            && (responder.requires_broadcaster
                || responder.requires_moderator
                || responder.requires_vip
                || responder.requires_subscriber)
    // Moderator
        || Permissions::MODERATOR == auth_level
            && (responder.requires_moderator
                || responder.requires_vip
                || responder.requires_subscriber)
    // VIP
        || Permissions::VIP == auth_level
            && (responder.requires_vip || responder.requires_subscriber)
    // Subscriber
        || Permissions::SUBSCRIBER == auth_level && responder.requires_subscriber
}
