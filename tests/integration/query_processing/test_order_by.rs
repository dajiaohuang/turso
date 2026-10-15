use crate::common::{ExecRows, TempDatabase};

#[turso_macros::test]
fn test_large_integer_literals_remain_distinct_from_real_literals(
    tmp_db: TempDatabase,
) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t(x)")?;
    conn.execute("INSERT INTO t VALUES (1)")?;

    let rows: Vec<(i64, String)> = conn.exec_rows(
        "SELECT -9223372036854775808, typeof(-9223372036854775808) \
         FROM t ORDER BY -9.223372036854776e18",
    );
    assert_eq!(rows, vec![(i64::MIN, "integer".to_string())]);

    let rows: Vec<(f64, String)> = conn.exec_rows(
        "SELECT -9.223372036854776e18, typeof(-9.223372036854776e18) \
         FROM t ORDER BY -9223372036854775808",
    );
    assert_eq!(rows, vec![(-9.223372036854776e18, "real".to_string())]);

    let rows: Vec<(i64, String)> = conn.exec_rows(
        "SELECT -9223372036854775808, typeof(-9223372036854775808) \
         FROM t GROUP BY -9.223372036854776e18",
    );
    assert_eq!(rows, vec![(i64::MIN, "integer".to_string())]);

    let rows: Vec<(f64, String)> = conn.exec_rows(
        "SELECT -9.223372036854776e18, typeof(-9.223372036854776e18) \
         FROM t GROUP BY -9223372036854775808",
    );
    assert_eq!(rows, vec![(-9.223372036854776e18, "real".to_string())]);
    Ok(())
}
