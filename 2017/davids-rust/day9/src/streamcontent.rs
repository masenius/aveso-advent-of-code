#[derive(Debug, PartialEq)]
pub enum StreamContent {
    Group(Vec<StreamContent>),
    Garbage(String)
}

impl StreamContent {
    pub fn visit(&self, visitor: &mut StreamContentVisitor) {
        visitor.visit(&self);
    }
}

pub struct StreamContentVisitor {
    score: u32,
    level: u32,
    garbage_amount: u32
}

impl StreamContentVisitor {
    pub fn new() -> Self {
        StreamContentVisitor {
            score: 0,
            level: 1,
            garbage_amount: 0
        }
    }

    pub fn visit(&mut self, content: &StreamContent) {
        match content {
            &StreamContent::Garbage(ref garbage) => {
                self.garbage_amount += garbage.len() as u32
            },
            &StreamContent::Group(ref group_contents) => {
                self.score += self.level;
                self.level += 1;
                for content in group_contents.iter() {
                    content.visit(self)
                }
                self.level -= 1;
            }
        }
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn garbage_amount(&self) -> u32 {
        self.garbage_amount
    }
}
