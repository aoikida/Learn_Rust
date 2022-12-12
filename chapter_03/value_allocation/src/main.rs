fn main() {

    //RustはC言語のように、変数型を宣言の際にしているする必要がない。
    //「型推論」と呼ばれる機能によって、変数型をコンパイラが推定してくれる。
    
    let mut x = 1;

    x = x + 1;

    println!("x = {}", x);

}

//関数型言語では、変数に値を割り当てることを、束縛と呼ぶ。
//代入 : 変数名がついた空の箱をメモリに用意し、そこに値を入れる。主役は「変数」であり、そこに「値」を割り当てる
//束縛 : 値を入れた箱をメモリに用意し、その値を指す変数を設定する。主役は「値」であり、その「値」を指す「変数」を割り当てる。