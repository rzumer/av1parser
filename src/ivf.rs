//
// https://wiki.multimedia.cx/index.php/IVF
//
use byteorder::{ByteOrder, LittleEndian};
use hex;
use std::io::Read;

pub const IVF_HEADER_SIZE: usize = 32;
pub const IVF_SIGNATURE: [u8; 4] = *b"DKIF";
pub const IVF_VERSION: u16 = 0;

///
/// IVF file header
///
#[derive(Debug)]
pub struct IvfHeader {
    pub codec: [u8; 4], // FourCC
    pub width: u16,     // [pel]
    pub height: u16,    // [pel]
    pub framerate: u32,
    pub timescale: u32,
    pub nframes: u32,
}

///
/// IVF frame
///
#[derive(Debug)]
pub struct IvfFrame {
    pub size: u32, // [byte]
    pub pts: u64,
}

///
/// parse IVF file header
///
pub fn parse_ivf_header(mut ivf: &[u8]) -> Result<IvfHeader, String> {
    assert_eq!(ivf.len(), IVF_HEADER_SIZE);
    // signature (4b)
    let mut sig = [0; 4];
    ivf.read_exact(&mut sig).unwrap();
    if sig != IVF_SIGNATURE {
        return Err(format!(
            "Invalid IVF signature(0x{})",
            hex::encode_upper(sig)
        ));
    }
    // version (2b)
    let mut ver = [0; 2];
    ivf.read_exact(&mut ver).unwrap();
    let ver = LittleEndian::read_u16(&ver);
    if ver != IVF_VERSION {
        return Err(format!("Invalid IVF version({})", ver));
    }
    // header length (2b)
    let mut hdrlen = [0; 2];
    ivf.read_exact(&mut hdrlen).unwrap();
    let hdrlen = LittleEndian::read_u16(&hdrlen);
    if hdrlen != IVF_HEADER_SIZE as u16 {
        return Err(format!("Invalid IVF header length({})", hdrlen));
    }
    // codec (4b)
    let mut codec = [0; 4];
    ivf.read_exact(&mut codec).unwrap();
    // width (2b), height (2b)
    let mut width = [0; 2];
    let mut height = [0; 2];
    ivf.read_exact(&mut width).unwrap();
    ivf.read_exact(&mut height).unwrap();
    let width = LittleEndian::read_u16(&width);
    let height = LittleEndian::read_u16(&height);
    // framerate (4b), timescale (4b)
    let mut framerate = [0; 4];
    let mut timescale = [0; 4];
    ivf.read_exact(&mut framerate).unwrap();
    ivf.read_exact(&mut timescale).unwrap();
    let framerate = LittleEndian::read_u32(&framerate);
    let timescale = LittleEndian::read_u32(&timescale);
    // number of frames (4b)
    let mut nframes = [0; 4];
    ivf.read_exact(&mut nframes).unwrap();
    let nframes = LittleEndian::read_u32(&nframes);

    Ok(IvfHeader {
        codec: codec,
        width: width,
        height: height,
        framerate: framerate,
        timescale: timescale,
        nframes: nframes,
    })
}

///
/// parse IVF frame header
///
pub fn parse_ivf_frame<R: Read>(bs: &mut R) -> Result<IvfFrame, String> {
    let mut hdr = [0; 4 + 8];
    match bs.read_exact(&mut hdr) {
        Ok(_) => (),
        Err(_) => return Err("IO error".to_owned()),
    };

    Ok(IvfFrame {
        size: LittleEndian::read_u32(&hdr[0..4]), // frame size (4b)
        pts: LittleEndian::read_u64(&hdr[4..]),   // presentation timestamp (8b)
    })
}
