mod digital_ocean {
    use core::panic;
    use reqwest::StatusCode;
    use sqlx::postgres::PgPoolOptions;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};
    use std::path::Path;

    fn read_file<P>(
        filename: P,
    ) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file =
            File::open(filename).expect("Error while reading file");
        Ok(BufReader::new(file).lines())
    }

    #[tokio::test]
    #[ignore = "Digital Ocean"]
    async fn successful_create_and_delete() {
        let mut config: HashMap<String, String> = HashMap::new();

        // Read Digital Ocean configuration
        if let Ok(lines) =
            read_file("./experimental/digitalocean_connection.txt")
        {
            for line in lines.flatten() {
                let parameter: Vec<&str> =
                    line.split(":").map(|s| s.trim()).collect();
                if parameter.len() != 2 {
                    panic!("Invalid configuration for Digital Ocean database!");
                } else {
                    config.insert(
                        parameter[0].to_string(),
                        parameter[1].to_string(),
                    );
                }
            }
        }

        // Construct connection string
        let connection_string = format!(
            "postgresql://{}:{}@{}:{}/{}",
            config.get("username").unwrap(),
            config.get("password").unwrap(),
            config.get("host").unwrap(),
            config.get("port").unwrap(),
            config.get("database").unwrap(),
        );

        // Create database connection
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&connection_string)
            .await
            .expect("Unable to connect to Digital Ocean database");

        // Create subscription
        let app_url = "https://rusty-krab-ymzqg.ondigitalocean.app";
        let client = reqwest::Client::new();
        let body = "name=Brian%20Fong&email=0xfrian%40gmail.com";
        let res_create = client
            .post(format!("{}/subscriptions", app_url))
            .header(
                "Content-Type",
                "application/x-www-form-urlencoded",
            )
            .body(body)
            .send()
            .await
            .expect("Failed to send POST request");
        assert_eq!(StatusCode::OK, res_create.status());

        // Read subscription
        let res_read =
            sqlx::query!("SELECT email, name FROM subscriptions")
                .fetch_all(&pool)
                .await
                .expect("Failed to fetch subscriptions");
        if res_read.len() != 1 {
            panic!("Expected only one row in subscriptions table");
        }
        let record = &res_read[0];
        assert_eq!("Brian Fong", record.name);
        assert_eq!("0xfrian@gmail.com", record.email);

        // Delete subscription
        let _ = sqlx::query!("DELETE FROM subscriptions WHERE name='Brian Fong' AND email='0xfrian@gmail.com'")
            .execute(&pool)
            .await;
        let res_read =
            sqlx::query!("SELECT email, name FROM subscriptions")
                .fetch_all(&pool)
                .await
                .expect("Failed to fetch subscriptions");
        assert_eq!(0, res_read.len());
    }
}
