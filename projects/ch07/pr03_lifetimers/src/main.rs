fn main() {
    let outer_scope;
    {
        let inner_scope = 5;
        outer_scope = &inner_scope;
    }
    println!("{outer_scope}");
}
