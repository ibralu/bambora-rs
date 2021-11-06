#[tokio::main]
async fn main() {
    let conf = bambora::new_config("MzgzNjEwMzU0OjNGMDhERjY0RkExNzQ3NDU5MzNGMzg3ODAxNjIzZjYz".into(), None);

    let res = bambora::apis::payments_api::payments_trans_id_get(&conf, 22).await.unwrap();
    println!("{:#?}", res);
}