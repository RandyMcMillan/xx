//use gnostr_bins::get_blockheight;
//use gnostr_bins::get_weeble;
//use gnostr_bins::get_wobble;

fn parse(input: &str) -> Option<Vec<String>> {
    let mut part = String::new();
    let mut collected = Vec::new();

    let mut char_iter = input.chars();

    if char_iter.next() != Some('[') {
        return None;
    }

    loop {
        match char_iter.next()? {
            ']' => {
                collected.push(part);
                return Some(collected);
            }
            ',' | ' ' => {
                if !part.is_empty() {
                    collected.push(part);
                    part = String::new();
                }
            }
            x => part.push(x),
        }
    }
}
fn main() {
    let weeble = gnostr_bins::get_weeble();
    let wobble = gnostr_bins::get_wobble();
    let blockheight = gnostr_bins::get_blockheight();

    println!(
        "{:}/{:}/{:}",
        weeble.unwrap(),
        blockheight.unwrap(),
        wobble.unwrap()
    );

    let relays_public = gnostr_bins::get_relays_public();
    println!("{:}", relays_public.clone().unwrap());
    //let version = xx::git::get_git_version();
    //println!("{}", version.unwrap());
    println!("{:?}", parse(&relays_public.as_ref().unwrap()));

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
