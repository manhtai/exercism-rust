#[macro_export]
macro_rules! hashmap {
    ( $($k:expr => $v:expr),* $(,)?) => {
        {
            let mut temp_hashmap = ::std::collections::HashMap::new();
            $(
                temp_hashmap.insert($k, $v);
            )*
            temp_hashmap
        }
    };

}

