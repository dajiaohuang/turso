use std::sync::Arc;
use turso_core::{ConnectOpts, Database, MemoryIO, Value, IO};
use turso_sqlite::SqliteDialect;

fn main() -> anyhow::Result<()> {
    let io: Arc<dyn IO> = Arc::new(MemoryIO::new());
    let db = Database::open_file(io, ":memory:", Arc::new(SqliteDialect))?;
    let conn = db.connect(ConnectOpts::new())?;
    conn.execute("CREATE TABLE t(a INTEGER, c INTEGER)")?;
    conn.execute("INSERT INTO t VALUES(1, -9223372036854775808)")?;
    conn.execute("BEGIN")?;
    conn.execute("CREATE TABLE prior(x INTEGER)")?;
    let mut prepared = conn.prepare("SELECT x FROM prior")?;
    assert!(conn
        .execute("CREATE TABLE n AS SELECT a, abs(c) FROM t")
        .is_err());
    conn.execute("INSERT INTO prior VALUES(7)")?;
    assert_eq!(prepared.run_collect_rows()?, vec![vec![Value::from_i64(7)]]);
    conn.execute("COMMIT")?;
    prepared.reset()?;
    assert_eq!(prepared.run_collect_rows()?, vec![vec![Value::from_i64(7)]]);
    println!("prepared statement survived schema rollback and commit");
    Ok(())
}
