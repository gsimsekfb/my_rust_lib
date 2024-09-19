
// Topics:
// my / builtin (traits, structs, generics), ctor, trait as ctor param


// todo
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::to_string_trait_impl)]

// 1.a - My Traits, My Structs, Generics
// ============================================================
// MyIntoString - trait and fn into_str() which converts input type by ref to String 
// MyInto<T> - Generic trait  and fn into_t() which converts input type by ref to T 
//
// Cat - struct with data member string "name"
// GenericCat<T> - Generic struct with data member type T "name"
//
// Impl MyIntoString, MyInto<String>, MyInto<T> for Cat
// For comparison impl Cat::name_() - member fn that returns ref to name
// For comparison impl GenericCat::name_() - member fn that returns ref to name

// Impl MyIntoString   for GenericCat<String> and GenericCat<T>
// Impl MyInto<String> for GenericCat<String> and GenericCat<T>
// Impl MyInto<T>      for GenericCat<String> and GenericCat<T> - not trivial
//
// Tests:
// Use combination of 2 traits and 2 structs - 4 tests in total


// ================= My Traits =================

trait MyIntoString {
    /// Convert this type (Self) into String
    fn into_str(&self)-> String;
}

trait MyInto<T> {
    /// Convert this type into the input type T
    fn into_t(&self) -> T;
}

// ================= My Struct =================
struct Cat { name: String }
struct GenericCat<T> { name: T }

// ========= impl my traits for Cat =======

impl MyIntoString for Cat {
    fn into_str(&self)-> String { self.name.clone() + "-MyIntoString" }
}
// For comparison: impl Cat is almost the same as impl a trait
impl Cat {
    fn name_(&self) -> &String { &self.name }
}

// Only one of these 2 can exist at a time
impl MyInto<String> for Cat {
    fn into_t(&self) -> String { self.name.clone() + "-MyInto<String>" }
}
// error[E0119]: conflicting implementations of trait `MyInto<String>` for type `Cat`
// impl<T: From<String>> MyInto<T> for Cat {
//     fn into_t(&self) -> T { self.name.clone().into() }
// }

// ========= impl my traits for GenericCat<String> and GenericCat<T> =======

// Only one of these 2 can exist at a time
// impl MyIntoString for GenericCat<String> {
//     fn into_str(&self)-> String { self.name.clone() }
// }
impl<T: Clone> MyIntoString for GenericCat<T> where String: From<T> {
    fn into_str(&self)-> String { self.name.clone().into() }
}
// For comparison: impl struct is almost the same as impl a trait
impl<T> GenericCat<T> {
    fn name_(&self) -> &T { &self.name }
}

// Only one of these 4 can exist at a time
// e.g. error[E0119]: conflicting implementations of trait `MyInto<String>` 
// for type `GenericCat<String>`
//
// 1
// impl MyInto<String> for GenericCat<String> {
//     fn into_t(&self) -> String { self.name.clone() }
// }
// 
// 3
// impl<T: Into<String> + Clone> MyInto<String> for GenericCat<T> {
// // or
// // impl<T: Clone> MyInto<String> for GenericCat<T> where String: From<T> {
//     fn into_t(&self) -> String { self.name.clone().into() }
// }
//
// 3
// impl<T: From<String>> MyInto<T> for GenericCat<String> {
//     fn into_t(&self) -> T { self.name.clone().into() }
// }
//
// 4
impl<T, U: Into<T> + Clone> MyInto<T> for GenericCat<U> {
    fn into_t(&self) -> T { self.name.clone().into() }
}


// 1.a
#[test] fn case_1_a() {

    // 1. Non-generic both - my struct Cat and my trait MyIntoString
    let cat = Cat { name: "tom".to_string() };
    // equivalent trait fn calls
    let name_1 = cat.into_str();
    let name_2 = MyIntoString::into_str(&cat);
    let name_3 = <Cat as MyIntoString>::into_str(&cat);
    assert_eq!(name_1, "tom-MyIntoString");
    assert_eq!(name_2, "tom-MyIntoString");
    assert_eq!(name_3, "tom-MyIntoString");

    // 2. Non-generic my struct Cat and generic my trait MyInto<T>
    let cat = Cat { name: "tom".to_string() };
    // equivalent trait fn calls
    let name_0 = cat.into_t();
    let name_1 = MyInto::<String>::into_t(&cat);
    let name_2 = <Cat as MyInto::<String>>::into_t(&cat);
    assert_eq!(name_0, "tom-MyInto<String>");
    assert_eq!(name_1, "tom-MyInto<String>");
    assert_eq!(name_2, "tom-MyInto<String>");

    // 3. Generic my struct GenericCat<T> and non-generic my trait MyIntoString
    let gen_cat = GenericCat::<String> { name: "tom".to_string() };
    // or
    let gen_cat = GenericCat { name: "tom".to_string() };
    let name_0 = gen_cat.into_str();
    let name_1 = MyIntoString::into_str(&gen_cat);
    let name_2 = <GenericCat<String> as MyIntoString>::into_str(&gen_cat);
    assert_eq!(name_0, "tom");
    assert_eq!(name_1, "tom");
    assert_eq!(name_2, "tom");

    // 4. Generic both - my struct GenericCat<T> and my trait MyInto<T>
    let gen_cat = GenericCat { name: "tom".to_string() };
    // or
    let gen_cat = GenericCat::<String> { name: "tom".to_string() };
    let name_0: String = gen_cat.into_t();
    let name_1 = MyInto::<String>::into_t(&gen_cat);
    let name_2 = <GenericCat<String> as MyInto<String>>::into_t(&gen_cat);
    assert_eq!(name_0, "tom");
    assert_eq!(name_1, "tom");
    assert_eq!(name_2, "tom");
}



// ----------------------------------------------------------------------------



// 1.b - My Traits, builtin types, generics
// ===========================================================
// i
// Impl MyIntoString and MyInto<String> traits for i32
// Impl MyInto<T> for i64 (Hint: not trivial)
// ii
// Impl MyIntoString for Option<i32>
// Impl MyIntoString for Option<T>
// iii
// Impl MyInto<String> for Option<String>
// Impl MyInto<T> for Option<String>
// Impl MyInto<String> for Option<T>
// Impl MyInto<U> for Option<T>

impl MyIntoString for i32 {
    fn into_str(&self)-> String { self.to_string() + "-MyIntoString" }
}

impl MyInto<String> for i32 {
    fn into_t(&self) -> String { self.to_string() + "-MyInto<String>" }
}

impl<T: From<i64>> MyInto<T> for i64 {
// or
// impl<T> MyInto<T> for i64 where i64: Into<T> {
    fn into_t(&self) -> T { (*self).into() }
}

//// Only one of these 2 can exist at a time
// impl MyIntoString for Option<i32> {
//     fn into_str(&self)-> String { self.unwrap().to_string() }
// }
// impl<T: Into<String> + Clone> MyIntoString for Option<T> { 
    // Error when T: i32, since Into<String> not impl'ed for i32
// Ok when T: i32, since ToString impl'ed for i32
impl<T: ToString + Clone> MyIntoString for Option<T> { 
    fn into_str(&self)-> String { self.clone().unwrap().to_string() }
}

//// Only one of these 4 can exist at a time
// impl MyInto<String> for Option<String> {
//     fn into_t(&self)-> String { self.clone().unwrap() }
// }
// impl<T: From<String> + Clone> MyInto<T> for Option<String> {
//     fn into_t(&self)-> T { self.clone().unwrap().into() }
// }
// impl<T: Into<String> + Clone> MyInto<String> for Option<T> {
//     fn into_t(&self)-> String { self.clone().unwrap().into() }
// }
impl<T: Into<U> + Clone, U> MyInto<U> for Option<T> {
    fn into_t(&self)-> U { self.clone().unwrap().into() }
}

// 1.b - My Traits/Generics for builtin types
#[test] fn case_1_b() {
    // i
    let num = 42;
    
    let s = num.to_string(); // std::string::ToString::to_string()
    assert_eq!(s, "42");

    let s = num.into_str();
    // or
    let s = MyIntoString::into_str(&num);
    assert_eq!(s, "42-MyIntoString");

    let s = num.into_t();
    // or
    let s = MyInto::<String>::into_t(&num);
    assert_eq!(s, "42-MyInto<String>");

    // ii
    let opt = Some(42);
    assert_eq!(opt.into_str(), "42"); // or same:
    assert_eq!(<Option<i32> as MyIntoString>::into_str(&opt), "42");

    // iii
    let opt = Some("ab".to_string());
    // assert_eq!(opt.into_t(), "ab"); 
        // error: type must be known at this point
        // when impl<...> MyInto<U> for Option<T> version used
    assert_eq!(<Option<String> as MyInto<String>>::into_t(&opt), "ab");

}

// 1.c - Builtin Traits/Generics with my types for builtin types
// ===========================================================
// Impl From for String - which produces String from Cat
// Impl From for String - which produces String from GenericCat<String>

//// 444
impl From<Cat> for String {
    fn from(cat: Cat) -> Self { cat.name }
}

//// 333
impl From<GenericCat<String>> for String {
    fn from(value: GenericCat<String>) -> Self { value.name }
}

// 1.c
#[test] fn case_1_c() {
    let cat = Cat::new_from_str("tom");
    let s = String::from(cat);
    assert_eq!(s, "tom");
    
    let gen_cat = GenericCat::<String>::new_from_str("tom");
    let s = String::from(gen_cat);
    assert_eq!(s, "tom");
}


// ----------------------------------------------------------------------------


// 2 - Struct, Generics, ctors, Trait as params
// ============================================================
//
// a
// Cat::name() - member fn that returns reference to name
// Cat::new_from_str() - associated fn ctor from &str
// Cat::new() - associated fn ctor from types that impl trait ToString
// Cat::new_() - associated fn ctor from types that impl trait MyIntoString
// Cat::new__() - associated fn ctor from types that impl generic trait MyInto<String>
//
// b
// GenericCat<T>::name() - generic member fn that returns reference to name
// GenericCat<T>::new_from_str() - associated fn ctor from &str (not trivial)
// GenericCat<T>::new_from_i32() - associated fn ctor from i32
// GenericCat<T>::new() - associated fn ctor from types that impl trait MyIntoString
// GenericCat<T>::new_() - associated fn ctor from types that impl generic trait MyInto<T>


// a
// ================= Cat =================

impl Cat {
    fn name(&self) -> &String { &self.name }
    fn new_from_str(name: &str) -> Self { Self { name: name.to_string() } }
    fn new(name: impl ToString) -> Self { Self { name: name.to_string() } }
    fn new_(s: &impl MyIntoString) -> Self { Self { name: s.into_str()} }
    fn new__(s: &impl MyInto<String>) -> Self { Self { name: s.into_t() } }
}

#[test] fn case_2_a() {
    let cat = Cat { name: "Tom".to_string() };
    assert_eq!(cat.name(), "Tom");

    let cat = Cat::new_from_str("Tom");
    assert_eq!(cat.name(), "Tom");

    let cat = Cat::new("Tom");
    assert_eq!(cat.name(), "Tom");

    let cat = Cat::new_(&42);
    // let cat = Cat::new_(42_f64);
        // error[E0277]: the trait bound `f64: MyIntoString` is not satisfied
    assert_eq!(cat.name(), "42-MyIntoString");

    let cat = Cat::new_(&Cat::new("Tom"));
    assert_eq!(cat.name(), "Tom-MyIntoString");

    let cat = Cat::new__(&Cat::new("Tom"));
    assert_eq!(cat.name(), "Tom-MyInto<String>");
}

// b
// ================= GenericCat<T> =================

impl<T> GenericCat<T> {
    fn name(&self) -> &T { &self.name }
}
// Err: duplicate definitions for name()
// impl GenericCat<String> {
//     fn name(&self) -> &String { &self.name }
// }

impl<T: for<'a> From<&'a str>> GenericCat<T> {
    fn new_from_str(s: &str) -> Self { Self { name: s.into() } }
}

impl<T: From<String>> GenericCat<T> {
    fn new_from_i32(num: i32) -> Self { Self { name: num.to_string().into() } }
    fn new(s: impl MyIntoString) -> Self { Self { name: s.into_str().into() } }
    fn new_(s: impl MyInto<T>) -> Self { Self { name: s.into_t() } }
}

#[test] fn case_2_b() {
    let gen_cat = GenericCat::<String>::new_from_str("Tom");
    assert_eq!(gen_cat.name(), "Tom");

    let gen_cat = GenericCat::<String>::new_from_i32(42);
    assert_eq!(gen_cat.name(), "42");

    let gen_cat = GenericCat::<String>::new(42);
    assert_eq!(gen_cat.name(), "42-MyIntoString");

    let gen_cat = GenericCat::<String>::new_(42);
    assert_eq!(gen_cat.name(), "42-MyInto<String>");
}



// 3 - Builtin Traits/Generics for my types
// ===============================================================
// a
// Impl FromStr for Cat
// Impl ToString for Cat
// Impl Into<String> for Cat
// Impl Into<T> for Cat - understand why this is not possible
//
// b
// Impl FromStr for GenericCat<T>
// Impl ToString for GenericCat<T> - also use `where` clause
// Impl Into<String> for GenericCat<String>
// Impl Into<String> for GenericCat<T> - understand the error
// y. Impl Into<T> for GenericCat<String> - understand why this is not possible
// z. Impl Into<T> for GenericCat<T> - understand why this is not possible

// a
impl std::str::FromStr for Cat {
    type Err = std::fmt::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { name: s.to_string() })
    }
}

impl ToString for Cat {
    fn to_string(&self) -> String { self.name.clone() }
}

//// 444
//// This section is commented due to this clippy warning:
/// warning: an implementation of `From` is preferred since it gives you 
/// `Into<_>` for free where the reverse isn't true
//
// impl Into<String> for Cat {
//     fn into(self) -> String { self.name }
// }

//// Error since Into<_> is already implemented in stdlib
//// in \rustlib\src\rust\library\core\src\convert\mod.rs
//// error[E0119]: conflicting implementations of trait `Into<_>` for type `Cat`
//// note: conflicting implementation in crate `core`: impl<T, U> Into<U> for T where U: From<T>;
//
// impl<T: From<String>> Into<T> for Cat {
//     fn into(self) -> T { self.name.into() }
// }

#[test] fn case_3_a() {
    let cat = Cat::new_from_str("Tom");
    assert_eq!(cat.to_string(), "Tom");

    let cat: Cat = std::str::FromStr::from_str("Tom").unwrap();
    // or
    let cat = <Cat as std::str::FromStr>::from_str("Tom").unwrap();
    assert_eq!(cat.name(), "Tom");

    assert_eq!(<Cat as Into<String>>::into(cat), "Tom"); // cat is moved
}


#[test] fn case_3_b() {
    let gen_cat = <GenericCat<String> as std::str::FromStr>::from_str("tom").unwrap();
    assert_eq!(gen_cat.name(), "tom");

    assert_eq!(gen_cat.to_string(), "tom"); // ToString

    //// 333
    //// This section is disabled due to a clippy warning. See details in impl
    // assert_eq!( <GenericCat<String> as Into<String>>::into(gen_cat), "tom" ); todo
}

// b
// ================= Builtin Traits/Generics for GenericCat<T>

// === FromStr

impl<T: From<String>> std::str::FromStr for GenericCat<T> {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok( Self { name: s.to_string().into() })
    }
}

// === ToString

impl<T: ToString> ToString for GenericCat<T> {
    fn to_string(&self) -> String { self.name.to_string() }
}
//// Same, using `where`
////
// impl<T> ToString for GenericCat<T> where T: ToString {
//     fn to_string(&self) -> String { self.name.to_string().into() }
// }

//// This can be used instead of GenericCat<T> version above,
//// But not together
////
// impl ToString for GenericCat<String> {
//     fn to_string(&self) -> String { self.name.clone() }
// }


// === Into<T>

//// 333
//// This section is commented due to this clippy warning:
//// warning: an implementation of `From` is preferred since it gives you 
//// `Into<_>` for free where the reverse isn't true
/*
//// Only one of these two can exist at a time
impl Into<String> for GenericCat<String> {
    fn into(self) -> String { self.name }
}
// error[E0119]: conflicting implementations of trait `Into<String>` for type `GenericCat<String>
// impl<T: Into<String>> Into<String> for GenericCat<T> {
//     fn into(self) -> String { self.name.into() }
// }
*/

//// y, z: Into<_> is already implemented in stdlib

// y
//// Error since impl<T, U> Into<U> for T is already implemented in stdlib
//// error[E0119]: conflicting implementations of trait `Into<_>` for type `GenericCat<String>`
//// note: conflicting implementation in crate `core`: impl<T, U> Into<U> for T where U: From<T>;
//
// impl<T: From<String>> Into<T> for GenericCat<String> {
//     fn into(self) -> T { self.name.into() }
// }

// z
//// Error since impl<T, U> Into<U> for T is already implemented in stdlib
//// in \rustlib\src\rust\library\core\src\convert\mod.rs
//// error[E0119]: conflicting implementations of trait `Into<_>` for type `GenericCat<_>`
//// note: conflicting implementation in crate `core`: impl<T, U> Into<U> for T where U: From<T>;
//
// impl<T: Into<S>, S> Into<S> for GenericCat<T> {
//     fn into(self) -> S { self.name.into() }
// }
