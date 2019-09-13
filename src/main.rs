use regex::Captures;
use regex::Regex;
fn main() {
    learn_captures();
    learn_find();
    learn_split();
    
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
fn learn_find(){
    let text = "Retroactively relinquishing remunerations is reprehensible.";
    println!("{}",Regex::new(r"\b\w{13}\b").unwrap().is_match(text));
    for mat in Regex::new(r"\b\w{13}\b").unwrap().find_iter(text) {
        println!("word={:?}",text.get(mat.start()..mat.end()).map(|v| &*v));
        println!("word={}",&text[mat.start()..mat.end()]);
    }
}
fn learn_split(){
    let re = Regex::new(r"[ \t]+").unwrap();
    let fields: Vec<&str> = re.split("a b \t  c\td    e").collect();
    println!("{:?}",fields);
}
fn learn_replace() {
    let re = Regex::new("([{,])\\s*\"?(\\w+)\"?\\s*:").unwrap();
    let str1 = "{xy:7,xx:\"kk\",\"key\":\"bbb\"}";
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

    let mut str2 = "{xy:7,xx:\"kk\",\"key\":\"bbb\"}";
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
       // str2=&str2.replace(&replace_str, &temp);
       println!("{}",str2.replace(&replace_str, &temp));
    }
}
