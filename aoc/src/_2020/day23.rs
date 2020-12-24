pub struct Runner {
    pub input: String,
}

impl crate::Solution for Runner {
    fn run_a(&self) -> String {
        let seed: Vec<usize> = self
            .input
            .trim()
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        let mut cups = [0; 10];
        for (i, v) in seed.iter().enumerate() {
            cups[*v] = seed[(i + 1) % seed.len()];
        }
        let mut cur = seed[0];
        for _ in 0..100 {
            cur = round(&mut cups, cur);
        }
        cur = 1;
        let mut out = String::from("");
        for _ in 0..8 {
            out = format!("{}{}", out, cups[cur]);
            cur = cups[cur];
        }
        out
    }

    fn run_b(&self) -> String {
        let seed: Vec<usize> = self
            .input
            .trim()
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        let mut cups = [0; 1000001];
        for (i, v) in seed.iter().enumerate() {
            cups[*v] = seed[(i + 1) % seed.len()];
        }
        for i in 11..1000001 {
            cups[i - 1] = i
        }
        cups[1000000] = seed[0];
        cups[*seed.last().unwrap()] = 10;
        let mut cur = seed[0];
        for _ in 0..10000000 {
            cur = round(&mut cups, cur);
        }
        (cups[1] * cups[cups[1]]).to_string()
    }
}

fn round(cups: &mut [usize], cur: usize) -> usize {
    let next = cups[cur];
    cups[cur] = cups[cups[cups[next]]];
    let mut tgt = if cur > 0 { cur - 1 } else { cups.len() - 1 };
    loop {
        let mut brk = true;
        if tgt == 0 {
            tgt = cups.len() - 1;
            brk = false;
        } else if tgt == next || tgt == cups[next] || tgt == cups[cups[next]] {
            tgt -= 1;
            brk = false;
        }
        if tgt == 0 {
            tgt = cups.len() - 1;
            brk = false;
        }
        if brk {
            break;
        }
    }
    cups[cups[cups[next]]] = cups[tgt];
    cups[tgt] = next;
    cups[cur]
}
