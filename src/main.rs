fn main() {
    let new_b = next(0b_0000_0000_0010_0000_0111_0000_0000_0000);
    print(new_b);
}

fn next(b: u64) -> u64 {
    (0..64).fold(0, |n, ij| {
        let (i, j) = (ij / 8, ij % 8);
        let c = (b >> ij) & 1;
        let l = (-1..=1).fold(0, |l, dx| (-1..=1).fold(l, |l, dy| 
            if dx == 0 && dy == 0 { l } else { l + ((b >> (((i as i8 + dx).rem_euclid(8) * 8 + (j as i8 + dy).rem_euclid(8)) as u8)) & 1) }
        ));
        n | if c == 1 && (l == 2 || l == 3) || c == 0 && l == 3 { 1 << ij } else { 0 }
    })
}

fn print(b: u64) {
    (0..8).for_each(|i| { 
        (0..8).for_each(|j| print!("{}", (b >> (i * 8 + j)) & 1));
        println!();
    });
}
