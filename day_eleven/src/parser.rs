use crate::models::SpaceType;

pub fn parse_to_space_items(input: Vec<String>) -> Vec<Vec<SpaceType>> {
    let mut id = 0;
    input
        .into_iter()
        .map(|each| {
            each.chars()
                .map(|c| {
                    SpaceType::from_char(c, {
                        id += 1;
                        id
                    })
                })
                .collect()
        })
        .collect()
}
