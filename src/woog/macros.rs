//! Macros for navigating the "Woog" domain
//!
//! # Generated Code -- edit _with care_.
//!
//! Don't mess with anything between `{"magic":"","kind":"CriticalBlockBegin"}`
//! and `{"magic":"","kind":"CriticalBlockEnd"}`. Otherwise, you should be free
//! to go wild. Happy hacking!
//!
//! Use the following invocation to reproduce:
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
//! ```shell
//!  sarzak gen -d woog sarzak -d true -m true -i true -e false
//! ```
// {"magic":"","kind":"IgnoreBlockEnd"}
// {"magic":"","version":"1.0.0"}

/// Macro to traverse [`ObjectMethod`][🦀] ➡ [`Visibility`][🦞], via _R7_
///
/// This macro expects a &[`ObjectMethod`][🦀], and returns a &[`Visibility`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referrer_to_referent_imp`
///
/// [🦀]: crate::woog::types::ObjectMethod
/// [🦞]: crate::woog::types::Visibility
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::woog::Parameter;
/// # use sarzak::sarzak::Type;
/// # use sarzak::woog::ObjectMethod;
/// # use sarzak::sarzak::Object;
/// # use sarzak::woog::Visibility;
/// # use sarzak::woog_get_one_viz_across_r7;
/// # let mut store = sarzak::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
///
/// let type_ing = Type::test_default(&mut sarzak_store);
///
/// let visibility_abx = Visibility::test_default(&mut store);
/// let billowy_loss = "makeshift_plate".to_owned();
/// let parameter = Parameter::new(&mut store, None, &type_ing, &visibility_abx, billowy_loss);
/// let brief_creator = "clever_plastic".to_owned();
/// let fragile_oranges = "tender_seashore".to_owned();
/// let futuristic_mask = "alike_pen".to_owned();
/// let object_xff = Object::default();
///
/// let type_nku = Type::test_default(&mut sarzak_store);
///
/// let visibility_rxs = Visibility::test_default(&mut store);
/// let amuck_dolls = "cloudy_support".to_owned();
/// let comfortable_can = "fragile_quill".to_owned();
///
/// let object_method = ObjectMethod::new(&mut store, Some(&parameter), &object_xff, &type_nku, &visibility_rxs, amuck_dolls, comfortable_can);
///
/// let visibility_kpm = woog_get_one_viz_across_r7!(object_method, store);
/// assert_eq!(&visibility_rxs, visibility_kpm);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_get_one_viz_across_r7-emit_binary_main"}}}
macro_rules! woog_get_one_viz_across_r7 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"visibility-emit_one_unconditional"}}}
        // nut::codegen::template::macros::emit_one_unconditional
        $store.exhume_visibility(&$input.visibility).unwrap()
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"visibility-emit_one_unconditional"}}}
    }};
}
pub use woog_get_one_viz_across_r7;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_get_one_viz_across_r7-emit_binary_main"}}}

/// Macro to traverse [`Visibility`][🦀] ➡ [`ObjectMethod`][🦞], via _R7_
///
/// This macro expects a &[`Visibility`][🦀], and returns a &[`ObjectMethod`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referent_to_referrer_imp`
///
/// [🦀]: crate::woog::types::Visibility
/// [🦞]: crate::woog::types::ObjectMethod
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::sarzak::Object;
/// # use sarzak::woog::ObjectMethod;
/// # use sarzak::woog::Parameter;
/// # use sarzak::sarzak::Type;
/// # use sarzak::woog::Visibility;
/// # use sarzak::woog_get_one_meth_across_r7;
/// # let mut store = sarzak::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
///
/// let type_fwk = Type::test_default(&mut sarzak_store);
///
/// let visibility_ato = Visibility::test_default(&mut store);
/// let conscious_head = "alive_mint".to_owned();
/// let parameter = Parameter::new(&mut store, None, &type_fwk, &visibility_ato, conscious_head);
/// let afraid_moon = "lamentable_wool".to_owned();
/// let low_pest = "chemical_wall".to_owned();
/// let mundane_amount = "orange_shoe".to_owned();
/// let object_oju = Object::default();
///
/// let type_mhb = Type::test_default(&mut sarzak_store);
///
/// let visibility_zdo = Visibility::test_default(&mut store);
/// let utopian_view = "quack_ray".to_owned();
/// let bewildered_calculator = "precious_sisters".to_owned();
///
/// let object_method = ObjectMethod::new(&mut store, Some(&parameter), &object_oju, &type_mhb, &visibility_zdo, utopian_view, bewildered_calculator);
/// let object_method_qws = woog_get_one_meth_across_r7!(visibility_zdo, store);
///
/// assert_eq!(&object_method, object_method_qws);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_get_one_meth_across_r7-emit_binary_main"}}}
macro_rules! woog_get_one_meth_across_r7 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"visibility-object_method-emit_one_unconditional_lookup"}}}
        // nut::codegen::template::macros::emit_one_unconditional_lookup
        $store
            .iter_object_method()
            .find(|z| z.1.visibility == $input.get_id())
            .map(|z| z.1)
            .unwrap()
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"visibility-object_method-emit_one_unconditional_lookup"}}}
    }};
}
pub use woog_get_one_meth_across_r7;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_get_one_meth_across_r7-emit_binary_main"}}}

/// Macro to traverse [`ObjectMethod`][🦀] ➡ [`Parameter`][🦞], via _R5(c)_
///
/// This macro expects a &[`ObjectMethod`][🦀], and returns an Option<&[`Parameter`][🦞]>.
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referrer_to_referent_imp`
///
/// [🦀]: crate::woog::types::ObjectMethod
/// [🦞]: crate::woog::types::Parameter
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::woog::Visibility;
/// # use sarzak::sarzak::Object;
/// # use sarzak::woog::Parameter;
/// # use sarzak::woog::ObjectMethod;
/// # use sarzak::sarzak::Type;
/// # use sarzak::woog_maybe_get_one_param_across_r5;
/// # let mut store = sarzak::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
///
/// let type_ehc = Type::test_default(&mut sarzak_store);
///
/// let visibility_tyy = Visibility::test_default(&mut store);
/// let raspy_pump = "prickly_dinosaurs".to_owned();
/// let parameter = Parameter::new(&mut store, None, &type_ehc, &visibility_tyy, raspy_pump);
/// let naughty_zebra = "spooky_balance".to_owned();
/// let versed_temper = "tricky_cloud".to_owned();
/// let unequaled_spy = "bloody_chicken".to_owned();
/// let object_jis = Object::default();
///
/// let type_ffv = Type::test_default(&mut sarzak_store);
///
/// let visibility_yky = Visibility::test_default(&mut store);
/// let muddled_ray = "near_pipe".to_owned();
/// let full_rain = "ill_informed_weight".to_owned();
/// let type_ycq = Type::test_default(&mut sarzak_store);
///
/// let visibility_bee = Visibility::test_default(&mut store);
/// let four_word = "superb_fact".to_owned();
///
/// let object_method = ObjectMethod::new(&mut store, Some(&parameter), &object_jis, &type_ffv, &visibility_yky, muddled_ray, full_rain);
///
/// let parameter_vit = woog_maybe_get_one_param_across_r5!(object_method, store);
/// assert_eq!(Some(&parameter), parameter_vit);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_maybe_get_one_param_across_r5-emit_binary_main"}}}
macro_rules! woog_maybe_get_one_param_across_r5 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"parameter-emit_one_conditional"}}}
        // nut::codegen::template::macros::emit_one_conditional
        match &$input.param {
            Some(i) => $store.exhume_parameter(i),
            None => None,
        }
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"parameter-emit_one_conditional"}}}
    }};
}
pub use woog_maybe_get_one_param_across_r5;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_maybe_get_one_param_across_r5-emit_binary_main"}}}

/// Macro to traverse [`Parameter`][🦀] ➡ [`ObjectMethod`][🦞], via _R5_
///
/// This macro expects a &[`Parameter`][🦀], and returns a &[`ObjectMethod`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referent_to_referrer_imp`
///
/// [🦀]: crate::woog::types::Parameter
/// [🦞]: crate::woog::types::ObjectMethod
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::woog::Parameter;
/// # use sarzak::sarzak::Object;
/// # use sarzak::woog::ObjectMethod;
/// # use sarzak::woog::Visibility;
/// # use sarzak::sarzak::Type;
/// # use sarzak::woog_get_one_meth_across_r5;
/// # let mut store = sarzak::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
///
/// let type_uvp = Type::test_default(&mut sarzak_store);
///
/// let visibility_lca = Visibility::test_default(&mut store);
/// let troubled_jam = "internal_space".to_owned();
/// let type_tvn = Type::test_default(&mut sarzak_store);
///
/// let visibility_akv = Visibility::test_default(&mut store);
/// let invincible_canvas = "bored_lawyer".to_owned();
/// let parameter = Parameter::new(&mut store, None, &type_tvn, &visibility_akv, invincible_canvas);
/// let unsuitable_fact = "hard_tiger".to_owned();
/// let easy_eyes = "useless_toes".to_owned();
/// let coordinated_oatmeal = "charming_cactus".to_owned();
/// let object_sec = Object::default();
///
/// let type_xwb = Type::test_default(&mut sarzak_store);
///
/// let visibility_nyy = Visibility::test_default(&mut store);
/// let curved_chain = "steep_care".to_owned();
/// let raspy_force = "fancy_toad".to_owned();
///
/// let object_method = ObjectMethod::new(&mut store, Some(&parameter), &object_sec, &type_xwb, &visibility_nyy, curved_chain, raspy_force);
///
/// let object_method_iqr = woog_get_one_meth_across_r5!(parameter, store);
/// assert_eq!(&object_method, object_method_iqr);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_get_one_meth_across_r5-emit_binary_main"}}}
macro_rules! woog_get_one_meth_across_r5 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"parameter-object_method-emit_one_unconditional_lookup", "is_uber":true}}}}
        // nut::codegen::template::macros::emit_one_unconditional_lookup
        $store
            .iter_object_method()
            .find(|z| z.1.param == Some($input.id))
            .map(|z| z.1)
            .unwrap()
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"parameter-object_method-emit_one_unconditional_lookup"}}}
    }};
}
pub use woog_get_one_meth_across_r5;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_get_one_meth_across_r5-emit_binary_main"}}}

/// Macro to traverse [`Parameter`][🦀] ➡ [`Parameter`][🦞], via _R1(c)_
///
/// This macro expects a &[`Parameter`][🦀], and returns an Option<&[`Parameter`][🦞]>.
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referrer_to_referent_imp`
///
/// [🦀]: crate::woog::types::Parameter
/// [🦞]: crate::woog::types::Parameter
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::sarzak::Type;
/// # use sarzak::woog::Visibility;
/// # use sarzak::woog::Parameter;
/// # use sarzak::woog_maybe_get_one_param_across_r1;
/// # let mut store = sarzak::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
///
/// let type_exz = Type::test_default(&mut sarzak_store);
///
/// let visibility_uqo = Visibility::test_default(&mut store);
/// let parsimonious_wash = "tightfisted_canvas".to_owned();
/// let type_caa = Type::test_default(&mut sarzak_store);
///
/// let visibility_hdu = Visibility::test_default(&mut store);
/// let permissible_pie = "limping_ray".to_owned();
///
/// let parameter = Parameter::new(&mut store, None, &type_exz, &visibility_uqo, parsimonious_wash);
/// let parameter_0 = Parameter::new(&mut store, Some(&parameter), &type_exz, &visibility_uqo, "smart_knee".to_owned());
///
/// let parameter_gbo = woog_maybe_get_one_param_across_r1!(parameter_0, store);
/// assert_eq!(Some(&parameter), parameter_gbo);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_maybe_get_one_param_across_r1-emit_binary_main"}}}
macro_rules! woog_maybe_get_one_param_across_r1 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"parameter-emit_one_conditional"}}}
        // nut::codegen::template::macros::emit_one_conditional
        match &$input.next {
            Some(i) => $store.exhume_parameter(i),
            None => None,
        }
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"parameter-emit_one_conditional"}}}
    }};
}
pub use woog_maybe_get_one_param_across_r1;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_maybe_get_one_param_across_r1-emit_binary_main"}}}

/// Macro to traverse [`Parameter`][🦀] ➡ [`Parameter`][🦞], via _R1_
///
/// This macro expects a &[`Parameter`][🦀], and returns a &[`Parameter`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referent_to_referrer_imp`
///
/// [🦀]: crate::woog::types::Parameter
/// [🦞]: crate::woog::types::Parameter
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::woog::Parameter;
/// # use sarzak::sarzak::Type;
/// # use sarzak::woog::Visibility;
/// # use sarzak::woog_get_one_param_across_r1;
/// # let mut store = sarzak::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
///
/// let type_hmj = Type::test_default(&mut sarzak_store);
///
/// let visibility_kny = Visibility::test_default(&mut store);
/// let evanescent_name = "fretful_actor".to_owned();
/// let type_pav = Type::test_default(&mut sarzak_store);
///
/// let visibility_eph = Visibility::test_default(&mut store);
/// let shiny_bait = "grotesque_mind".to_owned();
///
/// let parameter = Parameter::new(&mut store, None, &type_pav, &visibility_eph, shiny_bait);
/// let parameter_0 = Parameter::new(&mut store, Some(&parameter), &type_pav, &visibility_eph, "cut_farm".to_owned());
///
/// let parameter_xem = woog_get_one_param_across_r1!(parameter, store);
/// assert_eq!(&parameter_0, parameter_xem);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_get_one_param_across_r1-emit_binary_main"}}}
macro_rules! woog_get_one_param_across_r1 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"parameter-parameter-emit_one_unconditional_lookup", "is_uber":true}}}}
        // nut::codegen::template::macros::emit_one_unconditional_lookup
        $store
            .iter_parameter()
            .find(|z| z.1.next == Some($input.id))
            .map(|z| z.1)
            .unwrap()
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"parameter-parameter-emit_one_unconditional_lookup"}}}
    }};
}
pub use woog_get_one_param_across_r1;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_get_one_param_across_r1-emit_binary_main"}}}

/// Macro to traverse [`Parameter`][🦀] ➡ [`Visibility`][🦞], via _R8_
///
/// This macro expects a &[`Parameter`][🦀], and returns a &[`Visibility`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referrer_to_referent_imp`
///
/// [🦀]: crate::woog::types::Parameter
/// [🦞]: crate::woog::types::Visibility
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::sarzak::Type;
/// # use sarzak::woog::Visibility;
/// # use sarzak::woog::Parameter;
/// # use sarzak::woog_get_one_viz_across_r8;
/// # let mut store = sarzak::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
///
/// let type_spf = Type::test_default(&mut sarzak_store);
///
/// let visibility_shj = Visibility::test_default(&mut store);
/// let phobic_amusement = "waggish_bee".to_owned();
///
/// let parameter = Parameter::new(&mut store, None, &type_spf, &visibility_shj, phobic_amusement);
///
/// let visibility_hsv = woog_get_one_viz_across_r8!(parameter, store);
/// assert_eq!(&visibility_shj, visibility_hsv);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_get_one_viz_across_r8-emit_binary_main"}}}
macro_rules! woog_get_one_viz_across_r8 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"visibility-emit_one_unconditional"}}}
        // nut::codegen::template::macros::emit_one_unconditional
        $store.exhume_visibility(&$input.visibility).unwrap()
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"visibility-emit_one_unconditional"}}}
    }};
}
pub use woog_get_one_viz_across_r8;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_get_one_viz_across_r8-emit_binary_main"}}}

/// Macro to traverse [`Visibility`][🦀] ➡ [`Parameter`][🦞], via _R8_
///
/// This macro expects a &[`Visibility`][🦀], and returns a &[`Parameter`][🦞].
///
/// Generated by `nut::domain::generate_macros::generate_binary_macro_referent_to_referrer_imp`
///
/// [🦀]: crate::woog::types::Visibility
/// [🦞]: crate::woog::types::Parameter
// {"magic":"","kind":{"IgnoreBlockBegin":{}}}
///
/// # Example
///
///```
/// # use sarzak::woog::Visibility;
/// # use sarzak::woog::Parameter;
/// # use sarzak::sarzak::Type;
/// # use sarzak::woog_get_one_param_across_r8;
/// # let mut store = sarzak::woog::ObjectStore::new();
/// # let mut sarzak_store = sarzak::sarzak::ObjectStore::new();
///
/// let type_ejb = Type::test_default(&mut sarzak_store);
///
/// let visibility_tlb = Visibility::test_default(&mut store);
/// let roomy_place = "delirious_meat".to_owned();
///
/// let parameter = Parameter::new(&mut store, None, &type_ejb, &visibility_tlb, roomy_place);
/// let parameter_dmb = woog_get_one_param_across_r8!(visibility_tlb, store);
///
/// assert_eq!(&parameter, parameter_dmb);
///```
// {"magic":"","kind":"IgnoreBlockEnd"}
#[macro_export]
// {"magic":"","kind":{"CriticalBlockBegin":{"tag":"woog_get_one_param_across_r8-emit_binary_main"}}}
macro_rules! woog_get_one_param_across_r8 {
    ($input:expr, $store:expr) => {{
        // {"magic":"","kind":{"CriticalBlockBegin":{"tag":"visibility-parameter-emit_one_unconditional_lookup"}}}
        // nut::codegen::template::macros::emit_one_unconditional_lookup
        $store
            .iter_parameter()
            .find(|z| z.1.visibility == $input.get_id())
            .map(|z| z.1)
            .unwrap()
        // {"magic":"","kind":{"CriticalBlockEnd":{"tag":"visibility-parameter-emit_one_unconditional_lookup"}}}
    }};
}
pub use woog_get_one_param_across_r8;
// {"magic":"","kind":{"CriticalBlockEnd":{"tag":"woog_get_one_param_across_r8-emit_binary_main"}}}
