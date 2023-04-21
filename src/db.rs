use sqlx::{migrate::MigrateDatabase, Error, MySql, Pool};

pub fn get_url() -> String {
    std::env::var("DATABASE_URL").expect("Error reading .env DATABASE_URL")
}

pub async fn get_connection() -> Result<Pool<MySql>, Error> {
    sqlx::mysql::MySqlPool::connect(&get_url()).await
}

pub async fn create() {
    let url = get_url();
    if !MySql::database_exists(&url).await.unwrap_or(false) {
        println!("Creating database {}", &url);
        match MySql::create_database(&url).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }
}

pub async fn run_migrations() {
    let res = sqlx::migrate!()
        .run(
            &get_connection()
                .await
                .expect("Error getting connection for migration"),
        )
        .await;

    match res {
        Ok(_) => println!("Migrations run successfuly"),
        Err(err) => {
            println!("Error running migrations");
            println!("{:?}", err)
        }
    };
}
