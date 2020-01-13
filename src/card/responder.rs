use super::make_apdu;

type ApduBody = Vec<u8>;
#[derive(Debug)]
pub enum ApduRes {
    /// SW is 9000 or 9100
    Ok(ApduBody),
    /// SW is 6a86
    ParamIncorrect,
    /// SW is 67XX
    WrongLength,
    /// Errors not defined here
    OtherError(u8, u8),
}
impl ApduRes {
    pub fn new(sw1: u8, sw2: u8, data: ApduBody) -> Self {
        if (sw1 == 0x90 || sw1 == 0x91) && sw2 == 0x00 {
            Self::Ok(data)
        }
        if sw1 == 0x6a && sw2 == 0x86 {
            Self::ParamIncorrect
        }
        if sw1 == 0x67 {
            Self::WrongLength
        }
        ApduRes::OtherError(sw1, sw2)
    }
    pub fn from_apdu(apdu: &[u8]) -> Self {
        Self::new(
            apdu[0..apdu.len() - 2],
            apdu[apdu.len() - 2],
            apdu[apdu.len() - 1],
        )
    }
    pub fn unwrap(self) -> &'static [u8] {
        match self {
            Ok(t) => t,
            _ => panic!("Unwrap failed"),
        }
    }
}

type TransFunc = fn(&[u8]) -> ApduRes;
pub struct Responder {
    /// Function that transmit APDU request & make response into ApduRes
    transfunc: TransFunc,
}

type Result<T> = std::result::Result<T, &'static str>;

impl Responder {
    pub fn new(transfunc: TransFunc) -> Self {
        Self { transfunc }
    }
    pub fn transmit(&self, data: &[u8]) -> ApduRes {}
    pub fn select_df(&self, dfid: &[u8]) -> Result<()> {
        match self.transmit(make_apdu(0x00, 0xa4, (0x04, 0x0c), dfid, None)) {
            Ok(_) => Ok(()),
            _ => Err("Failed to SELECT DF"),
        }
    }

    pub fn select_ef(&self, efid: &[u8]) -> Result<()> {
        match (self.transfunc)(make_apdu(0x00, 0xa4, (0x02, 0x0c), efid, None)) {
            Ok(_) => Ok(()),
            _ => Err("Failed to SELECT EF"),
        }
    }

    pub fn select_jpki_ap(&self) -> Result<()> {
        self.select_df(b"\xD3\x92\xf0\x00\x26\x01\x00\x00\x00\x01")
    }
    pub fn select_jpki_token(&self) -> Result<()> {
        self.select_ef(b"\x00\x06")
    }
    pub fn select_jpki_cert_auth(&self) -> Result<()> {
        self.select_ef(b"\x00\x0a")
    }

    pub fn select_jpki_auth_pin(&self) -> Result<()> {
        self.select_ef(b"\x00\x18")
    }
    pub fn select_jpki_auth_key(&self) -> Result<()> {
        self.select_ef(b"\x00\x17")
    }
    pub fn get_challenge(&self, size: u8) -> Result<&[u8]> {
        match (self.transfunc)(&make_apdu(0x00, 0x84, (0, 0), &[], Some(size))) {
            Ok(data) => Ok(data),
            _ => Err("GET CHALLENGE failed"),
        }
    }
    pub fn verify_pin(&self, pin: &str) -> Result<()> {
        match (self.transfunc)(&make_apdu(0x00, 0x20, (0x00, 0x80), &pin.as_bytes(), None)) {
            Ok(_) => Ok(()),
            _ => Err("VERIFY PIN failed"),
        }
    }
    pub fn compute_sig(&self, hash_pkcs1: &[u8]) -> Result<&[u8]> {
        match (self.transfunc)(&make_apdu(0x80, 0x2a, (0x00, 0x80), hash_pkcs1, Some(0))) {
            // zero, the value of Le probably means 256. it overflowed.
            Ok(sig) => Ok(sig),
            _ => Err("COMPUTE DIGITAL SIGNATURE failed"),
        }
    }
    pub fn read_binary(&self) -> Result<()> {
        Ok(())
    }
}
// あとResponderって名前なんとかなりませんか
