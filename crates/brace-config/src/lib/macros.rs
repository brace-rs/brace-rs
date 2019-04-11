#[macro_export]
macro_rules! value {
    ([]) => {
        $crate::Value::array()
    };

    ([ $($tt:tt)+ ]) => {
        $crate::Value::from($crate::array!($($tt)+))
    };

    ({}) => {
        $crate::Value::table()
    };

    ({ $($tt:tt)+ }) => {
        $crate::Value::from($crate::table!($($tt)+))
    };

    ($other:expr) => {
        $crate::to_value(&$other).unwrap()
    };
}

#[macro_export]
macro_rules! array {
    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        std::vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        std::vec![$($elems),*]
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        $crate::array!(@array [$($elems,)* $crate::value!([$($array)*])] $($rest)*)
    };

    // Next element is a table.
    (@array [$($elems:expr,)*] {$($table:tt)*} $($rest:tt)*) => {
        $crate::array!(@array [$($elems,)* $crate::value!({$($table)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        $crate::array!(@array [$($elems,)* $crate::value!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        $crate::array!(@array [$($elems,)* $crate::value!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        $crate::array!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        $crate::value_unexpected!($unexpected)
    };

    () => {
        $crate::Array::new()
    };

    ( $($tt:tt)+ ) => {
        $crate::Array::from($crate::array!(@array [] $($tt)+))
    };
}

#[macro_export]
macro_rules! table {
    // Done.
    (@table $table:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@table $table:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        $table.insert(($($key)+).into(), $value);
        $crate::table!(@table $table () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@table $table:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        $crate::value_unexpected!($unexpected);
    };

    // Insert the last entry without trailing comma.
    (@table $table:ident [$($key:tt)+] ($value:expr)) => {
        $table.insert(($($key)+).into(), $value);
    };

    // Next value is an array.
    (@table $table:ident ($($key:tt)+) (= [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        $crate::table!(@table $table [$($key)+] ($crate::value!([$($array)*])) $($rest)*);
    };

    // Next value is a table.
    (@table $table:ident ($($key:tt)+) (= {$($next_table:tt)*} $($rest:tt)*) $copy:tt) => {
        $crate::table!(@table $table [$($key)+] ($crate::value!({$($next_table)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@table $table:ident ($($key:tt)+) (= $value:expr , $($rest:tt)*) $copy:tt) => {
        $crate::table!(@table $table [$($key)+] ($crate::value!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@table $table:ident ($($key:tt)+) (= $value:expr) $copy:tt) => {
        $crate::table!(@table $table [$($key)+] ($crate::value!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@table $table:ident ($($key:tt)+) (=) $copy:tt) => {
        $crate::value_unexpected!("");
    };

    // Missing assignment for last entry. Trigger a reasonable error message.
    (@table $table:ident ($($key:tt)+) () $copy:tt) => {
        $crate::value_unexpected!("");
    };

    // Misplaced assignment. Trigger a reasonable error message.
    (@table $table:ident () (= $($rest:tt)*) ($unexpected:tt $($copy:tt)*)) => {
        $crate::value_unexpected!($unexpected);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@table $table:ident ($($key:tt)*) (, $($rest:tt)*) ($unexpected:tt $($copy:tt)*)) => {
        $crate::value_unexpected!($unexpected);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false positives because the
    // parenthesization may be necessary here.
    (@table $table:ident () (($key:expr) = $($rest:tt)*) $copy:tt) => {
        $crate::table!(@table $table ($key) (= $($rest)*) (= $($rest)*));
    };

    // Munch a token into the current key.
    (@table $table:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        $crate::table!(@table $table ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    () => {
        $crate::Table::new()
    };

    ( $($tt:tt)+ ) => {
        {
            let mut table = std::collections::HashMap::new();
            $crate::table!(@table table () ($($tt)+) ($($tt)+));
            $crate::Table::from(table)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! value_unexpected {
    () => {};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_value() {
        let entry = value!("entry");
        let array = value!(["array"]);
        let table = value!({ "table" = true });

        assert_eq!(entry.val::<String>().unwrap(), "entry");
        assert_eq!(array.get::<String>("0").unwrap(), "array");
        assert_eq!(table.get::<bool>("table").unwrap(), true);
    }

    #[test]
    fn test_array() {
        let array1 = array![];
        let array2 = array![[]];
        let array3 = array![{}];
        let array4 = array!["a"];
        let array5 = array![[], {}, "a"];
        let array6 = array!['a', "b", ("c", "d")];

        assert_eq!(array1.0.len(), 0);
        assert_eq!(array2.0.len(), 1);
        assert_eq!(array3.0.len(), 1);
        assert_eq!(array4.0.len(), 1);
        assert_eq!(array5.0.len(), 3);
        assert_eq!(array6.0.len(), 3);
    }

    #[test]
    fn test_table() {
        let t = table! {
            "a" = "a",
            "b" = "b",
            "c" = {},
            "d" = [],
            "e" = ['f', "g", ("h", "i", true)],
            "j" = {
                "k" = "l",
                "m" = {
                    "n" = ["o", "p"],
                },
            },
            "q" = ("r", "s"),
        };

        assert_eq!(t.get::<String>("a").unwrap(), "a");
        assert_eq!(t.get::<String>("b").unwrap(), "b");
        assert!(t.get_value("c").unwrap().as_table().unwrap().0.is_empty());
        assert!(t.get_value("d").unwrap().as_array().unwrap().0.is_empty());
        assert_eq!(t.get_value("e").unwrap().as_array().unwrap().0.len(), 3);
        assert_eq!(t.get_value("e.2").unwrap().as_array().unwrap().0.len(), 3);
        assert_eq!(t.get::<String>("e.2.1").unwrap(), "i");
        assert_eq!(t.get::<String>("j.k").unwrap(), "l");
        assert_eq!(t.get::<String>("j.m.n.0").unwrap(), "o");
        assert_eq!(t.get::<String>("q.0").unwrap(), "r");
    }
}
