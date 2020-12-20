use aoc_input::*;

fn main() {
    let input = get_input_txt();
    let (timestamp, buses) = parse(&input);
    let (bus, time) = get_next_bus(timestamp, &buses);
    println!("Part 1: {}", bus * time);
    let run_time = get_bus_run(&buses);
    println!("Part 2: {}", run_time);
}

fn parse(input: &str) -> (i64, Vec<Option<i64>>) {
    let mut lines = input.lines();
    let timestamp = lines.next().unwrap().parse().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().ok())
        .collect();
    (timestamp, buses)
}

fn get_next_bus(timestamp: i64, buses: &[Option<i64>]) -> (i64, i64) {
    let mut result = (0, i64::MAX);
    for entry in buses {
        if let Some(bus) = entry {
            let previous_num = timestamp / bus;
            let next = (previous_num + 1) * bus;
            let time_until = next - timestamp;
            if time_until < result.1 {
                result = (*bus, time_until)
            }
        }
    }
    result
}

fn get_bus_run(buses: &[Option<i64>]) -> i64 {
    let mut offset_buses = buses
        .iter()
        .enumerate()
        .filter(|(_, bus)| bus.is_some())
        .map(|(i, bus)| (i as i64, bus.unwrap()))
        .collect::<Vec<_>>();
    offset_buses.sort_by_key(|x| x.1);
    offset_buses.reverse();
    search_by_sieve(&offset_buses, 1, 1)
}

fn search_by_sieve(offset_buses: &[(i64, i64)], timestamp: i64, increment: i64) -> i64 {
    if offset_buses.is_empty() {
        return timestamp;
    }
    let mut timestamp = timestamp;
    let this_bus = offset_buses[0];
    println!("len {} bus {:?} increment {}", offset_buses.len(), this_bus, increment);
    loop {
            if (timestamp + this_bus.0) % this_bus.1 == 0 {
                return search_by_sieve(&offset_buses[1..], timestamp, increment * this_bus.1);
            }
        
        timestamp += increment;
    }
}

#[test]
fn example_input_1() {
    let input = "939
7,13,x,x,59,x,31,19";
    let (timestamp, buses) = parse(input);
    let (bus, time) = get_next_bus(timestamp, &buses);
    assert_eq!(295, bus * time);
}

#[test]
fn example_input_2() {
    let input = "939
7,13,x,x,59,x,31,19";
    let (_, buses) = parse(input);
    let time = get_bus_run(&buses);
    assert_eq!(1068781, time);
}
