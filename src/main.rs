use std::io::{self};
use std::collections::{HashSet,HashMap};

mod bfs;
mod dataset;

use crate::bfs::bfs;
use crate::dataset::read_file;
use crate::dataset::create_adjacency_list;
use crate::bfs::find_route_with_waypoint;

pub fn read_user_input(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse() {
            Ok(value) => return value,
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

fn main() {
    match read_file("euroroad.txt") {
        Ok(data) => {
            let adjacency_list = create_adjacency_list(&data.1);
            let start_vertex = read_user_input("Enter current location: ");
            let waypoint = read_user_input("Waypoint (enter '999999' if none): ");
            let goal_vertex = read_user_input("Where we goin: ");

            let waypoints = if waypoint == 999999 {
                None
            } else {
                Some(waypoint)
            };

            match find_route_with_waypoint(&adjacency_list, start_vertex, waypoints, goal_vertex) {
                Some(route) => {
                    println!("Route: {:?}", route);
                }
                None => {
                    println!("No valid route found.");
                }
            }
        }
        Err(e) => eprintln!("Error reading file: {}", e),
    };
}

#[cfg(test)]
mod tests {
    use crate::create_adjacency_list;
    use crate::bfs;
    #[test]
    fn no_path_test() {
        let small_graph = vec![(1,2),(2,4),(2,5),(3,6),(6,7)];
        let adjacency = create_adjacency_list(&small_graph);
        let start = 1;
        let goal = 7;
        let result = bfs(&adjacency, start, goal);
        let expected_result: Option<Vec<usize>> = None;

        assert_eq!(result, expected_result)
    }
    #[test]
    fn specific_path_test() {
        let small_graph = vec![(1,2),(2,3),(2,4),(3,6),(6,7),(4,5),(5,8)];
        let adjacency = create_adjacency_list(&small_graph);
        let start = 1;
        let goal = 8;
        let result = bfs(&adjacency, start, goal);
        let expected_result = Some(vec![1, 2, 4, 5, 8]);

        assert_eq!(result, expected_result)
    }

}














