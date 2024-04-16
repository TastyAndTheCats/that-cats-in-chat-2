pub fn check(msg: &str) -> bool {
    // TODO: add a table for users to add offensive phrases to
    // TODO: make it have various severity levels and responses
    // TODO: have global ones that users can enable
    // TODO: let users share their phrase lists?
    let offensive_phrases = vec![
        " ͫfa͝mous?",
        "(͆bigfollows .com)!",
        "(bigfollows",
        "becomall of e̓",
        "bigfollows .com",
        "bigfollows",
        "bigfollows",
        "bigfollows*com",
        "Buy follower͢s",
        "buy ̢followers",
        "clck.ru",
        "cutt.ly",
        "cutt.ly/0OS3U25",
        "dogehype .com",
        "dogehype",
        "ez viewers",
        "fam͊o̸uͫs͐?",
        "f̣amou͛s͎?",
        "followers and viewers",
        "followers, primes",
        "foll͇owers",
        "fol̕l̲owers",
        "gg.gg",
        "https://bit.ly/357xhC",
        "https://clck.ru/R9gQ",
        "https://clck.ru/R9gQV",
        "https://cutt.ly/rlLbkiH",
        "mountviewers .com",
        "mountviewers",
        "o̤n ͤhttps://clck.ru/R9gQV",
        "on͊",
        "primes and viewers",
        "primes͝ ȃnd͓",
        "prim̑es a͂nͫd ͥv̕iewers",
        "primes̉",
        "pro/en",
        "R9gQV",
        "solo.to/freetest",
        "steamskill.pro",
        "steamskill",
        "u.to/HVXgGw",
        "viewers .shop",
        "v͊iewers",
        "W̊an̋na beco̸m̧e ͢fa̚mo͇us? Buy followers, ̐prim͎es̝ and viewers o̤n ͤhttps://clck.ru/R9gQV (bigfollows .com)!",
        "wanna become famous",
        "W̲anna beco̗me ͙famouͮs?",
        "wanna be͎comeͣ famous͗?",
        "Waͮnn̽a",
        "yourfollowz .com",
        "yourfollowz"
    ]; // TODO: put this list somewhere as text/CSV?
       // TODO: update this list, these are _OLD_ scams
    for phrase in offensive_phrases {
        if msg.contains(phrase) {
            // TODO: if bot is moderator, slap the sender
            return false;
        }
    }
    true
}
