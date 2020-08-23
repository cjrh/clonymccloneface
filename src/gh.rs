use github_rs::client::{Executor, Github};
use serde_json::Value;
use spinners::{Spinner, Spinners};
use std::path::PathBuf;
use subprocess::{Exec, Redirection};
use yansi::Paint;

pub fn get_repos_list(username: &str, token: &str, path: &Option<PathBuf>) {
    let client = Github::new(token).unwrap();
    let per_page: usize = 100;
    let mut page = 1;

    let write_path = match path {
        Some(p) => std::fs::canonicalize(p).unwrap(),
        None => std::env::current_dir().unwrap(),
    };

    while let Some(h) = get_repos_page(&client, &username, page, per_page) {
        // println!("Got {} repos", &h.len());
        for data in &h {
            // println!("{}", serde_json::to_string_pretty(&h).unwrap());
            let repo_name = data["name"].as_str().unwrap();
            let ssh_url = data["ssh_url"].as_str().unwrap();
            let fork: bool = data["fork"].as_bool().unwrap();
            let parent_ssh_url = get_fork_parent(&client, &username, &repo_name, &fork);
            clone_repo(&repo_name, &ssh_url, &fork, &write_path, parent_ssh_url);
        }
        if h.len() < per_page {
            break;
        }
        page += 1;
    }
    // println!("Done.");
}

fn clone_repo(
    repo_name: &str,
    ssh_url: &str,
    fork: &bool,
    write_path: &PathBuf,
    parent_ssh_url: Option<String>,
) {
    // println!("repo: {} fork? {} ssh_url: {}", &repo_name, &fork, &ssh_url);
    let target_repo_folder = write_path.join(repo_name);
    if target_repo_folder.exists() {
        println!("👍 Target clone exists, skipping: {:?}", &repo_name);
        return;
    };
    let msg = format!("Cloning {}...", &repo_name);
    let sp = Spinner::new(Spinners::Dots9, msg);
    let result = Exec::cmd("git")
        .arg("clone")
        .arg(&ssh_url)
        .arg(&target_repo_folder)
        .cwd(&write_path)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture();
    sp.stop();
    // Must clear the entire line, not only move the cursor back to start of line
    print!("\r\x1b[K");

    match result {
        Ok(cd) => {
            let s = Paint::green(format!("✅ Cloned {}", &repo_name));
            print!("{}", s);
            let _output = cd.stdout_str();
            // TODO: if verbose is supplied, print verbose
        }
        Err(e) => {
            eprintln!("\r❌ Failed to clone {}, error: {}", &ssh_url, e);
            return;
        }
    }

    if parent_ssh_url.is_none() {
        println!();
        return;
    }
    let upstream_ssh_url = parent_ssh_url.unwrap();
    let result = Exec::cmd("git")
        .arg("remote")
        .arg("add")
        .arg("upstream")
        .arg(&upstream_ssh_url)
        .cwd(&target_repo_folder)
        .stdout(Redirection::Pipe)
        .stderr(Redirection::Merge)
        .capture();
    match result {
        Ok(_cd) => {
            println!("{}", Paint::green("  (and set upstream repo)"));
        }
        Err(e) => {
            eprintln!("\r❌ Failed to set upstream: {}", e);
        }
    }
}

fn get_fork_parent(
    client: &Github,
    username: &str,
    repo_name: &str,
    fork: &bool,
) -> Option<String> {
    if !fork {
        return None;
    }

    let repo_endpoint = format!("repos/{}/{}", &username, &repo_name);
    let response = client
        .get()
        .custom_endpoint(&repo_endpoint)
        .execute::<Value>();

    match response {
        Ok((_headers, _status, json)) => {
            if let Some(json) = json {
                let parent = &json["parent"];
                let s = parent["ssh_url"].as_str().unwrap().to_owned();
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
    let repos_endpoint = format!("user/repos?type=all&per_page={}&page={}", per_page, page);
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
