use bevy_utils::{HashMap, HashSet};
use kahuna::cube_grid::CubeGrid;
use kahuna::hashset_state::HashsetState;
use kahuna::set_rule::*;
use serde::{Deserialize, Serialize};

const RIGHT: (isize, isize, isize) = (1, 0, 0);
const FRONT: (isize, isize, isize) = (0, 0, -1);
const LEFT: (isize, isize, isize) = (-1, 0, 0);
const BACK: (isize, isize, isize) = (0, 0, 1);
const ABOVE: (isize, isize, isize) = (0, 1, 0);
const BELOW: (isize, isize, isize) = (0, -1, 0);
#[derive(Serialize, Deserialize, Default)]
pub struct Prototypes(pub HashMap<String, Prototype>);

#[derive(Serialize, Deserialize, Default)]
pub struct Prototype {
    pub mesh_name: String,
    pub mesh_rotation: u8,
    pub pos_x: String,
    pub neg_x: String,
    pub pos_y: String,
    pub neg_y: String,
    pub pos_z: String,
    pub neg_z: String,
    pub constrain_to: String,
    pub constrain_from: String,
    pub weight: u32,
    // 0 - +x
    // 1 - -z
    // 2 - -x
    // 3 - +z
    // 4 - +y
    // 5 - -y
    pub valid_neighbors: [HashSet<String>; 6],
}

fn main() {
    let data = r#"
    {
        "p0": {
            "mesh_name": "wfc_module_0",
            "mesh_rotation": 0,
            "pos_x": "0",
            "neg_x": "1s",
            "pos_y": "0f",
            "neg_y": "1s",
            "pos_z": "-1",
            "neg_z": "-1",
            "constrain_to": "",
            "constrain_from": "",
            "weight": 1,
            "valid_neighbors": [
                [
                    "p1"
                ],
                [
                    "p3"
                ],
                [
                    "p1",
                    "p2",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p2",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p-1"
                ],
                [
                    "p-1"
                ]
            ]
        },
        "p1": {
            "mesh_name": "wfc_module_0",
            "mesh_rotation": 1,
            "pos_x": "1s",
            "neg_x": "0f",
            "pos_y": "0",
            "neg_y": "1s",
            "pos_z": "-1",
            "neg_z": "-1",
            "constrain_to": "",
            "constrain_from": "",
            "weight": 1,
            "valid_neighbors": [
                [
                    "p0",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p2"
                ],
                [
                    "p0"
                ],
                [
                    "p2",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p-1"
                ],
                [
                    "p-1"
                ]
            ]
        },
        "p2": {
            "mesh_name": "wfc_module_0",
            "mesh_rotation": 2,
            "pos_x": "1s",
            "neg_x": "0",
            "pos_y": "1s",
            "neg_y": "0f",
            "pos_z": "-1",
            "neg_z": "-1",
            "constrain_to": "",
            "constrain_from": "",
            "weight": 1,
            "valid_neighbors": [
                [
                    "p0",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p0",
                    "p1",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p3"
                ],
                [
                    "p1"
                ],
                [
                    "p-1"
                ],
                [
                    "p-1"
                ]
            ]
        },
        "p3": {
            "mesh_name": "wfc_module_0",
            "mesh_rotation": 3,
            "pos_x": "0f",
            "neg_x": "1s",
            "pos_y": "1s",
            "neg_y": "0",
            "pos_z": "-1",
            "neg_z": "-1",
            "constrain_to": "",
            "constrain_from": "",
            "weight": 1,
            "valid_neighbors": [
                [
                    "p2"
                ],
                [
                    "p0",
                    "p1",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p1",
                    "p2",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p0"
                ],
                [
                    "p-1"
                ],
                [
                    "p-1"
                ]
            ]
        },
        "p4": {
            "mesh_name": "wfc_module_1",
            "mesh_rotation": 0,
            "pos_x": "1s",
            "neg_x": "1s",
            "pos_y": "1s",
            "neg_y": "1s",
            "pos_z": "-1",
            "neg_z": "-1",
            "constrain_to": "",
            "constrain_from": "",
            "weight": 1,
            "valid_neighbors": [
                [
                    "p0",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p0",
                    "p1",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p1",
                    "p2",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p2",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p-1"
                ],
                [
                    "p-1"
                ]
            ]
        },
        "p5": {
            "mesh_name": "wfc_module_1",
            "mesh_rotation": 1,
            "pos_x": "1s",
            "neg_x": "1s",
            "pos_y": "1s",
            "neg_y": "1s",
            "pos_z": "-1",
            "neg_z": "-1",
            "constrain_to": "",
            "constrain_from": "",
            "weight": 1,
            "valid_neighbors": [
                [
                    "p0",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p0",
                    "p1",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p1",
                    "p2",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p2",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p-1"
                ],
                [
                    "p-1"
                ]
            ]
        },
        "p6": {
            "mesh_name": "wfc_module_1",
            "mesh_rotation": 2,
            "pos_x": "1s",
            "neg_x": "1s",
            "pos_y": "1s",
            "neg_y": "1s",
            "pos_z": "-1",
            "neg_z": "-1",
            "constrain_to": "",
            "constrain_from": "",
            "weight": 1,
            "valid_neighbors": [
                [
                    "p0",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p0",
                    "p1",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p1",
                    "p2",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p2",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p-1"
                ],
                [
                    "p-1"
                ]
            ]
        },
        "p7": {
            "mesh_name": "wfc_module_1",
            "mesh_rotation": 3,
            "pos_x": "1s",
            "neg_x": "1s",
            "pos_y": "1s",
            "neg_y": "1s",
            "pos_z": "-1",
            "neg_z": "-1",
            "constrain_to": "",
            "constrain_from": "",
            "weight": 1,
            "valid_neighbors": [
                [
                    "p0",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p0",
                    "p1",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p1",
                    "p2",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p2",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7"
                ],
                [
                    "p-1"
                ],
                [
                    "p-1"
                ]
            ]
        },
        "p-1": {
            "mesh_name": "-1",
            "mesh_rotation": 0,
            "pos_x": "-1f",
            "neg_x": "-1f",
            "pos_y": "-1f",
            "neg_y": "-1f",
            "pos_z": "-1f",
            "neg_z": "-1f",
            "constrain_to": "-1",
            "constrain_from": "-1",
            "weight": 1,
            "valid_neighbors": [
                [
                    "p-1"
                ],
                [
                    "p-1"
                ],
                [
                    "p-1"
                ],
                [
                    "p-1"
                ],
                [
                    "p0",
                    "p1",
                    "p2",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7",
                    "p-1"
                ],
                [
                    "p0",
                    "p1",
                    "p2",
                    "p3",
                    "p4",
                    "p5",
                    "p6",
                    "p7",
                    "p-1"
                ]
            ]
        }
    }"#;

    let prototypes: Prototypes = serde_json::from_str(data).unwrap();

    let mut all_state = HashsetState::<String> {
        hashset: HashSet::new(),
    };
    let mut weights: HashMap<String, u32> = HashMap::new();
    for prototype in prototypes.0.iter() {
        all_state.hashset.insert(prototype.0.clone());
        weights.insert(prototype.0.clone(), prototype.1.weight);
    }

    let observer = WeightedSetCollapseObserver::<String> { weights };
    let mut rule = SetCollapseRuleBuilder::new(observer, all_state.clone());
    for (k, v) in prototypes.0 {
        rule = rule
            .allow(
                &HashsetState::<String>::new_final(&k),
                &[(
                    RIGHT,
                    HashsetState::<String> {
                        hashset: v.valid_neighbors[0].clone(),
                    },
                )],
            )
            .allow(
                &HashsetState::<String>::new_final(&k),
                &[(
                    FRONT,
                    HashsetState::<String> {
                        hashset: v.valid_neighbors[1].clone(),
                    },
                )],
            )
            .allow(
                &HashsetState::<String>::new_final(&k),
                &[(
                    LEFT,
                    HashsetState::<String> {
                        hashset: v.valid_neighbors[2].clone(),
                    },
                )],
            )
            .allow(
                &HashsetState::<String>::new_final(&k),
                &[(
                    BACK,
                    HashsetState::<String> {
                        hashset: v.valid_neighbors[3].clone(),
                    },
                )],
            )
            .allow(
                &HashsetState::<String>::new_final(&k),
                &[(
                    ABOVE,
                    HashsetState::<String> {
                        hashset: v.valid_neighbors[4].clone(),
                    },
                )],
            )
            .allow(
                &HashsetState::<String>::new_final(&k),
                &[(
                    BELOW,
                    HashsetState::<String> {
                        hashset: v.valid_neighbors[5].clone(),
                    },
                )],
            );
    }

    // TODO: set predetermined states by modifying init_fn based on current coords
    let init_fn = |_x, _y, _z| all_state.clone();
    let mut space = CubeGrid::new(3, 3, 3, init_fn);
    kahuna::collapse(&mut space, &rule.build());

    // Print out the collapsed 3x3 cube layer by layer
    for y in 0..3 {
        print!("Layer: {}\n", y);
        for z in 0..3 {
            for x in 0..3 {
                print!("{:?} ", (&space[(x, y, z)]));
            }
            print!("\n");
        }
        print!("\n");
    }
}
