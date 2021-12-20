use std::cmp::max;
use std::fmt;
use std::fmt::Formatter;

struct Polynomial2 {
    coefficients : (f64, f64, f64),
}

impl Polynomial2 {
    fn new(a : f64, b: f64, c:f64 ) -> Self {
        Polynomial2{coefficients :(a,b,c)}
    }

    fn eval(&self, x : f64) -> f64 {
        return x*x*self.coefficients.0 + x*self.coefficients.1 + self.coefficients.2
    }

    fn max_point(&self) -> f64 {
        return -self.coefficients.1 / (2.0*self.coefficients.0);
    }
}

impl fmt::Debug for Polynomial2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}x^2 + {}x + {}", self.coefficients.0, self.coefficients.1, self.coefficients.2)
    }
}


fn polynomial_curve_fitting(points : Vec::<(f64, f64)>) -> Polynomial2 {
    let avg_x : f64 = points.iter().map(|(x,y)| x).sum::<f64>() / points.len() as f64;
    let avg_y : f64 = points.iter().map(|(x,y)| y).sum::<f64>() / points.len() as f64;

    let avg_x2 : f64 = points.iter().map(|(x,y)| x*x).sum::<f64>() / points.len() as f64;
    let avg_x3 : f64 = points.iter().map(|(x,y)| x*x*x).sum::<f64>() / points.len() as f64;
    let avg_x4 : f64 = points.iter().map(|(x,y)| x*x*x*x).sum::<f64>() / points.len() as f64;

    let avg_xy : f64 = points.iter().map(|(x,y)| x*y).sum::<f64>() / points.len() as f64;
    let avg_x2y : f64 = points.iter().map(|(x,y)| x*x*y).sum::<f64>() / points.len() as f64;

    let sxx = avg_x2 - avg_x * avg_x;
    let sxy = avg_xy - avg_x * avg_y;
    let sxx2 = avg_x3 - avg_x * avg_x2;
    let sx2x2 = avg_x4 - avg_x2 * avg_x2;
    let sx2y = avg_x2y - avg_x2 * avg_y;


    let b = (sxy * sx2x2 - sx2y * sxx2) / (sxx * sx2x2 - sxx2 * sxx2);
    let c = (sx2y * sxx - sxy * sxx2) / (sxx * sx2x2 - sxx2 * sxx2);
    let a = avg_y - b * avg_x - c * avg_x2;
    Polynomial2::new(c,b,a)
}

fn simulate_shot(mut initial_speed: (i64, i64), target_start: (i64, i64), target_end: (i64, i64)) -> Option<i64> {
    let mut pos : (i64, i64) = (0, 0);
    let mut max_height = i64::MIN;
    while pos.0 <= target_end.0 && pos.1 >= target_start.1 {
        max_height = max(max_height, pos.1);
        if pos.0 >= target_start.0 && pos.1 <= target_end.1 {
            return Some(max_height);
        }
        pos.0 += initial_speed.0;
        pos.1 += initial_speed.1;
        if initial_speed.0 >= 1 {
            initial_speed.0 -= 1;
        }
        else if initial_speed.0 <= -1 {
            initial_speed.0 += 1;
        }
        initial_speed.1 -= 1;
    }
    None
}

fn part1_answer(target_start: (i64, i64), target_end: (i64, i64)) -> i64 {
    let mut max_y_xy_coords : (i64, (i64, i64)) = (i64::MIN, (0,0));
    for x in -100..100 {
        for y in -100..100 {
            if let Some(max_y) = simulate_shot((x,y), target_start, target_end) {
                if max_y > max_y_xy_coords.0 {
                    max_y_xy_coords.0 = max_y;
                    max_y_xy_coords.1 = (x,y);
                }
            }
        }
    }
    max_y_xy_coords.0
}

fn part2_answer(target_start: (i64, i64), target_end: (i64, i64)) -> u64 {
    let mut valid_pairs : u64 = 0;
    for x in -100..1000 {
        for y in -1000..100 {
            if let Some(max_y) = simulate_shot((x,y), target_start, target_end) {
                valid_pairs += 1;
            }
        }
    }
    valid_pairs
}


fn main() {
    // I know I naively bruteforced this one, but I literally don't care
    dbg!(part1_answer((244, -91), (303, -54)));
    dbg!(part2_answer((244, -91), (303, -54)));
}