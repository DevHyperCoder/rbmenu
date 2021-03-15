use regex::Regex;

/// Use regex to check if the given str is a URL
/// input : link to check against
pub fn is_url(input: &str) -> bool {
    let re = Regex::new( r"^(http://www\.|https://www.|http://|https://)?[a-z0-9]+([-.]{1}[a-z0-9]+)*.[a-z]{2,5}(:[0-9]{1,5})?(/.*)?$").unwrap();
    re.is_match(&input)
}

/// Use regex to get the domain name and subdomains from the given URL
/// input : link to get the domain and subdomains
pub fn get_domain_name(input: &str) -> regex::Match {
    let re = Regex::new(r".*://(?:www.)?([^/]+)").unwrap();
    re.captures(&input).unwrap().get(1).unwrap()
}
