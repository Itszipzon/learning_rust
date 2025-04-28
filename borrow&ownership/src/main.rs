mod test;

use test::{CloneableTestStruct, CopyableTestStruct};

fn main() {
    let x = String::from("Hello, world!");
    let y = x.clone();

    println!("x: {}", x);
    println!("y: {}", y);

    let a = 1;
    let b = a + 5;
    println!("a: {}", a);
    println!("b: {}", b);

    let test1 = CloneableTestStruct::new(1, "field2".to_string());
    test1.display();
    let test2 = test1.clone(); //Clones the test1 object. Ownership of test1 is not moved
    test1.display();
    test2.display();

    let test3 = CopyableTestStruct::new(5, "field6");
    test3.display();
    let test4 = test3; //CopyableTestStruct is a copy type. Ownership of test3 is not moved because of it, but the value is copied
    test3.display();
    test4.display();

    {
        let test5 = CloneableTestStruct::new(7, "field8".to_string());
        test5.display();
        let test6 = test5.clone();
        test5.display();
        test6.display();
    } // test 5 and 6 goes out of scope. Freeing memory

    {
        let test7 = CopyableTestStruct::new(9, "field10");
        test7.display();
        let test8 = test7;
        test7.display();
        test8.display();
    } // test 7 and 8 goes out of scope.
}
