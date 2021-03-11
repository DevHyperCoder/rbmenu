use regex::Regex;

pub fn is_url(input: &str) -> bool {
    let re = Regex::new( r"^(http://www\.|https://www.|http://|https://)?[a-z0-9]+([-.]{1}[a-z0-9]+)*.[a-z]{2,5}(:[0-9]{1,5})?(/.*)?$").unwrap();
    re.is_match(&input)
}
