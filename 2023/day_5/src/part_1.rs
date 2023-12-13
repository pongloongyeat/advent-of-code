#[derive(Debug)]
struct DestinationToSource {
    destination: u64,
    source: u64,
    range: u64,
}

impl DestinationToSource {
    fn new(destination: u64, source: u64, range: u64) -> Self {
        Self {
            destination,
            source,
            range,
        }
    }

    fn is_destination_in_range(&self, destination: u64) -> bool {
        let diff = destination as i64 - self.range as i64;
        destination == self.destination
            || (destination > self.destination && diff < self.destination as i64)
    }

    fn get_source(&self, destination: u64) -> Option<u64> {
        if !self.is_destination_in_range(destination) {
            None
        } else {
            let diff = self.source as i64 - self.destination as i64;
            Some((destination as i64 + diff) as u64)
        }
    }
}

impl From<&str> for DestinationToSource {
    fn from(value: &str) -> Self {
        let mut data = value.trim().split(" ");
        assert_eq!(data.clone().count(), 3, "Invalid data parse for {value}");

        // Idk why but the input is swapped...
        let source = data.next().unwrap().parse::<u64>().unwrap();
        let destination = data.next().unwrap().parse::<u64>().unwrap();
        let range = data.next().unwrap().parse::<u64>().unwrap();
        Self::new(destination, source, range)
    }
}

enum DataType {
    Seed,
    SeedToSoil,
    SoilToFertiliser,
    FertiliserToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

pub fn process(input: &str) -> u32 {
    let mut seeds = vec![];
    let mut seed_to_soil = vec![];
    let mut soil_to_fertiliser = vec![];
    let mut fertiliser_to_water = vec![];
    let mut water_to_light = vec![];
    let mut light_to_temperature = vec![];
    let mut temperature_to_humidity = vec![];
    let mut humidity_to_location = vec![];

    let mut current_data_type = DataType::Seed;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds:") {
            let (_, _seeds) = line.split_once("seeds:").unwrap();
            let mut _seeds = _seeds
                .trim()
                .split(" ")
                .map(|seed| seed.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            seeds.append(&mut _seeds);
            continue;
        }

        if line.starts_with("seed-to-soil map:") {
            current_data_type = DataType::SeedToSoil;
            continue;
        }

        if line.starts_with("soil-to-fertilizer map:") {
            current_data_type = DataType::SoilToFertiliser;
            continue;
        }

        if line.starts_with("fertilizer-to-water map:") {
            current_data_type = DataType::FertiliserToWater;
            continue;
        }

        if line.starts_with("water-to-light map:") {
            current_data_type = DataType::WaterToLight;
            continue;
        }

        if line.starts_with("light-to-temperature map:") {
            current_data_type = DataType::LightToTemperature;
            continue;
        }

        if line.starts_with("temperature-to-humidity map:") {
            current_data_type = DataType::TemperatureToHumidity;
            continue;
        }

        if line.starts_with("humidity-to-location map:") {
            current_data_type = DataType::HumidityToLocation;
            continue;
        }

        match current_data_type {
            DataType::Seed => {
                // do nothing
            }
            DataType::SeedToSoil => seed_to_soil.push(DestinationToSource::from(line)),
            DataType::SoilToFertiliser => soil_to_fertiliser.push(DestinationToSource::from(line)),
            DataType::FertiliserToWater => {
                fertiliser_to_water.push(DestinationToSource::from(line))
            }
            DataType::WaterToLight => water_to_light.push(DestinationToSource::from(line)),
            DataType::LightToTemperature => {
                light_to_temperature.push(DestinationToSource::from(line))
            }
            DataType::TemperatureToHumidity => {
                temperature_to_humidity.push(DestinationToSource::from(line))
            }
            DataType::HumidityToLocation => {
                humidity_to_location.push(DestinationToSource::from(line))
            }
        }
    }

    seeds
        .iter()
        .map(|&seed| {
            get_location(
                seed,
                &seed_to_soil,
                &soil_to_fertiliser,
                &fertiliser_to_water,
                &water_to_light,
                &light_to_temperature,
                &temperature_to_humidity,
                &humidity_to_location,
            )
        })
        .min()
        .unwrap() as u32
}

fn lookup_source(destination: u64, map: &Vec<DestinationToSource>) -> u64 {
    let source = map.iter().find_map(|map| map.get_source(destination));
    match source {
        Some(source) => source,
        None => destination,
    }
}

fn get_location(
    seed: u64,
    seed_to_soil: &Vec<DestinationToSource>,
    soil_to_fertiliser: &Vec<DestinationToSource>,
    fertiliser_to_water: &Vec<DestinationToSource>,
    water_to_light: &Vec<DestinationToSource>,
    light_to_temperature: &Vec<DestinationToSource>,
    temperature_to_humidity: &Vec<DestinationToSource>,
    humidity_to_location: &Vec<DestinationToSource>,
) -> u64 {
    let soil = lookup_source(seed, seed_to_soil);
    let fertiliser = lookup_source(soil, soil_to_fertiliser);
    let water = lookup_source(fertiliser, fertiliser_to_water);
    let light = lookup_source(water, water_to_light);
    let temperature = lookup_source(light, light_to_temperature);
    let humidity = lookup_source(temperature, temperature_to_humidity);
    let location = lookup_source(humidity, humidity_to_location);

    location
}

#[cfg(test)]
mod tests {
    use super::{process, DestinationToSource};

    #[test]
    fn can_process_input() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let result = process(input);

        assert_eq!(result, 35);
    }

    #[test]
    fn can_get_source() {
        let map = DestinationToSource::new(50, 98, 2);
        assert_eq!(map.get_source(50), Some(98));
        assert_eq!(map.get_source(51), Some(99));
        assert_eq!(map.get_source(100), None);

        let map = DestinationToSource::new(52, 50, 48);
        for i in 0..map.range {
            let source = 50 + i;
            let destination = 52 + i;
            assert_eq!(map.get_source(destination), Some(source));
        }

        assert_eq!(map.get_source(98), Some(96));

        let map = DestinationToSource::from("1 0 69");
        assert_eq!(map.get_source(48), Some(47));
    }
}
