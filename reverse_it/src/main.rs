fn main() {
    println!("{}", reverse_it(123));
    println!("{}", reverse_it(-123));
}

pub fn reverse_it(v: i32) -> String {
    let is_negative = v < 0;
    let num_str = v.abs().to_string();
    let reverse_str: String = num_str.chars().rev().collect();

    let mut result = String::new();
    if is_negative{
        result.push('-');
    }
    result.push_str(&reverse_str);
    result.push_str(&v.abs().to_string());

    result

}