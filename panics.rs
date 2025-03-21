fn panic_example() {
    let v = vec![1, 2, 3];
    let v = vec![4, 5, 6]; 

    v.into_iter().for_each(|x| {
        match x {
            1 => panic!("Don't want to see the number 1"),
            _ => println!("{}", x),
        }
    });
}
