use crate::common::{ExecRows, TempDatabase};

#[turso_macros::test]
fn test_window_minmax_non_deterministic_order_keeps_source_collation(
    tmp_db: TempDatabase,
) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t0(c0)")?;
    conn.execute("INSERT INTO t0 VALUES (1), (2)")?;

    let rows: Vec<(i64, i64)> = conn.exec_rows(
        "SELECT rowid, max(c0) OVER (ORDER BY strftime(NULL) ROWS CURRENT ROW) \
         FROM t0 ORDER BY 1, 2",
    );
    assert_eq!(rows, vec![(1, 1), (2, 2)]);

    let rows: Vec<(i64, i64)> = conn.exec_rows(
        "SELECT rowid, min(c0) OVER (ORDER BY strftime(NULL) ROWS CURRENT ROW) \
         FROM t0 ORDER BY 1, 2",
    );
    assert_eq!(rows, vec![(1, 1), (2, 2)]);
    Ok(())
}
