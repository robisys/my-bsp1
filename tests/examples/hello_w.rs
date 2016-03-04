

fn t() {

            let s = "Hello World";

            assert_eq!(s, "Hello World");


}

#[test] fn non_macro() { t() }


//#[test] fn _macro() { t("hello_world_macro") }
