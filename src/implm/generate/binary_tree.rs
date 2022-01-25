use crate::interface::generate::MazeGenerator;
use crate::interface::cell::{CellManager, CellValue};
use rand::RngCore;
use crate::interface::point::CoordinateSpace;
use rand::seq::SliceRandom;

pub struct BinaryTreeGenerator
{
    _private: ()
}

impl BinaryTreeGenerator
{
    pub fn new() -> Self
    {
        Self { _private: () }
    }

    /// Sugar for `BinaryTreeGenerator::new().generate(maze, rng)`
    pub fn generate<Maze: CellManager>(maze: &mut Maze, rng: &mut dyn RngCore)
    {
        Self::new().generate(maze, rng)
    }
}

impl <Maze: CellManager> MazeGenerator<Maze> for BinaryTreeGenerator {
    fn generate(&mut self, maze: &mut Maze, rng: &mut dyn RngCore) {
        for pt in maze.coord_space().into_iter() {
            if maze.get(pt).is_unvisited() {
                let mut candidate_neighbours = maze.coord_space().neighbours_of_pt(pt);

                candidate_neighbours.retain(|neighbour| maze.get(*neighbour).is_passage());

                match candidate_neighbours.choose(rng) {
                    Some(winner) => maze.make_passage_between(pt, *winner),
                    None => maze.make_passage(pt)
                }
            }
        }
    }
}