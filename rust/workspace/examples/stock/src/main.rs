fn main() {
    let cur_cost = 41.7542;
    let cur_total = 3500.0;
    let new_cost = 39.0;
    let cur_price = 34.34;
    let buy_more = 1500.0;

    let r = ((cur_cost - new_cost) * cur_total) / (new_cost - cur_price);
    println!("buy more: {r}");

    let r = (cur_cost * cur_total + buy_more * cur_price) / (cur_total + buy_more);
    println!("new cost: {r}");

    let r = (new_cost * (cur_total + buy_more) - cur_cost * cur_total) / buy_more;
    println!("cur price: {r}");
}
