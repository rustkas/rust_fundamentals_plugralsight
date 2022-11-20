fn main() {
    let scope_test = "outer scope";
    println!("{}", scope_test);
    {
        let scope_test = "inner_scope";
        println!("{scope_test}");
    }
    println!("{scope_test}");
}
