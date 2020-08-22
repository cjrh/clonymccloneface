use github_rs::client::{Executor, Github};
use serde_json::Value;

pub fn get_repos_list(username: &str, token: &str) {
    let client = Github::new(token).unwrap();
    let per_page: usize = 100;
    let mut page = 1;

    while let Some(h) = get_repos_page(&client, &username, page, per_page) {
        println!("Got {} repos", &h.len());
        for data in &h {
            // println!("{}", serde_json::to_string_pretty(&h).unwrap());
            let repo_name = data["name"].as_str().unwrap();
            let ssh_url = data["ssh_url"].as_str().unwrap();
            let fork: bool = data["fork"].as_bool().unwrap();
            clone_repo(&repo_name, &ssh_url, &fork);
            if fork {
                if let Some(parent_ssh_url) = get_fork_parent(&client, &username, &repo_name) {
                    println!("....Parent ssh_url: {}", &parent_ssh_url)
                }
            }
        }
        if h.len() < per_page {
            break;
        }
        page += 1;
    }
    println!("Done.");
}

fn clone_repo(repo_name: &str, ssh_url: &str, fork: &bool) {
    println!("repo: {} fork? {} ssh_url: {}", &repo_name, &fork, &ssh_url);
}

fn get_fork_parent(client: &Github, username: &str, repo_name: &str) -> Option<String> {
    let repo_endpoint = format!("repos/{}/{}", &username, &repo_name);
    let response = client
        .get()
        .custom_endpoint(&repo_endpoint)
        .execute::<Value>();

    match response {
        Ok((_headers, _status, json)) => {
            if let Some(json) = json {
                let parent = &json["parent"];
                let s = parent["clone_url"].as_str().unwrap().to_owned();
                Some(s)
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

fn get_repos_page(
    client: &Github,
    username: &str,
    page: u32,
    per_page: usize,
) -> Option<Vec<Value>> {
    let repos_endpoint = format!(
        "users/{}/repos?type=all&per_page={}&page={}",
        username, per_page, page
    );
    let response = client
        .get()
        .custom_endpoint(&repos_endpoint)
        .execute::<Value>();

    match response {
        Ok((_headers, _status, json)) => {
            // println!("{:#?}", &headers);
            if let Some(json) = json {
                json.as_array().cloned()
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
