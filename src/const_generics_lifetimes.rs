//// const generics and lifetimes
// impl sum ...

/// 1
/// w/ fn

// a 
// e.g. sum of array elems

fn sum(arr: &[i32; 2]) -> i32 {
    arr.iter().sum()
}
fn gen_sum_i32<const N: usize>(arr: &[i32; N]) -> i32 {
    arr.iter().sum()
}
// fn gen_sum_t<T: for<'a> std::iter::Sum<&'a T>, const N: usize>(arr: &[T; N]) -> T {
// or using 'where'
fn gen_sum_t<T, const N: usize>(arr: &[T; N]) -> T 
where T: for<'a> std::iter::Sum<&'a T> {
    arr.iter().sum()
}

// b - in fn body
fn double<const N: i32>() -> i32 {
    N * 2
}

/// 2 
/// w/ struct field and associated const

struct Foo<const N: usize>([i32;N]);
impl<const N: usize> Foo<N> {
    const PWD: usize = N*10;
}

#[test] fn ex_1_and_2() {
    // 1
    // a
    assert_eq!(sum(&[1,2]), 3);
    // assert_eq!(sum([1,2,3]), 3);  // error
    //
    assert_eq!(gen_sum_i32(&[1,2]), 3);
    assert_eq!(gen_sum_i32(&[1,2,3]), 6); // Ok
    //
    assert_eq!(gen_sum_t(&[1,2]), 3);
    assert_eq!(gen_sum_t(&[1,2,3]), 6);
    // b
    assert_eq!(double::<4>(), 8);

    // 2
    let f = Foo([1,2]);
    let f = Foo([1,2,3]);
    assert_eq!(f.0[2], 3);
    assert_eq!(Foo::<4>::PWD, 40);
}


/// 3 
/// w/ trait
trait MyTrait<const R: usize> {
    type Item;
    fn r(&self) -> usize;
}
impl<const N: usize, const R: usize> MyTrait<R> for Foo<N> {
    type Item = [i32; N];
    fn r(&self) -> usize { R }
}

#[test] fn ex_3() {
    let foo = Foo([1,2]);
    assert_eq!(<Foo<2> as MyTrait<4>>::r(&foo), 4);
}


/// 4
/// restrictions
// const parameters may only appear as a standalone argument inside of a type 
// or array repeat expression.

// const generic parameters cannot be used in these cases:

// a
fn foo<const N: usize>() {
    // Cannot use in item definitions within a function body.
    // const BAD_CONST: [usize; N] = [1; N];
        // error[E0401]: can't use generic parameters from outer item
}

// b
// Not allowed to combine in other expressions in types, such as the
// arithmetic expression in the return type here.
    // error: generic parameters may not be used in const operations
// fn bad_function<const N: usize>() -> [u8; {N + 1}] {
//     // Similarly not allowed for array repeat expressions.
//     [1; {N + 1}]
// }
