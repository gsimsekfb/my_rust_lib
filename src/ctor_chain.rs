
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Foo {
    xx: u8,
    yy: u8
}

impl Foo {
    pub fn set_xx(&mut self, xx: u8) -> &mut Self {
        self.xx = xx;
        self
    }
    pub fn set_yy(&mut self, yy: u8) -> &mut Self {
        self.yy = yy;
        self
    }
}

#[test] fn ex1() {
    let mut foo = Foo::default();
    assert_eq!(foo, Foo { xx:0, yy: 0 });
    foo.set_xx(33).set_yy(55);
    assert_eq!(foo, Foo { xx:33, yy: 55 });
}