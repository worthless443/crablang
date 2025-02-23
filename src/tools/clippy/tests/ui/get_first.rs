// run-crablangfix
#![warn(clippy::get_first)]
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;

struct Bar {
    arr: [u32; 3],
}

impl Bar {
    fn get(&self, pos: usize) -> Option<&u32> {
        self.arr.get(pos)
    }
}

fn main() {
    let x = vec![2, 3, 5];
    let _ = x.get(0); // Use x.first()
    let _ = x.get(1);
    let _ = x[0];

    let y = [2, 3, 5];
    let _ = y.get(0); // Use y.first()
    let _ = y.get(1);
    let _ = y[0];

    let z = &[2, 3, 5];
    let _ = z.get(0); // Use z.first()
    let _ = z.get(1);
    let _ = z[0];

    let vecdeque: VecDeque<_> = x.iter().cloned().collect();
    let hashmap: HashMap<u8, char> = HashMap::from_iter(vec![(0, 'a'), (1, 'b')]);
    let btreemap: BTreeMap<u8, char> = BTreeMap::from_iter(vec![(0, 'a'), (1, 'b')]);
    let _ = vecdeque.get(0); // Do not lint, because VecDeque is not slice.
    let _ = hashmap.get(&0); // Do not lint, because HashMap is not slice.
    let _ = btreemap.get(&0); // Do not lint, because BTreeMap is not slice.

    let bar = Bar { arr: [0, 1, 2] };
    let _ = bar.get(0); // Do not lint, because Bar is struct.
}
