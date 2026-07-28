use structural_system::*;

#[test]
fn test_structural_system_new() {
    let system = StructuralSystem::new(4);
    assert!(system.step != None); // structure exists
}
