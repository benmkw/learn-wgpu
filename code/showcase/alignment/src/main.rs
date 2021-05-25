#![allow(dead_code)]

use crevice::std140::AsStd140;
use crevice::std430::AsStd430;
use type_layout::TypeLayout;

// by default rust does not give any guarantees about the layout of structs
#[derive(AsStd140, AsStd430, TypeLayout)]
struct AnyBlock {
    a: f32,
    b: Anon,
    c: f32,
    d: mint::Vector3<f32>,
    e: mint::ColumnMatrix3<f32>,
    f: f32,
}

#[derive(AsStd140, AsStd430, TypeLayout)]
struct End {
    d: mint::Vector3<f32>,
    f: f32,
}

#[derive(AsStd140, AsStd430, TypeLayout)]
struct Anon {
    u: f32,
    v: f32,
}

mod std140 {
    #[repr(C)]
    pub struct AnyBlock {
        a: f32,
        _p1: [u8; 3 * 4],
        b: Anon,
        _p2: [u8; 2 * 4],
        c: f32,
        _p3: [u8; 3 * 4],
        // each array entry has an alignment of 16, thus each float gets padded with 3 more floats
        d: [[f32; 4]; 3],
        // each array member has four byte padding at the end so its length 4 even though its Mat3
        e: [[f32; 4]; 3],
        f: f32,
        _p4: [u8; 3 * 4], // padding at the end
    }

    #[repr(C)]
    struct Anon {
        u: f32,
        v: f32,
    }
}

mod std430 {
    #[repr(C)]
    pub struct AnyBlock {
        a: f32,
        b: Anon,
        c: f32,
        // d has four byte padding at the end so its Vec4 instead of Vec3
        d: [f32; 4],
        // each array member has four byte padding at the end so its length 4 even though its Mat3
        e: [[f32; 4]; 3],
        f: f32,
        _p: [u8; 3 * 4], // padding at the end
    }

    #[repr(C)]
    struct Anon {
        u: f32,
        v: f32,
    }
}

fn main() {
    // plain rust type layout
    assert_eq!(std::mem::size_of::<AnyBlock>(), 68);

    // 140
    assert_eq!(std::mem::size_of::<std140::AnyBlock>(), 160);
    assert_eq!(
        std::mem::size_of::<<AnyBlock as AsStd140>::Std140Type>(),
        116 // still wrong?
    );
    println!("{}", <AnyBlock as AsStd140>::Std140Type::type_layout());

    // 430
    assert_eq!(std::mem::size_of::<std430::AnyBlock>(), 96);
    assert_eq!(
        std::mem::size_of::<<AnyBlock as AsStd430>::Std430Type>(),
        84 // still wrong?
    );
    println!("{}", <AnyBlock as AsStd430>::Std430Type::type_layout());

    // minimal example for issue wip
    println!("{}", <End as AsStd430>::Std430Type::type_layout());
    assert_eq!(
        std::mem::size_of::<<End as AsStd430>::Std430Type>(),
        16 // should be 20?
    );
}
