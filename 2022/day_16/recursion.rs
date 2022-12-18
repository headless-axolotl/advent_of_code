
type AdjacencyMatrix = Vec<Vec<i32>>;

pub struct Solver {
    start: usize,
    matrix: AdjacencyMatrix,
    pressures: Vec<i32>,
    visited: Vec<bool>
}

impl Solver {

    pub fn new(
        start: usize,
        matrix: AdjacencyMatrix,
        pressures: Vec<i32>
    ) -> Self {
        Solver {
            visited: vec![false; pressures.len()],
            start,
            matrix,
            pressures,
        }
    }

    pub fn solve(mut instance: Self, working_valves: &Vec<usize>) -> (i32, Self) {
        (
            instance.recurse(
                instance.start,
                26,
                working_valves
            ),
            instance
        )
    }

    fn recurse(&mut self, current_node: usize, time_left: i32, working_valves: &Vec<usize>) -> i32 {

        self.visited[current_node] = true;
        let mut answer = 0;

        for valve in working_valves {

            if self.visited[*valve] {
                continue;
            }

            let next_time = time_left - 1 - self.matrix[current_node][*valve];

            if(next_time <= 0) {
                continue;
            }
            
            let found_value = self.recurse(*valve, next_time, working_valves);
            answer = i32::max(answer, found_value);
        }

        self.visited[current_node] = false;

        answer + time_left * self.pressures[current_node]
    }

}
