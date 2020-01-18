extern crate rand;
extern crate plotlib;

use rand::Rng;
use rand::distributions::{Distribution, Standard};
use plotlib::page::Page;
use plotlib::scatter::{Scatter, Style};
use plotlib::style::{Marker, Point};
use plotlib::view::ContinuousView;

#[derive(Debug, Clone, Copy)]
struct PPoint {
    x: f64,
    y: f64
}

impl Distribution<PPoint> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PPoint {
        let (rand_x, rand_y) = rng.gen::<(f64, f64)>();
        PPoint {
            x: rand_x * 10.0,
            y: rand_y * 10.0,
        }
    }
}

// struct Kmeans {
//     clcn = None let clcn = vec![vec![0.0; 6]; size];
//     lbs = None let mut labels: Vec<u8> = vec![0; size];
//     ncls = ncls
//     tol = 0.0001
// }

// fn d2(p1: PPoint, p2: PPoint) -> f64 {
//     return (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)
// }

fn to_list(ps: Vec<PPoint>) -> Vec<(f64, f64)> {
    return ps.into_iter().map(|p| {(p.x, p.y)}).collect()
}

fn norm(x: Vec<f64>) -> f64 {
    return x.iter().map(|i| i * i).sum::<f64>().sqrt();
}

fn sub(a: Vec<u32>, b: Vec<f64>) -> Vec<f64> {
    let mut z: Vec<f64> = vec![0.0];
    for (i, (aval, bval)) in a.iter().zip(&b).enumerate() {
        z[i] = *aval as f64 - bval;
    }
    z
}

fn predict(x: Vec<Vec<u32>>, ncls: u32) -> Vec<u32> {
    let size = x.len();
    let clcn = vec![vec![0.0; 6]; size];
    let mut labels: Vec<u32> = vec![0; size];
    let mut md: Vec<f64> = Vec::new();
    for i in 0..size {
        md[i] = norm(sub(x[i], clcn[0]));
    }
    
    for cl in 1..ncls {
        for i in 0..size {
            let dist = 1.0; // norm(X[i] - self.clcn[cl])
            if dist < md[i] {
                md[i]     = dist;
                labels[i] = cl;
            }
        }
    }

    return labels;
}

// fn fit(n: i32, data: &[PPoint]) {
//     let clasters =
// }

fn main() {
    let mut rng = rand::thread_rng();
    let mut arr: Vec<PPoint> = Vec::new();
    for _ in 0..100 {
        arr.push(rng.gen());
    }



    let s: Scatter = Scatter::from_slice(&to_list(arr)[..]).style(&Style::new()
        .marker(Marker::Square)
        .colour("#DD3355"));
    let v = ContinuousView::new()
        .add(&s)
        .x_range(0., 10.)
        .y_range(0., 10.);
    Page::single(&v).save("scatter.svg").unwrap();
}