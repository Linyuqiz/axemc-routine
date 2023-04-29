use serde;
use serde_yaml;

use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct App {
    app_name: String,
    version: String,
    user_infos: Vec<UserInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserInfo {
    name: String,
    param: String,
    token: String,
}

#[tokio::main]
async fn main() {
    // Generate Theme Pattern：http://patorjk.com/software/taag/
    println!(
        r"
            ____                   ____                             __
           / __ \ ____ _ __  __   / __ \ ___   ____   ____   _____ / /_
          / / / // __ `// / / /  / /_/ // _ \ / __ \ / __ \ / ___// __/
         / /_/ // /_/ // /_/ /  / _, _//  __// /_/ // /_/ // /   / /_
        /_____/ \__,_/ \__, /  /_/ |_| \___// .___/ \____//_/    \__/
                      /____/               /_/
    "
    );

    let yaml_str = include_str!("../app.yml");
    let app: App = serde_yaml::from_str(yaml_str).expect("app.yaml read failed!");

    for user in app.user_infos.iter() {
        if let Ok(result) =
            execute_report(user.token.parse().unwrap(), user.param.parse().unwrap()).await
        {
            println!("{:?} --- {:?} --- {}", Local::now(), result, user.name);
        } else {
            println!("this daily report failed!");
        }
    }
}

async fn execute_report(
    token: String,
    params: Value,
) -> Result<HashMap<String, Value>, reqwest::Error> {
    let return_status = reqwest::Client::new()
        .post("https://stu.eurasia.edu/yqsb/jkdj/save?token=".to_owned() + &*token)
        .json(&params)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?;
    Ok(return_status)
}
