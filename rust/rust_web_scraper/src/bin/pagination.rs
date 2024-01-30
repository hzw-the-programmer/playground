fn main() {
    // initialize the vector that will store the scraped data
    let mut pokemon_products: Vec<PokemonProduct> = Vec::new();

    // pagination page to start from
    let first_page = "https://scrapeme.live/shop/page/1/";

    // define the supporting data structures
    let mut pages_to_scrape: Vec<String> = vec![first_page.to_owned()];
    let mut pages_discovered: std::collections::HashSet<String> = std::collections::HashSet::new();

    // current iteration
    let mut i = 1;
    // max number of iterations allowed
    let max_iterations = 5;

    while !pages_to_scrape.is_empty() && i <= max_iterations {
        // get the first element from the queue
        let page_to_scrape = pages_to_scrape.remove(0);

        // retrieve and parse the HTML document
        let response = reqwest::blocking::get(page_to_scrape);
        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);

        let html_product_selector = scraper::Selector::parse("li.product").unwrap();
        let html_products = document.select(&html_product_selector);

        for html_product in html_products {
            // scraping logic to retrieve the info
            // of interest
            let url = html_product
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .and_then(|a| a.value().attr("href"))
                .map(str::to_owned);
            let image = html_product
                .select(&scraper::Selector::parse("img").unwrap())
                .next()
                .and_then(|img| img.value().attr("src"))
                .map(str::to_owned);
            let name = html_product
                .select(&scraper::Selector::parse("h2").unwrap())
                .next()
                .map(|h2| h2.text().collect::<String>());
            let price = html_product
                .select(&scraper::Selector::parse(".price").unwrap())
                .next()
                .map(|price| price.text().collect::<String>());

            // instantiate a new Pokemon product
            // with the scraped data and add it to the list
            let pokemon_product = PokemonProduct {
                url,
                image,
                name,
                price,
            };
            pokemon_products.push(pokemon_product);
        }

        // get all pagination link elements
        let html_pagination_link_selector = scraper::Selector::parse("a.page-numbers").unwrap();
        let html_pagination_links = document.select(&html_pagination_link_selector);

        // iterate over them to find new pages to scrape
        for html_pagination_link in html_pagination_links {
            // get the pagination link URL
            let pagination_url = html_pagination_link
                .value()
                .attr("href")
                .unwrap()
                .to_owned();

            // if the page discovered is new
            if !pages_discovered.contains(&pagination_url) {
                pages_discovered.insert(pagination_url.clone());

                // if the page discovered should be scraped
                if !pages_to_scrape.contains(&pagination_url) {
                    pages_to_scrape.push(pagination_url.clone());
                }
            }
        }

        // increment the iteration counter
        i += 1;
    }

    // create the CSV output file
    let path = std::path::Path::new("products.csv");
    let mut writer = csv::Writer::from_path(path).unwrap();

    // append the header to the CSV
    writer
        .write_record(&["url", "image", "name", "price"])
        .unwrap();
    // populate the output file
    for product in pokemon_products {
        let url = product.url.unwrap();
        let image = product.image.unwrap();
        let name = product.name.unwrap();
        let price = product.price.unwrap();
        writer.write_record(&[url, image, name, price]).unwrap();
    }

    // free up the resources
    writer.flush().unwrap();
}

struct PokemonProduct {
    url: Option<String>,
    image: Option<String>,
    name: Option<String>,
    price: Option<String>,
}
