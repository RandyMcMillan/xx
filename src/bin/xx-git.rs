fn main() {
    //let version = xx::git::get_git_version();
    //println!("{}", version.unwrap());
    let git = xx::git::Git::new(".".into());

    if git.is_repo() {
        //println!("{}", git.is_repo());
        println!(
            "{:}",
            format!("{}", git.get_remote_url().unwrap().to_string())
        );
        println!("{:}", format!("{}", git.current_sha().unwrap().to_string()));
        println!(
            "{:}",
            format!("{}", git.current_sha_short().unwrap().to_string())
        );
        println!(
            "{:}",
            format!("{}", git.current_branch().unwrap().to_string())
        );
    }
}
