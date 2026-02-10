// interv-1

// Generic struct Wrapper, with member `inner`
// struct Foo with x i32
// We want to be able to do `wrapper.x` instead of `wrapper.inner.x``
// Hint: at the end







// ==============================



#[derive(Debug)]
struct Wrapper<T> { inner: T }

#[derive(Debug)]
struct Foo { x: i32 }

use std::ops::Deref;

impl<T> Deref for Wrapper<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// or
// impl Deref for Wrapper<Foo> {
//     type Target = Foo;
//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }


#[test]
fn test() {
    let foo = Foo { x: 42 };
    let wrapper = Wrapper { inner: foo };

    // !! after impl.ing Deref we can do `wrapper.x` instead of `wrapper.inner.x``
    let res = wrapper.x;
    assert_eq!(res, 42);
}



// Hint: Implement Deref for Wrapper
// Keys: Deref coercion, generics 