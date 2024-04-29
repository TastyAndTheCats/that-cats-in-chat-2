use twitch_irc::message::PrivmsgMessage;

use super::object::{Emoji, EmojiProperties};

pub fn cookies(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec![
            "cookies",
            "chocolate chip cookies",
            "oatmeal cookies",
            "oatmeal raisin cookies",
            "frosted oatmeal without raisins",
            "peanut butter cookies",
            "Girl Scout/Guide chocolate mint cookies",
            "Oreos",
            "tahini cookies",
            "Yo-Yo Biscuits",
            "mandelbrot cookies",
            "oatmeal cookies",
            "toruńskie pierniki",
            "nürnberger lebkuchen cookies",
            "black and white cookies",
            "kurabiiki",
            "engelsaugen cookies",
            "marranitos",
            "springerle cookies",
            "amaretti di Saronno cookies",
            "jødekager cookies",
            "neula cookies",
            "sequilhos",
            "meringue cookies",
            "pitha cookies",
            "cantuccini Toscani",
            "kourabiedes",
            "Anzac biscuits",
            "canestrelli cookies",
            "Whoopie Pies",
            "bizcochitos",
            "almendrados",
            "melomakaronas",
            "zimtsterne cookies",
            "ladyfinger cookies",
            "baci di dama cookies",
            "ricciarelli di Siena",
            "vánoční cukroví",
            "pernik cookies",
            "palmier pastry cookies",
            "koulourakia",
            "Scottish shortbread",
            "vanillekipferl cookies",
            "fortune cookies (use the !fortune command to see what's inside)",
            "French sablé butter cookies",
            "stroopwafel",
            "speculaas cookies",
            "småkager cookies (probably in a tin, amirite?)",
            "amaretti biscotti",
            "Madeleine cookies",
            "lebkuchen cookies",
            "graham crackers (wakey wakey hands off snakey)",
            "snickerdoodles",
            "alfajores",
            "date ma'amoul cookies",
            "pistachio ma'amoul cookies",
            "delicate French macarons",
        ],
        vec!["🍪"],
        Some(2),
        Some(12),
        None,
        Some(vec!["and a pat on the head"]),
        Some(vec!["Good job ;)"]),
    );
    emo.message(msg, command)
}

pub fn cupcakes(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec![
            "chocolate cupcakes",
            "vanilla cupcakes",
            "marble cupcakes",
            "fancy hipster-bakery cupcakes",
        ],
        vec!["🧁"],
        Some(6),
        Some(24),
        None,
        Some(vec![
            "in a pink box tied with string",
            "in a plastic clamshell",
        ]),
        Some(vec!["CUPCAKES!!!"]),
    );
    emo.message(msg, command)
}

pub fn muffins(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["muffin"],
        vec!["(✿◠‿◠)"],
        None,
        None,
        None,
        None,
        Some(vec![
            "Mmm.. blueberry.",
            "Mmm.. banana.",
            "Mmm.. cornbread.",
            "Mmm.. triple chocolate chunk.",
            "Mmm.. it's a honey muffin.",
            "Mmm.. it's a morning glory muffin (like carrot cake without icing).",
            "Mmm.. chocolate chip.",
            "Mmm.. zucchini chocolate chip - better than you think!",
            "Oh.. it's a bran muffin.",
            "Mmm... apple cinnamon.",
        ]),
    );
    emo.message(msg, command)
}

pub fn bread(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec![
            "loaves of bread",
            "loaves of artisan sourdough",
            "potato buns",
            "loaves of herb and cheddar bread",
            "loaves of marble rye",
        ],
        vec!["🍞"],
        Some(16),
        Some(48),
        None,
        None,
        Some(vec![
            "BREAD AVALANCHE",
            "",
            "Is this a bakery (or some sort of bread farm)?",
            "That's probably too many...",
        ]),
    );
    emo.message(msg, command)
}

pub fn croissants(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["croissants", "kouign-amanns"],
        vec!["🥐"],
        Some(2),
        Some(12),
        None,
        None,
        Some(vec!["They are SUPER flaky!"]),
    );
    emo.message(msg, command)
}

pub fn baguettes(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["baguette"],
        vec!["🥖"],
        None,
        None,
        None,
        Some(vec![
            "in a yoga mat bag LUL",
            "the most phallic of breads",
            "fresh out of the oven and still warm",
        ]),
        None,
    );
    emo.message(msg, command)
}

pub fn pretzels(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["soft pretzels"],
        vec!["🥨"],
        Some(3),
        Some(8),
        None,
        Some(vec![
            "with mustard",
            "covered in rock salt",
            "dipped in chocolate",
            "brushed with garlic butter",
        ]),
        Some(vec!["They're easier to make than you think!"]),
    );
    emo.message(msg, command)
}

pub fn bagels(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec![
            "everything bagels",
            "onion bagels",
            "cinnamon raisin bagels",
            "sesame seed bagels",
            "plain bagels",
        ],
        vec!["🥯"],
        Some(2),
        Some(6),
        None,
        Some(vec!["with cream cheese", "with butter", "with lemon curd"]),
        None,
    );
    emo.message(msg, command)
}

pub fn pancakes(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["pancakes"],
        vec!["🥞"],
        Some(2),
        Some(9),
        None,
        Some(vec!["with butter and real maple syrup"]),
        Some(vec!["Tasty could eat pancakes every day...🤤🤤🤤"]),
    );
    emo.message(msg, command)
}

pub fn waffles(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["waffles"],
        vec!["🧇"],
        Some(2),
        Some(5),
        None,
        Some(vec!["with butter and real maple syrup"]),
        Some(vec!["I'm actually drooling...🤤🤤🤤"]),
    );
    emo.message(msg, command)
}

pub fn flatbreads(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["flatbread"],
        vec!["🥙"],
        None,
        None,
        None,
        Some(vec![
            "stuffed with spiced meat and vegetables",
            "stuffed with curried vegetables",
            "stuffed with garlic and sausage",
        ]),
        Some(vec!["The bread is so warm and soft!"]),
    );
    emo.message(msg, command)
}

pub fn rice_crackers(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["rice crackers wrapped in nori", "senbei"],
        vec!["🍘"],
        Some(6),
        Some(12),
        None,
        Some(vec!["and some green tea"]),
        Some(vec!["It's nice."]),
    );
    emo.message(msg, command)
}

pub fn moon_cakes(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(vec!["moon cake"], vec!["🥮"], None, None, None, None, None);
    emo.message(msg, command)
}

pub fn doughnuts(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["doughnuts"],
        vec!["🍩"],
        Some(3),
        Some(12),
        None,
        Some(vec!["in a pink box tied with string", "in a paper bag"]),
        None,
    );
    emo.message(msg, command)
}

pub fn birthday_cake(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["whole cake", "whole birthday cake"],
        vec!["🎂"],
        None,
        None,
        None,
        Some(vec![
            "with sprinkles on top",
            "with almond shards around the outside",
            "with birthday candles in it",
            "covered in chocolate ganache",
        ]),
        Some(vec!["It looks so good!", ""]),
    );
    emo.message(msg, command)
}

pub fn cakes(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["slices of cake", "pieces of shortcake"],
        vec!["🍰"],
        Some(2),
        Some(6),
        None,
        Some(vec![
            "with strawberries",
            "with raspberries",
            "with rhubarb sauce",
        ]),
        Some(vec!["", "and whipped cream"]),
    );
    emo.message(msg, command)
}

pub fn pies(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["whole pie"],
        vec!["🥧"],
        None,
        None,
        None,
        Some(vec!["in a dish", "in a foil pie plate"]),
        Some(vec![
            "It's blueberry (not the cat)!",
            "It's apple!",
            "It's cherry!",
            "It's butterscotch!",
            "It's chocolate!",
            "It's strawberry rhubarb!",
            "It's lemon meringue!",
            "It's banana cream!",
            "It's burnt to shit!",
        ]),
    );
    emo.message(msg, command)
}

pub fn slice_of_pie(msg: &PrivmsgMessage, command: &str) -> String {
    let emo = Emoji::new(
        vec!["slice of pie"],
        vec!["🥧"],
        Some(1),
        Some(1),
        None,
        Some(vec![
            "on a plate",
            "in a bowl",
            "with ice cream",
            "with yogurt",
            "for breakfast",
        ]),
        Some(vec![
            "It's blueberry (not the cat)!",
            "It's apple!",
            "It's cherry!",
            "It's butterscotch!",
            "It's chocolate!",
            "It's strawberry rhubarb!",
            "It's lemon meringue!",
            "It's banana cream!",
            "It's burnt to shit!",
        ]),
    );
    emo.message(msg, command)
}
