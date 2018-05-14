# Exploring recursive trait resolution.

...for my own learning.

The source file implements a JSON Writer trait for a primitive type, vector, and a map.

We have a `write` method that is type constrained on having a JSON Writer implemnetation.

We then use a value made up of int, vect, and hashmap and the rust compiler conjurs up the right mix of implementations
to output the value.

Running the program...


```
$ cargo run
   Compiling derive v0.1.0 (file:///Users/richard/Developer/rust/derive)
    Finished dev [unoptimized + debuginfo] target(s) in 0.95 secs
     Running `target/debug/derive`
{ 'Team3' : [1, 2, 3],  'Team1' : [1, 2, 3],  'Team2' : [1, 2, 3]}
```

And if we try to `write` a vector of i64 values,
that won't work because we've not implemented JsonWriter for i64:

```
 Compiling derive v0.1.0 (file:///Users/richard/Developer/rust/derive)
error[E0277]: the trait bound `i64: JsonWriter` is not satisfied
  --> src/main.rs:60:16
   |
60 |     let json = write(scores);
   |                ^^^^^ the trait `JsonWriter` is not implemented for `i64`
   |
   = note: required because of the requirements on the impl of `JsonWriter` for `std::vec::Vec<i64>`
   = note: required because of the requirements on the impl of `JsonWriter` for `std::collections::HashMap<std::string::String, std::vec::Vec<i64>>`
note: required by `write`
  --> src/main.rs:15:1
   |
15 | fn write<T: JsonWriter>(t: T) -> String {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

```
