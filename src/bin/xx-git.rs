use gnostr_bins::get_weeble;
use gnostr_bins::get_blockheight;
use gnostr_bins::get_wobble;
fn main() {

println!("{:}/{:}/{:}",gnostr_bins::get_weeble().unwrap(),gnostr_bins::get_blockheight().unwrap(),gnostr_bins::get_wobble().unwrap());
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
        //println!(
        //    "{:}",
        //    format!("{}", git.current_sha_short().unwrap().to_string())
        //);
        println!(
            "{:}",
            format!("{}", git.current_branch().unwrap().to_string())
        );
    }
}
