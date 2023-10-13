fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result = String::new();
    result.push_str(str1);
    result.push_str(str2);
    result
}

fn main() {
    // Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");

    // Call the concatenate_strings function with references to string1 and string2
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the result to the console
    println!("{}", concatenated_string);
}
