const SIDE_SIZE: usize = 16;

fn main() {
    let mut points = vec![(0.5, 0.2), (0.8, 0.5), (0.3, 0.8), (0.6, 1.1), (0.9, 1.4)];

    show("Original", &points);

    chaikin_smoothing(&mut points, 2);

    show("Smoothed", &points);
}

fn show(label: &str, points: &Vec<(f64, f64)>) {
    println!("{}", label);

    let mut result = String::from("");

    for i in 0..SIDE_SIZE {
        result += "\n";
        let y = i as f64 / 10.0;

        for j in 0..SIDE_SIZE {
            let x = j as f64 / 10.0;

            let e = 0.05;
            let exists = points
                .iter()
                .any(|&p| (p.0 - x).abs() <= e && (p.1 - y).abs() <= e);

            result += if exists { "*" } else { "." };
        }
    }

    println!("{}\n", result);
}

fn chaikin_smoothing(points: &mut Vec<(f64, f64)>, iterations: i32) {
    for _ in 1..=iterations {
        if points.len() < 2 {
            break;
        }

        let mut temp = vec![points[0]];

        let points_len = points.len();
        for j in 1..points_len {
            let prev = points[j - 1];
            let current = points[j];

            temp.push((
                0.75 * prev.0 + 0.25 * current.0,
                0.75 * prev.1 + 0.25 * current.1,
            ));

            temp.push((
                0.25 * prev.0 + 0.75 * current.0,
                0.25 * prev.1 + 0.75 * current.1,
            ));
        }

        temp.push(points[points.len() - 1]);
        *points = temp;
    }
}
