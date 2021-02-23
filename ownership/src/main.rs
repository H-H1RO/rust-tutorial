fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("s2 is {}", s2);

    // Strignオブジェクトの所有権がs1からs2になったため下のコードはエラーになる
    // println!("s1 is {}", s1);

    let s2_len = calculate_length(s2); // Stringオブジェクトの所有権がs2からcalculate_lengthに移動
    println!("s2 length is {}", s2_len);

    // このコードはエラーになる
    // println!("s2 is {}", s2);
}

fn calculate_length(s: String) -> usize {
    s.len()
}

/**
 * 所有権を取る代わりに参照を取って文字数を計算
 */
fn calculate_length_references(s: &String) -> usize {
    s.len()
}
