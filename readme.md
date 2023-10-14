# GrapeForum

**A sweet and simple forum platform for everyone. (Early WIP)**

### Getting Started

Although I plan to make a more streamlined approach to running the software at some point, for now, it must be run manually.

Ensure you have the Rust Compiler installed as well as a running PostgreSQL database.\
In the root directory of the project, create a file called `.env` and create a variable called `DATABASE_URL`, and name it your DB url.\
For table creation, follow what is found in `in.sql`.\
Navigate to the root directory of the project and run `cargo run --release`.