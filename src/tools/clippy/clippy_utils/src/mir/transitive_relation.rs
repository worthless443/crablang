use crablangc_data_structures::fx::FxHashMap;
use crablangc_index::bit_set::HybridBitSet;
use crablangc_middle::mir;

#[derive(Default)]
pub(super) struct TransitiveRelation {
    relations: FxHashMap<mir::Local, Vec<mir::Local>>,
}

impl TransitiveRelation {
    pub fn add(&mut self, a: mir::Local, b: mir::Local) {
        self.relations.entry(a).or_default().push(b);
    }

    pub fn reachable_from(&self, a: mir::Local, domain_size: usize) -> HybridBitSet<mir::Local> {
        let mut seen = HybridBitSet::new_empty(domain_size);
        let mut stack = vec![a];
        while let Some(u) = stack.pop() {
            if let Some(edges) = self.relations.get(&u) {
                for &v in edges {
                    if seen.insert(v) {
                        stack.push(v);
                    }
                }
            }
        }
        seen
    }
}
