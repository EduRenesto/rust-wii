// The bootstrap code.
// It simply calls the rust_main function,
// exported from our crate

extern int rust_main();

int main(int argc, char* argv[]) {
    return rust_main();
}
