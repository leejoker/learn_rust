mod unsafes;

fn main() {
    unsafes::deref_value();
    unsafe {
        unsafes::dangerous();
    }
}
