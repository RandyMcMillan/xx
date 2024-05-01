use std::fs;

use git2::Repository;
use gpgme::{Context, Protocol, SignMode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all("/tmp/git2").unwrap();
    let mut ctx = Context::from_protocol(Protocol::OpenPgp)?;

    let repo = Repository::init("/tmp/git2").unwrap();
    let mut index = repo.index()?;

    index.add_all(["."].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let tree_id = repo.index()?.write_tree()?;
    let sig = repo.signature()?;
    let mut parents = Vec::new();

    if let Some(parent) = repo.head().ok().map(|h| h.target().unwrap()) {
        parents.push(repo.find_commit(parent)?);
    }

    let parents = parents.iter().collect::<Vec<_>>();
    let buf =
        repo.commit_create_buffer(&sig, &sig, "test", &repo.find_tree(tree_id)?, &parents)?;
    let contents = std::str::from_utf8(&buf).unwrap().to_string();
    let mut outbuf = Vec::new();

    ctx.set_armor(true);
    ctx.sign(SignMode::Detached, buf.as_str().unwrap(), &mut outbuf)?;

    let out = std::str::from_utf8(&outbuf).unwrap();
    let ret = repo.commit_signed(&contents, &out, None)?;
    let commit = repo.find_commit(ret)?;
    repo.branch("master", &commit, false)?; // :-)

    println!("{:?}", ret);
    Ok(())
}
