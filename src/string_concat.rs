
// 0. String from String and &strs
// Note:
//    String = String + &str + &str etc.
//                ^
//     first arg must be String
#[test] fn ex_0_string_from_strings_and_strs() {
    let str1 = String::from("W");
    let str2 = String::from("Z");
    let string = str1 + "." + &str2 + "-";
    assert_eq!(string, "W.Z-");
}

// 1. String from &str and &str
#[test] fn ex_1_string_from_strs() {
    let (str_1, str_2, str_3) = ("a", "b", "c");

    // a. String from &strs using Slice.join(&str)
    let _string = [str_1, str_2, str_3].join("");
    assert_eq!(_string, "abc");

    // b. String from &strs using format!()
    let _string = format!("{str_1}{str_2}");
    println!("_string: {_string:?}");
    assert_eq!(_string, "ab");
}

// 2. String from Strings
#[test] fn ex_2_string_from_strings() {
    // String from Strings using "+"
    let s1 = String::from("a");
    let s2 = String::from("b");
    let _string = s1 + &s2;
    assert_eq!(_string, "ab");

    // String from Strings using format!()
    let s1 = String::from("a");
    let s2 = String::from("b");
    let _string = format!("{s1}-{s2}");
    assert_eq!(_string, "a-b");
}

// 3. Misc
#[test] fn ex_3_misc() {
    let str = concat!("ab", "-", 4); // &str
    assert_eq!(str, "ab-4");

    // mut String::extend(iter)
    let mut str = String::from("abc");
    str.extend(['d', 'e', 'f'].iter());
    str.extend(vec!["-", "-"]);
    assert_eq!("abcdef--", &str);

    // vec!["aa", "bb"] -> vec.concat() -> String("aabb")
    let arr = ["aa", "bb"];
    let str: String = arr.concat();
    assert_eq!("aabb", &str);

    // vec!["aa", "bb"] -> vec.join() -> String("aa-bb")
    let arr = ["aa", "bb"];
    let str: String = arr.join("-");
    assert_eq!("aa-bb", &str);
}