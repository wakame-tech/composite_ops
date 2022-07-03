pub mod hashmap;

#[cfg(test)]
mod tests {
    use crate::hashmap::addable::AddableHashMap;
    use composite_ops_macro::CompositeOps;
    use std::{collections::HashMap, ops::Add};

    #[derive(Debug, PartialEq, Eq)]
    struct Exp(i32);

    impl Add for Exp {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Exp(self.0 + rhs.0)
        }
    }

    #[derive(Debug, CompositeOps, PartialEq, Eq)]
    struct Player {
        exp: Exp,
        items: AddableHashMap<String, i32>,
    }

    #[test]
    fn test_composite_add() {
        let p = Player {
            exp: Exp(1),
            items: AddableHashMap(
                vec![("a".to_string(), 3), ("b".to_string(), 4)]
                    .into_iter()
                    .collect::<HashMap<String, i32>>(),
            ),
        };
        let q = Player {
            exp: Exp(1),
            items: AddableHashMap(
                vec![("c".to_string(), 5)]
                    .into_iter()
                    .collect::<HashMap<String, i32>>(),
            ),
        };
        assert_eq!(
            p + q,
            Player {
                exp: Exp(2),
                items: AddableHashMap(
                    vec![
                        ("a".to_string(), 3),
                        ("b".to_string(), 4),
                        ("c".to_string(), 5),
                    ]
                    .into_iter()
                    .collect::<HashMap<String, i32>>()
                )
            }
        );
    }
}
