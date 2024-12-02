-- Definitions of modules
CREATE TABLE IF NOT EXISTS twitch_bot_responder_groups (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 1,
    parent INTEGER,
    FOREIGN KEY (parent) REFERENCES twitch_bot_responder_groups (id),
    UNIQUE (title, parent)
);

INSERT INTO twitch_bot_responder_groups (id, title, parent) VALUES
    (1, 'Core Functions', NULL),
    (7, 'Facts', 1),
    (8, 'Information', 1),
    (9, 'Twitch', 4),
    (10, 'YouTube', 4),
    (11, 'Chatter Status', 1),
    (12, 'Mathematics', 1),
    (13, 'Niceties', 1),
    (14, 'Emoji', 1),
    (2, 'Game-Related', NULL),
    (29, 'Colony-Builders', 2),
    (3, 'Third-Party', NULL),
    (5, 'User-Defined', 3),
    (4, 'API Consumers', 3),
    (6, 'Epic Games Store', 4),
    (15, 'Food', 14),
    (19, 'Baked Goods', 15),
    (20, 'Dairy', 15),
    (21, 'Desserts', 15),
    (22, 'Drinks', 15),
    (23, 'Fortune Cookie', 15),
    (24, 'Fruits', 15),
    (25, 'Meals', 15),
    (26, 'Meats', 15),
    (27, 'Utensils', 15),
    (28, 'Vegetables', 15),
    (16, 'Personal', 14),
    (17, 'Shapes', 14),
    (18, 'Events', 14);

-- Definitions of responders
CREATE TABLE IF NOT EXISTS twitch_bot_responders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    responder_group_id INTEGER REFERENCES twitch_bot_responder_groups (id),
    title TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 1,
    starts_with TEXT,
    ends_with TEXT,
    contains TEXT,
    response TEXT,
    response_fn TEXT,
    UNIQUE (responder_group_id, title)
);

-- just response
INSERT INTO twitch_bot_responders (responder_group_id, title, starts_with, contains, ends_with, response) VALUES
    (1, 'Say Hello', 'hello!', NULL, NULL, 'HeyGuys'),
    (1, '!test Command', NULL, '!test', NULL, 'TwitchConHYPE TwitchConHYPE TwitchConHYPE TwitchConHYPE TwitchConHYPE'),
    (11, 'Lurk', NULL, '!lurk', NULL, 'have distracted {sender} and they are now playing with the kitties'),
    (11, 'Unlurk', NULL, '!unlurk', NULL, 'has released {sender} from the cuteness trap and they have returned to spend time in chat'),
    (11, 'BRB', NULL, '!brb', NULL, 'have momentarily distracted {sender}'),
    (11, 'UnBRB', NULL, '!back', NULL, 'have become bored with {sender} and have allowed them to return to chat'),
    (13, 'Illuminati', NULL, 'illuminati', NULL, 'TheIlluminati TheIlluminati TheIlluminati TheIlluminati TheIlluminati'),
    (13, 'English Please!', '!english', NULL, NULL, 'Solo ingles Gracias / ภาษาอังกฤษเท่านั้น / Sadece Ingilizce / Только по Английски! Спасибо / 只有英文谢谢 / alleen Engels / אנגלית בלבד בבקשה / apenas inglês / Nur Englisch, Vielen Dank / انجليزي فقط. شكرا / que l''anglais / English only Please! Thank you. 💗'),
    (13, 'TERRIBLE', NULL, 'terrible|misery', NULL, 'Remember to use command ⭐Alt+F4⭐ if you''re not enjoying the stream. This will end your misery abruptly 🏳️ hope it''s working'),
    (13, 'Suicide Hotline', '!hotline', NULL, NULL, 'If you or someone you know is contemplating suicide, please reach out to a professional. You can find help at a National Suicide Prevention Lifeline => USA: 18002738255 | US Crisis textline: 741741 text HOME => Crisis Services Canada: 833-456-4566 | SMS 45645 => United Kingdom: 116 123 => Trans Lifeline (877-565-8860) => Others: https://en.wikipedia.org/wiki/List_of_suicide_crisis_lines https://suicidepreventionlifeline.org'),
    (13, 'Hello!', '!hello', NULL, NULL, 'Hello {sender}! HeyGuys'),
    (13, 'Are you Muted?', NULL, '!muted', NULL, 'Hey {channel_name}, I think you''re muted! {sender} says they can''t hear you!'),
    (13, 'FPS Issue', NULL, '!fps|!frames|!framedrop', NULL, 'Hey {channel_name}, you might be dropping frames - {sender} says they''re having trouble with your video feed'),
    (13, 'Wrong Scene', NULL, '!scene', NULL, 'Hey {channel_name}, check your feed - {sender} says they think you''re broadcasting the wrong scene');

-- just response_fn
INSERT INTO twitch_bot_responders (responder_group_id, title, starts_with, contains, ends_with, response_fn) VALUES
    (6, 'Epic Store Free Games', NULL, 'epic', NULL, 'api::epic_store::free_games'),
    (7, 'Dog Facts', NULL, '!dogfact', NULL, 'core::facts::dogfact'),
    (7, 'Cat Facts', NULL, '!catfact', NULL, 'core::facts::catfact'),
    (7, 'Number Facts', NULL, '!numfact|!numberfact', NULL, 'core::facts::numfact'),
    (7, 'Chuck Norris Facts', NULL, '!chuckfact|!norrisfact|!cnfact|!chucknorris', NULL, 'core::facts::chucknorris'),
    (7, 'Advice', NULL, '!advice', NULL, 'core::facts::advice'),
    (7, 'Dad Joke', NULL, '!dadjoke', NULL, 'core::facts::dadjoke'),
    (7, 'Rickyism', NULL, '!ricky|!tpb', NULL, 'core::facts::rickyism'),
    (4, 'Integer Sequences', NULL, '!oeis|!integers', NULL, 'api::oeis::lookup'),
    (8, 'Time', '!time', NULL, NULL, 'core::info::time'),
    (8, 'Weather', '!weather', NULL, NULL, 'api::openweathermap::weather'),
    (4, 'Chat with Blueberry', '!askb', NULL, NULL, 'api::ollama::blueberry'),
    (4, 'Wikipedia Lookup (Long)', '!wikipedia', NULL, NULL, 'api::wikipedia::lookup_full'),
    (4, 'Wikipedia Lookup (Short)', '!wiki', NULL, NULL, 'api::wikipedia::lookup'),
    (8, 'Dictionary', '!dict|!dictionary|!define|!def', NULL, NULL, 'api::words::dictionary'),
    (8, 'Thesaurus', '!thesaurus|!wordslike', NULL, NULL, 'api::words::thesaurus'),
    (9, 'Set Game', '!setgame', NULL, NULL, 'core::twitch::set_game'),
    (9, 'Set Title', '!settitle', NULL, NULL, 'core::twitch::set_title'),
    (10, 'Youtube Video Info', NULL, 'youtube.com|youtu.be', NULL, 'core::youtube::get_video_info'),
    (1, 'Chatbot Info', NULL, '!thecats|!thecatsinchat|!tcic', NULL, 'core::info'),
    (1, 'Chatbot Commands', NULL, '!commands|!catcommands', NULL, 'core::commands'),
    (12, 'Prime Number Checking', '!isprime|!primecheck', NULL, NULL, 'core::maths::prime_check'),
    (12, 'Coin Flip', NULL, '!coinflip|!flipcoin|!cointoss|!tosscoin', NULL, 'core::maths::coin_toss'),
    (13, 'Shoutout', NULL, '!gowatch|!gofollow|!so', NULL, 'core::niceties::shoutout');

-- just response_fn and only starts_with
INSERT INTO twitch_bot_responders (responder_group_id, title, starts_with, response_fn) VALUES
    (19, 'Cookies', '!cookie|!biscuit|!cookies|!biscuits', 'core::emoji::baked_goods::cookies'),
    (19, 'Cupcakes', '!cupcake|!cupcakes', 'core::emoji::baked_goods::cupcakes'),
    (19, 'Muffins', '!muffin|!muffins', 'core::emoji::baked_goods::muffins'),
    (19, 'Bread', '!bread|!loaf|!breads|!loaves', 'core::emoji::baked_goods::bread'),
    (19, 'Croissants', '!croissant|!croissants', 'core::emoji::baked_goods::croissants'),
    (19, 'Baguettes', '!baguette|!baguettes', 'core::emoji::baked_goods::baguettes'),
    (19, 'Pretzels', '!pretzel|!pretzels', 'core::emoji::baked_goods::pretzels'),
    (19, 'Bagels', '!bagel|!bagels', 'core::emoji::baked_goods::bagels'),
    (19, 'Pancakes', '!pancake|!pancakes', 'core::emoji::baked_goods::pancakes'),
    (19, 'Waffles', '!waffle|!waffles', 'core::emoji::baked_goods::waffles'),
    (19, 'Flatbreads', '!flatbread|!flatbreads', 'core::emoji::baked_goods::flatbreads'),
    (19, 'Rice Crackers', '!ricecracker|!ricecrackers', 'core::emoji::baked_goods::rice_crackers'),
    (19, 'Moon Cakes', '!mooncake|!mooncakes', 'core::emoji::baked_goods::moon_cakes'),
    (19, 'Doughnuts', '!doughnut|!donut|!doughnuts|!donuts', 'core::emoji::baked_goods::doughnuts'),
    (19, 'Birthday Cake', '!birthdaycake', 'core::emoji::baked_goods::birthday_cake'),
    (19, 'Cakes', '!shortcake|!cakeslice|!shortcakes|!cakes|!cakeslices', 'core::emoji::baked_goods::cakes'),
    (19, 'Pies', '!pie|!pies', 'core::emoji::baked_goods::pies'),
    (19, 'Pie Slice', '!pieslice|!sliceofpie', 'core::emoji::baked_goods::pie_slice'),
    (20, 'Cheese', '!cheese', 'core::emoji::dairy::cheese'),
    (20, 'Butter', '!butter', 'core::emoji::dairy::butter'),
    (20, 'Ice Cream', '!icecream', 'core::emoji::dairy::ice_cream'),
    (21, 'Shaved Ice', '!shavedice|!icecone', 'core::emoji::dessert::shaved_ice'),
    (21, 'Gelato', '!gelato', 'core::emoji::dessert::gelato'),
    (21, 'Chocolate', '!chocolate|!chocolatebar|!chocolates', 'core::emoji::dessert::chocolate'),
    (21, 'Candy', '!candy|!candies', 'core::emoji::dessert::candies'),
    (21, 'Lollipop', '!lollipop|!lollies|!lolly|!lollipops', 'core::emoji::dessert::lollipop'),
    (21, 'Custard', '!custard|!flan|!flans|!custards', 'core::emoji::dessert::custard'),
    (21, 'Honey', '!honey', 'core::emoji::dessert::honey'),
    (22, 'Red Bull', '!redbull|!red bull|!redbulls', 'core::emoji::drinks::redbull'),
    (22, 'Ice Cube', '!icecube', 'core::emoji::drinks::icecube'),
    (22, 'Bottle', '!bottle|!babybottle', 'core::emoji::drinks::bottle'),
    (22, 'Milk', 'milk|!glassesofmilk', 'core::emoji::drinks::milk'),
    (22, 'Coffee', '!coffee|!coffees', 'core::emoji::drinks::coffee'),
    (22, 'Tea', '!tea|!blacktea|!oolong|!teas', 'core::emoji::drinks::tea'),
    (22, 'Green Tea', '!greentea|!matcha', 'core::emoji::drinks::green_tea'),
    (22, 'Sake', '!sake|!sakes', 'core::emoji::drinks::sake'),
    (22, 'Champagne', '!champagne|!cork', 'core::emoji::drinks::champagne'),
    (22, 'Wine', '!wine|!glassesofwine', 'core::emoji::drinks::wine'),
    (22, 'Cocktail', '!cocktail', 'core::emoji::drinks::cocktail'),
    (22, 'Martini', '!martini', 'core::emoji::drinks::martini'),
    (22, 'Pina Colada', '!pinacolada', 'core::emoji::drinks::pina_colada'),
    (22, 'Daquiri', '!daquiri', 'core::emoji::drinks::daquiri'),
    (22, 'Margarita', '!margarita', 'core::emoji::drinks::margarita'),
    (22, 'Tropical Drink', '!tropicaldrink|!fruitydrink', 'core::emoji::drinks::tropicaldrink'),
    (22, 'Beer', '!beer|!beers', 'core::emoji::drinks::beer'),
    (22, 'Cheers', '!cheers', 'core::niceties::cheers'),
    (22, 'Stiff Drink', '!stiffdrink', 'core::emoji::drinks::stiff_drink'),
    (22, 'Whiskey', '!whiskey', 'core::emoji::drinks::whiskey'),
    (22, 'Solo Cup', '!solocup', 'core::emoji::drinks::solo_cup'),
    (22, 'Soft Drink', '!softdrink|!pop|!soda|!coke', 'core::emoji::drinks::soft_drink'),
    (22, 'Juice Box', '!juicebox|!juice|!juiceboxes', 'core::emoji::drinks::juice_box'),
    (22, 'Yerba Mate', '!yerbamate|!yerba', 'core::emoji::drinks::yerba_mate'),
    (7, 'Fortune Cookie', '!fortunecookie|!fortune', 'core::facts::fortune_cookie'),
    (24, 'Grapes', '!grape|!grapes', 'core::emoji::fruit::grapes'),
    (24, 'Melons', '!melon|!melons', 'core::emoji::fruit::melons'),
    (24, 'Watermelons', '!watermelon|!watermelons', 'core::emoji::fruit::watermelons'),
    (24, 'Tangerines', '!tangerine|!tangerines', 'core::emoji::fruit::tangerines'),
    (24, 'Lemons', '!lemon|!lemons', 'core::emoji::fruit::lemons'),
    (24, 'Bananas', '!banana|!bananas', 'core::emoji::fruit::bananas'),
    (24, 'Pineapple', '!pineapple', 'core::emoji::fruit::pineapple'),
    (24, 'Mangoes', '!mango|!mangoes', 'core::emoji::fruit::mangoes'),
    (24, 'Apples', '!apple|!apples', 'core::emoji::fruit::apples'),
    (24, 'Pears', '!pear|!pears', 'core::emoji::fruit::pears'),
    (24, 'Peaches', '!peach|!peaches', 'core::emoji::fruit::peaches'),
    (24, 'Cherries', '!cherry|!cherries', 'core::emoji::fruit::cherries'),
    (24, 'Strawberries', '!strawberry|!strawberries', 'core::emoji::fruit::strawberries'),
    (24, 'Kiwis', '!kiwi|!kiwis', 'core::emoji::fruit::kiwis'),
    (24, 'Tomatoes', '!tomato|!tomatoes', 'core::emoji::fruit::tomatoes'),
    (24, 'Coconuts', '!coconut|!coconuts', 'core::emoji::fruit::coconuts'),
    (24, 'Avocados', '!avocado|!avocados', 'core::emoji::fruit::avocados'),
    (24, 'Peppers', '!hotpepper|!jalapeno|!jalapeño|!habanero|!peppers|!hotpeppers|!jalapenos|!jalapeños|!habaneros', 'core::emoji::fruit::peppers'),
    (24, 'Cucumber', '!cucumber', 'core::emoji::fruit::cucumber'),
    (24, 'Jam', '!jam|!jamjar', 'core::emoji::fruit::jam'),
    (26, 'Volunteer as Tribute', '!tribute|!dwarfme|!volunteer', 'games::colony::tribute');

-- Set who can and can't use each command
CREATE TABLE IF NOT EXISTS twitch_bot_responder_permissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    requires_broadcaster BOOLEAN NOT NULL DEFAULT 0,
    requires_moderator BOOLEAN NOT NULL DEFAULT 0,
    requires_vip BOOLEAN NOT NULL DEFAULT 0,
    requires_subscriber BOOLEAN NOT NULL DEFAULT 0,
    requires_follower BOOLEAN NOT NULL DEFAULT 0
);

INSERT INTO twitch_bot_responder_permissions (id, title, requires_broadcaster, requires_moderator, requires_vip, requires_subscriber, requires_follower) VALUES
    (1, 'Anyone', 0, 0, 0, 0, 0),
    (2, 'Broadcaster-Only', 1, 0, 0, 0, 0),
    (3, 'Broadcaster and Mod', 1, 1, 0, 0, 0),
    (4, 'Broadcaster, Mod, and VIP', 1, 1, 1, 0, 0),
    (5, 'Subscriber+', 1, 1, 1, 1, 0),
    (6, 'Follower+', 1, 1, 1, 1, 1),
    (7, 'Moderator-Only', 0, 1, 0, 0, 0),
    (8, 'VIP-Only', 0, 0, 1, 0, 0),
    (9, 'Subscriber-Only', 0, 0, 0, 1, 0),
    (10, 'Follower-Only', 0, 0, 0, 0, 1);

-- Allow users to turn on and off whole modules
CREATE TABLE IF NOT EXISTS user_selected_modules (
    user_id INTEGER REFERENCES twitch_user (id),
    responder_group_id INTEGER REFERENCES twitch_bot_responder_groups (id) NOT NULL,
    active BOOLEAN NOT NULL DEFAULT 1,
    PRIMARY KEY (user_id, responder_group_id)
);

INSERT INTO user_selected_modules (user_id, responder_group_id) VALUES
    (167591621, 1);

-- Different auto-response profiles
CREATE TABLE IF NOT EXISTS twitch_bot_auto_response_profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    interval INTEGER DEFAULT 3600,
    distance INTEGER DEFAULT 50
);

INSERT INTO twitch_bot_auto_response_profiles (id, title, interval, distance) VALUES
    (1, 'Never', NULL, NULL),
    (2, 'Hourly', 3600, 50),
    (3, 'Daily', 86400, 50);

-- Allow users to turn on and off specific responders in a module
CREATE TABLE IF NOT EXISTS user_selected_responders (
    user_id INTEGER REFERENCES twitch_user (id),
    responder_id INTEGER REFERENCES twitch_bot_responders (id) NOT NULL,
    responder_profile INTEGER REFERENCES twitch_bot_auto_response_profiles (id) NOT NULL DEFAULT 1,
    active BOOLEAN NOT NULL DEFAULT 1,
    last_instance INTEGER NOT NULL DEFAULT 0,
    permissions INTEGER REFERENCES twitch_bot_responder_permissions (id) NOT NULL DEFAULT 3,
    cooldown INTEGER NOT NULL DEFAULT 10,
    per_user_cooldown INTEGER NOT NULL DEFAULT 60,
    include_specific_users TEXT DEFAULT NULL,
    exclude_specific_users TEXT DEFAULT NULL,
    count INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (user_id, responder_id)
);

-- Adds all of the possible responses to TastyAndTheCats' bot
INSERT INTO user_selected_responders (user_id, responder_id, permissions)
SELECT 167591621, id, 4 FROM twitch_bot_responders;
