use njord::sqlite::open;

pub fn setup_sqlite() {
    open("test_database.db");
}
