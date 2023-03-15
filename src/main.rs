use paste::paste;
fn main() {
    let mut product1 = Product {
        id: 425,
        title: String::from("Product 1"),
        sku: String::from("JIFEJ2"),
        url: String::from("https://example-product.com"),
    };

    println!("url = {}", product1.url());

    product1.set_url(String::from("https://changed.com"));

    println!("url after = {}", product1.url());
}

create_struct!(Product {
    id: u32,
    title: String,
    sku: String,
    url: String,
});

create_struct!(User {
    id: u32,
    name: String
});

#[macro_export]
macro_rules! create_struct  {
    ($struct_name:ident {$($field_name:ident : $field_type : ty),* $(,)*}  ) => {


        #[derive(Debug)]
        struct $struct_name {
                $($field_name : $field_type),*
        }

        paste!{
            impl $struct_name{
                fn new($($field_name:$field_type),*)  -> Self{
                    Self{
                        $($field_name),*
                    }
                }

                $(
                    fn $field_name(&self) -> &$field_type{
                        &self.$field_name
                    }
                )*

                $(
                    fn [<set_ $field_name>](&mut self,$field_name:$field_type) {
                        self.$field_name = $field_name
                    }
                )*
            }
        }

    };
}
