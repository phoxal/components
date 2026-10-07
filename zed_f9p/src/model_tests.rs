//! Static qualification of the component's authored native bindings, without a native engine.
use phoxal::artifact::document::{ComponentDocument, NativeTargetKind};

#[test]
fn every_capability_targets_the_owned_model_and_resources_exist() {
    let ComponentDocument::V0 {
        model,
        capabilities,
        assets,
    } = serde_yaml::from_str(include_str!("../component.yaml")).unwrap();
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let xml = std::fs::read_to_string(root.join(&model.file)).unwrap();
    let native = roxmltree::Document::parse(&xml).unwrap();
    assert_eq!(native.root_element().tag_name().name(), "mujoco");
    assert_eq!(
        native
            .descendants()
            .filter(|node| node.has_tag_name("body")
                && node.attribute("name") == Some(model.root_body.as_str()))
            .count(),
        1
    );
    for capability in capabilities.values() {
        let tag = match capability.target.kind {
            NativeTargetKind::Actuator => "velocity",
            NativeTargetKind::Joint => "joint",
            NativeTargetKind::Site => "site",
            NativeTargetKind::Camera => "camera",
        };
        assert_eq!(
            native
                .descendants()
                .filter(|node| node.has_tag_name(tag)
                    && node.attribute("name") == Some(capability.target.id.as_str()))
                .count(),
            1,
            "missing or ambiguous {tag} {}",
            capability.target.id
        );
        if let Some(joint) = &capability.joint {
            assert!(
                native.descendants().any(|node| node.has_tag_name("joint")
                    && node.attribute("name") == Some(joint.as_str()))
            );
        }
        for signal in capability.signals.values() {
            assert!(
                native
                    .descendants()
                    .any(|node| node.attribute("name") == Some(signal.as_str())
                        && node
                            .parent()
                            .is_some_and(|parent| parent.has_tag_name("sensor"))),
                "missing sensor {signal}"
            );
        }
    }
    for asset in assets {
        assert!(root.join(asset).exists());
    }
    for node in native
        .descendants()
        .filter(|node| node.has_tag_name("mesh"))
    {
        if let Some(file) = node.attribute("file") {
            assert!(root.join(file).is_file());
        }
    }
}
