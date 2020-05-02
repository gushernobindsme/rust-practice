mod outermost {
    pub fn middle_function() {}

    fn middle_secret_function() {}

    mod inside {
        pub fn inner_function() {}

        fn secret_function() {}
    }
}

fn try_me() {
    outermost::middle_function();
    // error outermost::middle_secret_function();
    // error outermost::inside::inner_function();
    // error outermost::inside::secret_function();
}
