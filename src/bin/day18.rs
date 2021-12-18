use std::{fs, time};
use itertools::Itertools;
use crate::Node::{Cons, Leaf};

#[derive(Debug, Clone)]
pub enum Node {
    Leaf(i32),
    Cons(Box<Node>, Box<Node>),
}

impl Node {
    // Ok... am I being daft? Is there an easier way to parse this?
    fn new(s: &str) -> Node {
        let s = s.trim();
        // Store the nodes we have, could be partially completed nodes.
        // The final entry will be the complete node.
        let mut stack: Vec<Node> = Vec::new();
        // Val stores the numbers we see so far that will be part of a leaf.
        let mut val: Option<i32> = None;
        for x in s.chars() {
            match x {
                '[' => (), // We can ignore this since we assume input is well formed.
                ']' => {
                    let right = if val.is_some() {
                        let v = val.unwrap();
                        val = None;
                        Leaf(v)
                    } else { stack.pop().unwrap() };
                    let left = stack.pop().unwrap();
                    stack.push(Cons(Box::from(left), Box::from(right)))
                }
                ',' => {
                    if val.is_some() {
                        let v = val.unwrap();
                        val = None;
                        stack.push(Leaf(v));
                    }
                }
                n => {
                    let x1 = String::from(n).parse::<i32>().unwrap();
                    val = match val {
                        None => Some(x1),
                        Some(v) => Some((v * 10) + x1)
                    };
                }
            }
        }
        assert_eq!(1, stack.len()); // Check our assumption that input is valid.
        stack.pop().unwrap()
    }

    fn sum(&self, other: &Node) -> Node {
        Node::Cons(Box::from(self.clone()), Box::from(other.clone())).reduce()
    }

    fn unwrap(self) -> i32 {
        match self {
            Leaf(x) => x,
            _ => panic!("Trying to unwrap a cons that is not leaf node"),
        }
    }

    fn add_to_leftmost(self, val: i32) -> Node {
        match self {
            Node::Leaf(n) => Node::Leaf(n + val),
            Node::Cons(left, right) => Node::Cons(Box::from(left.add_to_leftmost(val)), right)
        }
    }

    fn add_to_rightmost(self, val: i32) -> Node {
        match self {
            Node::Leaf(n) => Node::Leaf(n + val),
            Node::Cons(left, right) => Node::Cons(left, Box::from(right.add_to_rightmost(val)))
        }
    }

    fn split(self) -> (Node, bool) {
        match self {
            Node::Leaf(n) => {
                let res = if n >= 10 { Cons(Box::from(Leaf(n / 2)), Box::from(Leaf((n as f64 / 2 as f64).ceil() as i32))) } else { Leaf(n) };
                if n > 10 {
                    // dbg!(&self, &res);
                }
                (res, n >= 10)
            }
            Cons(left, right) => {
                let (l, l_changed) = left.clone().split();
                if l_changed {
                    return (Cons(Box::from(l), right), true);
                }
                let (r, r_changed) = right.clone().split();
                if r_changed {
                    return (Cons(left, Box::from(r)), true);
                }
                (Cons(Box::from(l), Box::from(r)), false)
            }
        }
    }

    // [[6,[5,[4,[3,2]]]],1] becomes [[6,[5,[7,0]]],3]
    // Note: This is really complex and confused me several times. Maybe there is a cleaner way to
    // implement this.
    fn explode(self, depth: i8) -> (Node, i32, i32, bool) {
        match self {
            n @ Node::Leaf(_) => (n, 0, 0, false),
            Node::Cons(left, right) => {
                if depth == 0 {
                    (Leaf(0), left.unwrap(), right.unwrap(), true)
                } else {
                    // Try transforming the left
                    let (new_left, add_l, add_r, left_expoded) = left.clone().explode(depth - 1);
                    if left_expoded {
                        let right = if add_r != 0 {
                            // println!("Adding {} to right most on left branch", add_r);
                            right.add_to_leftmost(add_r)
                        } else { *right };
                        // dbg!("Pivot left", &new_left, &right);
                        return (Node::Cons(Box::from(new_left), Box::from(right)), add_l, 0, left_expoded);
                    }

                    // Left side hasn't changed.
                    // Try doing the right
                    let (new_right, add_l, add_r, right_exploded) = right.explode(depth - 1);
                    if right_exploded {
                        let left = if add_l != 0 {
                            // println!("Adding {} to right most on left branch", add_l);
                            left.add_to_rightmost(add_l)
                        } else { *left };
                        // dbg!("Pivot right", &left, &new_right);
                        return (Node::Cons(Box::from(left), Box::from(new_right)), 0, add_r, right_exploded);
                    }

                    // Else just return
                    (Node::Cons(Box::from(new_left), Box::from(new_right)), 0, 0, false)
                }
            }
        }
    }

    fn reduce(self) -> Node {
        let v = self.reduce_once();
        if !v.1 {
            return v.0;
        } else {
            v.0.reduce()
        }
    }

    fn reduce_once(self) -> (Node, bool) {
        let x = self.clone().explode(4);
        if x.3 {
            (x.0, x.3)
        } else {
            // (x.0, false) // Use this if you want to see the result before splitting.
            self.split()
        }
    }

    fn calculate_magnitude(&self) -> i32 {
        match self {
            Leaf(n) => *n,
            Cons(left, right) => 3 * left.calculate_magnitude() + 2 * right.calculate_magnitude(),
        }
    }
}

fn main() {
    // Wow this question statement is long. I am just naively implementing what is given.
    // This is very reminiscent of self balancing trees.
    // This code has a lot of things that can be improved. For one, there's a lot of copying of
    // things around, which I really don't think needs to happen.
    let s: String = fs::read_to_string("./src/input18.txt").unwrap();
    let elements = s.split("\n")
        .filter(|x| !x.is_empty())
        .map(|x1| { Node::new(x1) })
        .collect_vec();

    // Part 1
    let ans = elements.clone().into_iter().reduce(|b, x2| { b.sum(&x2) }).unwrap();
    let ans = ans.calculate_magnitude();
    dbg!(ans);

    // Part 2. The program took some time for this: 4.75s on an 11g i5 processor.
    // I think it can be quicker.
    let now = time::Instant::now();
    let ans = elements.iter().combinations(2).fold(0, |b, x2| {
        let fst = x2.first().unwrap().clone();
        let snd = x2.last().unwrap().clone();
        let v = fst.sum(snd).calculate_magnitude();
        v.max(b)
    });
    dbg!(now.elapsed());
    dbg!(ans);
}

