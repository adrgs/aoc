use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use z3::ast::Ast;
use z3::{Config, Context, Optimize, ast};

#[derive(Debug)]
struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    dx: i64,
    dy: i64,
    dz: i64,
}

fn part1(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    let test_lb = 200000000000000;
    let test_ub = 400000000000000;

    let mut hailstones = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        
        let (coords, vel) = line.split_once(" @ ").unwrap();

        let coords = coords.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let vel = vel.split(',').map(|x| x.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>();

        hailstones.push(Hailstone {
            x: coords[0],
            y: coords[1],
            z: coords[2],
            dx: vel[0],
            dy: vel[1],
            dz: vel[2],
        });
    }

    for i in 0..hailstones.len() {
        for j in i+1..hailstones.len() {
            let hailstone_a = &hailstones[i];
            let hailstone_b = &hailstones[j];

            let cfg = Config::new(); 
            let ctx = Context::new(&cfg); 
            let opt = Optimize::new(&ctx);

            let ta = ast::Real::new_const(&ctx, "ta");
            let tb = ast::Real::new_const(&ctx, "tb");

            let zero = ast::Int::from_i64(&ctx, 0);
            opt.assert(&ta.ge(&ast::Real::from_int(&zero)));
            opt.assert(&tb.ge(&ast::Real::from_int(&zero)));

            let xa = ast::Real::from_int(&ast::Int::from_i64(&ctx, hailstone_a.x));
            let ya = ast::Real::from_int(&ast::Int::from_i64(&ctx, hailstone_a.y));
            let dxa = ast::Real::from_int(&ast::Int::from_i64(&ctx, hailstone_a.dx));
            let dya = ast::Real::from_int(&ast::Int::from_i64(&ctx, hailstone_a.dy));

            let xb = ast::Real::from_int(&ast::Int::from_i64(&ctx, hailstone_b.x));
            let yb = ast::Real::from_int(&ast::Int::from_i64(&ctx, hailstone_b.y));
            let dxb = ast::Real::from_int(&ast::Int::from_i64(&ctx, hailstone_b.dx));
            let dyb = ast::Real::from_int(&ast::Int::from_i64(&ctx, hailstone_b.dy));

            let xa = xa + dxa * &ta;
            let ya = ya + dya * &ta;

            let xb = xb + dxb * &tb;
            let yb = yb + dyb * &tb;

            let lb = ast::Real::from_int(&ast::Int::from_i64(&ctx, test_lb));
            let ub = ast::Real::from_int(&ast::Int::from_i64(&ctx, test_ub));

            opt.assert(&xa.ge(&lb));
            opt.assert(&xa.le(&ub));

            opt.assert(&ya.ge(&lb));
            opt.assert(&ya.le(&ub));

            opt.assert(&xa._eq(&xb));
            opt.assert(&ya._eq(&yb));

            opt.check(&[]);

            let model = opt.get_model().unwrap();
            
            if model.to_string() != "" {
                ans += 1;
            }
        }
    }

    println!("Part 1: {}", ans);

    Ok(())
}

fn part2(filename: &str) -> io::Result<()> {
    let path = Path::new(filename);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ans = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        
    }

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
