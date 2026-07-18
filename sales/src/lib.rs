#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        for (name, price) in &s.products {
            if *name == ele {
                self.items.push((name.clone(), *price))
            }
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32>  = self.items.iter().map(|(_,p)| *p).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut result = Vec::new();

        for chunk in prices.chunks(3) {
            if chunk.len() == 3 {
                let cheap = chunk[0];
                let total: f32 = chunk.iter().sum();
                let discount = cheap / total;
                for p in chunk {
                    let new_price = p * (1.0 - discount);
                    result.push((new_price * 100.0).round() / 100.0);
                }
            } else {
                for p in chunk {
                    result.push((p * 100.0).round() / 100.0);
                }
            }
        }

        self.receipt = result.clone();
        result
    }
}