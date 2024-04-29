use twitch_irc::message::PrivmsgMessage;

use database::models::responders::TwitchResponder;

mod baked_goods;
mod dairy;
mod drinks;
mod fruit;
mod object;

pub async fn dispatch(responder: &TwitchResponder, msg: &PrivmsgMessage, command: &str) -> String {
    let response_fn = responder.response_fn.as_ref().unwrap();
    if response_fn == "core::emoji::baked_goods::bagels" {
        baked_goods::bagels(msg, command)
    } else if response_fn == "core::emoji::baked_goods::baguettes" {
        baked_goods::baguettes(msg, command)
    } else if response_fn == "core::emoji::baked_goods::birthday_cake" {
        baked_goods::birthday_cake(msg, command)
    } else if response_fn == "core::emoji::baked_goods::bread" {
        baked_goods::bread(msg, command)
    } else if response_fn == "core::emoji::baked_goods::cakes" {
        baked_goods::cakes(msg, command)
    } else if response_fn == "core::emoji::baked_goods::cookies" {
        baked_goods::cookies(msg, command)
    } else if response_fn == "core::emoji::baked_goods::croissants" {
        baked_goods::croissants(msg, command)
    } else if response_fn == "core::emoji::baked_goods::cupcakes" {
        baked_goods::cupcakes(msg, command)
    } else if response_fn == "core::emoji::baked_goods::doughnuts" {
        baked_goods::doughnuts(msg, command)
    } else if response_fn == "core::emoji::baked_goods::flatbreads" {
        baked_goods::flatbreads(msg, command)
    } else if response_fn == "core::emoji::baked_goods::moon_cakes" {
        baked_goods::moon_cakes(msg, command)
    } else if response_fn == "core::emoji::baked_goods::muffins" {
        baked_goods::muffins(msg, command)
    } else if response_fn == "core::emoji::baked_goods::pancakes" {
        baked_goods::pancakes(msg, command)
    } else if response_fn == "core::emoji::baked_goods::pie_slice" {
        baked_goods::slice_of_pie(msg, command)
    } else if response_fn == "core::emoji::baked_goods::pies" {
        baked_goods::pies(msg, command)
    } else if response_fn == "core::emoji::baked_goods::pretzels" {
        baked_goods::pretzels(msg, command)
    } else if response_fn == "core::emoji::baked_goods::rice_crackers" {
        baked_goods::rice_crackers(msg, command)
    } else if response_fn == "core::emoji::baked_goods::waffles" {
        baked_goods::waffles(msg, command)
    } else if response_fn == "core::emoji::dairy::cheese" {
        dairy::cheese(msg, command)
    } else if response_fn == "core::emoji::dairy::butter" {
        dairy::butter(msg, command)
    } else if response_fn == "core::emoji::dairy::ice_cream" {
        dairy::ice_cream(msg, command)
    } else if response_fn == "core::emoji::drinks::red_bull" {
        drinks::red_bull(msg, command)
    } else if response_fn == "core::emoji::drinks::ice_cube" {
        drinks::ice_cube(msg, command)
    } else if response_fn == "core::emoji::drinks::bottle" {
        drinks::bottle(msg, command)
    } else if response_fn == "core::emoji::drinks::milk" {
        drinks::milk(msg, command)
    } else if response_fn == "core::emoji::drinks::coffee" {
        drinks::coffee(msg, command)
    } else if response_fn == "core::emoji::drinks::tea" {
        drinks::tea(msg, command)
    } else if response_fn == "core::emoji::drinks::green_tea" {
        drinks::green_tea(msg, command)
    } else if response_fn == "core::emoji::drinks::sake" {
        drinks::sake(msg, command)
    } else if response_fn == "core::emoji::drinks::champagne" {
        drinks::champagne(msg, command)
    } else if response_fn == "core::emoji::drinks::wine" {
        drinks::wine(msg, command)
    } else if response_fn == "core::emoji::drinks::cocktail" {
        drinks::cocktail(msg, command)
    } else if response_fn == "core::emoji::drinks::martini" {
        drinks::martini(msg, command)
    } else if response_fn == "core::emoji::drinks::pina_colada" {
        drinks::pina_colada(msg, command)
    } else if response_fn == "core::emoji::drinks::daquiri" {
        drinks::daquiri(msg, command)
    } else if response_fn == "core::emoji::drinks::margarita" {
        drinks::margarita(msg, command)
    } else if response_fn == "core::emoji::drinks::tropical_drink" {
        drinks::tropical_drink(msg, command)
    } else if response_fn == "core::emoji::drinks::beer" {
        drinks::beer(msg, command)
    } else if response_fn == "core::emoji::drinks::cheers" {
        drinks::cheers(msg, command)
    } else if response_fn == "core::emoji::drinks::stiff_drink" {
        drinks::stiff_drink(msg, command)
    } else if response_fn == "core::emoji::drinks::whiskey" {
        drinks::whiskey(msg, command)
    } else if response_fn == "core::emoji::drinks::solo_cup" {
        drinks::solo_cup(msg, command)
    } else if response_fn == "core::emoji::drinks::soft_drink" {
        drinks::soft_drink(msg, command)
    } else if response_fn == "core::emoji::drinks::juice_box" {
        drinks::juice_box(msg, command)
    } else if response_fn == "core::emoji::drinks::yerba_mate" {
        drinks::yerba_mate(msg, command)
    } else if response_fn == "core::emoji::fruit::grapes" {
        fruit::grapes(msg, command)
    } else if response_fn == "core::emoji::fruit::melons" {
        fruit::melons(msg, command)
    } else if response_fn == "core::emoji::fruit::watermelons" {
        fruit::watermelons(msg, command)
    } else if response_fn == "core::emoji::fruit::tangerines" {
        fruit::tangerines(msg, command)
    } else if response_fn == "core::emoji::fruit::oranges" {
        fruit::oranges(msg, command)
    } else if response_fn == "core::emoji::fruit::lemons" {
        fruit::lemons(msg, command)
    } else if response_fn == "core::emoji::fruit::bananas" {
        fruit::bananas(msg, command)
    } else if response_fn == "core::emoji::fruit::pineapple" {
        fruit::pineapple(msg, command)
    } else if response_fn == "core::emoji::fruit::mangoes" {
        fruit::mangoes(msg, command)
    } else if response_fn == "core::emoji::fruit::apples" {
        fruit::apples(msg, command)
    } else if response_fn == "core::emoji::fruit::pears" {
        fruit::pears(msg, command)
    } else if response_fn == "core::emoji::fruit::peaches" {
        fruit::peaches(msg, command)
    } else if response_fn == "core::emoji::fruit::cherries" {
        fruit::cherries(msg, command)
    } else if response_fn == "core::emoji::fruit::strawberries" {
        fruit::strawberries(msg, command)
    } else if response_fn == "core::emoji::fruit::kiwis" {
        fruit::kiwis(msg, command)
    } else if response_fn == "core::emoji::fruit::tomatoes" {
        fruit::tomatoes(msg, command)
    } else if response_fn == "core::emoji::fruit::coconuts" {
        fruit::coconuts(msg, command)
    } else if response_fn == "core::emoji::fruit::avocados" {
        fruit::avocados(msg, command)
    } else if response_fn == "core::emoji::fruit::peppers" {
        fruit::hot_peppers(msg, command)
    } else if response_fn == "core::emoji::fruit::cucumber" {
        fruit::cucumber(msg, command)
    } else if response_fn == "core::emoji::fruit::jam" {
        fruit::jam(msg, command)
    } else {
        "Unknown Function (emoji)".to_owned()
    }
}

// pub fn template(msg: &PrivmsgMessage, command: &str) -> String {
// let emo = Emoji::new(
//     vec![],
//     vec![],
//     Some(1),
//     Some(1),
//     Some(vec![]),
//     Some(vec![]),
//     Some(vec![]),
// );
//     emo.message(msg, command)
// }
