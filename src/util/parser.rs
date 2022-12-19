use crate::util::number::parse_usize;

pub struct Parser {
    input: String,
    position: usize
}

impl Parser {
    pub fn new<T>(input: T) -> Self
        where T: ToString {
        Parser { input: input.to_string(), position: 0 }
    }

    fn skip_whitespace(&mut self) {
        self.position += self.input.chars().skip(self.position).take_while(|c| c.is_whitespace()).count()
    }

    pub fn literal(&mut self, literal: &str) -> Result<(), String> {
        self.skip_whitespace();

        let actual = &self.input[self.position..self.position+literal.len()];
        if actual != literal {
            Err(format!("Expected '{}' to match '{}' ('{}':{})", actual, literal, self.input, self.position))
        } else {
            self.position += literal.len();
            Ok(())
        }
    }

    pub fn usize(&mut self) -> Result<usize, String> {
        self.skip_whitespace();

        let mut result = 0;

        // consume at least one numeric character
        let numbers: Vec<_> = self.input.chars().skip(self.position)
            .take_while(|c| c.is_numeric())
            .collect();
        if numbers.len() == 0 { return Err(format!("Expected to find a number. ('{}':{})", self.input, self.position)) }

        for char in numbers.iter() {
            result *= 10;
            result += parse_usize(char.to_string().as_str())?;
        }

        self.position += numbers.len();
        Ok(result)
    }

    pub fn isize(&mut self) -> Result<isize, String> {
        self.skip_whitespace();

        let modifier = if self.input.chars().nth(self.position) == Some('-') {
            self.position += 1;
            -1
        } else {
            1
        };

        Ok(modifier * (self.usize()?) as isize)
    }

    pub fn str(&mut self, len: usize) -> Result<String, String> {
        self.skip_whitespace();

        let result: Vec<_> = self.input.chars().skip(self.position).take(len).collect();
        if result.len() != len {
            Err(format!("Expected to read {} chars, but only got {}. ('{}':{})", len, result.len(), self.input, self.position))
        } else {
            self.position += len;
            Ok(result.iter().collect())
        }
    }

    pub fn is_exhausted(&self) -> bool {
        self.position >= self.input.len()
    }
}