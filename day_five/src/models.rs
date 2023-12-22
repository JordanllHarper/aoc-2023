#[derive(Debug, Clone)]
pub struct Almanac {
    pub original_seed_list: Vec<i64>,
    pub maps: Vec<ItemToItemSpec>,
}

impl Almanac {
    pub fn new(original_seed_list: Vec<i64>, maps: Vec<ItemToItemSpec>) -> Self {
        Self {
            original_seed_list,
            maps,
        }
    }
}
#[derive(Debug, Clone)]
pub struct ItemToItemSpec {
    pub item_to_item_type: ItemToItemType,
    pub map_numbers: Vec<Vec<i64>>,
}

impl ItemToItemSpec {
    pub fn new(item_to_item_type: ItemToItemType, lines: Vec<Vec<i64>>) -> Self {
        Self {
            item_to_item_type,
            map_numbers: lines,
        }
    }
    pub fn ranges(&self) -> Vec<MapRange> {
        self.map_numbers
            .iter()
            .map(|transform| MapRange::from(transform))
            .collect::<Vec<MapRange>>()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ItemToItemType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Range {
    pub start: i64,
    pub end: i64,
}
impl Range {
    pub fn difference(&self) -> i64 {
        self.end - self.start
    }
}

impl Range {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }
    pub fn number_falls_in_range(&self, num: i64) -> bool {
        num >= self.start && num <= self.end
    }
}
#[derive(Debug, Clone, Copy)]
pub struct MapRange {
    pub source: Range,
    pub destination: Range,
}

impl MapRange {
    pub fn from(line_of_numbers: &[i64]) -> Self {
        let amount_to_add_by = line_of_numbers.last().unwrap().checked_sub(1).unwrap(); // account for exclusive add
        let source_start = *line_of_numbers.get(1).unwrap();
        let source_end = source_start + amount_to_add_by;
        let source_range = Range::new(source_start, source_end);

        let destination_start = *line_of_numbers.first().unwrap();
        let destination_end = destination_start + amount_to_add_by;
        let destination_range = Range::new(destination_start, destination_end);
        MapRange::new(source_range, destination_range)
    }
    pub fn new(source: Range, destination: Range) -> Self {
        Self {
            source,
            destination,
        }
    }
    pub fn difference_between(&self) -> i64 {
        self.destination.start - self.source.start // 2 - 1 vs 1 - 2
    }
}

impl ItemToItemType {
    pub fn from_str(input: &str) -> Option<ItemToItemType> {
        let result = match input {
            "seed-to-soil" => ItemToItemType::SeedToSoil,
            "soil-to-fertilizer" => ItemToItemType::SoilToFertilizer,
            "fertilizer-to-water" => ItemToItemType::FertilizerToWater,
            "water-to-light" => ItemToItemType::WaterToLight,
            "light-to-temperature" => ItemToItemType::LightToTemperature,
            "temperature-to-humidity" => ItemToItemType::TemperatureToHumidity,
            "humidity-to-location" => ItemToItemType::HumidityToLocation,
            _ => return None,
        };

        Some(result)
    }
}
