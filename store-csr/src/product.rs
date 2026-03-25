#[derive(Clone, Debug)]
pub struct Product
{
    pub id: i64,
    pub name: String,
    pub description: String, 
    pub category: String,
    pub price: f64,
}

impl Product 
{
    pub fn new(id: i64, name: String, description: String, category: String, price: f64) -> Self 
    {
        Self
        {
            id,
            name,
            description,
            category,
            price, 
        }
    }
}
