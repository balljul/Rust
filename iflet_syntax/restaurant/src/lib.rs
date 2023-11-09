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

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
}
