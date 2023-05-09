fn main() {
    // the first variable will never change
    let never_changes: bool = false;
    // the second variable is flagged as mut, so it can change
    let mut this_changes: bool = true;
    println!("First value is {never_changes} and second value is {this_changes}");
    this_changes = false;
    println!("First value is now {never_changes} and second value is now {this_changes}");
}
