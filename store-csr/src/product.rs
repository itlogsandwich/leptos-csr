pub struct Product
{
    id: i64,
    name: String,
    description: String, 
    price: f64,
}

impl Product 
{
    pub fn new(id: i64, name: String, description: String, price: f64) -> Self 
    {
        Self
        {
            id,
            name,
            description,
            price, 
        }
    }
}