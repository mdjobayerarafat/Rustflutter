use disesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("database.db").expect("Error connecting to database")
}

pub fn run_migrations() {
    use diesel_migrations::{run, MigrationHarness};
    let connection = establish_connection();
    let migration_harness = MigrationHarness::new((connection, "migrations"));
    run(&*migration_harness).unwarp();
}