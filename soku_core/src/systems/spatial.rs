use rstar::{RTreeObject, AABB, RTree};
use hecs::{Entity, World};
use crate::components::{Shape, Transform};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandleType {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Rotation,
}

pub fn rotate_point(px: f32, py: f32, cx: f32, cy: f32, angle: f32) -> (f32, f32) {
    let s = angle.sin();
    let c = angle.cos();

    let x = px - cx;
    let y = py - cy;

    let xnew = x * c - y * s;
    let ynew = x * s + y * c;

    (xnew + cx, ynew + cy)
}

pub fn get_entity_handles(transform: &Transform, shape: &Shape) -> Vec<(HandleType, f32, f32)> {
    let raw_handles = match shape {
        Shape::Rectangle { width, height } => {
            vec![
                (HandleType::TopLeft, transform.x - width / 2.0, transform.y - height / 2.0),
                (HandleType::TopRight, transform.x + width / 2.0, transform.y - height / 2.0),
                (HandleType::BottomLeft, transform.x - width / 2.0, transform.y + height / 2.0),
                (HandleType::BottomRight, transform.x + width / 2.0, transform.y + height / 2.0),
                (HandleType::Rotation, transform.x, transform.y - height / 2.0 - 20.0),
            ]
        }
        Shape::Ellipse { radius_x, radius_y } => {
            vec![
                (HandleType::TopLeft, transform.x - radius_x, transform.y - radius_y),
                (HandleType::TopRight, transform.x + radius_x, transform.y - radius_y),
                (HandleType::BottomLeft, transform.x - radius_x, transform.y + radius_y),
                (HandleType::BottomRight, transform.x + radius_x, transform.y + radius_y),
                (HandleType::Rotation, transform.x, transform.y - radius_y - 20.0),
            ]
        }
        Shape::Triangle { base, height } => {
            vec![
                (HandleType::TopLeft, transform.x - base / 2.0, transform.y - height / 2.0),
                (HandleType::TopRight, transform.x + base / 2.0, transform.y - height / 2.0),
                (HandleType::BottomLeft, transform.x - base / 2.0, transform.y + height / 2.0),
                (HandleType::BottomRight, transform.x + base / 2.0, transform.y + height / 2.0),
                (HandleType::Rotation, transform.x, transform.y - height / 2.0 - 20.0),
            ]
        }
        Shape::Polygon { radius_x, radius_y, .. } => {
            vec![
                (HandleType::TopLeft, transform.x - radius_x, transform.y - radius_y),
                (HandleType::TopRight, transform.x + radius_x, transform.y - radius_y),
                (HandleType::BottomLeft, transform.x - radius_x, transform.y + radius_y),
                (HandleType::BottomRight, transform.x + radius_x, transform.y + radius_y),
                (HandleType::Rotation, transform.x, transform.y - radius_y - 20.0),
            ]
        }
        _ => vec![]
    };

    raw_handles.into_iter().map(|(t, x, y)| {
        let (rx, ry) = rotate_point(x, y, transform.x, transform.y, transform.rotation);
        (t, rx, ry)
    }).collect()
}

#[derive(Debug, Clone, Copy)]
pub struct SpatialObject {
    pub entity: Entity,
    pub aabb: AABB<[f32; 2]>,
}

impl RTreeObject for SpatialObject {
    type Envelope = AABB<[f32; 2]>;
    fn envelope(&self) -> Self::Envelope {
        self.aabb
    }
}

impl PartialEq for SpatialObject {
    fn eq(&self, other: &Self) -> bool {
        self.entity == other.entity
    }
}

pub fn calculate_aabb(transform: &Transform, shape: &Shape) -> AABB<[f32; 2]> {
    let (w, h) = match shape {
        Shape::Rectangle { width, height } => (*width, *height),
        Shape::Ellipse { radius_x, radius_y } => (radius_x * 2.0, radius_y * 2.0),
        Shape::Triangle { base, height } => (*base, *height),
        Shape::Polygon { radius_x, radius_y, .. } => (radius_x * 2.0, radius_y * 2.0),
        Shape::Path { points } => {
            let mut min_x = f32::MAX;
            let mut min_y = f32::MAX;
            let mut max_x = f32::MIN;
            let mut max_y = f32::MIN;
            for (px, py) in points {
                min_x = min_x.min(*px);
                min_y = min_y.min(*py);
                max_x = max_x.max(*px);
                max_y = max_y.max(*py);
            }
            return AABB::from_corners([transform.x + min_x, transform.y + min_y], [transform.x + max_x, transform.y + max_y]);
        }
    };

    let half_w = w / 2.0;
    let half_h = h / 2.0;

    let cos = transform.rotation.cos();
    let sin = transform.rotation.sin();

    let corners = [
        (-half_w, -half_h),
        (half_w, -half_h),
        (-half_w, half_h),
        (half_w, half_h),
    ];

    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;

    for (cx, cy) in corners {
        let rx = cx * cos - cy * sin;
        let ry = cx * sin + cy * cos;
        let x = transform.x + rx;
        let y = transform.y + ry;
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    AABB::from_corners([min_x, min_y], [max_x, max_y])
}

pub struct SpatialIndex {
    tree: RTree<SpatialObject>,
}

impl SpatialIndex {
    pub fn new() -> Self {
        Self {
            tree: RTree::new(),
        }
    }

    pub fn build_from_world(&mut self, world: &World) {
        let mut objects = Vec::new();
        for (entity, (transform, shape)) in world.query::<(&Transform, &Shape)>().iter() {
            objects.push(SpatialObject {
                entity,
                aabb: calculate_aabb(transform, shape),
            });
        }
        self.tree = RTree::bulk_load(objects);
    }

    pub fn insert(&mut self, entity: Entity, transform: &Transform, shape: &Shape) {
        self.tree.insert(SpatialObject {
            entity,
            aabb: calculate_aabb(transform, shape),
        });
    }

    pub fn remove(&mut self, entity: Entity, min: [f32; 2], max: [f32; 2]) {
        let obj = SpatialObject {
            entity,
            aabb: AABB::from_corners(min, max),
        };
        self.tree.remove(&obj);
    }

    pub fn query_point(&self, x: f32, y: f32) -> impl Iterator<Item = &SpatialObject> {
        let envelope = AABB::from_point([x, y]);
        self.tree.locate_in_envelope_intersecting(&envelope)
    }

    pub fn query_aabb(&self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> impl Iterator<Item = &SpatialObject> {
        let envelope = AABB::from_corners([min_x, min_y], [max_x, max_y]);
        self.tree.locate_in_envelope(&envelope)
    }
}
