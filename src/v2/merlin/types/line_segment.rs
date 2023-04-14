// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"line_segment-struct-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment-use-statements"}}}
use uuid::Uuid;

use crate::v2::merlin::types::bisection::Bisection;
use crate::v2::merlin::types::line::Line;
use crate::v2::merlin::types::line_segment_point::LineSegmentPoint;
use serde::{Deserialize, Serialize};

use crate::v2::merlin::store::ObjectStore as MerlinStore;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment-struct-documentation"}}}
/// Part of a Line
///
/// A line segment is in fact a straight line. It is used to compose a (poly) [`Line`].
///
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment-struct-definition"}}}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct LineSegment {
    pub id: Uuid,
    /// R4: [`LineSegment`] 'composes' [`Line`]
    pub line: Uuid,
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment-implementation"}}}
impl LineSegment {
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment-struct-impl-new"}}}
    /// Inter a new 'Line Segment' in the store, and return it's `id`.
    pub fn new(line: &Line, store: &mut MerlinStore) -> LineSegment {
        let id = Uuid::new_v4();
        let new = LineSegment {
            id: id,
            line: line.id,
        };
        store.inter_line_segment(new.clone());
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment-struct-impl-new_"}}}
    /// Inter a new 'Line Segment' in the store, and return it's `id`.
    pub fn new_(line: &Line) -> LineSegment {
        let id = Uuid::new_v4();
        let new = LineSegment {
            id: id,
            line: line.id,
        };
        new
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment-struct-impl-nav-forward-to-line"}}}
    /// Navigate to [`Line`] across R4(1-*)
    pub fn r4_line<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Line> {
        vec![store.exhume_line(&self.line).unwrap()]
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment-struct-impl-nav-backward-cond-to-bisection"}}}
    /// Navigate to [`Bisection`] across R14(1-1c)
    pub fn r14c_bisection<'a>(&'a self, store: &'a MerlinStore) -> Vec<&Bisection> {
        let bisection = store
            .iter_bisection()
            .find(|bisection| bisection.segment == self.id);
        match bisection {
            Some(ref bisection) => vec![bisection],
            None => Vec::new(),
        }
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment-struct-impl-nav-backward-one-to-point"}}}
    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"line_segment-struct-impl-nav-backward-assoc_many-to-line_segment_point"}}}
    /// Navigate to [`LineSegmentPoint`] across R5(1-M)
    pub fn r5_line_segment_point<'a>(&'a self, store: &'a MerlinStore) -> Vec<&LineSegmentPoint> {
        store
            .iter_line_segment_point()
            .filter_map(|line_segment_point| {
                if line_segment_point.segment == self.id {
                    Some(line_segment_point)
                } else {
                    None
                }
            })
            .collect()
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}