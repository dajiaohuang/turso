use std::sync::Arc;
use turso_core::{ConnectOpts, Database, MemoryIO, IO};
use turso_sqlite::SqliteDialect;

fn main() -> anyhow::Result<()> {
    for explicit in [false, true] {
        let io: Arc<dyn IO> = Arc::new(MemoryIO::new());
        let db = Database::open_file(io, ":memory:", Arc::new(SqliteDialect))?;
        let conn = db.connect(ConnectOpts::new())?;
        conn.execute("CREATE TABLE t(v)")?;
        conn.execute("CREATE TABLE src(n)")?;
        conn.execute("INSERT INTO src VALUES(1),(2)")?;
        if explicit { conn.execute("BEGIN")?; }
        let result = conn.execute("INSERT OR FAIL INTO t SELECT CASE WHEN n=1 THEN 1 ELSE 'abc' LIKE 'a%' ESCAPE 'xx' END FROM src");
        let rows = conn.prepare("SELECT v FROM t")?.run_collect_rows()?;
        println!("explicit={explicit} error={result:?} rows={rows:?}");
        if explicit { conn.execute("COMMIT")?; }
    }
    let io: Arc<dyn IO> = Arc::new(MemoryIO::new());
    let db = Database::open_file(io, ":memory:", Arc::new(SqliteDialect))?;
    let conn = db.connect(ConnectOpts::new())?;
    conn.execute("CREATE TABLE t(v CHECK(v != 2))")?;
    conn.execute("CREATE TABLE src(n)")?;
    conn.execute("INSERT INTO src VALUES(1),(2)")?;
    let result = conn.execute("INSERT OR FAIL INTO t SELECT n FROM src");
    let rows = conn.prepare("SELECT v FROM t")?.run_collect_rows()?;
    println!("control error={result:?} rows={rows:?}");
    let io: Arc<dyn IO> = Arc::new(MemoryIO::new());
    let db = Database::open_file(io, ":memory:", Arc::new(SqliteDialect))?;
    let conn = db.connect(ConnectOpts::new())?;
    conn.execute("CREATE TABLE t(v)")?;
    conn.execute("CREATE TRIGGER tr AFTER INSERT ON t WHEN NEW.v=2 BEGIN SELECT 'abc' LIKE 'a%' ESCAPE 'xx'; END")?;
    conn.execute("BEGIN")?;
    let result = conn.execute("INSERT OR FAIL INTO t VALUES(1),(2)");
    let rows = conn.prepare("SELECT v FROM t")?.run_collect_rows()?;
    println!("trigger explicit error={result:?} rows={rows:?}");
    conn.execute("COMMIT")?;
    Ok(())
}
