use std::collections::{HashMap, HashSet, VecDeque};

pub fn bfs(
    graph: &HashMap<usize, Vec<usize>>,
    start: usize,
    goal: usize,
) -> Option<Vec<usize>> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut parent_map = HashMap::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == goal {
            let mut path = Vec::new();
            let mut node = goal;

            while let Some(&parent) = parent_map.get(&node) {
                path.push(node);
                node = parent;
            }

            path.push(start);
            path.reverse();
            return Some(path);
        }

        if let Some(neighbors) = graph.get(&current) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                    parent_map.insert(neighbor, current);
                }
            }
        }
    }
    None
}

pub fn find_route_with_waypoint(
    adjacency_list: &HashMap<usize, Vec<usize>>,
    start: usize,
    waypoints: Option<usize>,
    goal: usize,
) -> Option<Vec<usize>> {
    let mut route = Vec::new();

    if let Some(waypoint) = waypoints {
        if let Some(route_to_waypoint) = bfs(&adjacency_list, start, waypoint) {
            route.extend(route_to_waypoint);
        } else {
            return None;
        }
    }
    if let Some(route_from_last_waypoint_to_goal) =
        bfs(&adjacency_list, *route.last().unwrap_or(&start), goal)
    {
        route.extend(route_from_last_waypoint_to_goal);
        Some(route)
    } else {
        None 
    }
}












