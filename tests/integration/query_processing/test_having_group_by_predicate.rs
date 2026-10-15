use crate::common::{ExecRows, TempDatabase};

#[turso_macros::test]
fn test_having_uses_grouped_predicate_value(tmp_db: TempDatabase) -> anyhow::Result<()> {
    let conn = tmp_db.connect_limbo();
    conn.execute("CREATE TABLE t(a INTEGER PRIMARY KEY, c TEXT)")?;
    conn.execute("INSERT INTO t VALUES (1, 'y'), (2, 'xy'), (3, 'z')")?;

    let rows: Vec<(String, String)> =
        conn.exec_rows("SELECT 'q1', c FROM t GROUP BY c, a IS NULL HAVING a IS NULL");
    assert!(rows.is_empty());

    let rows: Vec<(String, String)> =
        conn.exec_rows("SELECT 'q2', c FROM t GROUP BY c, a = 2 HAVING a = 2");
    assert_eq!(rows, vec![("q2".to_string(), "xy".to_string())]);

    let rows: Vec<(String, String)> =
        conn.exec_rows("SELECT 'q3', c FROM t GROUP BY c, a > 1 HAVING a > 1");
    assert_eq!(
        rows,
        vec![
            ("q3".to_string(), "xy".to_string()),
            ("q3".to_string(), "z".to_string()),
        ]
    );

    let rows: Vec<(String, String)> =
        conn.exec_rows("SELECT 'q4', c FROM t GROUP BY c, a IN (2, 3) HAVING a IN (2, 3)");
    assert_eq!(
        rows,
        vec![
            ("q4".to_string(), "xy".to_string()),
            ("q4".to_string(), "z".to_string()),
        ]
    );

    let rows: Vec<(String, String, i64)> =
        conn.exec_rows("SELECT 'q5', c, a > 1 FROM t GROUP BY c, a > 1 HAVING a > 1");
    assert_eq!(
        rows,
        vec![
            ("q5".to_string(), "xy".to_string(), 1),
            ("q5".to_string(), "z".to_string(), 1),
        ]
    );
    Ok(())
}
