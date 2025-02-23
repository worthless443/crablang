// run-pass

//! Make sure that we read and write enum discriminants correctly for corner cases caused
//! by layout optimizations.

const OVERFLOW: usize = {
    // Tests for https://github.com/crablang/crablang/issues/62138.
    #[repr(u8)]
    #[allow(dead_code)]
    enum WithWraparoundInvalidValues {
        X = 1,
        Y = 254,
    }

    #[allow(dead_code)]
    enum Foo {
        A,
        B,
        C(WithWraparoundInvalidValues),
    }

    let x = Foo::B;
    match x {
        Foo::B => 0,
        _ => panic!(),
    }
};

const MORE_OVERFLOW: usize = {
    pub enum Infallible {}

    // The check that the `bool` field of `V1` is encoding a "niche variant"
    // (i.e. not `V1`, so `V3` or `V4`) used to be mathematically incorrect,
    // causing valid `V1` values to be interpreted as other variants.
    #[allow(dead_code)]
    pub enum E1 {
        V1 { f: bool },
        V2 { f: Infallible },
        V3,
        V4,
    }

    // Computing the discriminant used to be done using the niche type (here `u8`,
    // from the `bool` field of `V1`), overflowing for variants with large enough
    // indices (`V3` and `V4`), causing them to be interpreted as other variants.
    #[allow(dead_code)]
    pub enum E2<X> {
        V1 { f: bool },

        /*_00*/ _01(X), _02(X), _03(X), _04(X), _05(X), _06(X), _07(X),
        _08(X), _09(X), _0A(X), _0B(X), _0C(X), _0D(X), _0E(X), _0F(X),
        _10(X), _11(X), _12(X), _13(X), _14(X), _15(X), _16(X), _17(X),
        _18(X), _19(X), _1A(X), _1B(X), _1C(X), _1D(X), _1E(X), _1F(X),
        _20(X), _21(X), _22(X), _23(X), _24(X), _25(X), _26(X), _27(X),
        _28(X), _29(X), _2A(X), _2B(X), _2C(X), _2D(X), _2E(X), _2F(X),
        _30(X), _31(X), _32(X), _33(X), _34(X), _35(X), _36(X), _37(X),
        _38(X), _39(X), _3A(X), _3B(X), _3C(X), _3D(X), _3E(X), _3F(X),
        _40(X), _41(X), _42(X), _43(X), _44(X), _45(X), _46(X), _47(X),
        _48(X), _49(X), _4A(X), _4B(X), _4C(X), _4D(X), _4E(X), _4F(X),
        _50(X), _51(X), _52(X), _53(X), _54(X), _55(X), _56(X), _57(X),
        _58(X), _59(X), _5A(X), _5B(X), _5C(X), _5D(X), _5E(X), _5F(X),
        _60(X), _61(X), _62(X), _63(X), _64(X), _65(X), _66(X), _67(X),
        _68(X), _69(X), _6A(X), _6B(X), _6C(X), _6D(X), _6E(X), _6F(X),
        _70(X), _71(X), _72(X), _73(X), _74(X), _75(X), _76(X), _77(X),
        _78(X), _79(X), _7A(X), _7B(X), _7C(X), _7D(X), _7E(X), _7F(X),
        _80(X), _81(X), _82(X), _83(X), _84(X), _85(X), _86(X), _87(X),
        _88(X), _89(X), _8A(X), _8B(X), _8C(X), _8D(X), _8E(X), _8F(X),
        _90(X), _91(X), _92(X), _93(X), _94(X), _95(X), _96(X), _97(X),
        _98(X), _99(X), _9A(X), _9B(X), _9C(X), _9D(X), _9E(X), _9F(X),
        _A0(X), _A1(X), _A2(X), _A3(X), _A4(X), _A5(X), _A6(X), _A7(X),
        _A8(X), _A9(X), _AA(X), _AB(X), _AC(X), _AD(X), _AE(X), _AF(X),
        _B0(X), _B1(X), _B2(X), _B3(X), _B4(X), _B5(X), _B6(X), _B7(X),
        _B8(X), _B9(X), _BA(X), _BB(X), _BC(X), _BD(X), _BE(X), _BF(X),
        _C0(X), _C1(X), _C2(X), _C3(X), _C4(X), _C5(X), _C6(X), _C7(X),
        _C8(X), _C9(X), _CA(X), _CB(X), _CC(X), _CD(X), _CE(X), _CF(X),
        _D0(X), _D1(X), _D2(X), _D3(X), _D4(X), _D5(X), _D6(X), _D7(X),
        _D8(X), _D9(X), _DA(X), _DB(X), _DC(X), _DD(X), _DE(X), _DF(X),
        _E0(X), _E1(X), _E2(X), _E3(X), _E4(X), _E5(X), _E6(X), _E7(X),
        _E8(X), _E9(X), _EA(X), _EB(X), _EC(X), _ED(X), _EE(X), _EF(X),
        _F0(X), _F1(X), _F2(X), _F3(X), _F4(X), _F5(X), _F6(X), _F7(X),
        _F8(X), _F9(X), _FA(X), _FB(X), _FC(X), _FD(X), _FE(X), _FF(X),

        V3,
        V4,
    }

    if let E1::V2 { .. } = (E1::V1 { f: true }) {
        unreachable!()
    }
    if let E1::V1 { .. } = (E1::V1 { f: true }) {
    } else {
        unreachable!()
    }

    if let E2::V1 { .. } = E2::V3::<Infallible> {
        unreachable!()
    }
    if let E2::V3 { .. } = E2::V3::<Infallible> {
    } else {
        unreachable!()
    }

    0
};

fn main() {
    assert_eq!(OVERFLOW, 0);
    assert_eq!(MORE_OVERFLOW, 0);
}
