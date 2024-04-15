pub fn check(msg: &str) -> bool {
    // TODO: add a table for users to add offensive phrases to
    // TODO: make it have various severity levels and responses
    // TODO: have global ones that users can enable
    // TODO: let users share their phrase lists?
    let offensive_phrases = vec![("bigfollows")];
    for phrase in offensive_phrases {
        if msg.contains(phrase) {
            return false;
        }
    }
    true
}
