pub fn common_collections() {
    //? unlike arrays and tuples this data is stored on the heap

    //vector(unknown length) != array (known length)

    println!("Vec<T>");

    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v1 = Vec::new();

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        println!("The third element is {}", third);

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element."),
        }
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

pub fn strings() {
    let mut s = String::new();

    //? to string cam be used on any type that implements Display

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    //concat string slice toa String
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // push takes as a parameter a char

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // println!("s1 = {}", s1)
    println!("s3 = {}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);

    let s1 = String::from("hello");
    // let h = s1[0];

    let hello = "Здравствуйте";

    let s = &hello[0..4]; //? this works

    let s = &hello[0..3]; // this will panic

    println!("hello = {}", s);

    for c in "Зд".chars() {
        println!("{}", c);
    }

    for b in "Зд".bytes() {
        println!("{}", b);
    }
}

use std::collections::HashMap;

pub fn hash_maps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); //? both these values get moved into the map
                                         // field_name and field_value are invalid at this point, try using them and
                                         // see what compiler error you get!

    // println!("field_name = {}", field_name);
    // println!("field_value = {}", field_value);

    //TODO updating a hashmap

    // you need to choose between:

    //? overwriting the value

    {
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores);
    }

    //? adding a key and a value if the key is not present

    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }

    //? updating a value based on th old value

    {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
        //iterating through a hashmap happens arbitrarily
    }

    //TODO implement the proposed problems
}
