const Z: u32 = 100;
fn main() {
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
