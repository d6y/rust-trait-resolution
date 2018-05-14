// We want to turn data structures into valid JSON values
trait JsonWriter {
    fn write(&self) -> String;
}

// We can implement this for basic types, such as...
impl JsonWriter for i32 {
    fn write(&self) -> String {
        self.to_string()
    }
}

// We can write a generic method that will turn anything that is
// JSON-writable into a String:
fn write<T: JsonWriter>(t: T) -> String {
    t.write()
}

// We can write instances for collections,
// providing the content of the collection is JSON-writable
// (NB: this isn't a great example as JSON colllections can contain mixed types
// Youd really want to represent JSON via an ADT first.)
impl<T: JsonWriter> JsonWriter for Vec<T> {
    fn write(&self) -> String {
        let content = self.iter()
            .map(|x| x.write())
            .collect::<Vec<String>>()
            .join(", ");
        format!("[{}]", content)
    }
}

// And likewise for a Map
use std::collections::HashMap;
impl<K: JsonWriter> JsonWriter for HashMap<String, K> {
    fn write(&self) -> String {
        let content = self.iter()
            .map(|(k, v)| format!(" '{}' : {}", k.to_string(), v.write()))
            .collect::<Vec<String>>()
            .join(", ");
        format!("{{{}}}", content)
    }
}

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3];

    // `scores` is a map containg a list of integers.
    // I think it's fair to say I do not yet understand borrowing etc
    let scores: HashMap<String, Vec<i32>> = [
        ("Team1".to_string(), numbers.clone()),
        ("Team2".to_string(), numbers.clone()),
        ("Team3".to_string(), numbers.clone()),
    ].iter()
        .cloned()
        .collect();

    // Using the three impl building blocks we can write
    // a value made up of Map, List and i32:
    let json = write(scores);
    println!("{}", json);
}
