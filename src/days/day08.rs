use crate::days::Day;
use crate::util::collection::CollectionExtension;
use crate::util::geometry::{Directions, Grid, Point};

pub const DAY8: Day = Day {
    puzzle1,
    puzzle2
};

fn puzzle1(input: &String) {
    let forest = parse_input(input).unwrap();

    let visible_trees = forest.get_visible_tree_count();
    println!("There are {} visible trees in this forest", visible_trees);
}

fn puzzle2(input: &String) {
    let forest = parse_input(input).unwrap();

    let best_score = forest.get_best_scenic_score().unwrap();
    println!("Best scenic score in this forest: {}", best_score);
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Forest {
    trees: Grid<i32>
}

impl Forest {
    fn get_visible_tree_count(&self) -> usize {
        self.trees.points().iter().filter(|p| self.is_tree_visible(p)).count()
    }

    fn is_tree_visible(&self, tree: &Point) -> bool {
        if let Some(height) = self.trees.get(tree) {
            self.trees.get_in_direction(tree, Directions::Top).iter().all(|v| v < &height) ||
            self.trees.get_in_direction(tree, Directions::Right).iter().all(|v| v < &height) ||
            self.trees.get_in_direction(tree, Directions::Bottom).iter().all(|v| v < &height) ||
            self.trees.get_in_direction(tree, Directions::Left).iter().all(|v| v < &height)
        } else {
            false
        }
    }

    fn get_visible_trees_from(&self, tree: &Point, direction: Directions) -> Vec<i32> {
        let mut result = vec![];

        if let Some(height) = self.trees.get(tree) {
            for tree in self.trees.get_in_direction(tree, direction) {
                result.push(tree);
                if tree >= height {
                    break;
                }
            }
        }

        result
    }

    fn get_scenic_score(&self, tree: &Point) -> usize {
        let top = self.get_visible_trees_from(tree, Directions::Top).len();
        let right = self.get_visible_trees_from(tree, Directions::Right).len();
        let bottom = self.get_visible_trees_from(tree, Directions::Bottom).len();
        let left = self.get_visible_trees_from(tree, Directions::Left).len();

        top * right * bottom * left
    }

    fn get_best_scenic_score(&self) -> Option<usize> {
        self.trees.points().iter().map(|p| self.get_scenic_score(p)).max()
    }
}

fn parse_input(input: &str) -> Result<Forest, String> {
    let lines = input.lines().collect::<Vec<_>>();
    let lens = lines.iter().map(|l| l.len()).collect::<Vec<_>>().deduplicate();
    if lens.len() != 1 {
        return Err(format!("Expected input lines to all have the same length, but got: {}", lens.iter().map(|l| l.to_string()).collect::<Vec<_>>().join(", ")));
    }

    Ok(Forest { trees: input.parse()? })
}

#[cfg(test)]
mod tests {
    use crate::days::day08::parse_input;

    const TEST_INPUT: &str = "\
        30373\n\
        25512\n\
        65332\n\
        33549\n\
        35390\n\
    ";

    #[test]
    fn test_parse_input() {
        let result = parse_input(TEST_INPUT);
        assert!(result.is_ok(), "Expected a success result");
    }

    #[test]
    fn test_is_tree_visible() {
        let forest = parse_input(TEST_INPUT).unwrap();
        assert_eq!(true, forest.is_tree_visible(&(0, 0).into()));
        assert_eq!(true, forest.is_tree_visible(&(1, 1).into()));
        assert_eq!(false, forest.is_tree_visible(&(1, 3).into()));
        assert_eq!(false, forest.is_tree_visible(&(3, 1).into()));
        assert_eq!(false, forest.is_tree_visible(&(2, 2).into()));
        assert_eq!(false, forest.is_tree_visible(&(3, 3).into()));
    }

    #[test]
    fn test_get_visible_tree_count() {
        let forest = parse_input(TEST_INPUT).unwrap();
        assert_eq!(21, forest.get_visible_tree_count());
    }

    #[test]
    fn test_get_scenic_score() {
        let forest = parse_input(TEST_INPUT).unwrap();
        assert_eq!(4, forest.get_scenic_score(&(2, 1).into()));
        assert_eq!(8, forest.get_scenic_score(&(2, 3).into()));
    }

    #[test]
    fn test_get_best_scenic_score() {
        let forest = parse_input(TEST_INPUT).unwrap();
        assert_eq!(Some(8), forest.get_best_scenic_score());
    }
}