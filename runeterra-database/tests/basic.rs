use runeterra_database::db::Db;

#[test]
// makes sure database json paths are set up correctly
fn basic() {
    let db = Db::new();
    assert_ne!(db.collection.0.len(), 0);
}
