// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

fn validate_name(name: &String) {
    if name.len() == 0 || name.as_bytes().len() > 300 {
        panic!()
    }
}

fn validate_price(n: &u32) {
    if *n == 0 {
        panic!()
    }
}

fn validate_quantity(n: &u32) {
    if *n == 0 {
        panic!()
    }
}

pub struct Order {
    pub product_name: String,
    pub quantity: u32,
    pub unit_price: u32,
}

impl Order {
    pub fn new(name: String, qty: u32, price: u32) -> Order {
        validate_name(&name);
        validate_price(&price);
        validate_quantity(&qty);

        Order {
            product_name: name,
            quantity: qty,
            unit_price: price,
        }
    }

    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, val: String) {
        validate_name(&val);
        self.product_name = val;
    }

    pub fn set_quantity(&mut self, val: u32) -> &u32 {
        validate_quantity(&val);
        self.quantity = val;
        &self.quantity
    }

    pub fn set_unit_price(&mut self, val: u32) -> &u32 {
        validate_price(&val);
        self.unit_price = val;
        &self.unit_price
    }
}
