use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Eq)]
pub struct Number {
	pub value: i32,
	pub predescessor : Option<RefNumber>
}

impl Number {
	fn new(value: i32) -> Number {
		Number {
			value,
			predescessor: None
		}
	}
}

impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        self.value == other.value
    }
}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

#[derive(Debug, Eq)]
pub struct RefNumber {
	pub refValue : Rc<RefCell<Number>>
}

impl Clone for RefNumber {
    fn clone(&self) -> RefNumber {
		RefNumber {
			refValue:  Rc::clone(&self.refValue)
		}
	}
}


impl PartialOrd for RefNumber {
    fn partial_cmp(&self, other: &RefNumber) -> Option<Ordering> {
        other.refValue.borrow().value.partial_cmp(&self.refValue.borrow().value)
    }
}

impl PartialEq for RefNumber {
    fn eq(&self, other: &RefNumber) -> bool {
        self.refValue.borrow().value == other.refValue.borrow().value
    }
}

impl Ord for RefNumber {
    fn cmp(&self, other: &RefNumber) -> Ordering {
        self.refValue.borrow().value.cmp(&other.refValue.borrow().value)
    }
}

impl Hash for RefNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.refValue.borrow().hash(state);
    }
}

fn main() {
	let mut all_state: HashSet<RefNumber> = HashSet::new();
    let value: RefNumber = RefNumber {refValue :Rc::new(RefCell::new(Number::new(42)))};
    let value2: RefNumber = RefNumber {refValue :Rc::new(RefCell::new(Number::new(19)))};

	all_state.insert(value.clone());
	all_state.insert(value2.clone());
	value2.refValue.borrow_mut().predescessor = Some(value.clone());
    let mut open_list: BinaryHeap<RefNumber> = BinaryHeap::new();
	open_list.push(value);
	open_list.push(value2);

	println!("value in open_list = {}", open_list.peek().unwrap().refValue.borrow().value);
	let refNumber = open_list.peek().unwrap().refValue.borrow();

	match &refNumber.predescessor {
		Some(val)	=> println!("the predescessor in open_list is = {}", val.refValue.borrow().value),
		None		=> {},
	}
}