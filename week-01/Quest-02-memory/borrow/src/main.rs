use borrow::*;

fn main() {
	let s = "hello";
	let s1 = "camelCase".to_string();
	let s2 = "olá!";

	println!("\tstr_len(\"{}\") = {}", s, str_len(s));
	println!("\tstr_len(\"{}\") = {}", s1, str_len(&s1));
	println!("\tstr_len(\"{}\") = {}", s2, str_len(s2));
}

// $ cargo run
// str_len("hello") = 5
// str_len("camelCase") = 9
// str_len("olá!") = 4
// $
