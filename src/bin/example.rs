
#[derive(Clone)]
struct FreshRange {
    lower: u128,
    upper: u128
}

impl FreshRange {
    fn new(lower: u128, upper: u128) -> Self {
        FreshRange { lower, upper }
    }

    fn len(&self) -> u128 {
        if self.upper < self.lower {
            println!("Range Error: {}-{}", self.lower, self.upper);
            panic!("WTF")
        }
        self.upper - self.lower + 1
    }

    /// where b is the range being passed as a reference 
    fn check_overlap(&self, b: &FreshRange) -> RangeOverlap {
        if self == b {
            return RangeOverlap::AEqualsB
        }
        else if self.lower > b.lower 
        && self.upper < b.upper {
            return RangeOverlap::AInsideB
        }
        else if self.lower < b.lower 
        && self.upper > b.upper {
            return RangeOverlap::AContainsB
        }
        else if self.lower < b.upper
        && self.lower > b.lower {
            return RangeOverlap::AStartsInB
        } 
        else if self.upper < b.upper
        && self.upper > b.lower {
            return RangeOverlap::AEndsInB
        } 
        else {
            RangeOverlap::None
        }
    }
}

impl PartialEq for FreshRange {
    fn eq(&self, other: &Self) -> bool {
        self.lower == other.lower
        && self.upper == other.upper
    }
}

#[derive(Debug)]
enum RangeOverlap {
    AContainsB,
    AInsideB,
    AEndsInB,
    AStartsInB,
    AEqualsB,
    None,
}

fn main() {
   let n = FreshRange::new(162232350328601, 162461408006849)
      .check_overlap(&FreshRange::new(161774338930928, 162461408006849));

   println!("{:?}", n);
}
