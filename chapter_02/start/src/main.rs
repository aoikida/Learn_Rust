// fn 関数名(引数 : 引数の型) -> 返り値で関数を宣言
fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    let ans = if y == 0 {
        None
    } else {
        Some(x / y)
    }; //セミコロンがついていると、文(返り値を持たない)となる。
    ans
}

fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    if y == 0 {
        Err("div by zero")
    } else {
        Ok(x / y)
    }
}
 // Option型は、値がないということがあり得ることを示す型である。
fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans {
        println!("{}", x)
    } else {
        println!("None")
    }
}
// 引数や返却値の型に型パラメータを使う場合には、関数名と引数の間に型パラメータを含むことを示す宣言を<>で囲んで示す。
fn func_ex_print_some_match<T: std::fmt::Display>(ans: Option<T>) {
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None")
    }
}

fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(res) => println!("{}", res),
        Err(str) => println!("{}", str)
    }
}

//メイン関数
fn main() {
    //関数の呼び出し

    func_ex_print_some(func_ex_div_some(10, 5));

    func_ex_print_some(func_ex_div_some(10, 0));

    func_ex_print_some_match(func_ex_div_some(10, 5));

    func_ex_print_some_match(func_ex_div_some(10, 0));

    func_ex_print_result(func_ex_div_result(10, 5));

    func_ex_print_result(func_ex_div_result(10, 0));


}




