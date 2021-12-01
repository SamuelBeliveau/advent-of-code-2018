pub fn solve_11_a() {
    let grid_serial_number = 4172u32;

    let mut power_levels = vec![vec![0; 300]; 300];

    let mut max_power_level = 0;
    let mut max_coordinates = String::new();

    for y in 1..=300 {
        for x in 1..=300 {
            power_levels[y - 1][x - 1] = get_power_level(x as u32, y as u32, grid_serial_number);
        }
    }

    for square_size in 1..=300 {
        println!("Testing square size of {}...", square_size);
        for y in 1..=300 - square_size - 1 {
            for x in 1..=300 - square_size - 1 {
                let mut total_power_level = 0;

                for yy in 0..=square_size - 1 {
                    for xx in 0..=square_size - 1 {
                        total_power_level += power_levels[y + yy - 1][ x + xx - 1];
                    }
                }

                if total_power_level > max_power_level {
                    max_power_level = total_power_level;
                    max_coordinates = format!("{},{}", x, y);
                }
            }
        }
        println!("So far, max coordinates is {} and power is {}", max_coordinates, max_power_level);
    }

    println!("Max coordinates is {} and power is {}", max_coordinates, max_power_level);
}

fn get_rack_id(x: u32) -> u32 {
    x + 10
}

fn get_power_level(x: u32, y: u32, serial_number: u32) -> i32 {
    let rack_id = get_rack_id(x);
    let mut power_level: i32 = (rack_id * y) as i32;
    power_level += serial_number as i32;
    power_level *= rack_id as i32;
    power_level = extract_hundreds(power_level) as i32;
    power_level -= 5;
    power_level
}

fn extract_hundreds(power_level: i32) -> u32 {
    let chars: Vec<char> = power_level.to_string().chars().collect();
    if chars.len() < 3 {
        0
    } else {
        chars[chars.len() - 3].to_digit(10).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_hundreds() {
        assert_eq!(extract_hundreds(987), 9);
        assert_eq!(extract_hundreds(54), 0);
        assert_eq!(extract_hundreds(1234532), 5);
        assert_eq!(extract_hundreds(0), 0);
    }

    #[test]
    fn test_get_power_level() {
        assert_eq!(get_power_level(122, 79, 57), -5);
        assert_eq!(get_power_level(217, 196, 39), 0);
        assert_eq!(get_power_level(101, 153, 71), 4);
    }
}
