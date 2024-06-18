pub fn storing_lists_of_values_with_vectors() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);

    // direct access returns value and panics if out of bounds, get return an Option<T> 

    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }


    // The code in Listing 8-6 might look like it should work: why should a reference to the first element care about changes at the end of the vector? This error is due to the way vectors work: because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.
    // let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0];

    // v.push(6);

    // println!("The first element is: {first}");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements. The for loop in Listing 8-8 will add 50 to each element.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("{v:?}");


}

pub fn storing_utf8_encoded_text_with_strings() {
    let mut s = String::new();

    let data = "initial contents";

    s = data.to_string();

    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("Hello, ");
    let s2 = "world!";
    // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    s1.push_str(s2);
    println!("s2 = {}", s2);


    let mut s = String::from("lo");
    // push takes a character
    s.push('l');


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used


    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");

    // ERROR
    // let s1 = String::from("hello");
    // let h = s1[0];

}

pub fn storing_keys_with_associated_values_in_hash_maps() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // For types that implement the Copy trait, like i32, the values are copied into the hash map. For owned values like String, the values will be moved and the hash map will be the owner of those values, as demonstrated in Listing 8-22.

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!


    // Overwriting a Value

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Adding a key only if it doesn't exist

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);


    // Update based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}

pub fn convert_to_pig_latin(input: &str) -> String {
    let vowels: &[char] = &['a', 'e', 'i', 'o', 'u'];
    let mut result = String::new();

    for word in input.split_whitespace() {
        let mut chars = word.chars();
        let first_char = chars.next().unwrap();

        if vowels.contains(&first_char.to_ascii_lowercase()) {
            result.push_str(word);
            result.push_str("-hay ");
        } else {
            let mut modified_word = chars.collect::<String>();
            modified_word.push(first_char);
            result.push_str(&modified_word);
            result.push_str("-ay ");
        }
    }

    println!("{}", result.trim().to_string());

    return result.trim().to_string();
}