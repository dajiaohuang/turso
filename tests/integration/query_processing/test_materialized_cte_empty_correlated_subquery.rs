use crate::common::{ExecRows, TempDatabase};

#[turso_macros::test]
fn test_materialized_cte_is_opened_when_correlated_subquery_loop_is_empty(
    tmp_db: TempDatabase,
) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t0(c0)")?;

    let rows: Vec<(i64, i64)> = conn.exec_rows(
        "WITH x AS MATERIALIZED (SELECT 0 AS a) \
         SELECT a, COALESCE((SELECT (SELECT a FROM x WHERE a = t0.c0) FROM t0), -1) FROM x",
    );

    assert_eq!(rows, vec![(0, -1)]);
    Ok(())
}
