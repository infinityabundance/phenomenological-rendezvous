fn main() {
    if let Err(err) = phenomenological_rendezvous::cli::run() {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
