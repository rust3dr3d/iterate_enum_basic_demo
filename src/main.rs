/*
    We are importing IntoEnumIterator and EnumIter
    to enable iterating over an enum
    Display will let us directly print the enum variants without Debug flag(:?)
    It will also let us use .to_string() (see example II)
*/
use strum::{IntoEnumIterator, EnumIter, Display};


#[derive(EnumIter, Display)]
enum CardTypes{
    AmericanExpress,
    MasterCard,
    Discover,
    Visa,
    PayPal,
    #[strum(serialize = "JCB")] //We want "Jcb" variant to be printed as "JCB"
    Jcb,
}


fn main() {
    println!();

    // Directly iterating over the enum
    for card in CardTypes::iter(){
        print!("{}, ", card);
    }
    print!("\n\n");

    // Adding all enum variants to a Vec<String>
    // Iterating over it using a while let loop
    let v:Vec<String> = CardTypes::iter()
                        .map(|x| x.to_string()) // Convert each x to String
                        .collect();

    let mut iterator = v.iter();
    while let Some(card) = iterator.next(){
        println!("{}", card);
    }
}
