use ascii_converter::{run, handle_error};

fn main() {
    if let Err(e) = run() {
        handle_error(e);
    }
}