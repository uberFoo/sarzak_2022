//! v2::sarzak Object From Trait Implementations
//!
//! These are [`From`] trait implementations for the domain: _sarzak_. They are
//! generated to be used during the extrusion process. This is the process
//! by which instances of one domain are transformed into instances of another.
//! In this case the source domain is `v1::sarzak`.
//!
//! It is hoped that the model has not changed enough to render
//! these implementations useless. In any case it's expected that
//! the generated code will need to be manually edited.
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::sarzak-from-impl-file"}}}
// {"magic":"","directive":{"Start":{"directive":"ignore-orig","tag":"v2::sarzak-from-impl-definition"}}}
use crate::v2::sarzak::types::{
    AcknowledgedEvent, Associative, AssociativeReferent, AssociativeReferrer, Attribute, Binary,
    Event, External, Isa, Object, Referent, Referrer, State, Subtype, Supertype,
};
use crate::v2::sarzak::ObjectStore;

use crate::v1::sarzak::types::{
    AcknowledgedEvent as FromAcknowledgedEvent, Associative as FromAssociative,
    AssociativeReferent as FromAssociativeReferent, AssociativeReferrer as FromAssociativeReferrer,
    Attribute as FromAttribute, Binary as FromBinary, Event as FromEvent, External as FromExternal,
    Isa as FromIsa, Object as FromObject, Referent as FromReferent, Referrer as FromReferrer,
    State as FromState, Subtype as FromSubtype, Supertype as FromSupertype,
};
use crate::v1::sarzak::ObjectStore as SarzakStore;

impl From<&SarzakStore> for ObjectStore {
    fn from(from: &SarzakStore) -> Self {
        let mut to = ObjectStore::new();

        for (_, instance) in from.iter_acknowledged_event() {
            let instance = AcknowledgedEvent::from(instance);
            to.inter_acknowledged_event(instance);
        }

        for (_, instance) in from.iter_associative() {
            let instance = Associative::from(instance);
            to.inter_associative(instance);
        }

        for (_, instance) in from.iter_associative_referent() {
            let instance = AssociativeReferent::from(instance);
            to.inter_associative_referent(instance);
        }

        for (_, instance) in from.iter_associative_referrer() {
            let instance = AssociativeReferrer::from(instance);
            to.inter_associative_referrer(instance);
        }

        for (_, instance) in from.iter_attribute() {
            let instance = Attribute::from(instance);
            to.inter_attribute(instance);
        }

        for (_, instance) in from.iter_binary() {
            let instance = Binary::from(instance);
            to.inter_binary(instance);
        }

        for (_, instance) in from.iter_event() {
            let instance = Event::from(instance);
            to.inter_event(instance);
        }

        for (_, instance) in from.iter_external() {
            let instance = External::from(instance);
            to.inter_external(instance);
        }

        for (_, instance) in from.iter_isa() {
            let instance = Isa::from(instance);
            to.inter_isa(instance);
        }

        for (_, instance) in from.iter_object() {
            let instance = Object::from(instance);
            to.inter_object(instance);
        }

        for (_, instance) in from.iter_referent() {
            let instance = Referent::from(instance);
            to.inter_referent(instance);
        }

        for (_, instance) in from.iter_referrer() {
            let instance = Referrer::from(instance);
            to.inter_referrer(instance);
        }

        for (_, instance) in from.iter_state() {
            let instance = State::from(instance);
            to.inter_state(instance);
        }

        for (_, instance) in from.iter_subtype() {
            let instance = Subtype::from(instance);
            to.inter_subtype(instance);
        }

        for (_, instance) in from.iter_supertype() {
            let instance = Supertype::from(instance);
            to.inter_supertype(instance);
        }

        to
    }
}

impl From<&FromAcknowledgedEvent> for AcknowledgedEvent {
    fn from(src: &FromAcknowledgedEvent) -> Self {
        Self {
            id: src.id,
            event_id: src.event_id,
            state_id: src.state_id,
        }
    }
}

impl From<&FromAssociative> for Associative {
    fn from(src: &FromAssociative) -> Self {
        Self {
            id: src.id,
            number: src.number,
            one: src.one,
            other: src.other,
            from: src.from,
        }
    }
}

impl From<&FromAssociativeReferent> for AssociativeReferent {
    fn from(src: &FromAssociativeReferent) -> Self {
        Self {
            id: src.id,
            obj_id: src.obj_id,
        }
    }
}

impl From<&FromAssociativeReferrer> for AssociativeReferrer {
    fn from(src: &FromAssociativeReferrer) -> Self {
        Self {
            id: src.id,
            obj_id: src.obj_id,
        }
    }
}

impl From<&FromAttribute> for Attribute {
    fn from(src: &FromAttribute) -> Self {
        Self {
            id: src.id,
            name: src.name.clone(),
            obj_id: src.obj_id,
            ty: src.ty,
        }
    }
}

impl From<&FromBinary> for Binary {
    fn from(src: &FromBinary) -> Self {
        Self {
            id: src.id,
            number: src.number,
            to: src.to,
            from: src.from,
        }
    }
}

impl From<&FromEvent> for Event {
    fn from(src: &FromEvent) -> Self {
        Self {
            id: src.id,
            name: src.name.clone(),
            obj_id: src.obj_id,
        }
    }
}

impl From<&FromExternal> for External {
    fn from(src: &FromExternal) -> Self {
        Self {
            id: src.id,
            name: src.name.clone(),
            path: src.path.clone(),
        }
    }
}

impl From<&FromIsa> for Isa {
    fn from(src: &FromIsa) -> Self {
        Self {
            id: src.id,
            number: src.number,
            supertype: src.supertype,
        }
    }
}

impl From<&FromObject> for Object {
    fn from(src: &FromObject) -> Self {
        Self {
            description: src.description.clone(),
            id: src.id,
            key_letters: src.key_letters.clone(),
            name: src.name.clone(),
        }
    }
}

impl From<&FromReferent> for Referent {
    fn from(src: &FromReferent) -> Self {
        Self {
            description: src.description.clone(),
            id: src.id,
            cardinality: src.cardinality,
            conditionality: src.conditionality,
            obj_id: src.obj_id,
        }
    }
}

impl From<&FromReferrer> for Referrer {
    fn from(src: &FromReferrer) -> Self {
        Self {
            description: src.description.clone(),
            id: src.id,
            referential_attribute: src.referential_attribute.clone(),
            cardinality: src.cardinality,
            conditionality: src.conditionality,
            obj_id: src.obj_id,
        }
    }
}

impl From<&FromState> for State {
    fn from(src: &FromState) -> Self {
        Self {
            id: src.id,
            name: src.name.clone(),
            obj_id: src.obj_id,
        }
    }
}

impl From<&FromSubtype> for Subtype {
    fn from(src: &FromSubtype) -> Self {
        Self {
            id: src.id,
            isa: src.isa,
            obj_id: src.obj_id,
        }
    }
}

impl From<&FromSupertype> for Supertype {
    fn from(src: &FromSupertype) -> Self {
        Self {
            id: src.id,
            obj_id: src.obj_id,
        }
    }
}

// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}
// {"magic":"","directive":{"End":{"directive":"ignore-orig"}}}