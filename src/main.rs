use regex::Captures;
use regex::Regex;
fn main() {
    learn_captures();
    learn_replace();
}
fn learn_captures() {
    let re = Regex::new(r"(?x)(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2})").unwrap();
    let caps = re.captures("2010-03-14,2010-03-19").unwrap();
    for caps in re.captures_iter("2010-03-14,2010-03-19") {
        // Note that all of the unwraps are actually OK for this regex
        // because the only way for the regex to match is if all of the
        // capture groups match. This is not true in general though!
        println!(
            "year: {}, month: {}, day: {}",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str(),
            caps.get(3).unwrap().as_str()
        );
    }
    println!("{},{},{}", &caps["year"], &caps["month"], &caps["day"]);

}
fn learn_replace() {
    let re = Regex::new("([{,])\\s*\"?(\\w+)\"?\\s*:").unwrap();
    let mut str1 = "{xy:7,xx:\"kk\",\"key\":\"bbb\"}";
    println!("{}", re.replace_all(str1, "$1\"$2\":"));
    let result = re.replace_all(str1, |caps: &Captures| {
        let temp = format!(
            "{}\"{}\":",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str()
        );
        println!("format={}", temp);
        temp
    });
    println!("result={:?}", result);

    for caps in re.captures_iter(str1) {
        let temp = format!(
            "{}\"{}\":",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str()
        );
        let replace_str= format!(
            "{}{}:",
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str()
        );
        str1=&str1.replace(caps, &temp);
    }
}
