#![allow(dead_code)]
// Circle and Points (https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=1132&lang=jp)
use geometry_library::*;

fn main() {
    while let Some(points) = input() {
        let points = points
            .into_iter()
            .map(|p| Point::new(p))
            .collect::<Vec<_>>();
        let mut max = 1;
        for i in 0..points.len() - 1 {
            for j in i + 1..points.len() {
                let circle1 = Circle::<f64>::new(points[i], 1.0);
                let circle2 = Circle::<f64>::new(points[j], 1.0);
                let cross_points = Circle::cross_points(circle1, circle2);
                for cp in cross_points {
                    let mut est = 2;
                    for (k, &p) in points.iter().enumerate() {
                        if k == i || k == j {
                            continue;
                        } else if p.distance(cp) <= 1.0 {
                            est += 1;
                        }
                    }
                    max = std::cmp::max(max, est)
                }
            }
        }
        println!("{}", max);
    }
}

#[inline(always)]
fn input() -> Option<Vec<(f64, f64)>> {
    let n = read_line::<usize>()[0];
    if n == 0 {
        None
    } else {
        let mut points = Vec::with_capacity(n);
        for _ in 0..n {
            let point = read_line::<f64>();
            points.push((point[0], point[1]));
            //
        }
        Some(points)
    }
}

#[inline(always)]
fn read_line<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s.trim()
        .split_whitespace()
        .map(|c| T::from_str(c).unwrap())
        .collect()
}

pub mod geometry_library {

    #[derive(Debug, Copy, Clone)]
    pub struct Circle<T> {
        center: Point<T>,
        radius: T,
    }

    impl<T> Circle<T>
    where
        T: Copy
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::cmp::PartialOrd
            + std::convert::From<f64>,
        Circle<f64>: std::convert::From<Circle<T>>,
    {
        #[inline(always)]
        pub fn new(center: Point<T>, radius: T) -> Self {
            Self { center, radius }
        }

        #[inline(always)]
        /// See here(https://poporix.hatenablog.com/entry/2020/03/03/223631).
        pub fn cross_points(circle1: Circle<T>, circle2: Circle<T>) -> Vec<Point<f64>> {
            let circle1: Circle<f64> = circle1.into();
            let circle2: Circle<f64> = circle2.into();
            let d = circle1.center.distance(circle2.center);
            if circle1.radius + circle2.radius < d {
                Vec::new()
            } else if circle1.radius + circle2.radius == d {
                let p2p1 = circle2.center - circle1.center;
                let p1h = p2p1 * circle1.radius / (circle1.radius + circle2.radius);
                vec![circle1.center + p1h]
            } else {
                let cos = (circle1.radius * circle1.radius + d * d
                    - circle2.radius * circle2.radius)
                    / 2.0
                    / circle1.radius
                    / d;
                let rc = circle1.radius * cos;
                let rs = (circle1.radius * circle1.radius - rc * rc).sqrt();
                let p2p1 = circle2.center - circle1.center;
                let e1 = p2p1 / d;
                let p1h = p2p1 * rc / d;
                let ha1 = Point(-e1.1, e1.0) * rs;
                let ha2 = Point(e1.1, -e1.0) * rs;
                vec![circle1.center + p1h + ha1, circle1.center + p1h + ha2]
            }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub struct Point<T>(T, T);

    impl<T> Point<T>
    where
        T: std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::cmp::PartialOrd
            + Zero
            + Copy,
    {
        /// (T, T) -> Point<T>
        #[inline(always)]
        pub fn new(p: (T, T)) -> Self {
            Point(p.0, p.1)
        }

        /// Point<T> -> (T, T)
        #[inline(always)]
        pub fn convert(self) -> (T, T) {
            (self.0, self.1)
        }

        /// Check whether segment p1-p2 is cross to segment p3-p4 or not
        #[inline(always)]
        pub fn is_cross(p1: Point<T>, p2: Point<T>, p3: Point<T>, p4: Point<T>) -> bool {
            let d1 = p1.direction(p3, p4);
            let d2 = p2.direction(p3, p4);
            let d3 = p3.direction(p1, p2);
            let d4 = p4.direction(p1, p2);
            (((d1 > T::ZERO && d2 < T::ZERO) || (d1 < T::ZERO && d2 > T::ZERO))
                && ((d3 > T::ZERO && d4 < T::ZERO) || (d3 < T::ZERO && d4 > T::ZERO)))
                || (d1 == T::ZERO && p1.is_on_segment(p3, p4))
                || (d2 == T::ZERO && p2.is_on_segment(p3, p4))
                || (d3 == T::ZERO && p3.is_on_segment(p1, p2))
                || (d4 == T::ZERO && p4.is_on_segment(p1, p2))
        }

        /// Return direction from line segment p1-p2 to self
        #[inline(always)]
        pub fn direction(self, p1: Point<T>, p2: Point<T>) -> T {
            let v1 = (p2.0 - p1.0, p2.1 - p1.1);
            let v2 = (self.0 - p1.0, self.1 - p1.1);
            v1.0 * v2.1 - v1.1 * v2.0
        }
        /// when self.direction(p1, p2) == 0 then,
        #[inline(always)]
        pub fn is_on_segment(self, p1: Point<T>, p2: Point<T>) -> bool {
            let min = |a, b| {
                if a < b {
                    a
                } else {
                    b
                }
            };
            let max = |a, b| {
                if a < b {
                    b
                } else {
                    a
                }
            };
            min(p1.0, p2.0) <= self.0
                && self.0 <= max(p1.0, p2.0)
                && min(p1.1, p2.1) <= self.1
                && self.1 <= max(p1.1, p2.1)
        }
    }

    impl<T> Point<T>
    where
        T: Copy + std::convert::Into<f64>,
    {
        pub fn distance(self, other: Point<T>) -> f64 {
            ((self.0.into() - other.0.into()) * (self.0.into() - other.0.into())
                + (self.1.into() - other.1.into()) * (self.1.into() - other.1.into()))
            .sqrt()
        }
    }

    impl<T: std::ops::Sub<Output = T> + Copy> std::ops::Sub for Point<T> {
        type Output = Self;
        fn sub(self, other: Self) -> Self::Output {
            Self(self.0 - other.0, self.1 - other.1)
        }
    }

    impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add for Point<T> {
        type Output = Self;
        fn add(self, other: Self) -> Self::Output {
            Self(self.0 + other.0, self.1 + other.1)
        }
    }

    impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for Point<T> {
        type Output = Self;
        fn mul(self, rhs: T) -> Self::Output {
            Self(self.0 * rhs, self.1 * rhs)
        }
    }

    impl<T: std::ops::Div<Output = T> + Copy> std::ops::Div<T> for Point<T> {
        type Output = Self;
        fn div(self, rhs: T) -> Self::Output {
            Self(self.0 / rhs, self.1 / rhs)
        }
    }

    pub trait Zero {
        const ZERO: Self;
    }

    macro_rules! impl_zero {
        ( $($e:ty),* ) => {
            $(
                impl Zero for $e {
                    const ZERO: Self = 0;
                }
            )*
        };
    }

    impl_zero!(isize, i8, i16, i32, i64, i128, usize, u8, u16, u32, u64, u128);

    impl Zero for f32 {
        const ZERO: Self = 0.0;
    }

    impl Zero for f64 {
        const ZERO: Self = 0.0;
    }

    #[cfg(test)]
    mod tests_geometry {
        use super::*;

        #[test]
        fn for_cross_points() {
            let input = vec![
                (
                    vec![(6.47634, 7.69628), (5.16828, 4.79915), (6.69533, 6.20378)],
                    2,
                ),
                (
                    vec![
                        (7.15296, 4.08328),
                        (6.50827, 2.69466),
                        (5.91219, 3.86661),
                        (5.29853, 4.16097),
                        (6.10838, 3.46039),
                        (6.34060, 2.41599),
                    ],
                    5,
                ),
                (
                    vec![
                        (7.90650, 4.01746),
                        (4.10998, 4.18354),
                        (4.67289, 4.01887),
                        (6.33885, 4.28388),
                        (4.98106, 3.82728),
                        (5.12379, 5.16473),
                        (7.84664, 4.67693),
                        (4.02776, 3.87990),
                    ],
                    5,
                ),
                (
                    vec![
                        (6.65128, 5.47490),
                        (6.42743, 6.26189),
                        (6.35864, 4.61611),
                        (6.59020, 4.54228),
                        (4.43967, 5.70059),
                        (4.38226, 5.70536),
                        (5.50755, 6.18163),
                        (7.41971, 6.13668),
                        (6.71936, 3.04496),
                        (5.61832, 4.23857),
                        (5.99424, 4.29328),
                        (5.60961, 4.32998),
                        (6.82242, 5.79683),
                        (5.44693, 3.82724),
                        (6.70906, 3.65736),
                        (7.89087, 5.68000),
                        (6.23300, 4.59530),
                        (5.92401, 4.92329),
                        (6.24168, 3.81389),
                        (6.22671, 3.62210),
                    ],
                    11,
                ),
                (vec![(5.16817, 2.23610)], 1),
            ]
            .into_iter();
            for (points, ans) in input {
                let points = points
                    .into_iter()
                    .map(|p| Point::new(p))
                    .collect::<Vec<_>>();
                let mut max = 1;
                for i in 0..points.len() - 1 {
                    for j in i + 1..points.len() {
                        let circle1 = Circle::<f64>::new(points[i], 1.0);
                        let circle2 = Circle::<f64>::new(points[j], 1.0);
                        let cross_points = Circle::cross_points(circle1, circle2);
                        for cp in cross_points {
                            let mut est = 2;
                            for (k, &p) in points.iter().enumerate() {
                                if k == i || k == j {
                                    continue;
                                } else if p.distance(cp) <= 1.0 {
                                    est += 1;
                                }
                            }
                            max = std::cmp::max(max, est)
                        }
                    }
                }
                assert_eq!(max, ans);
            }
        }

        #[test]
        fn for_is_cross() {
            let a = (-1, 0);
            let b = (2, 0);
            let mut points = vec![(1, 1), (-1, 1), (-1, -1), (1, -1)];
            points.push(points[0]);
            assert_eq!(
                points
                    .windows(2)
                    .filter(|v| Point::<i32>::is_cross(
                        Point::new(a),
                        Point::new(b),
                        Point::new(v[0]),
                        Point::new(v[1])
                    ))
                    .count()
                    / 2
                    + 1,
                2
            );

            let a = (-3, 1);
            let b = (3, 1);
            let mut points = vec![
                (2, 2),
                (1, 2),
                (1, 0),
                (-1, 0),
                (-1, 2),
                (-2, 2),
                (-2, -1),
                (2, -1),
            ];
            points.push(points[0]);
            assert_eq!(
                points
                    .windows(2)
                    .filter(|v| Point::<i32>::is_cross(
                        Point::new(a),
                        Point::new(b),
                        Point::new(v[0]),
                        Point::new(v[1])
                    ))
                    .count()
                    / 2
                    + 1,
                3
            );

            let a = (-3.0, 1.0);
            let b = (3.0, 1.0);
            let mut points = vec![
                (2.0, 2.0),
                (1.0, 2.0),
                (1.0, 0.0),
                (-1.0, 0.0),
                (-1.0, 2.0),
                (-2.0, 2.0),
                (-2.0, -1.0),
                (2.0, -1.0),
            ];
            points.push(points[0]);
            assert_eq!(
                points
                    .windows(2)
                    .filter(|v| Point::<f64>::is_cross(
                        Point::new(a),
                        Point::new(b),
                        Point::new(v[0]),
                        Point::new(v[1])
                    ))
                    .count()
                    / 2
                    + 1,
                3
            );
        }
    }
}