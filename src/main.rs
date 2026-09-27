/*
         ╭─────────────────────────────────────────────────────────╮
         │                   CONCRETE LIFETIMES                    │
         ╰─────────────────────────────────────────────────────────╯
*/

fn main() {
    /*
    {
        let i = 5;
    }
    let j = i;
    println!("i: {i}")
    */

    /*
        let str1 = String::from("Hello");
        str_fn(str1);
        let str2 = str1; // WARN: str1 is moved to str2, str1 is no longer valid
        // println!("str1: {str1}");
    */

    /*
        let i;
        {
            let j = 5;
            i = &j;
            println!("i inside block: {i}");
        }
        // println!("i: {i}"); // WARN: j is dropped, i is dangling reference
    */

    // We can have multiple immutable references to a value, but only one mutable reference at a time.
    // This is because mutable references allow us to change the value, and if we had multiple mutable
    // references, we could end up with data races.
    let mut vec1 = vec![6, 5, 8, 9];
    let ref1 = &vec1;
    println!("ref1: {ref1:?}");
    // WARN: Use of non lexical lifetimes (NLL) allows us to have a mutable reference after an immutable
    // WARN: reference is no longer used.
    let ref2 = &mut vec1;
    ref2.push(10);
    println!("ref2: {ref2:?}");
}
/*
    fn str_fn(s: String) {
        println!("s: {s}");
    }
*/
