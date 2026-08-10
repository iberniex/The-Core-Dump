#[derive(PartialEq, Debug)]
pub struct Shoes {
    size: u32,
    style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoes>, req_shoe_size: u32) -> Vec<Shoes> {
    shoes
        .into_iter()
        .filter(|x| x.size == req_shoe_size)
        .collect()
}
pub fn learning_iterators() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {val}");
    // }

    let total: i32 = v1_iter.sum();

    println!("The sum of {v1:?} is {total}");

    let list = vec![1, 2, 3];

    let iter_list = list.iter();

    let modified_list: Vec<_> = iter_list.map(|x| x + 1).collect();
    assert_eq!(modified_list, vec![2, 3, 4]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoes {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoes {
                size: 13,
                style: String::from("sandal"),
            },
            Shoes {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoes {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoes {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}
