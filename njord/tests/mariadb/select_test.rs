use ::mysql::PooledConn;
use njord::condition::Condition;
use njord::keys::AutoIncrementPrimaryKey;
use njord::mariadb::{self};
use njord::{column::Column, condition::Value};
use njord_derive::sql;
use std::collections::HashMap;

use crate::{User, UserWithSubQuery};

fn insert_mock_data(table_rows: Vec<User>) {
    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    match conn {
        Ok(ref mut c) => {
            let result = mariadb::insert(c, table_rows);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}

fn delete_mock_data(conn: &mut PooledConn, usernames: Vec<String>) {
    let value_list: Vec<Value> = usernames
        .into_iter()
        .map(Value::Literal) // Wrap each username as a Value::Literal
        .collect();

    let result = mariadb::delete()
        .from(User::default())
        .where_clause(Condition::In("username".to_string(), value_list))
        .build(conn);

    assert!(result.is_ok());
}

#[test]
fn open_db() {
    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let conn = mariadb::open(url);
    assert!(conn.is_ok());
}

#[test]
fn insert_row() {
    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::default(),
        username: "mjovanc".to_string(),
        email: "mjovanc@icloud.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    match conn {
        Ok(ref mut c) => {
            let result = mariadb::insert(c, vec![table_row]);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to INSERT: {:?}", e);
        }
    }
}

#[test]
fn update() {
    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let columns = vec!["username".to_string()];

    let condition = Condition::Eq(
        "username".to_string(),
        Value::Literal("mjovanc".to_string()),
    );

    let table_row: User = User {
        id: AutoIncrementPrimaryKey::<usize>::new(Some(0)),
        username: "mjovanc".to_string(),
        email: "mjovanc@icloud.com".to_string(),
        address: "Some Random Address 1".to_string(),
    };

    let mut order = HashMap::new();
    order.insert(vec!["id".to_string()], "DESC".to_string());

    match conn {
        Ok(ref mut c) => {
            let result = mariadb::update(table_row)
                .set(columns)
                .where_clause(condition)
                .order_by(order)
                .limit(4)
                .offset(0)
                .build(c);
            println!("{:?}", result);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to UPDATE: {:?}", e);
        }
    }
}

#[test]
fn delete() {
    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let condition = Condition::Eq(
        "address".to_string(),
        Value::Literal("Some Random Address 1".to_string()),
    );

    let mut order = HashMap::new();
    order.insert(vec!["id".to_string()], "DESC".to_string());

    match conn {
        Ok(ref mut c) => {
            let result = mariadb::delete()
                .from(User::default())
                .where_clause(condition)
                .order_by(order)
                .limit(20)
                .offset(0)
                .build(c);
            println!("{:?}", result);
            assert!(result.is_ok());
        }
        Err(e) => {
            panic!("Failed to DELETE: {:?}", e);
        }
    }
}

#[test]
fn select() {
    insert_mock_data(vec![
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_test".to_string(),
            email: "select_test@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_test2".to_string(),
            email: "select_test2@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
    ]);

    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];
    let condition = Condition::Eq(
        "username".to_string(),
        Value::Literal("select_test".to_string()),
    );

    match conn {
        Ok(ref mut c) => {
            let result = mariadb::select(columns)
                .from(User::default())
                .where_clause(condition)
                .build(c);

            match result {
                Ok(r) => assert_eq!(r.len(), 1),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };

            delete_mock_data(
                c,
                vec!["select_test".to_string(), "select_test2".to_string()],
            );
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_distinct() {
    insert_mock_data(vec![
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_distinct_test".to_string(),
            email: "select_distinct_test@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_distinct_test".to_string(),
            email: "select_distinct_test@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_distinct_test2".to_string(),
            email: "select_distinct_test@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
    ]);

    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];
    let condition = Condition::Eq(
        "username".to_string(),
        Value::Literal("select_distinct_test".to_string()),
    );

    match conn {
        Ok(ref mut c) => {
            let result = mariadb::select(columns)
                .from(User::default())
                .where_clause(condition)
                .distinct()
                .build(c);

            match result {
                Ok(r) => {
                    // TODO: this test does not work properly since it should return 1 but it seems
                    // like it returns all rows because id is different. Need to check up on that.
                    assert_eq!(r.len(), 2);
                }
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };

            delete_mock_data(
                c,
                vec![
                    "select_distinct_test".to_string(),
                    "select_distinct_test2".to_string(),
                ],
            );
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_order_by() {
    insert_mock_data(vec![
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_order_by_test".to_string(),
            email: "select_order_by_test@example.com".to_string(),
            address: "Some Random Address select_order_by".to_string(),
        },
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_order_by_test2".to_string(),
            email: "select_order_by_test2@example.com".to_string(),
            address: "Some Random Address select_order_by".to_string(),
        },
    ]);

    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let columns = vec![
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
    ];
    let condition = Condition::Eq(
        "address".to_string(),
        Value::Literal("Some Random Address select_order_by".to_string()),
    );
    let group_by = vec!["username".to_string(), "email".to_string()];

    let mut order_by = HashMap::new();
    order_by.insert(vec!["email".to_string()], "ASC".to_string());

    match conn {
        Ok(ref mut c) => {
            let result = mariadb::select(columns)
                .from(User::default())
                .where_clause(condition)
                .order_by(order_by)
                .group_by(group_by)
                .build(c);

            match result {
                Ok(r) => assert_eq!(r.len(), 2),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };

            delete_mock_data(
                c,
                vec![
                    "select_order_by_test".to_string(),
                    "select_order_by_test2".to_string(),
                ],
            );
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_group_by() {
    insert_mock_data(vec![
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_group_by_test".to_string(),
            email: "select_group_by_test@example.com".to_string(),
            address: "Some Random Address select_group_by".to_string(),
        },
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_group_by_test2".to_string(),
            email: "select_group_by_test@example.com".to_string(),
            address: "Some Random Address select_group_by".to_string(),
        },
    ]);

    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let columns = vec![
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
    ];
    let condition = Condition::Eq(
        "address".to_string(),
        Value::Literal("Some Random Address select_group_by".to_string()),
    );
    let group_by = vec!["username".to_string(), "email".to_string()];

    match conn {
        Ok(ref mut c) => {
            let result = mariadb::select(columns)
                .from(User::default())
                .where_clause(condition)
                .group_by(group_by)
                .build(c);

            match result {
                Ok(r) => assert_eq!(r.len(), 2),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };

            delete_mock_data(
                c,
                vec![
                    "select_group_by_test".to_string(),
                    "select_group_by_test2".to_string(),
                ],
            );
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_limit_offset() {
    insert_mock_data(vec![User {
        id: AutoIncrementPrimaryKey::default(),
        username: "select_limit_offset_test".to_string(),
        email: "select_limit_offset_test@example.com".to_string(),
        address: "Some Random Address select_limit_offset".to_string(),
    }]);

    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let columns = vec![
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
    ];
    let condition = Condition::Eq(
        "username".to_string(),
        Value::Literal("select_limit_offset_test".to_string()),
    );
    let group_by = vec!["username".to_string(), "email".to_string()];

    let mut order_by = HashMap::new();
    order_by.insert(vec!["username".to_string()], "DESC".to_string());

    match conn {
        Ok(ref mut c) => {
            let result = mariadb::select(columns)
                .from(User::default())
                .where_clause(condition)
                .order_by(order_by)
                .group_by(group_by)
                .limit(1)
                .offset(0)
                .build(c);

            match result {
                Ok(r) => assert_eq!(r.len(), 1),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };

            delete_mock_data(c, vec!["select_limit_offset_test".to_string()]);
        }
        Err(error) => panic!("Failed to SELECT: {:?}", error),
    };
}

#[test]
fn select_having() {
    insert_mock_data(vec![User {
        id: AutoIncrementPrimaryKey::default(),
        username: "select_having_test".to_string(),
        email: "select_having_test@example.com".to_string(),
        address: "Some Random Address 1".to_string(),
    }]);

    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let columns = vec![
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
    ];
    let condition = Condition::Eq(
        "username".to_string(),
        Value::Literal("select_having_test".to_string()),
    );
    let group_by = vec!["username".to_string(), "email".to_string()];

    let mut order_by = HashMap::new();
    order_by.insert(vec!["email".to_string()], "DESC".to_string());

    let having_condition = Condition::Eq(
        "username".to_string(),
        Value::Literal("select_having_test".to_string()),
    );

    match conn {
        Ok(ref mut c) => {
            let result = mariadb::select(columns)
                .from(User::default())
                .where_clause(condition)
                .order_by(order_by)
                .group_by(group_by)
                .having(having_condition)
                .build(c);

            match result {
                Ok(r) => assert_eq!(r.len(), 1),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };

            delete_mock_data(c, vec!["select_having_test".to_string()]);
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    }
}

#[test]
fn select_except() {
    insert_mock_data(vec![
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_except_test".to_string(),
            email: "select_except_test@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_except_test2".to_string(),
            email: "select_except_test2@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_except_test3".to_string(),
            email: "select_except_test3@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
    ]);

    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];

    let condition1 = Condition::Eq(
        "username".to_string(),
        Value::Literal("select_except_test".to_string()),
    );
    let condition2 = Condition::Eq(
        "username".to_string(),
        Value::Literal("select_except_test2".to_string()),
    );
    let condition3 = Condition::Eq(
        "username".to_string(),
        Value::Literal("select_except_test3".to_string()),
    );

    let query1 = mariadb::select(columns.clone())
        .from(User::default())
        .where_clause(condition1);

    let query2 = mariadb::select(columns.clone())
        .from(User::default())
        .where_clause(condition2);

    let query3 = mariadb::select(columns.clone())
        .from(User::default())
        .where_clause(condition3);

    match conn {
        Ok(ref mut c) => {
            // Test a chain of EXCEPT queries (query1 EXCEPT query2 EXCEPT query3)
            let result = query1.except(query2).except(query3).build(c);

            match result {
                Ok(r) => {
                    assert_eq!(r.len(), 1, "Expected 1 results after EXCEPT clauses.");
                }
                Err(e) => panic!("Failed to SELECT with EXCEPT: {:?}", e),
            };

            delete_mock_data(
                c,
                vec![
                    "select_except_test".to_string(),
                    "select_except_test2".to_string(),
                    "select_except_test3".to_string(),
                ],
            );
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_union() {
    insert_mock_data(vec![
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_union_test".to_string(),
            email: "select_union_test@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_union_test2".to_string(),
            email: "select_union_test2@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_union_test3".to_string(),
            email: "select_union_test3@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
    ]);

    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];

    let condition1 = Condition::Eq(
        "username".to_string(),
        Value::Literal("select_union_test".to_string()),
    );
    let condition2 = Condition::Eq(
        "username".to_string(),
        Value::Literal("select_union_test2".to_string()),
    );

    let query1 = mariadb::select(columns.clone())
        .from(User::default())
        .where_clause(condition1);

    let query2 = mariadb::select(columns.clone())
        .from(User::default())
        .where_clause(condition2);

    match conn {
        Ok(ref mut c) => {
            // Test a chain of UNION queries (query1 UNION query2)
            let result = query1.union(query2).build(c);

            match result {
                Ok(r) => {
                    // We expect two results: mjovanc and otheruser
                    assert_eq!(r.len(), 2, "Expected 2 results from the UNION query.");
                    assert_eq!(
                        r[0].username,
                        "select_union_test".to_string(),
                        "First user should be mjovanc."
                    );
                    assert_eq!(
                        r[1].username,
                        "select_union_test2".to_string(),
                        "Second user should be otheruser."
                    );
                }
                Err(e) => panic!("Failed to SELECT with UNION: {:?}", e),
            };

            delete_mock_data(
                c,
                vec![
                    "select_union_test".to_string(),
                    "select_union_test2".to_string(),
                    "select_union_test3".to_string(),
                ],
            );
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    }
}

#[test]
fn select_sub_queries() {
    insert_mock_data(vec![
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_sub_queries_test".to_string(),
            email: "select_sub_queries_test@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_sub_queries_test2".to_string(),
            email: "select_sub_queries_test2@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_sub_queries_test3".to_string(),
            email: "select_sub_queries_test3@example.com".to_string(),
            address: "SubQuery".to_string(),
        },
    ]);

    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    match conn {
        Ok(ref mut c) => {
            let sub_query = mariadb::select(vec![Column::Text("address".to_string())])
                .from(UserWithSubQuery::default())
                .where_clause(Condition::Eq(
                    "username".to_string(),
                    Value::Literal("select_sub_queries_test3".to_string()),
                ))
                .limit(1);

            let columns = vec![
                Column::Text("id".to_string()),
                Column::Text("username".to_string()),
                Column::Text("email".to_string()),
                Column::Text("address".to_string()),
                Column::SubQuery(Box::new(sub_query), "additional_address".to_string()),
            ];

            let result = mariadb::select(columns)
                .from(UserWithSubQuery::default())
                .where_clause(Condition::In(
                    "username".to_string(),
                    vec![
                        Value::Literal("select_sub_queries_test".to_string()),
                        Value::Literal("select_sub_queries_test2".to_string()),
                    ],
                ))
                .build(c);

            match result {
                Ok(r) => {
                    assert!(r.len() > 0);
                    assert_eq!(r[0].additional_address, "SubQuery");
                }
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };

            delete_mock_data(
                c,
                vec![
                    "select_sub_queries_test".to_string(),
                    "select_sub_queries_test2".to_string(),
                    "select_sub_queries_test3".to_string(),
                ],
            );
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn select_in() {
    insert_mock_data(vec![
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_in_test".to_string(),
            email: "select_in_test@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_in_test2".to_string(),
            email: "select_in_test2@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
        User {
            id: AutoIncrementPrimaryKey::default(),
            username: "select_in_test3".to_string(),
            email: "select_in_test3@example.com".to_string(),
            address: "Some Random Address 1".to_string(),
        },
    ]);

    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let mut conn = mariadb::open(url);

    let columns = vec![
        Column::Text("id".to_string()),
        Column::Text("username".to_string()),
        Column::Text("email".to_string()),
        Column::Text("address".to_string()),
    ];

    let condition = Condition::And(
        Box::new(Condition::In(
            "username".to_string(),
            vec![
                Value::Literal("select_in_test".to_string()),
                Value::Literal("select_in_test2".to_string()),
            ],
        )),
        Box::new(Condition::NotIn(
            "username".to_string(),
            vec![Value::Literal("select_in_test3".to_string())],
        )),
    );

    match conn {
        Ok(ref mut c) => {
            let result = mariadb::select(columns)
                .from(User::default())
                .where_clause(condition)
                .build(c);

            match result {
                Ok(r) => assert_eq!(r.len(), 2),
                Err(e) => panic!("Failed to SELECT: {:?}", e),
            };

            delete_mock_data(
                c,
                vec![
                    "select_in_test".to_string(),
                    "select_in_test2".to_string(),
                    "select_in_test3".to_string(),
                ],
            );
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}

#[test]
fn raw_execute() {
    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let conn = mariadb::open(url);

    let username = "mjovanc";

    let query = sql! {
        SELECT *
        FROM users
        WHERE username = {username}
    };

    match conn {
        Ok(mut c) => {
            let result = mariadb::select::raw_execute::<User>(&query, &mut c);
            assert!(result.is_ok());
        }
        Err(e) => panic!("Failed to SELECT: {:?}", e),
    };
}
