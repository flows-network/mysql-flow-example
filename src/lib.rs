use mysql_async::{prelude::*, Opts, OptsBuilder, Pool, PoolConstraints, PoolOpts};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

// flows
use flowsnet_platform_sdk::logger;
use webhook_flows::{create_endpoint, request_handler, send_response};

#[derive(Debug, Deserialize, Serialize)]
struct Order {
    order_id: i32,
    production_id: i32,
    quantity: i32,
    amount: f32,
    shipping: f32,
    tax: f32,
    shipping_address: String,
}

impl Order {
    fn new(
        order_id: i32,
        production_id: i32,
        quantity: i32,
        amount: f32,
        shipping: f32,
        tax: f32,
        shipping_address: String,
    ) -> Self {
        Self {
            order_id,
            production_id,
            quantity,
            amount,
            shipping,
            tax,
            shipping_address,
        }
    }
}

fn get_url() -> String {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        let opts = Opts::from_url(&url).expect("DATABASE_URL invalid");
        if opts
            .db_name()
            .expect("a database name is required")
            .is_empty()
        {
            panic!("database name is empty");
        }
        url
    } else {
        "mysql://root:pass@127.0.0.1:3306/mysql".into()
    }
}

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn on_deploy() {
    logger::init();
    log::info!("run()");
    let database_url = std::env::var("DATABASE_URL").unwrap();
    log::info!("database_url {}", database_url);
    create_endpoint().await;
}

#[request_handler]
async fn webhook_handler(
    headers: Vec<(String, String)>,
    path: String,
    query: HashMap<String, Value>,
    body: Vec<u8>,
) {
    logger::init();
    log::info!("header: {:?}", headers);
    log::info!("query: {:?}", query);
    log::info!("path: {:?}", path);
    log::info!("body: {:?}", body);

    // handler queries
    let action = query.get("action").unwrap().as_str().unwrap();

    // connect to database
    log::info!("connect to database");
    let opts = Opts::from_url(&*get_url()).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let constraints = PoolConstraints::new(5, 10).unwrap();
    let pool_opts = PoolOpts::default().with_constraints(constraints);
    let pool = Pool::new(builder.pool_opts(pool_opts));
    let mut conn = pool.get_conn().await.unwrap();

    // create table if no tables exist
    log::info!("check and create table if no tables exist");
    let result = r"SHOW TABLES LIKE 'orders';"
        .with(())
        .map(&mut conn, |s: String| String::from(s))
        .await
        .unwrap();
    if result.len() == 0 {
        // table doesn't exist, create a new one
        log::info!("table doesn't exist, create a new one");
        r"CREATE TABLE orders (order_id INT, production_id INT, quantity INT, amount FLOAT,
          shipping FLOAT, tax FLOAT, shipping_address VARCHAR(20));"
            .ignore(&mut conn)
            .await
            .unwrap();
    }

    // handle action
    let response = match action {
        "init" => {
            // init order data
            r"DELETE FROM orders;".ignore(&mut conn).await.unwrap();
            let orders = vec![
                Order::new(1, 12, 2, 56.0, 15.0, 2.0, String::from("Mataderos 2312")),
                Order::new(2, 15, 3, 256.0, 30.0, 16.0, String::from("1234 NW Bobcat")),
                Order::new(3, 11, 5, 536.0, 50.0, 24.0, String::from("20 Havelock")),
                Order::new(4, 8, 8, 126.0, 20.0, 12.0, String::from("224 Pandan Loop")),
                Order::new(5, 24, 1, 46.0, 10.0, 2.0, String::from("No.10 Jalan Besar")),
            ];
            r"INSERT INTO orders (order_id, production_id, quantity, amount, shipping, tax, shipping_address)
              VALUES (:order_id, :production_id, :quantity, :amount, :shipping, :tax, :shipping_address)"
                .with(orders.iter().map(|order| {
                    params! {
                        "order_id" => order.order_id,
                        "production_id" => order.production_id,
                        "quantity" => order.quantity,
                        "amount" => order.amount,
                        "shipping" => order.shipping,
                        "tax" => order.tax,
                        "shipping_address" => &order.shipping_address,
                    }
                }))
                .batch(&mut conn)
                .await.unwrap();
            json!({"status": "success"}).to_string()
        }
        "queryAll" => {
            // query order data
            let loaded_orders = r"SELECT * FROM orders"
                .with(())
                .map(
                    &mut conn,
                    |(
                        order_id,
                        production_id,
                        quantity,
                        amount,
                        shipping,
                        tax,
                        shipping_address,
                    )| {
                        Order::new(
                            order_id,
                            production_id,
                            quantity,
                            amount,
                            shipping,
                            tax,
                            shipping_address,
                        )
                    },
                )
                .await
                .unwrap();
            json!({"status": "success", "data": loaded_orders}).to_string()
        }
        "queryById" => {
            let order_id = query.get("order_id").unwrap().as_str().unwrap();
            // query order data
            let loaded_orders = r"SELECT * FROM orders WHERE order_id = :order_id"
                .with(params! {"order_id" => order_id})
                .map(
                    &mut conn,
                    |(
                        order_id,
                        production_id,
                        quantity,
                        amount,
                        shipping,
                        tax,
                        shipping_address,
                    )| {
                        Order::new(
                            order_id,
                            production_id,
                            quantity,
                            amount,
                            shipping,
                            tax,
                            shipping_address,
                        )
                    },
                )
                .await
                .unwrap();
            json!({"status": "success", "data": loaded_orders}).to_string()
        }
        "deleteById" => {
            // delete order by order_id
            let order_id = query.get("order_id").unwrap().as_str().unwrap();
            r"DELETE FROM orders WHERE order_id = :order_id"
                .with(vec![params! {"order_id" => order_id}])
                .batch(&mut conn)
                .await
                .unwrap();
            json!({"status": "success"}).to_string()
        }
        "updateAddressById" => {
            // update order address info by order_id
            let order_id = query.get("order_id").unwrap().as_str().unwrap();
            let shipping_address = query.get("shipping_address").unwrap().as_str().unwrap();
            r"UPDATE orders SET shipping_address = :shipping_address WHERE order_id = :order_id"
                .with(vec![
                    params! {"order_id" => order_id, "shipping_address" => shipping_address},
                ])
                .batch(&mut conn)
                .await
                .unwrap();
            json!({"status": "success"}).to_string()
        }
        _ => {
            let message = format!("unknown action: {}", action);
            log::info!("{}", message);
            json!({"status": "fail", "message": message}).to_string()
        }
    };

    // disconnect from database
    drop(conn);
    pool.disconnect().await.unwrap();

    send_response(
        200,
        vec![(
            String::from("content-type"),
            String::from("text/plain; charset=UTF-8"),
        )],
        response.as_bytes().to_vec(),
    )
}
