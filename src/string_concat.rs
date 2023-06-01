
// 1. string from &str and &str
#[test] fn ex_1_string_from_strs() {
    let (str_1, str_2, str_3) = ("a", "b", "c");
    // using join
    let _string = [str_1, str_2, str_3].join("");
    assert_eq!(_string, "abc");
    // using format
    let _string = format!("{}{}", str_1, str_2);
    println!("_string: {:?}", _string);
    assert_eq!(_string, "ab");
}

// 2. string from strings
#[test] fn ex_2_string_from_strings() {
    //
    let s1 = String::from("a");
    let s2 = String::from("b");
    let _string = s1 + &s2;
    assert_eq!(_string, "ab");
    //
    let s1 = String::from("a");
    let s2 = String::from("b");
    let _string = format!("{}-{}", s1, s2);
    assert_eq!(_string, "a-b");
}

// 3. misc
#[test] fn ex_3_misc() {
    //
    let s = concat!("ab", "-", 4);
    assert_eq!(s, "ab-4");
    //
    let mut str = String::from("abc");
    str.extend(['d', 'e', 'f'].iter());
    str.extend(vec!["-", "-"]);
    assert_eq!("abcdef--", &str);
    //
    let vec = vec!["aa", "bb"];
    let str: String = vec.concat();
    assert_eq!("aabb", &str);
    //
    let vec = vec!["aa", "bb"];
    let str: String = vec.join("-");
    assert_eq!("aa-bb", &str);
}