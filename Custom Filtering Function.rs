// Define a struct called FilterCondition
struct FilterCondition<T> {
    value: T,
}

// Implement a method called is_match on the FilterCondition struct
impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        self.value == *item
    }
}

// Define a function called custom_filter
fn custom_filter<T>(collection: &[T], condition: &FilterCondition<T>) -> Vec<T>
where
    T: PartialEq + Clone,
{
    let mut result = Vec::new();

    for item in collection {
        if condition.is_match(item) {
            result.push(item.clone());
        }
    }

    result
}

fn main() {
    // Create a collection (vector) with some elements
    let original_collection = vec![1, 2, 3, 4, 5, 2, 6, 7, 8];

    // Initialize a FilterCondition object with the desired value
    let condition = FilterCondition { value: 2 };

    // Call the custom_filter function with the collection and the FilterCondition object
    let filtered_result = custom_filter(&original_collection, &condition);

    // Print the filtered result to the console
    println!("Original Collection: {:?}", original_collection);
    println!("Filtered Result: {:?}", filtered_result);
}
