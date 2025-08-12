pub struct PostRust {
    content: String,
}

impl PostRust {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> WaitingForApprovalPost {
        WaitingForApprovalPost {
            content: self.content,
        }
    }
}

pub struct WaitingForApprovalPost {
    content: String,
}

impl WaitingForApprovalPost {
    pub fn approve(self) -> PostRust {
        println!("Approving requested waited post");
        PostRust {
            content: self.content
        }
    }
}

