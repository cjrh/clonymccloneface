use anyhow::Result;
use octocrab::Octocrab;
use spinners::{Spinner, Spinners};
use std::path::{Path, PathBuf};
use subprocess::{Exec, Redirection};
use yansi::Paint;

pub async fn get_repos_list(username: &str, token: &str, path: &Option<PathBuf>) -> Result<()> {
    let o = Octocrab::builder()
        .personal_token(token.to_string())
        .build()?;

    let o2 = o.clone();

    // Repos list
    let mut repos = vec![];
    let mut finished = false;
    let mut page = 1;
    while !finished {
        let page_repos = o2
            .current()
            .list_repos_for_authenticated_user()
            .type_("all")
            .page(page)
            .per_page(10)
            .send()
            .await?
            .into_iter()
            .collect::<Vec<_>>();

        if !page_repos.is_empty() {
            repos.extend(page_repos);
            page += 1;
        } else {
            finished = true;
        }
    }

    let write_path = match path {
        Some(p) => std::fs::canonicalize(p).unwrap(),
        None => std::env::current_dir().unwrap(),
    };

    for r in repos {
        let reponame = r.name.clone();
        let x = match o.repos(username, &reponame).get().await {
            Ok(x) => x,
            Err(_) => {
                println!("Error fetching {reponame}");
                continue;
            }
        };
        let repo_name = &r.name;
        if already_cloned(repo_name, &write_path) {
            println!("✔️  Target clone exists, skipping: {:?}", &repo_name);
            continue;
        }
        let ssh_url = x.ssh_url.as_ref().unwrap();
        let parent_ssh_url = x.parent.as_ref().and_then(|p| p.ssh_url.clone());
        clone_repo(repo_name, ssh_url, &write_path, parent_ssh_url);
    }

    Ok(())
}

fn already_cloned(repo_name: &str, write_path: &Path) -> bool {
    let target_repo_folder = write_path.join(repo_name);
    target_repo_folder.exists()
}

fn clone_repo(
    repo_name: &str,
    ssh_url: &str,
    write_path: &PathBuf,
    parent_ssh_url: Option<String>,
) {
    let target_repo_folder = write_path.join(repo_name);
    let msg = format!("Cloning {}...", &repo_name);
    let mut sp = Spinner::new(Spinners::Dots9, msg);
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
