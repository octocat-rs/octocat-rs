/// Gets the owner and repository from the `html_url` field used by so many of
/// our model.
pub fn owner_and_repo(html_url: String) -> (String, String) {
    let f = |s: &str| {
        if s.contains("https:") || s.is_empty() || s.eq("github.com") {
            None
        } else {
            Some(s.to_owned())
        }
    };

    let split: Vec<String> = html_url.split('/').filter_map(f).collect();

    (split[0].clone(), split[1].clone())
}
