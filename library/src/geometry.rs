#![allow(dead_code)]

pub mod geometry_library {

    #[derive(Debug, Copy, Clone)]
    pub struct Point<T>(T, T);

    impl<T> Point<T>
    where
        T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::cmp::Ord + Zero + Copy,
    {
        /// (T, T) -> Point<T>
        #[inline]
        pub fn new(p: (T, T)) -> Self {
            Point(p.0, p.1)
        }

        /// Point<T> -> (T, T)
        #[inline]
        pub fn convert(self) -> (T, T) {
            (self.0, self.1)
        }

        /// Check whether segment p1-p2 is cross to segment p3-p4 or not
        #[inline]
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
        #[inline]
        pub fn direction(self, p1: Point<T>, p2: Point<T>) -> T {
            let v1 = (p2.0 - p1.0, p2.1 - p1.1);
            let v2 = (self.0 - p1.0, self.1 - p1.1);
            v1.0 * v2.1 - v1.1 * v2.0
        }
        /// when self.direction(p1, p2) == 0 then,
        #[inline]
        pub fn is_on_segment(self, p1: Point<T>, p2: Point<T>) -> bool {
            use std::cmp::max;
            use std::cmp::min;
            min(p1.0, p2.0) <= self.0
                && self.0 <= max(p1.0, p2.0)
                && min(p1.1, p2.1) <= self.1
                && self.1 <= max(p1.1, p2.1)
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

    #[cfg(test)]
    mod tests_geometry {
        use super::*;

        #[test]
        fn for_abc016d() {
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
        }
    }
}
