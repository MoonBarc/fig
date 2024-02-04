//! Generic Register Allocator

use std::{collections::HashMap, ops::Range, hash::{self, Hasher, Hash}};

use crate::be::ir::Register;

use super::ir::{IrBlock, IrOperand};

pub struct RegisterAllocator {
    registers: u8
}

impl RegisterAllocator {
    pub fn new(n_reg: u8) -> Self {
        Self { registers: n_reg }
    } 

    pub fn allocate_for(&self, b: &mut IrBlock) {
        println!("ralloc!");
        let mut ranges = HashMap::<IrOperand, Range<usize>>::new();
        let mut graph = InterferenceGraph::new();
        // determine ranges!
        for (ln, i) in b.ops.iter().enumerate() {
            // is it declaring anything?
            if let Some(into_reg) = &i.result_into {
                // update its live times
                let range_of_this_line = ln..ln+1;
                let times = ranges.get(&into_reg).unwrap_or(&range_of_this_line);
                // variable updated
                ranges.insert(into_reg.clone(), times.start..ln+1);
            }

            // is it used anywhere?
            for o in &i.ops {
                let range_of_this_line = ln - 1..ln;
                let times = ranges.get(&o).unwrap_or(&range_of_this_line);
                let s = times.start;
                // variable used
                ranges.insert(o.clone(), s..ln);
            }

            // CONSTRUCT *THE* GRAPH
            for (instr, range) in &ranges {
                // add edges for each register that is live at the same time
                for (instr2, range2) in &ranges {
                    if range.contains(&range2.start) || range2.contains(&range.start) {
                        graph.add_edge(instr, instr2);
                    }
                }
            }
        }

        let mut to_delete = vec![];

        // delete nodes with less than n edges
        for (instr, node) in &graph.nodes {
            if node.edges.len() < self.registers as usize {
                to_delete.push(instr.clone());
            }
        }

        for ops in to_delete {
            let connections_to_remove =
                std::mem::take(&mut graph.nodes.get_mut(&ops));

            let mut to_delete = vec![];

            for (i, _) in connections_to_remove.iter().enumerate() {
                to_delete.push(i);
            }

            for i in to_delete.iter().rev() {
                graph.nodes.get_mut(&ops)
                    .unwrap()
                    .edges
                    .remove(*i);
            }
        }

        let mut colormap = HashMap::new();
        loop {
            let mut need_to_spill = false;
            // color the graph
            for (operand, node) in &graph.nodes {
                let mut colors = vec![false; self.registers as usize];
                for neigh in &node.edges {
                    if let Some(c) = colormap.get(neigh).unwrap_or(&None) {
                        colors[*c as usize] = true;
                        println!("c");
                    }
                }
                let mut color = None;
                for (i, col) in colors.iter().enumerate() {
                    if !col {
                        color = Some(i as Register);
                        break;
                    }
                }
                if color == None {
                    need_to_spill = true;
                }

                colormap.insert(operand.clone(), color);
            }

            if !need_to_spill {
                break
            } else {
                todo!("REGISTER SPILLING REQUIRED!!!!")
            }
        }

        println!("Color Map: {:?}", colormap);

        let g = move |op: &IrOperand| {
            IrOperand::Register(colormap.get(op).unwrap().unwrap())
        };

        // APPLY!
        for instr in &mut b.ops { 
            if let Some(ref mut into) = instr.result_into {
                *into = g(into);
            }
            for op in &mut instr.ops {
                *op = g(op);
            }
        }
        println!("Finished coloring!");
    }
}

pub trait RegAllocProfile {
    fn make() -> RegisterAllocator;
}

pub struct InterferenceGraph {
    pub nodes: HashMap<IrOperand, RegisterNode>
}

impl InterferenceGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new()
        }
    }

    fn ensure_init(&mut self, a: &IrOperand) -> &mut RegisterNode {
        if self.nodes.get(a).is_none() {
            // initialize a new node
            let mut hash = std::collections::hash_map::DefaultHasher::new();
            a.hash(&mut hash);
            let hash = hash.finish();
            println!("init {:?} -- {:?}", a, hash);
            self.nodes.insert(a.clone(), RegisterNode { edges: vec![], color: None });
        }
        self.nodes.get_mut(a).unwrap()
    }

    pub fn add_edge(&mut self, aa: &IrOperand, bb: &IrOperand) {
        let a = self.ensure_init(aa);
        a.edges.push(bb.clone());
        let b = self.ensure_init(aa);
        b.edges.push(aa.clone());
    }
}

pub struct RegisterNode {
    pub edges: Vec<IrOperand>,
    pub color: Option<u8> // = register
}
