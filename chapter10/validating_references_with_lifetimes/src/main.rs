fn main() {
    /* dangling refference
    {
        let r;                  // ---------+--'a
        {                       //          |
            let x = 5;          // --+'b    |
            r = &x;             //   |      |
        }                       // --+      |
        println!("r: {}", r);   //          |
    }*/                         // ---------+

    // The Borrow Cheacker
    {
        let x = 5;              // ---------+--'b
                                //          |
        let r = & x;            // --+--'a  |
                                //   |      |
        println!("r: {}", r);   //   |      |
                                // --+      |
    }                           // ---------+

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);


    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    /* error
    let string1 = String::from("long string is long");
    let result;

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */
}

fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'a> (x: &'a str, y: &str) -> &'a str {
    x
}


/* error
fn longest3<'a> (x: &str, y:&str) -> 'a str {
    let result = String::from("really long string");
    result.as_str()
}
*/
