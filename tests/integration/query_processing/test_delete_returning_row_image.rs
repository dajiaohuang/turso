use crate::common::{ExecRows, TempDatabase};

#[turso_macros::test]
fn test_delete_returning_from_subquery_reads_deleted_row_image(
    tmp_db: TempDatabase,
) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE log(id INTEGER PRIMARY KEY, v INT)")?;
    for (sql, first, last) in [
        ("DELETE FROM log RETURNING id, (SELECT s.w FROM (SELECT log.v * 2 AS w) AS s)", 40, 0),
        ("DELETE FROM log RETURNING id, (WITH s AS (SELECT log.v * 2 AS w) SELECT w FROM s)", 40, 0),
        ("DELETE FROM log RETURNING id, (WITH s AS MATERIALIZED (SELECT log.v * 2 AS w) SELECT w FROM s)", 40, 0),
        ("DELETE FROM log RETURNING id, (SELECT s.w FROM (SELECT log.v * 2 AS w UNION ALL SELECT 99) AS s LIMIT 1)", 40, 0),
        ("DELETE FROM log RETURNING id, (SELECT log.v * 2)", 40, 0),
        ("UPDATE log SET v = v + 1 RETURNING id, (SELECT s.w FROM (SELECT log.v * 2 AS w) AS s)", 42, 2),
    ] {
        conn.execute("INSERT INTO log VALUES (1, 20), (2, NULL), (3, 0)")?;
        let mut stmt = conn.prepare(sql)?;
        let mut rows = Vec::new();
        stmt.run_with_row_callback(|row| {
            rows.push((row.get::<i64>(0)?, row.get::<&turso_core::Value>(1)?.clone()));
            Ok(())
        })?;
        assert_eq!(
            rows,
            vec![
                (1, turso_core::Value::from_i64(first)),
                (2, turso_core::Value::Null),
                (3, turso_core::Value::from_i64(last)),
            ],
            "{sql}"
        );
        conn.execute("DELETE FROM log")?;
    }
    conn.execute("INSERT INTO log VALUES (1, 20), (2, NULL), (3, 0)")?;
    let rows: Vec<(i64, i64)> = conn.exec_rows(
        "SELECT id, coalesce((SELECT s.w FROM (SELECT log.v * 2 AS w) AS s), -1) FROM log ORDER BY id",
    );
    assert_eq!(rows, vec![(1, 40), (2, -1), (3, 0)]);
    let rows: Vec<(i64,)> =
        conn.exec_rows("SELECT (SELECT s.w FROM (SELECT 7 AS w) AS s) FROM log ORDER BY id");
    assert_eq!(rows, vec![(7,), (7,), (7,)]);
    Ok(())
}
