#[derive(Debug, PartialEq)]
pub enum StreamContent<'a> {
    Group(Vec<StreamContent<'a>>),
    Garbage(&'a str)
}

impl<'a> StreamContent<'a> {
    pub fn visit(&self, visitor: &mut StreamContentVisitor) {
        visitor.visit(&self);
    }
}

pub struct StreamContentVisitor {
    score: u32,
    level: u32
}

impl StreamContentVisitor {
    pub fn new() -> Self {
        StreamContentVisitor {
            score: 0,
            level: 1
        }
    }

    pub fn visit(&mut self, content: &StreamContent) {
        match content {
            &StreamContent::Garbage(_) => {},
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
}
