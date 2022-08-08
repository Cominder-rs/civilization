use crate::common_structs::Env;

pub fn init_service() {
    dotenv::dotenv().ok();

    let env = match std::env::var("ENV").expect("Specify ENV variable").as_str() {
        "dev" => Env::Dev,
        "prod" => Env::Prod,
        "test" => Env::Test,
        _ => panic!("ENV var can be either dev, prod, test")
    };


}