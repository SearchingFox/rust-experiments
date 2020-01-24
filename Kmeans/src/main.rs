// extern crate rand;
// extern crate plotlib;
use rand::Rng;
use rand::distributions::{Distribution, Standard};
use rand::seq::SliceRandom;
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

struct Kmeans {
    clcn: Vec<Vec<f64>>,
    lbs: Vec<u8>,
    ncls: u8,
    tol: f64
}

// fn d2(p1: PPoint, p2: PPoint) -> f64 {
//     return (p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)
// }

fn to_list(ps: Vec<PPoint>) -> Vec<(f64, f64)> {
    ps.into_iter().map(|p| {(p.x, p.y)}).collect()
}

fn norm(x: Vec<f64>) -> f64 {
    x.iter().map(|i| i * i).sum::<f64>().sqrt()
}

fn sub(a: Vec<f64>, b: Vec<f64>) -> Vec<f64> {
    let mut z: Vec<f64> = vec![0.0];
    for (i, (aval, bval)) in a.iter().zip(&b).enumerate() {
        z[i] = aval - bval;
    }
    z
}

fn predict(km: Kmeans, x: Vec<Vec<f64>>) -> Vec<u8> {
    let size = x.len();
    //let clcn = vec![vec![0.0; 6]; size];
    let mut labels: Vec<u8> = vec![0; size];
    let mut md: Vec<f64> = Vec::new();
    for i in 0..size {
        md[i] = norm(sub(x[i].clone(), km.clcn[0].clone()));
    }
    
    for cl in 1..km.ncls {
        for i in 0..size {
            let dist = norm(sub(x[i].clone(), km.clcn[cl as usize].clone()));
            if dist < md[i] {
                md[i]     = dist;
                labels[i] = cl;
            }
        }
    }

    return labels;
}

fn fit(mut km: Kmeans, x: Vec<Vec<f64>>) -> Kmeans {
    let mut rng = &mut rand::thread_rng();
    let smp = x.len();
    let clcn_n: Vec<i32> = (0..smp as i32).collect::<Vec<i32>>().as_slice().choose_multiple(&mut rng, km.ncls as usize).cloned().collect();
    println!("{:?}", clcn_n);
    km.clcn = Vec::new();
    for i in 0..clcn_n.len() {
        km.clcn.push((x[clcn_n[i] as usize]).clone());
    }
    km.lbs = vec![0; smp];
    let mut md = vec![0.0; smp];

    loop {
        for i in 0..smp {
            md[i] = norm(sub(x[i].clone(), km.clcn[0].clone()));
            km.lbs[i] = 0
        }

        for cl in 1..km.ncls {
            for i in 0..smp {
                let dist = norm(sub(x[i].clone(), km.clcn[cl as usize].clone()));
                if dist < md[i] {
                    md[i] = dist;
                    km.lbs[i] = cl
                }
            }
        }

        //nc = np.array([X[self.lbs == i].sum(axis=0) / X[self.lbs == i].shape[0] ])
        let nc: Vec<Vec<f64>> = Vec::new();
        // for i in 0..km.ncls {
        //     if km.lbs.contains(&i) {
        //         nc.push(x[i as usize].iter().map(|l| l.sum::<f64>() / (x[i as usize].len() as f64)).collect());
        //     }
        // }
        //if (abs(sub(nc, clcn)) < self.tol).all(): break
        if true {
            break
        }
        km.clcn = nc;
    }

    return km;
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut arr: Vec<PPoint> = Vec::new();
    for _ in 0..100 {
        arr.push(rng.gen());
    }

    let mut km = Kmeans {
        clcn: Vec::new(),
        lbs: Vec::new(),
        ncls: 3,
        tol: 0.001
    };

    fit(km, vec![vec![1.0, 2.0, 100.0, 12.0, 43.0, 463.0, 25.0, 4.20, 6.0, 86.0],
                vec![1.0, 2.0, 100.0, 12.0, 43.0, 463.0, 25.0, 4.20, 6.0, 86.0],
                vec![1.0, 2.0, 100.0, 12.0, 43.0, 463.0, 25.0, 4.20, 6.0, 86.0],
                vec![1.0, 2.0, 100.0, 12.0, 43.0, 463.0, 25.0, 4.20, 6.0, 86.0],
                vec![1.0, 2.0, 100.0, 12.0, 43.0, 463.0, 25.0, 4.20, 6.0, 86.0],
                vec![1.0, 2.0, 100.0, 12.0, 43.0, 463.0, 25.0, 4.20, 6.0, 86.0],
                vec![1.0, 2.0, 100.0, 12.0, 43.0, 463.0, 25.0, 4.20, 6.0, 86.0],
                vec![1.0, 2.0, 100.0, 12.0, 43.0, 463.0, 25.0, 4.20, 6.0, 86.0],
                vec![1.0, 2.0, 100.0, 12.0, 43.0, 463.0, 25.0, 4.20, 6.0, 86.0]]);

    let s: Scatter = Scatter::from_slice(&to_list(arr)[..]).style(&Style::new()
        .marker(Marker::Square)
        .colour("#DD3355"));
    let v = ContinuousView::new()
        .add(&s)
        .x_range(0., 10.)
        .y_range(0., 10.);
    Page::single(&v).save("scatter.svg").unwrap();
}