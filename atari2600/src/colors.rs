pub use common::colors::Palette;

/// Creates a TIA palette of RGBA colors out of an `u32` array slice. See
/// [`common::colors::create_palette`] for the color representation details.
///
/// Note: TIA only uses 7 bits for representing colors, and bit 0 is unused. For
/// simplicity, we just store each color twice so that accessing the palette
/// with bit 0 set either to 0 or 1 yields the same RGBA pixel. See
/// [`tia::VideoOutput.pixel`](../tia/struct.VideoOutput.html#structfield.pixel)
pub fn create_tia_palette(colors: &[u32]) -> Palette {
    common::colors::create_palette(colors)
        .iter()
        .flat_map(|c| vec![*c, *c])
        .collect()
}

/// Returns an NTSC palette. Source:
/// http://www.qotile.net/minidig/docs/tia_color.html
pub fn ntsc_palette() -> Palette {
    create_tia_palette(&[
        0x000000, 0x404040, 0x6C6C6C, 0x909090, 0xB0B0B0, 0xC8C8C8, 0xDCDCDC, 0xECECEC, 0x444400,
        0x646410, 0x848424, 0xA0A034, 0xB8B840, 0xD0D050, 0xE8E85C, 0xFCFC68, 0x702800, 0x844414,
        0x985C28, 0xAC783C, 0xBC8C4C, 0xCCA05C, 0xDCB468, 0xECC878, 0x841800, 0x983418, 0xAC5030,
        0xC06848, 0xD0805C, 0xE09470, 0xECA880, 0xFCBC94, 0x880000, 0x9C2020, 0xB03C3C, 0xC05858,
        0xD07070, 0xE08888, 0xECA0A0, 0xFCB4B4, 0x78005C, 0x8C2074, 0xA03C88, 0xB0589C, 0xC070B0,
        0xD084C0, 0xDC9CD0, 0xECB0E0, 0x480078, 0x602090, 0x783CA4, 0x8C58B8, 0xA070CC, 0xB484DC,
        0xC49CEC, 0xD4B0FC, 0x140084, 0x302098, 0x4C3CAC, 0x6858C0, 0x7C70D0, 0x9488E0, 0xA8A0EC,
        0xBCB4FC, 0x000088, 0x1C209C, 0x3840B0, 0x505CC0, 0x6874D0, 0x7C8CE0, 0x90A4EC, 0xA4B8FC,
        0x00187C, 0x1C3890, 0x3854A8, 0x5070BC, 0x6888CC, 0x7C9CDC, 0x90B4EC, 0xA4C8FC, 0x002C5C,
        0x1C4C78, 0x386890, 0x5084AC, 0x689CC0, 0x7CB4D4, 0x90CCE8, 0xA4E0FC, 0x003C2C, 0x1C5C48,
        0x387C64, 0x509C80, 0x68B494, 0x7CD0AC, 0x90E4C0, 0xA4FCD4, 0x003C00, 0x205C20, 0x407C40,
        0x5C9C5C, 0x74B474, 0x8CD08C, 0xA4E4A4, 0xB8FCB8, 0x143800, 0x345C1C, 0x507C38, 0x6C9850,
        0x84B468, 0x9CCC7C, 0xB4E490, 0xC8FCA4, 0x2C3000, 0x4C501C, 0x687034, 0x848C4C, 0x9CA864,
        0xB4C078, 0xCCD488, 0xE0EC9C, 0x442800, 0x644818, 0x846830, 0xA08444, 0xB89C58, 0xD0B46C,
        0xE8CC7C, 0xFCE08C,
    ])
}

/// Returns an NTSC palette. Source:
/// https://www.randomterrain.com/atari-2600-memories-tutorial-andrew-davie-11.html
pub fn _ntsc_palette_alternative() -> Palette {
    create_tia_palette(&[
        0x000000, 0x1A1A1A, 0x393939, 0x5B5B5B, 0x7E7E7E, 0xA2A2A2, 0xC7C7C7, 0xEDEDED, 0x190200,
        0x3A1F00, 0x5D4100, 0x826400, 0xA78800, 0xCCAD00, 0xF2D219, 0xFEFA40, 0x370000, 0x5E0800,
        0x832700, 0xA94900, 0xCF6C00, 0xF58F17, 0xFEB438, 0xFEDF6F, 0x470000, 0x730000, 0x981300,
        0xBE3216, 0xE45335, 0xFE7657, 0xFE9C81, 0xFEC6BB, 0x440008, 0x6F001F, 0x960640, 0xBB2462,
        0xE14585, 0xFE67AA, 0xFE8CD6, 0xFEB7F6, 0x2D004A, 0x570067, 0x7D058C, 0xA122B1, 0xC743D7,
        0xED65FE, 0xFE8AF6, 0xFEB5F7, 0x0D0082, 0x3300A2, 0x550FC9, 0x782DF0, 0x9C4EFE, 0xC372FE,
        0xEB98FE, 0xFEC0F9, 0x000091, 0x0A05BD, 0x2822E4, 0x4842FE, 0x6B64FE, 0x908AFE, 0xB7B0FE,
        0xDFD8FE, 0x000072, 0x001CAB, 0x033CD6, 0x205EFD, 0x4081FE, 0x64A6FE, 0x89CEFE, 0xB0F6FE,
        0x00103A, 0x00316E, 0x0055A2, 0x0579C8, 0x239DEE, 0x44C2FE, 0x68E9FE, 0x8FFEFE, 0x001F02,
        0x004326, 0x006957, 0x008D7A, 0x1BB19E, 0x3BD7C3, 0x5DFEE9, 0x86FEFE, 0x002403, 0x004A05,
        0x00700C, 0x09952B, 0x28BA4C, 0x49E06E, 0x6CFE92, 0x97FEB5, 0x002102, 0x004604, 0x086B00,
        0x289000, 0x49B509, 0x6BDB28, 0x8FFE49, 0xBBFE69, 0x001501, 0x103600, 0x305900, 0x537E00,
        0x76A300, 0x9AC800, 0xBFEE1E, 0xE8FE3E, 0x1A0200, 0x3B1F00, 0x5E4100, 0x836400, 0xA88800,
        0xCEAD00, 0xF4D218, 0xFEFA40, 0x380000, 0x5F0800, 0x842700, 0xAA4900, 0xD06B00, 0xF68F18,
        0xFEB439, 0xFEDF70,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Pixel;
    use image::Rgba;

    #[test]
    fn creating_palette() {
        assert_eq!(create_tia_palette(&[]), Palette::new());
        assert_eq!(
            create_tia_palette(&[0x123456]),
            vec![
                *Rgba::from_slice(&[0x12, 0x34, 0x56, 0xFF]),
                *Rgba::from_slice(&[0x12, 0x34, 0x56, 0xFF]),
            ]
        );

        let three_color_palette = create_tia_palette(&[0xFEDCBA, 0x5A0345, 0x12A5E4]);
        assert_eq!(
            three_color_palette,
            vec![
                *Rgba::from_slice(&[0xFE, 0xDC, 0xBA, 0xFF]),
                *Rgba::from_slice(&[0xFE, 0xDC, 0xBA, 0xFF]),
                *Rgba::from_slice(&[0x5A, 0x03, 0x45, 0xFF]),
                *Rgba::from_slice(&[0x5A, 0x03, 0x45, 0xFF]),
                *Rgba::from_slice(&[0x12, 0xA5, 0xE4, 0xFF]),
                *Rgba::from_slice(&[0x12, 0xA5, 0xE4, 0xFF]),
            ]
        );
    }
}
