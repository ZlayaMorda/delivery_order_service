use delivery_order_service::run;
use delivery_order_service::utils::errors::AppError;


#[tokio::main]
async fn main() -> Result<(), AppError> {
    run("dev").await
}

