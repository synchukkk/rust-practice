const WIDTH: usize = 11; // ширина (має бути непарною для симетрії)
const HEIGHT: usize = 11; // висота (має бути непарною для симетрії)

fn main() {
    let mut output = String::new();
    let mid = HEIGHT / 2;
    for i in 0..HEIGHT {
        let stars = if i <= mid {
            1 + 2 * i
        } else {
            1 + 2 * (HEIGHT - i - 1)
        };
        let spaces = (WIDTH - stars) / 2;
        for _ in 0..spaces {
            output.push(' ');
        }
        for _ in 0..stars {
            output.push('*');
        }
        output.push('\n');
    }
    print!("{}", output);
}
