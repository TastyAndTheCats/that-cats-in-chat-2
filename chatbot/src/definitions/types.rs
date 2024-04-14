use twitch_irc::login::RefreshingLoginCredentials;
use twitch_irc::transport::tcp::{TCPTransport, TLS};
use twitch_irc::TwitchIRCClient;

use crate::definitions::structs::RefreshingTokenStorage;

pub type TwitchClientType =
    TwitchIRCClient<TCPTransport<TLS>, RefreshingLoginCredentials<RefreshingTokenStorage>>;