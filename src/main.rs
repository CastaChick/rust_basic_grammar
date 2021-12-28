const Z: u32 = 100;

fn main() {
    variables_and_mutability();
    data_types();
    parameter_function(100);
    let five = return_five();
    println!("The value of five: {}", five);
    control_flow();
}

fn variables_and_mutability() {
    // let x = 5;
    // mutを付けると変数がmutableになる
    let mut x = 5;
    println!("The value of x: {}", x);
    x = 6;
    println!("The value of x: {}", x);

    // const Y = 5;
    // 定数の宣言には型を明示する事が必要
    const Y: u32 = 5;
    println!("The value of constant Y: {}", Y);
    // 定数はGlobal scopeで宣言可能
    println!("The value of global constant Z: {}", Z);

    // Shadowing
    let a = 5;
    let a = a + 1;

    {
        let a = a * 2;
        println!("The value of a in the inner scope is: {}", a); // a = 12
    }
    println!("The value of a: {}", a); // a = 6

    // mutによる再代入は同一の型である必要がある
    // 再宣言においては異なる型での変数宣言が可能
    // let mut spaces = "    ";
    // spaces = spaces.len();
    let spaces = "     ";
    let spaces = spaces.len();
    println!("The length of spaces: {}", spaces);
}

fn data_types() {
    // parse時には必ず型のアノテーションが必要
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("the value of guess: {}", guess);

    // Numeric operations
    let _sum = 5 + 10; // 15
    let _difference = 95.5 - 4.3; // 91.2
    let _product = 4 * 30; // 120
    let _quotient = 12.1 / 1.1; // 11
    let _floored = 2 / 3; // 0
    let _reminder = 43 % 5; // 3

    // Tuple
    let tup: (i32, i8, i64) = (10, 5, 20);
    let (x, y, z) = tup;
    let first_value = tup.0;
    println!("The first value of tup: {}", first_value);
    println!("The second value of tup: {}", tup.1);
    println!("The values of tup: {}, {}, {}", x, y, z);
    println!("The values of tup: {:?}", tup);
    
    // Array
    let arr: [i32; 5] = [0, 1, 2, 3, 4];
    println!("The values of arr: {:?}", arr);
    println!("The first value of arr: {}", arr[0]);
    let repeat_arr = [3; 5]; // [3, 3, 3, 3, 3]
    println!("The values of repeat_arr: {:?}", repeat_arr);
}

fn parameter_function(x: i32) {
    println!("The value of given parameter: {}", x);
}

fn return_five() -> i32 {
    // 式には行末のセミコロンをつけない
    5
}

fn control_flow() {
    let number = 3;
    println!("The values of number: {}", number);
    if number < 5 {
        println!("number < 5");
    } else if number == 5{
        println!("number == 5");
    } else {
        println!("number > 5");
    }
    
    let condition = true;
    // ifは式なので代入文の右辺に置く事ができる
    let number = if condition {
        5
    } else {
        // 全ての文において返ってくる値の型は同じである必要がある
        // "test"
        10
    }; // 代入文なのでセミコロンが必要
    println!("The value of number: {}", number);

    let mut i = 0;
    println!("Start loop");
    loop {
        i = i + 1;
        println!("{} times", i);
        if i >= 5 {
            break
        }
    }

    let mut i = 0;
    println!("Start while loop");
    while i < 5 {
        i = i + 1;
        println!("{} times", i);
    }

    let arr = [1, 2, 3, 4, 5];
    println!("Print each values of {:?}", arr);
    for element in arr.iter() {
        println!("The value of element: {}", element);
    }
    
    println!("Start for loop");
    for i in 1..6 {
        println!("{} times", i);
    }
}
