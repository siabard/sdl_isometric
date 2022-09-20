//! Quadtree implementation
//! 공간을 4개의 section으로 나누고,
//! 재귀적으로 해당 섹션을 4개씩 나누는 구획을 만든다.
//! 구획을 나누는 방법은
//! 구획내 일정 이상의 요소가 존재하면
//! 해당 구획을 최소 크기가지 나누는 것이다.
//! QuadTree는 O(n*log_n) 복잡도를 가진다.

/// Data Structure : Point
use uuid::Uuid;

use crate::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    x: f64,
    y: f64,
    pub userdata: Uuid,
}

impl Point {
    pub fn new(x: f64, y: f64, userdata: Uuid) -> Point {
        Point { x, y, userdata }
    }
}

impl Rectangle {
    pub fn new(x: f64, y: f64, w: f64, h: f64) -> Rectangle {
        Rectangle { x, y, w, h }
    }

    fn contains(&self, point: &Point) -> bool {
        let px = point.x;
        let py = point.y;

        px >= self.x && px <= self.x + self.w && py >= self.y && py <= self.y + self.h
    }

    fn intersects(&self, other: &Rectangle) -> bool {
        let p1x = self.x;
        let p1y = self.y;

        let p1w = self.w;
        let p1h = self.h;

        let p2x = other.x;
        let p2y = other.y;
        let p2w = other.w;
        let p2h = other.h;

        p1x < p2x + p2w && p1x + p1w > p2x && p1y < p2y + p2h && p1y + p1h > p2y
    }
}

/// QuadTree
/// Array of reference to subsection
/// How big can be quadtree?
/// section에는 일정 수만 가둘 수 있다.

#[derive(Clone, PartialEq, Debug)]
pub struct QuadTree {
    boundary: Rectangle,
    capacity: usize,
    points: Vec<Point>,
    northeast: Option<Box<QuadTree>>,
    northwest: Option<Box<QuadTree>>,
    southwest: Option<Box<QuadTree>>,
    southeast: Option<Box<QuadTree>>,
    divided: bool,
}

impl QuadTree {
    pub fn new(boundary: Rectangle, capacity: usize) -> QuadTree {
        QuadTree {
            boundary,
            capacity,
            points: vec![],
            northeast: None,
            northwest: None,
            southwest: None,
            southeast: None,
            divided: false,
        }
    }

    pub fn insert(&mut self, point: Point) -> bool {
        if self.boundary.contains(&point) {
            if self.points.len() < self.capacity {
                self.points.push(point);
                return true;
            } else {
                if !self.divided {
                    self.subdivide();
                }
                if self.northeast.as_mut().unwrap().insert(point) {
                    return true;
                }

                if self.northwest.as_mut().unwrap().insert(point) {
                    return true;
                }

                if self.southeast.as_mut().unwrap().insert(point) {
                    return true;
                }

                if self.southwest.as_mut().unwrap().insert(point) {
                    return true;
                }
            }
        }

        false
    }

    fn subdivide(&mut self) {
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.w;
        let h = self.boundary.h;

        self.northwest = Some(Box::new(QuadTree::new(
            Rectangle::new(x, y, w / 2.0, h / 2.0),
            self.capacity,
        )));

        self.northeast = Some(Box::new(QuadTree::new(
            Rectangle::new(x + w / 2.0, y, w / 2.0, h / 2.0),
            self.capacity,
        )));

        self.southwest = Some(Box::new(QuadTree::new(
            Rectangle::new(x, y + h / 2.0, w / 2.0, h / 2.0),
            self.capacity,
        )));

        self.southeast = Some(Box::new(QuadTree::new(
            Rectangle::new(x + w / 2.0, y + h / 2.0, w / 2.0, h / 2.0),
            self.capacity,
        )));

        self.divided = true;
    }

    pub fn query(&self, range: Rectangle) -> Vec<Point> {
        let mut result = vec![];
        if self.boundary.intersects(&range) {
            for point in &self.points {
                if range.contains(point) {
                    result.push(*point);
                }
            }

            if self.divided {
                result.extend(self.northeast.as_ref().unwrap().query(range));
                result.extend(self.northwest.as_ref().unwrap().query(range));
                result.extend(self.southeast.as_ref().unwrap().query(range));
                result.extend(self.southwest.as_ref().unwrap().query(range));
            }
        }

        result
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn quad_setup() {
        use crate::quadtree::*;

        let boundary = Rectangle::new(200.0, 200.0, 400.0, 400.);
        let qt = QuadTree::new(boundary, 4);

        assert_eq!(
            QuadTree::new(Rectangle::new(200.0, 200.0, 400.0, 400.0), 4),
            qt
        );
    }

    #[test]
    fn quad_insert() {
        use crate::quadtree::*;

        let boundary = Rectangle::new(0.0, 0.0, 600.0, 400.);
        let mut qt = QuadTree::new(boundary, 4);

        qt.insert(Point::new(96.0, 16.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(208.0, 48.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(176.0, 64.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(144.0, 32.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(
            122.6984220213842,
            24.110823695873734,
            uuid::Uuid::new_v4(),
        ));
        qt.insert(Point::new(160.0, 64.0, uuid::Uuid::new_v4()));

        let uuid_test = uuid::Uuid::new_v4();
        qt.insert(Point::new(96.0, 32.0, uuid_test));

        qt.insert(Point::new(112.0, 48.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(176.0, 48.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(80.0, 16.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(80.0, 32.0, uuid::Uuid::new_v4()));

        let uuid_test_2 = uuid::Uuid::new_v4();
        qt.insert(Point::new(0.0, 0.0, uuid_test_2));

        qt.insert(Point::new(96.0, 48.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(160.0, 48.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(208.0, 64.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(224.0, 64.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(304.0, 0.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(112.0, 32.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(144.0, 48.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(
            239.50852519647427,
            140.26591808429174,
            uuid::Uuid::new_v4(),
        ));
        qt.insert(Point::new(144.0, 0.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(128.0, 32.0, uuid::Uuid::new_v4()));

        let uuid_test_3 = uuid::Uuid::new_v4();
        qt.insert(Point::new(80.0, 48.0, uuid_test_3));

        qt.insert(Point::new(192.0, 48.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(192.0, 64.0, uuid::Uuid::new_v4()));
        qt.insert(Point::new(112.0, 16.0, uuid::Uuid::new_v4()));

        let range: Rectangle = Rectangle::new(65.0, 56.0, 48.0, 64.0);
        let candidates = qt.query(range);
        println!("{:?}", candidates);
        let uuid_candidates: Vec<uuid::Uuid> =
            candidates.into_iter().map(|point| point.userdata).collect();

        assert_eq!(uuid_candidates.contains(&uuid_test), true);
    }
}
