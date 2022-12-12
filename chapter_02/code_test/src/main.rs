fn add_i32(x: i32, y: i32) -> i32 {
    x + y
}

#[test]

fn test1() {
    assert_eq!(add_i32(1, 2), 3); //成功
}

#[test]

fn test2() {
    assert_eq!(add_i32(2, 4), 7); //失敗
}

fn main() {
    println!("{}", add_i32(2, 5));
}

//#[]の形でコード内に置かれるものは「属性」と呼ばれ、何らかの付加情報を与えるもの。
//#[test]が付いている関数は、テストの際に実行する関数であり、cargo buildで作成される実行モジュールにはその関数を含めないことを示している。
