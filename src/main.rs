mod coin;


fn main() {
    let mut coin = coin::Coin::new(20);


    let mut coin_value = coin::Coin::get_value(&coin);
    println!("{}", coin_value);    
    coin.set_value(45);

    coin_value = coin::Coin::get_value(&coin);

    println!("{}", coin_value);
    


}
