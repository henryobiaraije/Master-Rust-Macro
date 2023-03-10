use std::cell::Cell;

fn main() {
    let foo = Foo {
        raw: "the raw ",
        cell: Cell::new("the cell"),
    };
}
struct Bar<'a> {
    bar: &'a str,
}
impl<'a> Bar<'a> {
    fn new(foo: &'a Foo<'a>) -> Bar<'a> {
        Bar { bar: foo.raw }
    }
}

pub struct Foo<'a> {
    raw: &'a str,
    cell: Cell<&'a str>,
}
impl<'a> Foo<'a> {
    fn get_bar(&self) -> Bar {
        Bar::new(&self)
    }
}
