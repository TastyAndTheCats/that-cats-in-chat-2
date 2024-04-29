pub fn ordinal(num: i32) -> String {
    let mut str_num = format!("{}", num);
    if str_num.ends_with("0")
        | str_num.ends_with("4")
        | str_num.ends_with("5")
        | str_num.ends_with("6")
        | str_num.ends_with("7")
        | str_num.ends_with("8")
        | str_num.ends_with("9")
        | str_num.ends_with("11")
        | str_num.ends_with("12")
    {
        str_num.push_str("th");
    } else if str_num.ends_with("1") {
        str_num.push_str("st");
    } else if str_num.ends_with("2") {
        str_num.push_str("nd");
    } else if str_num.ends_with("3") {
        str_num.push_str("rd")
    } else if str_num.ends_with("13")
        | str_num.ends_with("14")
        | str_num.ends_with("15")
        | str_num.ends_with("16")
        | str_num.ends_with("17")
        | str_num.ends_with("18")
        | str_num.ends_with("19")
    {
        str_num.push_str("th");
    }
    str_num
}
