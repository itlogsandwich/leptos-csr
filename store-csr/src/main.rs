use leptos_router::{hooks::use_params, hooks::use_navigate, params::Params, path, components::{Router, Route, Routes, ParentRoute, A, Outlet}};
use leptos::{prelude::*, Params, mount::mount_to_body};
use product::Product;

mod product;

fn remove_product(product_id: i64, set_products: WriteSignal<Vec<Product>>)
{
    set_products.write().retain(|p| p.id != product_id)
}

fn clear_fields(
    set_name: WriteSignal<String>,
    set_description: WriteSignal<String>,
    set_category: WriteSignal<String>,
    set_price: WriteSignal<f64>,
)
{
    set_name.set(String::from(""));
    set_description.set(String::from(""));
    set_category.set(String::from(""));
    set_price.set(0.0);
}

#[derive(Params, PartialEq)]
struct IdParams 
{
    id: Option<usize>,
}

#[component]
fn ViewProduct(products: ReadSignal<Vec<Product>>) -> impl IntoView
{

    let params = use_params::<IdParams>();
    
    let product_id = move ||
    {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id)
            .unwrap_or_default()
    };
    

    let product = move ||  products.get().into_iter().find(|p| p.id == product_id() as i64);

    view!
    {
        <div class="card">
            <div style="display: flex; flex-direction: row-reverse;" >
                <A href="/products"> "Close" </A> 
            </div>

            {move || product().map(|p|
            {
                view! 
                {
                    <div class="details-div">
                        <label>"Name:"</label>
                        <label> {p.name} </label> 
                    </div>

                    <div class="details-div">
                        <label> "Description:" </label>
                        <label> {p.description} </label> 
                    </div>

                    <div class="details-div">
                        <label> "Category:" </label>
                        <label> {p.category} </label> 
                    </div>

                    <div class="details-div">
                        <label> "Price" </label>
                        <label> "$"{p.price} </label> 
                    </div>

                    <div style="display: flex; gap:20px; justify-content: space-evenly">
                        <A href="/products/create">
                            "Add Product"
                        </A>
                        <A href=format!("/products/edit/{}", p.id)>
                            "Edit Product"
                        </A>
                    </div>
                }
            }.into_any())
            .unwrap_or_else(|| 
            view!
            {
                <div style="display:flex; justify-content: center; align-items: center;">
                    <h1>"There has been an error viewing the product!"</h1>
                </div>
            }.into_any())}

        </div>
    }
}

#[component]
fn UpdateProduct(
    name_signal: RwSignal<String>,
    description_signal: RwSignal<String>,
    category_signal: RwSignal<String>,
    price_signal: RwSignal<f64>,
    products_signal: RwSignal<Vec<Product>>,
) -> impl IntoView
{
    let params = use_params::<IdParams>();
    
    let product_id = move ||
    {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id)
            .unwrap_or_default()
    };

    let (name,  set_name) = name_signal.split();
    let (description, set_description) = description_signal.split();
    let (price, set_price) = price_signal.split();
    let (category, set_category) = category_signal.split();
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

            <label for="category-select"> "Category" </label>
            <select name="category-select" id="categories" bind:value=(category, set_category)>
                <option value="Food & Beverage"> "Food & Beverage" </option>
                <option value="Electronics"> "Electronics" </option>
                <option value="Apparel"> "Apparel" </option>
                <option value="Miscellaneous"> "Miscellaneous" </option>
            </select>

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
                set_products.write().iter_mut().find(|p| p.id == product_id() as i64).map(|p| 
                {
                    *p = Product
                    {
                        name: name.get(),
                        description: description.get(),
                        category: category.get(),
                        price: price.get(),
                        ..*p
                    }
                });

                clear_fields(set_name, set_description, set_category, set_price);

                let navigate = use_navigate();
                navigate("/products", Default::default());
            }
            >
                "Edit Product"
            </button>
        </div>
    }
}

#[component]
fn AddProduct(
    id_signal: RwSignal<i64>,
    name_signal: RwSignal<String>,
    description_signal: RwSignal<String>,
    category_signal: RwSignal<String>,
    price_signal: RwSignal<f64>,
    products_signal: RwSignal<Vec<Product>>,
) -> impl IntoView
{

    let (id, set_id) = id_signal.split();
    let (name,  set_name) = name_signal.split();
    let (description, set_description) = description_signal.split();
    let (category, set_category) = category_signal.split();
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

            <label for="category-select"> "Category" </label>
            <select name="category-select" id="categories" bind:value=(category, set_category)>
                <option value="Food & Beverage"> "Food & Beverage" </option>
                <option value="Electronics"> "Electronics" </option>
                <option value="Apparel"> "Apparel" </option>
                <option value="Miscellaneous"> "Miscellaneous" </option>
            </select>

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

                let product = Product::new(id.get(), name.get(), description.get(), category.get(), price.get()); 

                set_products.write().push(product);

                clear_fields(set_name, set_description, set_category, set_price);

                let navigate = use_navigate();
                navigate("/products", Default::default());
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
                    <th>"Category"</th>
                    <th>"Price"</th>
                    <th>"Actions"</th>
                </tr>
                <For
                    each=move || products.get()
                    key= |p| p.id
                    children=move |product|
                    {
                        view!
                        {
                            <tr>
                                {move || products.get().iter().find(|p| p.id == product.id).map(|p| 
                                {
                                    view!
                                    {
                                        <td>{p.id}</td>
                                        <td>{p.name.clone()}</td>
                                        <td>{p.description.clone()}</td>
                                        <td>{p.category.clone()}</td>
                                        <td>{p.price}</td>
                                    }
                                })}
                                <td style="margin: 10px;">
                                    <A href=format!("/products/view/{}", product.id)>
                                        "View"
                                    </A>
                                    <A href=format!("/products/edit/{}",product.id)>
                                        "Edit"
                                    </A>
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
    let labels = ["Home", "Products"];
    view!
    {
        <div class="home-container">
            <A href="/">
                <div class="card">
                    <label>
                        {labels[0]}
                    </label>
                </div>
            </A>
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
    let category_signal = RwSignal::new(String::from(""));
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
                                category_signal=category_signal
                                price_signal=price_signal
                                products_signal=products_signal 
                            /> 
                        } 
                        />

                        <Route path=path!("/view/:id") view=move || view!
                        {
                            <ViewProduct products=products_signal.read_only() />
                        }
                        /> 
                        <Route path=path!("/edit/:id") view=move || view! 
                        { 
                            <UpdateProduct 
                                name_signal=name_signal
                                description_signal=description_signal
                                category_signal=category_signal
                                price_signal=price_signal
                                products_signal=products_signal 
                            /> 
                        }
                        />
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

fn main() 
{
    _ = console_error_panic_hook::set_once();
    mount_to_body(App);
}
