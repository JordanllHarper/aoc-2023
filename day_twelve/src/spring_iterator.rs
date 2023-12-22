use crate::models::{KnownSpringSpan, KnownSpringType, SpringType};

pub struct SpringIterator {
    pub springs: Vec<KnownSpringType>,
    pub index: usize,
}

impl SpringIterator {
    pub fn new(springs: Vec<KnownSpringType>) -> Self {
        Self { springs, index: 0 }
    }
}
/// Spans of a group of a single type
impl Iterator for SpringIterator {
    //
    // the index of the previous group_start, the type of the group and the length of the group
    type Item = KnownSpringSpan;

    /// Returns the next group of springs start and its type
    fn next(&mut self) -> Option<Self::Item> {
        let start = match self.springs.get(self.index) {
            Some(v) => v,
            None => {
                return None;
            }
        };
        for i in self.index..self.springs.len() {
            let searched_forward = match self.springs.get(i) {
                Some(v) => v,
                None => {
                    return Some(KnownSpringSpan::new(
                        self.index.try_into().unwrap(),
                        *start,
                        i.try_into().unwrap(),
                    ))
                }
            };
            if searched_forward == start {
                continue;
            } else {
                let result = Some(KnownSpringSpan::new(
                    self.index.try_into().unwrap(),
                    *start,
                    (TryInto::<i32>::try_into(i).unwrap()) - 1,
                ));
                self.index = i;
                return result;
            }
        }
        let result = Some(KnownSpringSpan::new(
            self.index.try_into().unwrap(),
            *start,
            (TryInto::<i32>::try_into(self.springs.len()).unwrap()) - 1,
        ));

        self.index = self.springs.len() + 1;
        result
    }
}
