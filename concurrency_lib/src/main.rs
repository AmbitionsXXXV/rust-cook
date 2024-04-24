use threads::{create_concurrent_pipelines, generate_short_term_threads};

mod threads;

fn main() {
    generate_short_term_threads();

    create_concurrent_pipelines();
}
