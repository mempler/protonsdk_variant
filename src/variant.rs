use byteorder::{WriteBytesExt, LittleEndian};

#[derive(Debug)]
pub enum Variant {
    None(),
    Float(f32),
    Str(String),
    Vec2(f32, f32),
    Vec3(f32, f32, f32),
    UInt(u32),
    Int(i32),
}

impl Variant {
    fn get_id(&self) -> u8 {
        match *self {
            Variant::None()        => 0,
            Variant::Float(_)      => 1,
            Variant::Str(_)        => 2,
            Variant::Vec2(_, _)    => 3,
            Variant::Vec3(_, _, _) => 4,
            Variant::UInt(_)       => 5,
            Variant::Int(_)        => 9,
        }
    }

    pub fn pack<W>(&self, w: &mut W) -> std::io::Result<()>
        where W :
            std::io::Write + std::io::Seek
    {
        w.write_u8(self.get_id())?;

        match self {
            Variant::None() => {},
            Variant::Float(v) => w.write_f32::<LittleEndian>(*v)?,
            Variant::Int(v) => w.write_i32::<LittleEndian>(*v)?,
            Variant::UInt(v) => w.write_u32::<LittleEndian>(*v)?,

            Variant::Vec2(x, y) => {
                w.write_f32::<LittleEndian>(*x)?;
                w.write_f32::<LittleEndian>(*y)?;
            },
            Variant::Vec3(x, y, z) => {
                w.write_f32::<LittleEndian>(*x)?;
                w.write_f32::<LittleEndian>(*y)?;
                w.write_f32::<LittleEndian>(*z)?;
            },

            Variant::Str(v) => {
                w.write_i32::<LittleEndian>(v.len() as i32)?;
                w.write_all(v.as_bytes())?;
            }
        };

        Ok(())
    }
}

impl From<i8> for Variant {
    fn from(v: i8) -> Self {
        Variant::Int(v as i32)
    }
}

impl From<i16> for Variant {
    fn from(v: i16) -> Self {
        Variant::Int(v as i32)
    }
}

impl From<i32> for Variant {
    fn from(v: i32) -> Self {
        Variant::Int(v as i32)
    }
}

impl From<i64> for Variant {
    fn from(v: i64) -> Self {
        Variant::Int(v as i32)
    }
}

impl From<i128> for Variant {
    fn from(v: i128) -> Self {
        Variant::Int(v as i32)
    }
}



impl From<u8> for Variant {
    fn from(v: u8) -> Self {
        Variant::UInt(v as u32)
    }
}

impl From<u16> for Variant {
    fn from(v: u16) -> Self {
        Variant::UInt(v as u32)
    }
}

impl From<u32> for Variant {
    fn from(v: u32) -> Self {
        Variant::UInt(v as u32)
    }
}

impl From<u64> for Variant {
    fn from(v: u64) -> Self {
        Variant::UInt(v as u32)
    }
}

impl From<u128> for Variant {
    fn from(v: u128) -> Self {
        Variant::UInt(v as u32)
    }
}



impl From<f32> for Variant {
    fn from(v: f32) -> Self {
        Variant::Float(v as f32)
    }
}

impl From<f64> for Variant {
    fn from(v: f64) -> Self {
        Variant::Float(v as f32)
    }
}



impl From<String> for Variant {
    fn from(v: String) -> Self {
        Variant::Str(v)
    }
}

impl From<&str> for Variant {
    fn from(v: &str) -> Self {
        Variant::Str(v.to_string())
    }
}


impl From<&[f32; 2]> for Variant {
    fn from(v: &[f32; 2]) -> Self {
        Variant::Vec2(v[0], v[1])
    }
}

impl From<&[f32; 3]> for Variant {
    fn from(v: &[f32; 3]) -> Self {
        Variant::Vec3(v[0], v[1], v[2])
    }
}

