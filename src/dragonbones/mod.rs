//! Dragon bones skeleton format (json)[http://docs.egret.com/dragonbones/en/docs/dbLibs/5foramt]

use semver::Version;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Skeleton {
    /// DragonBones data name
    pub name: String,
    /// Data version
    pub version: Version,
    /// Compatible version
    pub compatible_version: Version,
    /// Animation frame rate
    pub frame_rate: f32,
    /// Custom data (optional property, null by default)
    #[serde(default)]
    pub user_data: Value,
    /// Skeleton list
    pub armature: Vec<Armature>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AnimationType {
    Armature,
    MovieClip,
    Stage,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Armature {
    /// Skeleton name (one DragonBones data can contain multiple skeleton)
    pub name: String,
    /// Animation frame rate (optional property, use global frame rate by default)
    #[serde(default = "default_armature_frame_rate")]
    pub frame_rate: f32,
    /// Animation type (optional property, "Armature" by default)
    /// ["Armature": skeleton animation, "MovieClip": basic animation, "Stage": stage animation]
    #[serde(rename = "type")]
    pub animation_type: AnimationType,
    /// Custom data (optional property, null by default)
    #[serde(default)]
    pub user_data: Value,
    /// Behavior list that’s added to the back of stage (optional property, null by default)
    #[serde(default)]
    pub default_actions: Vec<Action>,
    /// Bone list of this skeleton contains
    pub bone: Vec<Bone>,
    /// Slot list of this skeleton
    pub slot: Vec<Slot>,
    /// Skin list of this skeleton contains
    pub skin: Vec<Skin>,
    /// IK Constraint list of this skeleton contains
    pub ik: Vec<Ik>,
    /// The animations list of this skeleton contains
    pub animation: Vec<Animation>,
}

const fn default_armature_frame_rate() -> f32 {
    -1.0
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct Action {
    #[serde(flatten)]
    actions: [String; 2],
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Bone {
    /// Bone name
    pub name: String,
    /// Parent bone name
    pub parent: String,
    /// Custom data [any type] (optional property, null by default)
    #[serde(default)]
    pub user_data: Value,
    /// Bone’s registration to skeleton: replacement/chamfer/zoom (optional properties, null by default)
    #[serde(default)]
    pub transform: Transform,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct Transform {
    /// Horizontal displacement (Optional properties, 0.00 by default)
    pub x: f32,
    /// Vertical displacement (optional properties, 0.00 by default)
    pub y: f32,
    /// Horizontal chamfer (optional properties, 0.0000 by default)
    pub sk_x: f32,
    /// Vertical chamfer (optional properties  0.0000 by default)
    pub sk_y: f32,
    /// horizontal zoom (optional properties, 1.0000 by default)
    pub sc_x: f32,
    /// Vertical zoom (optional properties, 1.0000 by default)
    pub sc_y: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            sk_x: 0.0,
            sk_y: 0.0,
            sc_x: 1.0,
            sc_y: 1.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Slot {
    /// Slot name
    pub name: String,
    /// Parent bone name of the slot
    pub parent: String,
    /// Index of display object by default (optional properties, 0 by default)
    #[serde(default = "default_slot_display_index")]
    pub display_index: isize,
    /// Blend mode (optional properties, null by default)
    #[serde(default = "default_slot_blend_mode")]
    pub blend_mode: BlendMode,
    /// Custom data [any type] (optional properties, null by default)
    #[serde(default)]
    pub user_data: Value,
    /// Color overlay of display object (Optional properties, null by default)
    #[serde(default)]
    pub color: Color,
    /// Behavior list that’s added to the stage (optional properties, null by default)
    #[serde(default)]
    pub actions: Vec<Action>,
}

const fn default_slot_blend_mode() -> BlendMode {
    BlendMode
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub struct BlendMode;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct Color {
    /// Transparent overlay [0~100] (Optional properties default: 100)
    pub a_m: f32,
    /// red overlay [0~100] (Optional properties default: 100)
    pub r_m: f32,
    /// green overlay [0~100] (Optional properties default: 100)
    pub g_m: f32,
    /// blue overlay [0~100] (Optional properties default: 100)
    pub b_m: f32,
    /// Transparent skew [-255~255] (Optional properties default: 0)
    pub a_o: f32,
    /// red skew [-255~255] (Optional properties
    pub r_o: f32,
    /// green skew [-255~255] (Optional properties default: 0)
    pub g_o: f32,
    /// blue skew [-255~255] (Optional properties default: 0)
    pub b_o: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            a_m: 100.0,
            r_m: 100.0,
            g_m: 100.0,
            b_m: 100.0,
            a_o: 0.0,
            r_o: 0.0,
            g_o: 0.0,
            b_o: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    /// Skin name
    pub name: String,
    /// Slot list of this skin contains
    pub slot: SkinSlot,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SkinSlot {
    /// Slot name
    pub name: String,
    /// Display object list of this slot contains
    pub display: Vec<Display>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BoundBoxType {
    Rectangle,
    Ellipse,
    Polygon,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Display {
    /// Display object name
    pub name: String,
    /// Display object type (Optional properties, image by default)
    /// ["image": chartlet, "armature": skeleton, "mesh": mesh, ... Other extension types]
    #[serde(default, rename = "type")]
    pub display_type: String,
    /// Skeleton name of sub skeleton point to, map name of mesh contains (optional property, null by default, only available to sub skeleton and mesh)
    #[serde(default)]
    pub path: String,
    /// Index of shared mesh (optional property, null by default, only available to mesh)
    #[serde(default)]
    pub share: String,
    /// If inherit animation or not (optional property, true by default, only available to shared mesh)
    #[serde(rename = "inheritFDD", default = "default_display_inherit_fdd")]
    pub inherit_fdd: bool,
    /// Bound box type (optional property, rectangle by default, only available to bound box)
    /// ["rectangle": rectangle, "ellipse": ellipse, "polygon": Custom polygon]
    #[serde(default = "default_display_sub_type")]
    pub sub_type: BoundBoxType,
    // TODO: Not sure about this data type
    /// Display object color (optional property, 0 by default, only available to bound box)
    #[serde(default)]
    pub color: u32,
    /// Display object displacement/chamfer/zoom relative to bone (optional properties default: null)
    #[serde(default)]
    pub transform: Transform,
    /// Pivot point of display object (Optional properties, null by default, no available to sekleton)
    #[serde(default = "default_display_pivot")]
    pub pivot: [f32; 2],
    /// Width and height of rectangle or ellipse (Optional properties, 0 by default, only available to bound box)，
    #[serde(default)]
    pub width: f32,
    #[serde(default)]
    pub height: f32,
    // ! FIXME
    // TODO: The following arrays must be flat
    /// Coordinate list of the vertex relative to the display object pivot point (optional property, null by default, only available to mesh or custom polygon bound box)
    /// [x0, y0, x1, y1, ...]
    #[serde(default)]
    pub vertices: Vec<[f32; 2]>,
    /// Vertex UV coordinate list (Optional properties, null by default, only available to mesh)
    /// [u0, v0, u1, v1, ...]
    #[serde(default)]
    pub uvs: Vec<[f32; 2]>,
    /// Triangle vertex index list (Optional properties, null by default, only available to mesh)
    #[serde(default)]
    pub triangles: Vec<u32>,
    /// Vertex weight list (Optional properties, null by default, only available to mesh)
    /// [Bone number, bone index, weight, ..., ...]
    #[serde(default)]
    pub weights: Vec<(usize, usize, f32)>,
    /// matrix transformation for skin slot registration (Optional properties, null by default, only available to mesh)
    /// [a, b, c, d, tx, ty]
    #[serde(default)]
    pub slot_pose: Vec<[f32; 6]>,
    /// Matrix transformation for skin bone registration (optional attribute default: null, valid only for grid)
    /// [bone index, a, b, c, d, tx, ty, ...]
    #[serde(default)]
    pub bone_pose: Vec<(usize, f32, f32, f32, f32, f32, f32)>,
}

const fn default_display_sub_type() -> BoundBoxType {
    BoundBoxType::Rectangle
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Ik {
    /// ik constraint name
    name: String,
    /// Bound bone name
    bone: String,
    /// Target bone name
    target: String,
    /// Bend direction (Optional properties, true by default)
    /// [true: positive direction / clockwise, false: reverse direction / counterclockwise]
    #[serde(default = "default_ik_bend_positive")]
    bend_positive: bool,
    /// Length of skeleton chain (Optional properties, 0 by default)
    /// [0: only constrain bone, n: Constrain bone and bone’s parent bone of up n-level]
    #[serde(default = "default_ik_chain")]
    chain: usize,
    /// Weight [0.00: No constraint ~ 1.00: Full constraint] (Optional properties, 0 by default)
    #[serde(default = "default_ik_weight")]
    weight: f32,
}

const fn default_ik_bend_positive() -> bool {
    true
}

const fn default_ik_chain() -> usize {
    0
}

const fn default_ik_weight() -> f32 {
    0.0
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Animation {
    /// Animation name
    pub name: String,
    /// Loop number [0: loop infinitely, n: loop n times] (Optional property, 1 by default)
    #[serde(rename = "loop", default = "default_animation_loop")]
    pub loop_number: usize,
    /// Animation frame length (Optional properties, 1 by default)
    #[serde(default = "default_animation_duration")]
    pub duration: f32,
    /// Key frame list of this animation contains (Optional properties, null by default)
    #[serde(default)]
    pub frame: Vec<AnimationFrame>,
    /// Depth order timeline
    #[serde(default)]
    pub z_order: Vec<AnimationZOrder>,
    /// The skeleton timeline list of this animation contains (Optional properties, null by default)
    #[serde(default)]
    pub bone: Vec<AnimationBone>,
    /// Slot timeline list of this animation contains
    #[serde(default)]
    pub slot: Vec<AnimationSlot>,
    /// The freeform timeline list that this animation contains (Optional properties, null by default)
    #[serde(default)]
    pub ffd: Vec<AnimationFdd>,
}

const fn default_animation_loop() -> usize {
    1
}

const fn default_animation_duration() -> f32 {
    1.0
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct AnimationFrame {
    // Frame length (Optional properties, 1 by default)
    pub duration: f32,
    // Frame sound (Optional properties, null by default)
    pub sound: String,
    /// Frame behaviors list (Optional properties, null by default)
    pub events: Vec<Event>,
    /// Frame behavior list (Optional properties, null by default)
    pub actions: Vec<Action>,
}

impl Default for AnimationFrame {
    fn default() -> Self {
        Self {
            duration: default_animation_duration(),
            sound: "".into(),
            events: vec![],
            actions: vec![],
        }
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct Event {
    /// Event name
    pub name: String,
    /// Bone name (Optional properties, null by default)
    pub bone: String,
    /// Slot name (Optional properties, null by default)
    pub slot: String,
    // Event parameter list (Optional properties, null by default)
    pub ints: Vec<i32>,
    pub floats: Vec<f32>,
    pub strings: Vec<String>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct AnimationZOrder {
    pub frame: Vec<AnimationZOrderFrame>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct AnimationZOrderFrame {
    /// Frame length (Optional properties,1 by default)
    pub duration: f32,
    // !FIXME
    // TODO:
    /// Slot offset [slotIndexA, offsetA, slotIndexB, offsetB, ...] (Optional properties, null by default)
    #[serde(default)]
    pub z_order: Vec<(usize, isize)>,
}

impl Default for AnimationZOrderFrame {
    fn default() -> Self {
        Self {
            duration: default_animation_duration(),
            z_order: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnimationBone {
    /// Timeline name (corresponding to the bone name)
    pub name: String,
    /// Timeline zoom (Optional properties, 1.00 by default)
    #[serde(default = "default_scale")]
    pub scale: f32,
    /// Timeline offset (Optional properties, 0.00 by default)
    #[serde(default)]
    pub offset: f32,
    /// The key frame list of this timeline contains (Optional properties, null by default)
    #[serde(default)]
    pub frame: Vec<AnimationBoneFrame>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct AnimationTween {
    /// Easing type [0: use tweenEasing or curve to describe easing type, 1~N: Other extension easing type enumeration (about the details of enumeration we will define it in the future)] Optional properties, 0 by default
    pub tween_type: usize,
    /// Easing value [0.00: linear, null: no easing] (Optional properties, null by default)
    pub tween_easing: f32,
    /// Bezier curve easing parameter list [x1, y1, x2, y2, ...] (optional properties, null by default)
    pub curve: Vec<f32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct AnimationBoneFrame {
    /// Frame length (Optional properties, 1 by default)
    pub duration: f32,
    pub tween: AnimationTween,
    /// Bone displacement/ bevel / scaling (Optional properties, null by default)
    pub transform: Transform,
}

impl Default for AnimationBoneFrame {
    fn default() -> Self {
        Self {
            duration: default_animation_duration(),
            tween: Default::default(),
            transform: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnimationSlot {
    /// Timeline name (corresponding to slot name)
    pub name: String,
    /// Key frame list of this timeline contains (Optional properties, null by default)
    #[serde(default)]
    pub frame: Vec<AnimationSlotFrame>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct AnimationSlotFrame {
    /// Frame length (Optional properties, 1 by default)
    pub duration: f32,
    #[serde(flatten)]
    pub tween: AnimationTween,
    /// The display object index of this frame (the corresponding slot display object list in the skin) (Optional properties,0 by default)
    pub display_index: isize,
    /// Color overlay of display object (Optional properties, null by default)
    pub color: Color,
    /// The executed action behavior list when playing current frame (Optional properties, null by default)
    pub actions: Vec<Action>,
}

impl Default for AnimationSlotFrame {
    fn default() -> Self {
        Self {
            duration: default_animation_duration(),
            tween: Default::default(),
            display_index: 0,
            color: Default::default(),
            actions: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AnimationFdd {
    /// Timeline name
    pub name: String,
    /// Skin name
    pub skin: String,
    /// Slot name
    pub slot: String,
    /// Key frame list of this timeline contains (Optional properties, null by default)
    #[serde(default)]
    pub frame: Vec<AnimationFddFrame>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct AnimationFddFrame {
    /// Frame length (Optional properties, 1 by default)
    pub duration: f32,
    #[serde(flatten)]
    pub tween: AnimationTween,
    /// Vertex coordinate list index offset (Optional properties, null by default)
    pub offset: isize,
    // TODO: Fatten list
    /// Vertex coordinate list x0, y0, x1, y1, ...: relative displacement [Optional properties, null by default]
    pub vertices: Vec<[f32; 2]>,
}

impl Default for AnimationFddFrame {
    fn default() -> Self {
        Self {
            duration: default_animation_duration(),
            tween: Default::default(),
            offset: 0,
            vertices: vec![],
        }
    }
}

const fn default_scale() -> f32 {
    1.0
}

const fn default_slot_display_index() -> isize {
    0
}

fn default_display_pivot() -> [f32; 2] {
    [0.5; 2]
}

const fn default_display_inherit_fdd() -> bool {
    true
}
