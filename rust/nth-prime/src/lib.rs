fn is_prime(n: u32) -> bool {
    let max = (n as f64).sqrt() as u32;
    for value in 2..max + 1 {
        if n % value == 0 {
            return false;
        }
    }

    return n > 1;
}

pub fn nth(n: u32) -> u32 {
    let mut number = 2u32;
    let mut vector = vec![];
    loop {
        if is_prime(number) {
            vector.push(number);
        }

        match vector.get(n as usize) {
            Some(element) => return *element,
            None => (),
        }

        number = number + 1;
    }
}
