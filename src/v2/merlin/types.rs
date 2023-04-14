//! This is the second iteration of the drawing domain. The first sucked.
//!
//! This domain represents the visual aspect of a model.
// {"magic":"","directive":{"Start":{"directive":"allow-editing","tag":"v2::merlin-module-definition-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::merlin-module-definition"}}}
pub mod anchor;
pub mod bisection;
pub mod bottom;
pub mod edge;
pub mod glyph;
pub mod inflection;
pub mod left;
pub mod line;
pub mod line_segment;
pub mod line_segment_point;
pub mod many;
pub mod one;
pub mod point;
pub mod relationship_name;
pub mod relationship_phrase;
pub mod right;
pub mod sub;
pub mod top;
pub mod x_box;
pub mod x_super;

pub use crate::v2::merlin::anchor::Anchor;
pub use crate::v2::merlin::bisection::Bisection;
pub use crate::v2::merlin::bottom::BOTTOM;
pub use crate::v2::merlin::edge::Edge;
pub use crate::v2::merlin::glyph::Glyph;
pub use crate::v2::merlin::glyph::GlyphEnum;
pub use crate::v2::merlin::inflection::INFLECTION;
pub use crate::v2::merlin::left::LEFT;
pub use crate::v2::merlin::line::Line;
pub use crate::v2::merlin::line_segment::LineSegment;
pub use crate::v2::merlin::line_segment_point::LineSegmentPoint;
pub use crate::v2::merlin::many::MANY;
pub use crate::v2::merlin::one::ONE;
pub use crate::v2::merlin::point::Point;
pub use crate::v2::merlin::point::PointEnum;
pub use crate::v2::merlin::relationship_name::RelationshipName;
pub use crate::v2::merlin::relationship_phrase::RelationshipPhrase;
pub use crate::v2::merlin::right::RIGHT;
pub use crate::v2::merlin::sub::SUB;
pub use crate::v2::merlin::top::TOP;
pub use crate::v2::merlin::x_box::XBox;
pub use crate::v2::merlin::x_super::X_SUPER;
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"allow-editing"}}}