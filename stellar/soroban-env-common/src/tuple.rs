use crate::{FromValEnum, ToValEnum};

macro_rules! find_len_idx {
    () => { 0 };
    ($head:tt $($tail:tt)*) => {
        std::cmp::max($head, find_len_idx!($($tail)*))
    };
}

macro_rules! derive_to_val_for_tuples {
    ( $($typ:ident $idx:tt)* ) => (
        impl<$($typ:ToValEnum),*> ToValEnum for ($($typ,)*) {
            fn to_val(&self) -> crate::Val {
                let tuple = vec![$(self.$idx.to_val()),*];
                crate::Val::TupleVal(tuple)
            }
        }

        impl<$($typ:FromValEnum),*> FromValEnum for ($($typ,)*) {
            fn from_val(val: crate::Val) -> Option<Self> {
                match val {
                    crate::Val::TupleVal(tuple) => {
                        if tuple.len() != find_len_idx!($($idx)*) + 1 {
                            return None;
                        }
                        Some(($(
                            $typ::from_val(tuple[$idx].clone())?,
                        )*))
                    }
                    _ => None,
                }
            }
        }
    );
}

derive_to_val_for_tuples! {}
derive_to_val_for_tuples! { T0 0}
derive_to_val_for_tuples! { T0 0 T1 1}
derive_to_val_for_tuples! { T0 0 T1 1 T2 2}
derive_to_val_for_tuples! { T0 0 T1 1 T2 2 T3 3}
derive_to_val_for_tuples! { T0 0 T1 1 T2 2 T3 3 T4 4}
derive_to_val_for_tuples! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5}
derive_to_val_for_tuples! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6}
derive_to_val_for_tuples! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7}
derive_to_val_for_tuples! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8}
derive_to_val_for_tuples! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9}
derive_to_val_for_tuples! { T0 0 T1 1 T2 2 T3 3 T4 4 T5 5 T6 6 T7 7 T8 8 T9 9 T10 10}

#[cfg(test)]
mod test {
    use crate::{Storage, Symbol};

    #[test]
    fn test_has_storage_with_enum() {
        let storage = Storage::default();
        let instance = storage.instance();
        let symb = Symbol::from("test");
        let value = (10u32, 20);
        instance.set(&symb, &value);
        assert!(instance.has(&symb));
        assert_eq!(instance.get::<Symbol, (u32, i32)>(&symb), Some(value));
    }
}
