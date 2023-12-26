use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use z3::ast::Ast;
use z3::{ast, Config, Context, Optimize, Solver};

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

        let coords = coords
            .split(',')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let vel = vel
            .split(',')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

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
        for j in i + 1..hailstones.len() {
            let hailstone_a = &hailstones[i];
            let hailstone_b = &hailstones[j];

            let cfg = Config::new();
            let ctx = Context::new(&cfg);
            let solver = Optimize::new(&ctx);

            let ta = ast::Real::new_const(&ctx, "ta");
            let tb = ast::Real::new_const(&ctx, "tb");

            let zero = ast::Int::from_i64(&ctx, 0);
            solver.assert(&ta.ge(&ast::Real::from_int(&zero)));
            solver.assert(&tb.ge(&ast::Real::from_int(&zero)));

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

            solver.assert(&xa.ge(&lb));
            solver.assert(&xa.le(&ub));

            solver.assert(&ya.ge(&lb));
            solver.assert(&ya.le(&ub));

            solver.assert(&xa._eq(&xb));
            solver.assert(&ya._eq(&yb));

            if solver.check(&[]) == z3::SatResult::Sat {
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

    let mut hailstones = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        let (coords, vel) = line.split_once(" @ ").unwrap();

        let coords = coords
            .split(',')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        let vel = vel
            .split(',')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        hailstones.push(Hailstone {
            x: coords[0],
            y: coords[1],
            z: coords[2],
            dx: vel[0],
            dy: vel[1],
            dz: vel[2],
        });
    }

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let xr = ast::Int::new_const(&ctx, "xr");
    let yr = ast::Int::new_const(&ctx, "yr");
    let zr = ast::Int::new_const(&ctx, "zr");

    let dxr = ast::Int::new_const(&ctx, "dxr");
    let dyr = ast::Int::new_const(&ctx, "dyr");
    let dzr = ast::Int::new_const(&ctx, "dzr");

    let zero = ast::Int::from_i64(&ctx, 0);

    for i in 0..hailstones.len() {
        let hailstone = &hailstones[i];

        let x = ast::Int::from_i64(&ctx, hailstone.x);
        let y = ast::Int::from_i64(&ctx, hailstone.y);
        let z = ast::Int::from_i64(&ctx, hailstone.z);

        let dx = ast::Int::from_i64(&ctx, hailstone.dx);
        let dy = ast::Int::from_i64(&ctx, hailstone.dy);
        let dz = ast::Int::from_i64(&ctx, hailstone.dz);

        let t = ast::Int::new_const(&ctx, format!("t{}", i));

        solver.assert(&(&xr + &dxr * &t)._eq(&(&x + &dx * &t)));
        solver.assert(&(&yr + &dyr * &t)._eq(&(&y + &dy * &t)));
        solver.assert(&(&zr + &dzr * &t)._eq(&(&z + &dz * &t)));

        solver.assert(&t.ge(&zero));
    }

    solver.check();

    let model = solver.get_model().unwrap();

    let ans = model.get_const_interp(&xr).unwrap().as_i64().unwrap()
        + model.get_const_interp(&yr).unwrap().as_i64().unwrap()
        + model.get_const_interp(&zr).unwrap().as_i64().unwrap();

    println!("Part 2: {}", ans);

    Ok(())
}

fn main() {
    part1("./src/input.txt").unwrap();
    part2("./src/input.txt").unwrap();
}
