trait AsString {
    fn as_str(&self) -> String;
}


struct VkComment {
    author: i32,
    text: String
}

struct VkPost<'a> {
    author: i32,
    text: String,
    comments: Vec<&'a VkComment>
}


impl AsString for VkComment {
    fn as_str(&self) -> String {
        format!("{}: {}", self.author, self.text)
    }
}

impl AsString for VkPost<'_> {
    fn as_str(&self) -> String {
        let mut result = format!("{}\n{}", self.author, self.text);
        for comment in &self.comments {
            let s = format!("\n -- {}", comment.as_str());
            result.push_str(&s);
        }
        result
    }
}




fn main() {
    let comment = VkComment{author: 1, text: String::from("Nice)")};
    let post = VkPost{author: 205, text: String::from("OK"), comments: vec![&comment]};

    println!("{}", comment.as_str());
    println!("{}", post.as_str());
}
