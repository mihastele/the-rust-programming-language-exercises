pub fn box_t_to_point_to_data_on_the_heap() {
    let b = Box::new(5);
    println!("b = {b}");
}

pub fn treating_smart_pointer_like_a_regular_references_with_deref_trait() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}