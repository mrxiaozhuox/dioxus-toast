use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct ID(usize);

impl ID {
    
    pub fn new() -> Self {
        Self(100000)
    }

    pub fn add(&mut self) -> usize {

        let current = self.0;
        if self.0 >= usize::MAX {
            self.0 = 100000;
        } else {
            self.0 += 1;
        }

        current
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}