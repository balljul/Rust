mod front_of_house {
    #[allow(unused)]
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_tabl() {}
    }

    #[allow(unused)]
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

#[allow(unused)]
mod back_of_house {
    pub enum Appetizer {
        Option01,
        Option02,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_food: String,
    }   
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_food: String::from("Oregano"),
            }
        }
    }
}

#[allow(unused)]
pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();

    let toast = String::from("tast");

    let mut my_toast = back_of_house::Breakfast::summer(&toast);
    let mut enum_test = back_of_house::Appetizer::Option01;
    my_toast.toast = String::from("asdf");
    // my_toast.seasonal_food = String::from("invalid");

}
