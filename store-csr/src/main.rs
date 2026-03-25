use leptos_router::{path, components::{Router, Route, Routes, ParentRoute, A, Outlet}};
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use product::Product;

mod product;

fn remove_product(product_id: i64, set_products: WriteSignal<Vec<Product>>) -> ()
{
    set_products.write().retain(|p| p.id != product_id)
}

#[component]
fn UpdateProduct() -> impl IntoView
{
    view!
    {
        <h1> "Update Product" </h1>
    }
}

#[component]
fn AddProduct(
    id_signal: RwSignal<i64>,
    name_signal: RwSignal<String>,
    description_signal: RwSignal<String>,
    price_signal: RwSignal<f64>,
    products_signal: RwSignal<Vec<Product>>,
) -> impl IntoView
{

    let (id, set_id) = id_signal.split();
    let (name,  set_name) = name_signal.split();
    let (description, set_description) = description_signal.split();
    let (price, set_price) = price_signal.split();

    let set_products = products_signal.write_only();

    view!
    {
        <div class="card">
            <div style="display: flex; flex-direction: row-reverse;" >
                <A href="/products"> "Close" </A> 
            </div>

            <label for="name-input">"Name"</label>
            <input class="input" name="name-input" type="text" bind:value=(name, set_name)/>

            <label for="description-input"> "Description" </label>
            <input class="input" name="description-input" type="text" bind:value=(description, set_description)/>

            <label for="price-input"> "Price" </label>
            <input class="input" name="price-input" type="number" prop:value=price on:input=move |e| 
            {
                let val = event_target_value(&e)
                    .parse::<f64>()
                    .unwrap_or(0.0);
                set_price.set(val);
            }
            />
            <button class="btn" on:click=move |_|
            {
                set_id.set(id.get() + 1);

                let product = Product::new(id.get(), name.get(), description.get(), price.get()); 

                set_products.write().push(product);
            }
            >
                "Add Product"
            </button>
        </div>
    }
}

#[component]
fn ListDisplay(products_signal: RwSignal<Vec<Product>>) -> impl IntoView
{
    let (products, set_products) = products_signal.split();
    
    view!
    {
    
        <div class="main-container">
         
        <h1> "Products" </h1>
        <A href="/products/create"> "+ Add Product" </A>
            <table class="products-table">
                <tr>
                    <th>"ID Number"</th>
                    <th>"Name" </th>
                    <th>"Description"</th>
                    <th>"Price"</th>
                    <th> "Actions" </th>
                </tr>
                <For
                    each=move || products.get()
                    key= |p| p.id
                    children=move |product|
                    {
                        view!
                        {
                            <tr>
                                <td>{product.id}</td>
                                <td> {product.name}</td>
                                <td> {product.description}</td>
                                <td> "$"{product.price}</td>
                                <td style="margin: 10px;">
                                    <button>
                                        "View"
                                    </button>
                                    <button>
                                        "Edit"
                                    </button>
                                    <button class="btn" type="button" on:click=move|_|
                                        remove_product(product.id, set_products)
                                    >
                                    "Remove"
                                    </button>
                                </td>
                            </tr>
                        }
                    }
                />
            </table>
            <div class="add-container">
                <Outlet />
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView
{
    let labels = vec!["Home", "Products"];
    view!
    {
        <div class="home-container">
            <A href="/products">
                <div class="card">
                    <label>
                        {labels[1]}
                    </label>
                </div>
            </A>
        </div>
    }
}

#[component]
fn App() -> impl IntoView
{
    let id_signal = RwSignal::new(0);
    let name_signal = RwSignal::new(String::from(""));
    let description_signal = RwSignal::new(String::from(""));
    let price_signal = RwSignal::new(0.0);
    let products_signal = RwSignal::new(Vec::<Product>::new());
    
    view!
    {
        <Router>
            <nav id="nav-bar">
                <ul>
                    <li> <A href="/">"Home"</A> </li> 
                    <li> <A href="/products">"Products"</A> </li>
                </ul>
            </nav> 
            <main>
                <Routes fallback=|| "Not Found">
                    <Route path=path!("/") view=Home />
                    <ParentRoute path=path!("/products") view= move || view! { <ListDisplay products_signal=products_signal /> } >
                        <Route path=path!("") view= move || view! { } /> 
                        <Route path=path!("/create") view= move || view! 
                        { 
                            <AddProduct
                                id_signal=id_signal 
                                name_signal=name_signal
                                description_signal=description_signal
                                price_signal=price_signal
                                products_signal=products_signal 
                            /> 
                        } 
                        />
                        <Route path=path!(":id") view=UpdateProduct />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

fn main() 
{
    mount_to_body(App);
}
