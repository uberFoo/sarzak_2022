//! v2::drawing Object Store
//!
//! The ObjectStore contains instances of objects in the domain.
//! The instances are stored in a hash map, keyed by the object's UUID.
//! This is used during code generation, and probably not useful elsewhere.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::drawing-object-store-file"}}}
//!
//! # Contents:
//!
//! * [`Anchor`]
//! * [`AssociativeUi`]
//! * [`BinaryUi`]
//! * [`Edge`]
//! * [`IsaUi`]
//! * [`ObjectEdge`]
//! * [`ObjectUi`]
//! * [`Point`]
//! * [`RelationshipUi`]
//! * [`SubtypeAnchors`]
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::drawing-object-store-definition"}}}
use std::collections::HashMap;
use std::{fs, io, path::Path, time::SystemTime};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v2::drawing::types::{
    Anchor, AssociativeUi, BinaryUi, Edge, IsaUi, ObjectEdge, ObjectUi, Point, RelationshipUi,
    SubtypeAnchors, BOTTOM, LEFT, RIGHT, TOP,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectStore {
    anchor: HashMap<Uuid, (Anchor, SystemTime)>,
    associative_ui: HashMap<Uuid, (AssociativeUi, SystemTime)>,
    binary_ui: HashMap<Uuid, (BinaryUi, SystemTime)>,
    edge: HashMap<Uuid, (Edge, SystemTime)>,
    isa_ui: HashMap<Uuid, (IsaUi, SystemTime)>,
    object_edge: HashMap<Uuid, (ObjectEdge, SystemTime)>,
    object_ui: HashMap<Uuid, (ObjectUi, SystemTime)>,
    point: HashMap<Uuid, (Point, SystemTime)>,
    relationship_ui: HashMap<Uuid, (RelationshipUi, SystemTime)>,
    subtype_anchors: HashMap<Uuid, (SubtypeAnchors, SystemTime)>,
}

impl ObjectStore {
    pub fn new() -> Self {
        let mut store = Self {
            anchor: HashMap::new(),
            associative_ui: HashMap::new(),
            binary_ui: HashMap::new(),
            edge: HashMap::new(),
            isa_ui: HashMap::new(),
            object_edge: HashMap::new(),
            object_ui: HashMap::new(),
            point: HashMap::new(),
            relationship_ui: HashMap::new(),
            subtype_anchors: HashMap::new(),
        };

        // Initialize Singleton Subtypes
        store.inter_edge(Edge::Bottom(BOTTOM));
        store.inter_edge(Edge::Left(LEFT));
        store.inter_edge(Edge::Right(RIGHT));
        store.inter_edge(Edge::Top(TOP));

        store
    }

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::drawing-object-store-methods"}}}
    /// Inter [`Anchor`] into the store.
    ///
    pub fn inter_anchor(&mut self, anchor: Anchor) {
        self.anchor.insert(anchor.id, (anchor, SystemTime::now()));
    }

    /// Exhume [`Anchor`] from the store.
    ///
    pub fn exhume_anchor(&self, id: &Uuid) -> Option<&Anchor> {
        self.anchor.get(id).map(|anchor| &anchor.0)
    }

    /// Exhume [`Anchor`] from the store — mutably.
    ///
    pub fn exhume_anchor_mut(&mut self, id: &Uuid) -> Option<&mut Anchor> {
        self.anchor.get_mut(id).map(|anchor| &mut anchor.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Anchor>`.
    ///
    pub fn iter_anchor(&self) -> impl Iterator<Item = &Anchor> {
        self.anchor.values().map(|anchor| &anchor.0)
    }

    /// Get the timestamp for Anchor.
    ///
    pub fn anchor_timestamp(&self, anchor: &Anchor) -> SystemTime {
        self.anchor
            .get(&anchor.id)
            .map(|anchor| anchor.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`AssociativeUi`] into the store.
    ///
    pub fn inter_associative_ui(&mut self, associative_ui: AssociativeUi) {
        self.associative_ui
            .insert(associative_ui.id, (associative_ui, SystemTime::now()));
    }

    /// Exhume [`AssociativeUi`] from the store.
    ///
    pub fn exhume_associative_ui(&self, id: &Uuid) -> Option<&AssociativeUi> {
        self.associative_ui
            .get(id)
            .map(|associative_ui| &associative_ui.0)
    }

    /// Exhume [`AssociativeUi`] from the store — mutably.
    ///
    pub fn exhume_associative_ui_mut(&mut self, id: &Uuid) -> Option<&mut AssociativeUi> {
        self.associative_ui
            .get_mut(id)
            .map(|associative_ui| &mut associative_ui.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, AssociativeUi>`.
    ///
    pub fn iter_associative_ui(&self) -> impl Iterator<Item = &AssociativeUi> {
        self.associative_ui
            .values()
            .map(|associative_ui| &associative_ui.0)
    }

    /// Get the timestamp for AssociativeUi.
    ///
    pub fn associative_ui_timestamp(&self, associative_ui: &AssociativeUi) -> SystemTime {
        self.associative_ui
            .get(&associative_ui.id)
            .map(|associative_ui| associative_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`BinaryUi`] into the store.
    ///
    pub fn inter_binary_ui(&mut self, binary_ui: BinaryUi) {
        self.binary_ui
            .insert(binary_ui.id, (binary_ui, SystemTime::now()));
    }

    /// Exhume [`BinaryUi`] from the store.
    ///
    pub fn exhume_binary_ui(&self, id: &Uuid) -> Option<&BinaryUi> {
        self.binary_ui.get(id).map(|binary_ui| &binary_ui.0)
    }

    /// Exhume [`BinaryUi`] from the store — mutably.
    ///
    pub fn exhume_binary_ui_mut(&mut self, id: &Uuid) -> Option<&mut BinaryUi> {
        self.binary_ui.get_mut(id).map(|binary_ui| &mut binary_ui.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, BinaryUi>`.
    ///
    pub fn iter_binary_ui(&self) -> impl Iterator<Item = &BinaryUi> {
        self.binary_ui.values().map(|binary_ui| &binary_ui.0)
    }

    /// Get the timestamp for BinaryUi.
    ///
    pub fn binary_ui_timestamp(&self, binary_ui: &BinaryUi) -> SystemTime {
        self.binary_ui
            .get(&binary_ui.id)
            .map(|binary_ui| binary_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Edge`] into the store.
    ///
    pub fn inter_edge(&mut self, edge: Edge) {
        self.edge.insert(edge.id(), (edge, SystemTime::now()));
    }

    /// Exhume [`Edge`] from the store.
    ///
    pub fn exhume_edge(&self, id: &Uuid) -> Option<&Edge> {
        self.edge.get(id).map(|edge| &edge.0)
    }

    /// Exhume [`Edge`] from the store — mutably.
    ///
    pub fn exhume_edge_mut(&mut self, id: &Uuid) -> Option<&mut Edge> {
        self.edge.get_mut(id).map(|edge| &mut edge.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Edge>`.
    ///
    pub fn iter_edge(&self) -> impl Iterator<Item = &Edge> {
        self.edge.values().map(|edge| &edge.0)
    }

    /// Get the timestamp for Edge.
    ///
    pub fn edge_timestamp(&self, edge: &Edge) -> SystemTime {
        self.edge
            .get(&edge.id())
            .map(|edge| edge.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`IsaUi`] into the store.
    ///
    pub fn inter_isa_ui(&mut self, isa_ui: IsaUi) {
        self.isa_ui.insert(isa_ui.id, (isa_ui, SystemTime::now()));
    }

    /// Exhume [`IsaUi`] from the store.
    ///
    pub fn exhume_isa_ui(&self, id: &Uuid) -> Option<&IsaUi> {
        self.isa_ui.get(id).map(|isa_ui| &isa_ui.0)
    }

    /// Exhume [`IsaUi`] from the store — mutably.
    ///
    pub fn exhume_isa_ui_mut(&mut self, id: &Uuid) -> Option<&mut IsaUi> {
        self.isa_ui.get_mut(id).map(|isa_ui| &mut isa_ui.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, IsaUi>`.
    ///
    pub fn iter_isa_ui(&self) -> impl Iterator<Item = &IsaUi> {
        self.isa_ui.values().map(|isa_ui| &isa_ui.0)
    }

    /// Get the timestamp for IsaUi.
    ///
    pub fn isa_ui_timestamp(&self, isa_ui: &IsaUi) -> SystemTime {
        self.isa_ui
            .get(&isa_ui.id)
            .map(|isa_ui| isa_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`ObjectEdge`] into the store.
    ///
    pub fn inter_object_edge(&mut self, object_edge: ObjectEdge) {
        self.object_edge
            .insert(object_edge.id, (object_edge, SystemTime::now()));
    }

    /// Exhume [`ObjectEdge`] from the store.
    ///
    pub fn exhume_object_edge(&self, id: &Uuid) -> Option<&ObjectEdge> {
        self.object_edge.get(id).map(|object_edge| &object_edge.0)
    }

    /// Exhume [`ObjectEdge`] from the store — mutably.
    ///
    pub fn exhume_object_edge_mut(&mut self, id: &Uuid) -> Option<&mut ObjectEdge> {
        self.object_edge
            .get_mut(id)
            .map(|object_edge| &mut object_edge.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectEdge>`.
    ///
    pub fn iter_object_edge(&self) -> impl Iterator<Item = &ObjectEdge> {
        self.object_edge.values().map(|object_edge| &object_edge.0)
    }

    /// Get the timestamp for ObjectEdge.
    ///
    pub fn object_edge_timestamp(&self, object_edge: &ObjectEdge) -> SystemTime {
        self.object_edge
            .get(&object_edge.id)
            .map(|object_edge| object_edge.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`ObjectUi`] into the store.
    ///
    pub fn inter_object_ui(&mut self, object_ui: ObjectUi) {
        self.object_ui
            .insert(object_ui.id, (object_ui, SystemTime::now()));
    }

    /// Exhume [`ObjectUi`] from the store.
    ///
    pub fn exhume_object_ui(&self, id: &Uuid) -> Option<&ObjectUi> {
        self.object_ui.get(id).map(|object_ui| &object_ui.0)
    }

    /// Exhume [`ObjectUi`] from the store — mutably.
    ///
    pub fn exhume_object_ui_mut(&mut self, id: &Uuid) -> Option<&mut ObjectUi> {
        self.object_ui.get_mut(id).map(|object_ui| &mut object_ui.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, ObjectUi>`.
    ///
    pub fn iter_object_ui(&self) -> impl Iterator<Item = &ObjectUi> {
        self.object_ui.values().map(|object_ui| &object_ui.0)
    }

    /// Get the timestamp for ObjectUi.
    ///
    pub fn object_ui_timestamp(&self, object_ui: &ObjectUi) -> SystemTime {
        self.object_ui
            .get(&object_ui.id)
            .map(|object_ui| object_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`Point`] into the store.
    ///
    pub fn inter_point(&mut self, point: Point) {
        self.point.insert(point.id, (point, SystemTime::now()));
    }

    /// Exhume [`Point`] from the store.
    ///
    pub fn exhume_point(&self, id: &Uuid) -> Option<&Point> {
        self.point.get(id).map(|point| &point.0)
    }

    /// Exhume [`Point`] from the store — mutably.
    ///
    pub fn exhume_point_mut(&mut self, id: &Uuid) -> Option<&mut Point> {
        self.point.get_mut(id).map(|point| &mut point.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, Point>`.
    ///
    pub fn iter_point(&self) -> impl Iterator<Item = &Point> {
        self.point.values().map(|point| &point.0)
    }

    /// Get the timestamp for Point.
    ///
    pub fn point_timestamp(&self, point: &Point) -> SystemTime {
        self.point
            .get(&point.id)
            .map(|point| point.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`RelationshipUi`] into the store.
    ///
    pub fn inter_relationship_ui(&mut self, relationship_ui: RelationshipUi) {
        self.relationship_ui
            .insert(relationship_ui.id(), (relationship_ui, SystemTime::now()));
    }

    /// Exhume [`RelationshipUi`] from the store.
    ///
    pub fn exhume_relationship_ui(&self, id: &Uuid) -> Option<&RelationshipUi> {
        self.relationship_ui
            .get(id)
            .map(|relationship_ui| &relationship_ui.0)
    }

    /// Exhume [`RelationshipUi`] from the store — mutably.
    ///
    pub fn exhume_relationship_ui_mut(&mut self, id: &Uuid) -> Option<&mut RelationshipUi> {
        self.relationship_ui
            .get_mut(id)
            .map(|relationship_ui| &mut relationship_ui.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, RelationshipUi>`.
    ///
    pub fn iter_relationship_ui(&self) -> impl Iterator<Item = &RelationshipUi> {
        self.relationship_ui
            .values()
            .map(|relationship_ui| &relationship_ui.0)
    }

    /// Get the timestamp for RelationshipUi.
    ///
    pub fn relationship_ui_timestamp(&self, relationship_ui: &RelationshipUi) -> SystemTime {
        self.relationship_ui
            .get(&relationship_ui.id())
            .map(|relationship_ui| relationship_ui.1)
            .unwrap_or(SystemTime::now())
    }

    /// Inter [`SubtypeAnchors`] into the store.
    ///
    pub fn inter_subtype_anchors(&mut self, subtype_anchors: SubtypeAnchors) {
        self.subtype_anchors
            .insert(subtype_anchors.id, (subtype_anchors, SystemTime::now()));
    }

    /// Exhume [`SubtypeAnchors`] from the store.
    ///
    pub fn exhume_subtype_anchors(&self, id: &Uuid) -> Option<&SubtypeAnchors> {
        self.subtype_anchors
            .get(id)
            .map(|subtype_anchors| &subtype_anchors.0)
    }

    /// Exhume [`SubtypeAnchors`] from the store — mutably.
    ///
    pub fn exhume_subtype_anchors_mut(&mut self, id: &Uuid) -> Option<&mut SubtypeAnchors> {
        self.subtype_anchors
            .get_mut(id)
            .map(|subtype_anchors| &mut subtype_anchors.0)
    }

    /// Get an iterator over the internal `HashMap<&Uuid, SubtypeAnchors>`.
    ///
    pub fn iter_subtype_anchors(&self) -> impl Iterator<Item = &SubtypeAnchors> {
        self.subtype_anchors
            .values()
            .map(|subtype_anchors| &subtype_anchors.0)
    }

    /// Get the timestamp for SubtypeAnchors.
    ///
    pub fn subtype_anchors_timestamp(&self, subtype_anchors: &SubtypeAnchors) -> SystemTime {
        self.subtype_anchors
            .get(&subtype_anchors.id)
            .map(|subtype_anchors| subtype_anchors.1)
            .unwrap_or(SystemTime::now())
    }

    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}

    // {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::drawing-object-store-persistence"}}}
    /// Persist the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn persist<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        let path = path.as_ref();
        let path = path.join("drawing.json");
        fs::create_dir_all(&path)?;

        // Persist Anchor.
        {
            let path = path.join("anchor");
            fs::create_dir_all(&path)?;
            for anchor_tuple in self.anchor.values() {
                let path = path.join(format!("{}.json", anchor_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Anchor, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != anchor_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &anchor_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &anchor_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.anchor.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist AssociativeUI.
        {
            let path = path.join("associative_ui");
            fs::create_dir_all(&path)?;
            for associative_ui_tuple in self.associative_ui.values() {
                let path = path.join(format!("{}.json", associative_ui_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (AssociativeUi, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != associative_ui_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &associative_ui_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &associative_ui_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.associative_ui.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist BinaryUI.
        {
            let path = path.join("binary_ui");
            fs::create_dir_all(&path)?;
            for binary_ui_tuple in self.binary_ui.values() {
                let path = path.join(format!("{}.json", binary_ui_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (BinaryUi, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != binary_ui_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &binary_ui_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &binary_ui_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.binary_ui.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Edge.
        {
            let path = path.join("edge");
            fs::create_dir_all(&path)?;
            for edge_tuple in self.edge.values() {
                let path = path.join(format!("{}.json", edge_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Edge, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != edge_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &edge_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &edge_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.edge.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist IsaUI.
        {
            let path = path.join("isa_ui");
            fs::create_dir_all(&path)?;
            for isa_ui_tuple in self.isa_ui.values() {
                let path = path.join(format!("{}.json", isa_ui_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (IsaUi, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != isa_ui_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &isa_ui_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &isa_ui_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.isa_ui.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Object Edge.
        {
            let path = path.join("object_edge");
            fs::create_dir_all(&path)?;
            for object_edge_tuple in self.object_edge.values() {
                let path = path.join(format!("{}.json", object_edge_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ObjectEdge, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != object_edge_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &object_edge_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &object_edge_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.object_edge.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist ObjectUI.
        {
            let path = path.join("object_ui");
            fs::create_dir_all(&path)?;
            for object_ui_tuple in self.object_ui.values() {
                let path = path.join(format!("{}.json", object_ui_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (ObjectUi, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != object_ui_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &object_ui_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &object_ui_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.object_ui.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Point.
        {
            let path = path.join("point");
            fs::create_dir_all(&path)?;
            for point_tuple in self.point.values() {
                let path = path.join(format!("{}.json", point_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (Point, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != point_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &point_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &point_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.point.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist RelationshipUI.
        {
            let path = path.join("relationship_ui");
            fs::create_dir_all(&path)?;
            for relationship_ui_tuple in self.relationship_ui.values() {
                let path = path.join(format!("{}.json", relationship_ui_tuple.0.id()));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (RelationshipUi, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != relationship_ui_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &relationship_ui_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &relationship_ui_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.relationship_ui.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        // Persist Subtype Anchors.
        {
            let path = path.join("subtype_anchors");
            fs::create_dir_all(&path)?;
            for subtype_anchors_tuple in self.subtype_anchors.values() {
                let path = path.join(format!("{}.json", subtype_anchors_tuple.0.id));
                if path.exists() {
                    let file = fs::File::open(&path)?;
                    let reader = io::BufReader::new(file);
                    let on_disk: (SubtypeAnchors, SystemTime) = serde_json::from_reader(reader)?;
                    if on_disk.0 != subtype_anchors_tuple.0 {
                        let file = fs::File::create(path)?;
                        let mut writer = io::BufWriter::new(file);
                        serde_json::to_writer_pretty(&mut writer, &subtype_anchors_tuple)?;
                    }
                } else {
                    let file = fs::File::create(&path)?;
                    let mut writer = io::BufWriter::new(file);
                    serde_json::to_writer_pretty(&mut writer, &subtype_anchors_tuple)?;
                }
            }
            for file in fs::read_dir(&path)? {
                let file = file?;
                let path = file.path();
                let file_name = path.file_name().unwrap().to_str().unwrap();
                let id = file_name.split(".").next().unwrap();
                if let Ok(id) = Uuid::parse_str(id) {
                    if !self.subtype_anchors.contains_key(&id) {
                        fs::remove_file(path)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Load the store.
    ///
    /// The store is persisted as a directory of JSON files. The intention
    /// is that this directory can be checked into version control.
    /// In fact, I intend to add automaagic git integration as an option.
    pub fn load<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let path = path.as_ref();
        let path = path.join("drawing.json");

        let mut store = Self::new();

        // Load Anchor.
        {
            let path = path.join("anchor");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let anchor: (Anchor, SystemTime) = serde_json::from_reader(reader)?;
                store.anchor.insert(anchor.0.id, anchor);
            }
        }

        // Load AssociativeUI.
        {
            let path = path.join("associative_ui");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let associative_ui: (AssociativeUi, SystemTime) = serde_json::from_reader(reader)?;
                store
                    .associative_ui
                    .insert(associative_ui.0.id, associative_ui);
            }
        }

        // Load BinaryUI.
        {
            let path = path.join("binary_ui");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let binary_ui: (BinaryUi, SystemTime) = serde_json::from_reader(reader)?;
                store.binary_ui.insert(binary_ui.0.id, binary_ui);
            }
        }

        // Load Edge.
        {
            let path = path.join("edge");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let edge: (Edge, SystemTime) = serde_json::from_reader(reader)?;
                store.edge.insert(edge.0.id(), edge);
            }
        }

        // Load IsaUI.
        {
            let path = path.join("isa_ui");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let isa_ui: (IsaUi, SystemTime) = serde_json::from_reader(reader)?;
                store.isa_ui.insert(isa_ui.0.id, isa_ui);
            }
        }

        // Load Object Edge.
        {
            let path = path.join("object_edge");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let object_edge: (ObjectEdge, SystemTime) = serde_json::from_reader(reader)?;
                store.object_edge.insert(object_edge.0.id, object_edge);
            }
        }

        // Load ObjectUI.
        {
            let path = path.join("object_ui");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let object_ui: (ObjectUi, SystemTime) = serde_json::from_reader(reader)?;
                store.object_ui.insert(object_ui.0.id, object_ui);
            }
        }

        // Load Point.
        {
            let path = path.join("point");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let point: (Point, SystemTime) = serde_json::from_reader(reader)?;
                store.point.insert(point.0.id, point);
            }
        }

        // Load RelationshipUI.
        {
            let path = path.join("relationship_ui");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let relationship_ui: (RelationshipUi, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .relationship_ui
                    .insert(relationship_ui.0.id(), relationship_ui);
            }
        }

        // Load Subtype Anchors.
        {
            let path = path.join("subtype_anchors");
            let mut entries = fs::read_dir(path)?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let path = entry.path();
                let file = fs::File::open(path)?;
                let reader = io::BufReader::new(file);
                let subtype_anchors: (SubtypeAnchors, SystemTime) =
                    serde_json::from_reader(reader)?;
                store
                    .subtype_anchors
                    .insert(subtype_anchors.0.id, subtype_anchors);
            }
        }

        Ok(store)
    }
    // {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
