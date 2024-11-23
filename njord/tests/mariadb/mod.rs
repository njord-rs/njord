mod delete_test;
mod insert_test;
mod open_test;
mod select_joins_test;
mod select_test;
mod update_test;

use std::fs;
use lazy_static::lazy_static;
use log::{error, info};
use mysql::Pool;
use mysql::prelude::Queryable;
use testcontainers::{Container, ContainerAsync, ContainerRequest, ImageExt};
use testcontainers::core::ContainerPort;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::mariadb::Mariadb;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use njord::keys::AutoIncrementPrimaryKey;
use njord::table::Table;
use njord_derive::Table;

#[derive(Table, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: AutoIncrementPrimaryKey<usize>,
    pub username: String,
    pub email: String,
    pub address: String,
}

#[derive(Table, Clone)]
#[table_name = "users"]
pub struct UserWithSubQuery {
    pub id: AutoIncrementPrimaryKey<usize>,
    pub username: String,
    pub email: String,
    pub address: String,
    pub additional_address: String,
}

#[derive(Table, Clone)]
#[table_name = "categories"]
pub struct Category {
    pub id: AutoIncrementPrimaryKey<usize>,
    pub name: String,
}

#[derive(Table, Clone)]
#[table_name = "products"]
pub struct Product {
    pub id: AutoIncrementPrimaryKey<usize>,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock_quantity: usize,
    // pub category: Category, // one-to-one relationship
    pub category_id: usize,
    pub discount: f64,
}

#[derive(Table)]
#[table_name = "users"]
pub struct UsersWithJoin {
    username: String,
    price: f64,
    name: String,
}

#[derive(Table)]
#[table_name = "categories"]
pub struct CategoryWithJoin {
    name: String,
    description: String,
    price: f64,
    stock_quantity: usize,
    discount: f64,
    category_name: String,
}

pub struct MariadbContainer {
    container: ContainerAsync<Mariadb>,
}

impl MariadbContainer {
    pub async fn new() -> Self {
        let container = Mariadb::default()
            .with_tag("11")
            .with_env_var("MARIADB_ROOT_PASSWORD", "njord_rootpwd")
            .with_env_var("MARIADB_DATABASE", "njord_db")
            .with_env_var("MARIADB_USER", "njord_user")
            .with_env_var("MARIADB_PASSWORD", "njord_password")
            .with_mapped_port(3307, ContainerPort::Tcp(3306)).start().await.unwrap();
        MariadbContainer { container }
    }
}

impl Drop for MariadbContainer {
    fn drop(&mut self) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let _ = self.container.stop().await.is_ok();
        });
    }
}

lazy_static! {
    static ref MARIADB_CONTAINER: Mutex<Option<MariadbContainer>> = Mutex::new(None);
}

pub fn setup_mariadb() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let container = MariadbContainer::new().await;
        *MARIADB_CONTAINER.lock().await = Some(container);
    });
}

pub fn teardown_mariadb() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        if let Some(container) = MARIADB_CONTAINER.lock().await.take() {
            drop(container);
        }
    });
}

pub fn create_schema() {
    let url = "mysql://njord_user:njord_password@localhost:3307/njord_db";
    let pool = match Pool::new(url) {
        Ok(pool) => pool,
        Err(e) => {
            error!("Failed to create pool: {:?}", e);
            return;
        }
    };
    let mut conn = match pool.get_conn() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get connection: {:?}", e);
            return;
        }
    };
    let schema_sql = match fs::read_to_string("../../db/test/mariadb.sql") {
        Ok(sql) => sql,
        Err(e) => {
            error!("Failed to read schema file: {:?}", e);
            return;
        }
    };
    if let Err(e) = conn.query_drop(schema_sql) {
        error!("Failed to execute schema SQL: {:?}", e);
    } else {
        info!("Schema created successfully");
    }
}

fn main() {
    env_logger::init();
    setup_mariadb();
    create_schema();
    let result = std::panic::catch_unwind(|| {
        // Run all tests
        let args: Vec<String> = std::env::args().collect();
        let test_bin = &args[1];
        let status = std::process::Command::new(test_bin)
            .args(&args[2..])
            .status()
            .expect("Failed to run tests");
        assert!(status.success());
    });
    teardown_mariadb();
    if let Err(err) = result {
        std::panic::resume_unwind(err);
    }
}

// pub fn create_schema()
// pub fn populate_database()