use crate::common::{limbo_exec_rows, TempDatabase};
use asserting::prelude::*;

#[turso_macros::test]
fn json_insert_does_not_replace_existing_document_root(tmp_db: TempDatabase) {
    let conn = tmp_db.connect_limbo();

    let result = limbo_exec_rows(
        &conn,
        r#"SELECT json_insert('[]', '$', 1),
                  json_insert('{"a":2}', '$', 1),
                  json_insert('[5]', '$', json('{"b":1}')),
                  json(jsonb_insert('[5]', '$', 1))"#,
    );
    assert_that!(result).is_equal_to(vec![row!["[]", r#"{"a":2}"#, "[5]", "[5]",]]);
}
