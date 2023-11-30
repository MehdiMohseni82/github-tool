use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

#[tokio::main]
async fn main() {
    println!("********************************************* GitHub SOX *******************************************");
    println!("Welcome to Github tool. This tool is designed to get information from Github repositories.");
    println!("");

    loop {
        println!("");
        println!("Available commands:");
        println!("1. Export all releases to CSV format.");
        println!("2. Exit");

        let input = get_console_input();

        match input.as_str() {
            "1" => {
                print!("Enter Repository name: ");
                io::stdout().flush().unwrap();
                let repo_name = get_console_input();

                print!("Enter Repository owner: ");
                io::stdout().flush().unwrap();
                let repo_owner = get_console_input();

                print!("Enter Github fine-grainded access token: ");
                io::stdout().flush().unwrap();
                let token = get_console_input();

                let releases = list_runs(&repo_name, &repo_owner, &token).await;

                match releases {
                    Ok(data) => {
                        let mut csv_data = String::new();

                        for release in &data {
                            let released_items: Vec<&str> = release.body.split('\n').collect();
                            let take = released_items.len().saturating_sub(2);

                            let release_line = format!(
                                "{},{}, {}",
                                release.name, release.created_at, release.author.login
                            );

                            csv_data.push_str(&release_line);
                            csv_data.push('\n');

                            for released_item in released_items.iter().skip(1).take(take) {
                                csv_data.push_str(released_item);
                                csv_data.push('\n');
                            }
                        }

                        print!("Export is done. Enter the path to save the file: ");
                        io::stdout().flush().unwrap();
                        let file_path = get_console_input();

                        let mut file = File::create(file_path).expect("Failed to create file");
                        file.write_all(csv_data.as_bytes())
                            .expect("Failed to write to file");
                    }
                    Err(error) => {
                        print!("{:?}", error)
                    }
                }
                println!("Exporting all releases to CSV format...");
            }
            "2" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid command"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Release {
    pub url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub html_url: String,
    pub id: u64,
    pub author: Author,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub tarball_url: String,
    pub zipball_url: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Author {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub site_admin: bool,
}

async fn list_runs(
    repo_name: &str,
    repo_owner: &str,
    token: &str,
) -> Result<Vec<Release>, Box<dyn Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases",
        repo_owner, repo_name
    );

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "Awesome-Octocat-App")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("Accept", "application/vnd.github+json")
        .send()
        .await
        .unwrap()
        .json::<Vec<Release>>()
        .await?;

    Ok(response)
}

fn get_console_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
