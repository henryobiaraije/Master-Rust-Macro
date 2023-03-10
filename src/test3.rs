fn main() {
    let order = Order {
        date: "2022/12/23",
        sku: Cell::new("FIEJFIW"),
    };
    // let product = Product::new(&order);
    let product = order.get_product();
}

struct Product<'a> {
    name: &'a str,
}
impl<'a> Product<'a> {
    fn new(order: &'a Order<'a>) -> Product<'a> {
        Product { name: order.date }
    }
}

////////////////
struct Order<'a> {
    date: &'a str,
    sku: Cell<&'a str>,
}

impl<'a> Order<'a> {
    fn get_product(&'a self) -> Product<'a> {
        Product::new(&self)
    }
}
