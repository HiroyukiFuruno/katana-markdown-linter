mod document;
mod protocol;
mod range;
mod server;

pub fn run_stdio() -> Result<(), String> {
    server::run_stdio()
}
