#[derive(Debug, PartialEq)]
struct Shoe {
    size: u32,
    style: String,
}

#[allow(unused)]
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|shoe| shoe.size == shoe_size)
        .collect()
}

#[allow(unused)]
fn shoes_in_size_as_refs(shoes: &[Shoe], shoe_size: u32) -> Vec<&Shoe> {
    shoes.iter().filter(|shoe| shoe.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_next_method() {
        // Clippy was giving warnings about useless use of vec! macro, so v1 is an array here.
        // The results should be same with a vec as well.
        let v1 = [1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn filter_shoes_by_size() {
        let all_shoes = vec![
            Shoe {
                size: 5,
                style: String::from("red"),
            },
            Shoe {
                size: 4,
                style: String::from("white"),
            },
            Shoe {
                size: 3,
                style: String::from("red"),
            },
            Shoe {
                size: 4,
                style: String::from("blue"),
            },
        ];

        let size_4_shoes = shoes_in_size(all_shoes, 4);

        assert_eq!(
            size_4_shoes,
            vec![
                Shoe {
                    size: 4,
                    style: "white".to_string(),
                },
                Shoe {
                    size: 4,
                    style: "blue".to_string(),
                },
            ]
        );
    }

    #[test]
    fn filter_shoes_by_size_as_refs() {
        let all_shoes = vec![
            Shoe {
                size: 5,
                style: String::from("red"),
            },
            Shoe {
                size: 4,
                style: String::from("white"),
            },
            Shoe {
                size: 3,
                style: String::from("red"),
            },
            Shoe {
                size: 4,
                style: String::from("blue"),
            },
        ];

        let size_4_shoes = shoes_in_size_as_refs(&all_shoes, 4);

        assert_eq!(size_4_shoes, vec![&all_shoes[1], &all_shoes[3]]);
        // OR
        assert_eq!(size_4_shoes.len(), 2);
        assert_eq!(size_4_shoes[1].size, 4);
        assert_eq!(size_4_shoes[1].style, "white".to_string());
        assert_eq!(size_4_shoes[3].size, 4);
        assert_eq!(size_4_shoes[3].style, "blue".to_string());
    }
}
