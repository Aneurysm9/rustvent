use itertools::sorted;

pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        return self
            .input
            .split("\n")
            .filter(|v| v.len() > 0)
            .map(|v| to_int(v))
            .fold(std::isize::MIN, |a, b| a.max(b))
            .to_string();
    }

    fn run_b(&self) -> String {
        let mut prev = 0;
        for val in sorted(
            self.input
                .split("\n")
                .filter(|v| v.len() > 0)
                .map(|v| to_int(v)),
        ) {
            if prev != 0 && prev + 1 != val {
                prev = prev + 1;
                break;
            }
            prev = val;
        }
        return prev.to_string();
    }
}

fn to_int(seat: &str) -> isize {
    let mut out = seat.replace("F", "0");
    out = out.replace("L", "0");
    out = out.replace("B", "1");
    out = out.replace("R", "1");
    isize::from_str_radix(&out, 2).unwrap()
}
