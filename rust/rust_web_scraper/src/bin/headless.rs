// define a custom data structure
// to store the scraped data
struct PokemonProduct {
    url: String,
    image: String,
    name: String,
    price: String,
}

fn main() {
    let mut pokemon_products: Vec<PokemonProduct> = Vec::new();

    let browser = headless_chrome::Browser::default().unwrap();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to("https://scrapeme.live/shop/").unwrap();

    let html_products = tab.wait_for_elements("li.product").unwrap();

    for html_product in html_products {
        // scraping logic...
        let url = html_product
            .wait_for_element("a")
            .unwrap()
            .get_attributes()
            .unwrap()
            .unwrap()
            .get(1)
            .unwrap()
            .to_owned();
        let image = html_product
            .wait_for_element("img")
            .unwrap()
            .get_attributes()
            .unwrap()
            .unwrap()
            .get(5)
            .unwrap()
            .to_owned();
        let name = html_product
            .wait_for_element("h2")
            .unwrap()
            .get_inner_text()
            .unwrap();
        let price = html_product
            .wait_for_element(".price")
            .unwrap()
            .get_inner_text()
            .unwrap();
        let pokemon_product = PokemonProduct {
            url,
            image,
            name,
            price,
        };

        pokemon_products.push(pokemon_product);
    }

    // CSV export
    let path = std::path::Path::new("products.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();
    writer
        .write_record(&["url", "image", "name", "price"])
        .unwrap();

    // populate the output file
    for product in pokemon_products {
        let url = product.url;
        let image = product.image;
        let name = product.name;
        let price = product.price;
        writer.write_record(&[url, image, name, price]).unwrap();
    }

    writer.flush().unwrap();
}
