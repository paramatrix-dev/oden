use anvil::{Axis, Cuboid, Cylinder, IntoLength, Plane, Rectangle, Sphere, point};
use oden::compile_input;

#[test]
fn test_cube() {
    let text = "
        part Box:
            part.add(Cube(4mm))
        ";
    let actual = compile_input(text);
    assert_eq!(actual, Ok(Cuboid::from_mm(4., 4., 4.)))
}

#[test]
fn test_cuboid() {
    let text = "
        part Box:
            part.add(Cuboid(4mm, 5mm, 6mm))
        ";
    let actual = compile_input(text);
    assert_eq!(actual, Ok(Cuboid::from_mm(4., 5., 6.)))
}

#[test]
fn test_sphere() {
    let text = "
        part Box:
            part.add(Sphere(5mm))
        ";
    let actual = compile_input(text);
    assert_eq!(actual, Ok(Sphere::from_radius(5.mm())))
}

#[test]
fn test_cylinder() {
    let text = "
        part Box:
            part.add(Cylinder(5mm, 6mm))
        ";
    let actual = compile_input(text);
    assert_eq!(actual, Ok(Cylinder::from_radius(5.mm(), 6.mm())))
}

#[test]
fn test_centered_cuboid_mixed_units() {
    let text = "
        part Box:
            part.add(Cuboid(4m, 5mm, 6mm))
        ";
    let actual = compile_input(text);
    assert_eq!(actual, Ok(Cuboid::from_mm(4000., 5., 6.)))
}

#[test]
fn test_centered_cuboid_different_spacing() {
    let text = "
        part Box:
            part.add(  Cuboid(4mm,5mm,  6mm ) )
        ";
    assert_eq!(compile_input(text), Ok(Cuboid::from_mm(4., 5., 6.)))
}

#[test]
fn test_centered_cuboid_with_variable() {
    let text = "
        part Box:
            width = 4mm
            lenght = 5mm
            height = 6mm
            part.add(Cuboid(width, lenght, height))
        ";
    assert_eq!(compile_input(text), Ok(Cuboid::from_mm(4., 5., 6.)))
}

#[test]
fn test_add_two_cuboids() {
    let text = "
        part Box:
            part.add(Cuboid(1mm, 1mm, 5mm))
            part.add(Cuboid(5mm, 1mm, 1mm))
        ";
    assert_eq!(
        compile_input(text),
        Ok(Cuboid::from_mm(1., 1., 5.).add(&Cuboid::from_mm(5., 1., 1.)))
    )
}

#[test]
fn test_subtract() {
    let text = "
        part Box:
            part.add(Cuboid(1mm, 1mm, 5mm))
            part.subtract(Cuboid(1mm, 1mm, 1mm))
        ";
    assert_eq!(
        compile_input(text),
        Ok(Cuboid::from_mm(1., 1., 5.).subtract(&Cuboid::from_mm(1., 1., 1.)))
    )
}

#[test]
fn test_intersect() {
    let text = "
        part Box:
            part.add(Cuboid(1mm, 1mm, 5mm))
            part.intersect(Cuboid(1mm, 1mm, 1mm))
        ";
    assert_eq!(
        compile_input(text),
        Ok(Cuboid::from_mm(1., 1., 5.).intersect(&Cuboid::from_mm(1., 1., 1.)))
    )
}

#[test]
fn test_move_to() {
    let text = "
        part Box:
            part.add(Cuboid(5mm, 5mm, 5mm).move_to(1mm, 1mm, 1mm))
        ";
    assert_eq!(
        compile_input(text),
        Ok(Cuboid::from_mm(5., 5., 5.).move_to(point!(1.mm(), 1.mm(), 1.mm())))
    )
}

#[test]
fn test_statement_boogaloo() {
    let text = "
        part Box:
            part.add(
                Cuboid(
                    1mm,
                    1mm,
                    5mm)
            ).add(
                Cuboid(5mm, 1mm, 1mm)
            )
        ";
    assert_eq!(
        compile_input(text),
        Ok(Cuboid::from_mm(1., 1., 5.).add(&Cuboid::from_mm(5., 1., 1.)))
    )
}

#[test]
fn test_comment() {
    let text = "
        part Box:
            // this is a comment and not valid oden code
            part.add(Cube(5mm))
        ";
    assert_eq!(compile_input(text), Ok(Cuboid::from_mm(5., 5., 5.)))
}

#[test]
fn test_inline_comment() {
    let text = "
        part Box:
            part.add(Cube(5mm)) // this is a comment and not valid oden code
        ";
    assert_eq!(compile_input(text), Ok(Cuboid::from_mm(5., 5., 5.)))
}

#[test]
fn test_rectangle_extrude() {
    let text = "
        part Box:
            sketch = Rectangle(5mm, 6mm)
            part.add(sketch.extrude(Plane.XY(), 7mm))
        ";
    assert_eq!(
        compile_input(text),
        Ok(Rectangle::from_dim(5.mm(), 6.mm())
            .extrude(Plane::xy(), 7.mm())
            .unwrap())
    )
}

#[test]
fn test_cuboid_circular_pattern() {
    let text = "
        part Box:
            part = Cuboid(1m, 1m, 1m).move_to(1m, 1m, 1m).circular_pattern(Axis.Z(), 4)
        ";
    assert_eq!(
        compile_input(text),
        Ok(Cuboid::from_m(1., 1., 1.)
            .move_to(point!(1.m(), 1.m(), 1.m()))
            .circular_pattern(Axis::<3>::z(), 4))
    )
}
