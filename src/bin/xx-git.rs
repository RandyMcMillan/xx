fn main() {
    let version = xx::git::get_git_version();
    println!("{}", version.unwrap());
    let git = xx::git::Git::new(".".into());

    if git.is_repo() {
        println!("{}", git.is_repo());
    }
}
