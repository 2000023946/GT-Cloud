mod api;
mod applications;
mod bootstrap;
mod domain;
mod infrastructure;
mod ports;
mod observability;

fn main() {
    println!("Data plane started");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_runs_successfully() {
        main();
    }
}
