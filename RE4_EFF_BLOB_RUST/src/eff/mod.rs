use scalar_types::Endian;
use std::io::{BufRead, BufReader, Cursor, Error, ErrorKind, Read, Result, Seek, SeekFrom, Write};
use std::path::Path;

pub struct TableEntry {
    pub id: u32,
    pub _unknown: u32,
}

impl TableEntry {
    pub fn new<StreamT: Read + Seek>(
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Option<TableEntry> {
        let id = Endian::from_stream(stream)?.cast(endianness)?;
        let unknown = Endian::from_stream(stream)?.cast(endianness)?;

        Some(TableEntry {
            id,
            _unknown: unknown,
        })
    }

    pub fn write<StreamT: Write>(
        &self,
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Result<()> {
        let id = Endian::<u32>::new(self.id)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let unknown = Endian::<u32>::new(self._unknown)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;

        stream.write_all(&id.to_ne_bytes())?;
        stream.write_all(&unknown.to_ne_bytes())?;

        Ok(())
    }
}

pub struct EarLink {
    pub id: u16,
    pub ear_link_id: u16,
    pub _unknown: u32,
}

impl EarLink {
    pub fn new<StreamT: Read + Seek>(
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Option<EarLink> {
        let id = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let ear_link_id = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let unknown = Endian::<u32>::from_stream(stream)?.cast(endianness)?;

        Some(EarLink {
            id,
            ear_link_id,
            _unknown: unknown,
        })
    }

    pub fn write<StreamT: Write>(
        &self,
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Result<()> {
        let id = Endian::new(self.id)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let ear_link_id = Endian::new(self.ear_link_id)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let unknown = Endian::new(self._unknown)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;

        stream.write_all(&id.to_ne_bytes())?;
        stream.write_all(&ear_link_id.to_ne_bytes())?;
        stream.write_all(&unknown.to_ne_bytes())?;

        Ok(())
    }
}

pub struct TextureMetadata {
    pub texture_height: u16,
    pub texture_width: u16,
    pub effect_height: u16,
    pub effect_width: u16,
    pub texture_count: u16,
    pub unknown_1: u8,
    pub unknown_2: u8,
}

impl TextureMetadata {
    pub fn new<StreamT: Read + Seek>(
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Option<TextureMetadata> {
        let texture_height = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let texture_width = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let effect_height = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let effect_width = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let texture_count = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let unknown_1 = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let unknown_2 = Endian::<u8>::from_stream(stream)?.cast(endianness)?;

        Some(TextureMetadata {
            texture_height,
            texture_width,
            effect_height,
            effect_width,
            texture_count,
            unknown_1,
            unknown_2,
        })
    }

    pub fn write<StreamT: Write>(
        &self,
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Result<()> {
        let texture_height = Endian::new(self.texture_height)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let texture_width = Endian::new(self.texture_width)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let effect_height = Endian::new(self.effect_height)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let effect_width = Endian::new(self.effect_width)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let texture_count = Endian::new(self.texture_count)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let unknown_1 = Endian::new(self.unknown_1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let unknown_2 = Endian::new(self.unknown_2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;

        stream.write_all(&texture_height.to_ne_bytes())?;
        stream.write_all(&texture_width.to_ne_bytes())?;
        stream.write_all(&effect_height.to_ne_bytes())?;
        stream.write_all(&effect_width.to_ne_bytes())?;
        stream.write_all(&texture_count.to_ne_bytes())?;
        stream.write_all(&unknown_1.to_ne_bytes())?;
        stream.write_all(&unknown_2.to_ne_bytes())?;

        Ok(())
    }
}

pub struct EffectGroup {
    pub _unknownX02: u16,
    pub _unknownX04: u16,
    pub _unknownX06: u16,
    pub _unknownX08: u16,
    pub _unknownX0A: u8,
    pub _unknownX0B: u8,
    pub _unknownX0C: f32,
    pub _unknownX10: f32,
    pub _unknownX14: f32,
    pub _unknownX18: f32,
    pub _unknownX1C: f32,
    pub _unknownX20: f32,
    pub _unknownX24: u8,
    pub effects: Vec<Effect>,
}

impl EffectGroup {
    fn load_effects<StreamT: Read + Seek + Seek>(
        stream: &mut StreamT,
        count: &u16,
        endianness: &Endian<()>,
    ) -> Option<Vec<Effect>> {
        let mut result = Vec::<Effect>::new();
        result.reserve(*count as usize);

        // Read Data
        for _ in 0..*count as usize {
            result.push(Effect::new(stream, endianness)?);
        }

        Some(result)
    }

    pub fn new<StreamT: Read + Seek>(
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Option<EffectGroup> {
        let effect_count = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let _unknownX02 = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let _unknownX04 = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let _unknownX06 = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let _unknownX08 = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let _unknownX0A = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let _unknownX0B = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let _unknownX0C = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let _unknownX10 = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let _unknownX14 = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let _unknownX18 = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let _unknownX1C = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let _unknownX20 = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let _unknownX24 = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        stream.seek(SeekFrom::Current(0xb)).ok()?;
        let effects = EffectGroup::load_effects(stream, &effect_count, endianness)?;

        Some(EffectGroup {
            _unknownX02,
            _unknownX04,
            _unknownX06,
            _unknownX08,
            _unknownX0A,
            _unknownX0B,
            _unknownX0C,
            _unknownX10,
            _unknownX14,
            _unknownX18,
            _unknownX1C,
            _unknownX20,
            _unknownX24,
            effects,
        })
    }

    pub fn write<StreamT: Write + Seek>(
        &self,
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Result<()> {
        let effect_count = Endian::new(self.effects.len() as u16)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknownX02 = Endian::new(self._unknownX02)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknownX04 = Endian::new(self._unknownX04)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknownX06 = Endian::new(self._unknownX06)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknownX08 = Endian::new(self._unknownX08)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown3 = Endian::new(self._unknownX0A)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown4 = Endian::new(self._unknownX0B)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown5 = Endian::new(self._unknownX0C)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown6 = Endian::new(self._unknownX10)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown7 = Endian::new(self._unknownX14)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown8 = Endian::new(self._unknownX18)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown9 = Endian::new(self._unknownX1C)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown10 = Endian::new(self._unknownX20)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown11 = Endian::new(self._unknownX24)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;

        stream.write_all(&effect_count.to_ne_bytes())?;
        stream.write_all(&_unknownX02.to_ne_bytes())?;
        stream.write_all(&_unknownX04.to_ne_bytes())?;
        stream.write_all(&_unknownX06.to_ne_bytes())?;
        stream.write_all(&_unknownX08.to_ne_bytes())?;
        stream.write_all(&_unknown3.to_ne_bytes())?;
        stream.write_all(&_unknown4.to_ne_bytes())?;
        stream.write_all(&_unknown5.to_ne_bytes())?;
        stream.write_all(&_unknown6.to_ne_bytes())?;
        stream.write_all(&_unknown7.to_ne_bytes())?;
        stream.write_all(&_unknown8.to_ne_bytes())?;
        stream.write_all(&_unknown9.to_ne_bytes())?;
        stream.write_all(&_unknown10.to_ne_bytes())?;
        stream.write_all(&_unknown11.to_ne_bytes())?;
        stream.seek(SeekFrom::Current(0xb))?;

        for effect in &self.effects {
            effect.write(stream, endianness)?;
        }

        Ok(())
    }
}

pub struct Effect {
    pub state_id: u8,
    pub esp_id: u8,
    pub texture_id: u8,
    pub _unknown_x03: u8,
    pub time: u16,
    pub parent: u8,
    pub part: u8,
    pub flags: u32,
    pub position: (f32, f32, f32),
    pub random: (f32, f32, f32),
    pub speed: (f32, f32, f32),
    pub delta_speed: f32,
    pub random_speed: (f32, f32, f32),
    pub acceleration: (f32, f32, f32),
    pub random_acceleration: (f32, f32, f32),
    pub rotate: (f32, f32, f32),
    pub random_rotate: (f32, f32, f32),
    pub rotate_acceleration: (f32, f32, f32),
    pub random_rotate_acceleration: (f32, f32, f32),
    pub width: f32,
    pub height: f32,
    pub random_size: f32,
    pub grow: f32,
    pub delta_grow: f32,
    pub rgba: (u8, u8, u8, u8),
    pub delta_color: (f32, f32, f32, f32),
    pub delta_color_attack: u16, // Come back to this
    pub delta_color_start_frame: u16,
    pub _unknown_xb4: u16,
    pub delta_size_start_frame: u16,
    pub life_time: u16,       // padding?
    pub animation_speed: u32, // clammed 0xff
    pub _unknown_xbe: u16,       // padding?
    pub release_time: u8,
    pub blend: u16,          // clamped 0xff
    pub simulation_type: u8, // enum {None, Normal, Offset, Replace}
    pub simulation_power: u8,
    pub mask_texture_id: u8,
    pub value_in: u8,  // Research needed
    pub value_out: u8, // Research needed
    pub work_0: u8,    // Research needed
    pub work_1: u8,    // Research needed
    pub work_2: u8,    // Research needed
    pub work_3: u8,    // Research needed
    pub work_4: u32,   // Research needed
    pub work_5: u32,   // Research needed
    pub work_6: u32,   // Research needed
    pub vector_0: (f32, f32, f32),
    pub vector_1: (f32, f32, f32),
    pub vector_2: (f32, f32, f32),
    pub spline_0: u8,
    pub spline_1: u8,
    pub spline_2: u8,
    pub spline_3: u8,
    pub _unknown_x100: u32, // padding?
    pub path_own: u8,
    pub path_number: u8,
    pub path_start: u8,
    pub path_random: u8,
    pub eff_type: u8,      // enum {ESP, Control}
    pub control_id: u8,    // Only when type is Control
    pub control_flag: u16, // Reserach needed
    pub control_interval: u8,
    pub control_number: u8,
    pub control_rp: u8,
    pub _unknown_x10f: u8, // padding?
    pub control_life: u16,
    pub _unknown_x112: u16, // padding?
    pub _unknown_x114: u16, // padding?
    pub _unknown_x116: u16, // padding?
    pub control_path_scale: (f32, f32, f32),
    pub control_path_delta_size: u8,
    pub control_path_delta_speed: u8,
    pub control_path_delta_alpha: u8,
    pub control_path_delta_interval: u8,
    pub control_path_random_interval: u8,
    pub control_path_rotation: (u8, u8),
    pub control_path_flag: u8,
}

impl Effect {
    pub fn new<StreamT: Read + Seek>(
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Option<Effect> {
        let state_id = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let esp_id = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let texture_id = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let _unknown_x03 = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let time = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let parent = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let part = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let flags = Endian::<u32>::from_stream(stream)?.cast(endianness)?;
        let position_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let position_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let position_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let speed_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let speed_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let speed_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let delta_speed = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_speed_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_speed_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_speed_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let acceleration_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let acceleration_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let acceleration_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_acceleration_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_acceleration_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_acceleration_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let rotation_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let rotation_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let rotation_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_rotation_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_rotation_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_rotation_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let rotation_acceleration_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let rotation_acceleration_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let rotation_acceleration_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_rotation_acceleration_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_rotation_acceleration_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_rotation_acceleration_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let width = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let height = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let random_size = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let grow = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let delta_grow = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let r = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let g = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let b = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let a = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let delta_r = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let delta_g = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let delta_b = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let delta_a = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let delta_color_attack = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let delta_color_start_frame = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let _unknown_xb4 = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let delta_size_start_frame = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let life_time = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let animation_speed = Endian::<u32>::from_stream(stream)?.cast(endianness)?;
        let _unknown_xbe = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let release_time = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let blend = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let simulation_type = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let simulation_power = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let mask_texture_id = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let value_in = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let value_out = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let work_0 = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let work_1 = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let work_2 = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let work_3 = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let work_4 = Endian::<u32>::from_stream(stream)?.cast(endianness)?;
        let work_5 = Endian::<u32>::from_stream(stream)?.cast(endianness)?;
        let work_6 = Endian::<u32>::from_stream(stream)?.cast(endianness)?;
        let vector_0_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let vector_0_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let vector_0_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let vector_1_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let vector_1_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let vector_1_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let vector_2_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let vector_2_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let vector_2_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let spline_0 = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let spline_1 = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let spline_2 = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let spline_3 = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let _unknown_x100 = Endian::<u32>::from_stream(stream)?.cast(endianness)?;
        let path_own = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let path_number = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let path_start = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let path_random = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let eff_type = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let control_id = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let control_flag = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let control_interval = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let control_number = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let control_rp = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let _unknown_x10f = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let control_life = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let _unknown_x112 = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let _unknown_x114 = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let _unknown_x116 = Endian::<u16>::from_stream(stream)?.cast(endianness)?;
        let control_path_scale_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let control_path_scale_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let control_path_scale_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
        let control_path_delta_size = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let control_path_delta_speed = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let control_path_delta_alpha = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let control_path_delta_interval = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let control_path_random_interval = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let control_path_rotation_x = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let control_path_rotation_y = Endian::<u8>::from_stream(stream)?.cast(endianness)?;
        let control_path_flag = Endian::<u8>::from_stream(stream)?.cast(endianness)?;

        Some(Effect {
            state_id,
            esp_id,
            texture_id,
            _unknown_x03,
            time,
            parent,
            part,
            flags,
            position: (position_x, position_y, position_z),
            random: (random_x, random_y, random_z),
            speed: (speed_x, speed_y, speed_z),
            delta_speed,
            random_speed: (random_speed_x, random_speed_y, random_speed_z),
            acceleration: (acceleration_x, acceleration_y, acceleration_z),
            random_acceleration: (
                random_acceleration_x,
                random_acceleration_y,
                random_acceleration_z,
            ),
            rotate: (rotation_x, rotation_y, rotation_z),
            random_rotate: (random_rotation_x, random_rotation_y, random_rotation_z),
            rotate_acceleration: (
                rotation_acceleration_x,
                rotation_acceleration_y,
                rotation_acceleration_z,
            ),
            random_rotate_acceleration: (
                random_rotation_acceleration_x,
                random_rotation_acceleration_y,
                random_rotation_acceleration_z,
            ),
            width,
            height,
            random_size,
            grow,
            delta_grow,
            rgba: (r, g, b, a),
            delta_color: (delta_r, delta_g, delta_b, delta_a),
            delta_color_attack,
            delta_color_start_frame,
            _unknown_xb4,
            delta_size_start_frame,
            life_time,
            animation_speed,
            _unknown_xbe,
            release_time,
            blend,
            simulation_type,
            simulation_power,
            mask_texture_id,
            value_in,
            value_out,
            work_0,
            work_1,
            work_2,
            work_3,
            work_4,
            work_5,
            work_6,
            vector_0: (vector_0_x, vector_0_y, vector_0_z),
            vector_1: (vector_1_x, vector_1_y, vector_1_z),
            vector_2: (vector_2_x, vector_2_y, vector_2_z),
            spline_0,
            spline_1,
            spline_2,
            spline_3,
            _unknown_x100,
            path_own,
            path_number,
            path_start,
            path_random,
            eff_type,
            control_id,
            control_flag,
            control_interval,
            control_number,
            control_rp,
            _unknown_x10f,
            control_life,
            _unknown_x112,
            _unknown_x114,
            _unknown_x116,
            control_path_scale: (
                control_path_scale_x,
                control_path_scale_y,
                control_path_scale_z,
            ),
            control_path_delta_size,
            control_path_delta_speed,
            control_path_delta_alpha,
            control_path_delta_interval,
            control_path_random_interval,
            control_path_rotation: (control_path_rotation_x, control_path_rotation_y),
            control_path_flag,
        })
    }

    pub fn write<StreamT: Write>(
        &self,
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Result<()> {
        let state_id = Endian::new(self.state_id)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let esp_id = Endian::new(self.esp_id)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let texture_id = Endian::new(self.texture_id)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown_x03 = Endian::new(self._unknown_x03)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let time = Endian::new(self.time)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let parent = Endian::new(self.parent)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let part = Endian::new(self.part)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let flags = Endian::new(self.flags)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let position_x = Endian::new(self.position.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let position_y = Endian::new(self.position.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let position_z = Endian::new(self.position.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_x = Endian::new(self.random.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_y = Endian::new(self.random.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_z = Endian::new(self.random.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let speed_x = Endian::new(self.speed.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let speed_y = Endian::new(self.speed.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let speed_z = Endian::new(self.speed.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let delta_speed = Endian::new(self.delta_speed)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_speed_x = Endian::new(self.random_speed.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_speed_y = Endian::new(self.random_speed.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_speed_z = Endian::new(self.random_speed.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let acceleration_x = Endian::new(self.acceleration.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let acceleration_y = Endian::new(self.acceleration.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let acceleration_z = Endian::new(self.acceleration.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_acceleration_x = Endian::new(self.random_acceleration.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_acceleration_y = Endian::new(self.random_acceleration.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_acceleration_z = Endian::new(self.random_acceleration.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let rotation_x = Endian::new(self.rotate.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let rotation_y = Endian::new(self.rotate.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let rotation_z = Endian::new(self.rotate.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_rotation_x = Endian::new(self.random_rotate.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_rotation_y = Endian::new(self.random_rotate.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_rotation_z = Endian::new(self.random_rotate.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let rotation_acceleration_x = Endian::new(self.rotate_acceleration.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let rotation_acceleration_y = Endian::new(self.rotate_acceleration.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let rotation_acceleration_z = Endian::new(self.rotate_acceleration.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_rotation_acceleration_x = Endian::new(self.random_rotate_acceleration.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_rotation_acceleration_y = Endian::new(self.random_rotate_acceleration.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_rotation_acceleration_z = Endian::new(self.random_rotate_acceleration.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let width = Endian::new(self.width)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let height = Endian::new(self.height)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let random_size = Endian::new(self.random_size)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let grow = Endian::new(self.grow)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let delta_grow = Endian::new(self.delta_grow)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let r = Endian::new(self.rgba.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let g = Endian::new(self.rgba.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let b = Endian::new(self.rgba.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let a = Endian::new(self.rgba.3)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let delta_r = Endian::new(self.delta_color.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let delta_g = Endian::new(self.delta_color.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let delta_b = Endian::new(self.delta_color.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let delta_a = Endian::new(self.delta_color.3)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let delta_color_attack = Endian::new(self.delta_color_attack)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let delta_color_start_frame = Endian::new(self.delta_color_start_frame)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown_xb4 = Endian::new(self._unknown_xb4)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let delta_size_start_frame = Endian::new(self.delta_size_start_frame)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let life_time = Endian::new(self.life_time)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let animation_speed = Endian::new(self.animation_speed)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown_xbe = Endian::new(self._unknown_xbe)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let release_time = Endian::new(self.release_time)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let blend = Endian::new(self.blend)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let simulation_type = Endian::new(self.simulation_type)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let simulation_power = Endian::new(self.simulation_power)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let mask_texture_id = Endian::new(self.mask_texture_id)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let value_in = Endian::new(self.value_in)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let value_out = Endian::new(self.value_out)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let work_0 = Endian::new(self.work_0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let work_1 = Endian::new(self.work_1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let work_2 = Endian::new(self.work_2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let work_3 = Endian::new(self.work_3)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let work_4 = Endian::new(self.work_4)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let work_5 = Endian::new(self.work_5)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let work_6 = Endian::new(self.work_6)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let vector_0_x = Endian::new(self.vector_0.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let vector_0_y = Endian::new(self.vector_0.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let vector_0_z = Endian::new(self.vector_0.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let vector_1_x = Endian::new(self.vector_1.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let vector_1_y = Endian::new(self.vector_1.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let vector_1_z = Endian::new(self.vector_1.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let vector_2_x = Endian::new(self.vector_2.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let vector_2_y = Endian::new(self.vector_2.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let vector_2_z = Endian::new(self.vector_2.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let spline_0 = Endian::new(self.spline_0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let spline_1 = Endian::new(self.spline_1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let spline_2 = Endian::new(self.spline_2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let spline_3 = Endian::new(self.spline_3)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown_x100 = Endian::new(self._unknown_x100)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let path_own = Endian::new(self.path_own)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let path_number = Endian::new(self.path_number)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let path_start = Endian::new(self.path_start)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let path_random = Endian::new(self.path_random)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let eff_type = Endian::new(self.eff_type)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_id = Endian::new(self.control_id)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_flag = Endian::new(self.control_flag)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_interval = Endian::new(self.control_interval)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_number = Endian::new(self.control_number)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_rp = Endian::new(self.control_rp)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown_x10f = Endian::new(self._unknown_x10f)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_life = Endian::new(self.control_life)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown_x112 = Endian::new(self._unknown_x112)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown_x114 = Endian::new(self._unknown_x114)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let _unknown_x116 = Endian::new(self._unknown_x116)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_path_scale_x = Endian::new(self.control_path_scale.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_path_scale_y = Endian::new(self.control_path_scale.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_path_scale_z = Endian::new(self.control_path_scale.2)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_path_delta_size = Endian::new(self.control_path_delta_size)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_path_delta_speed = Endian::new(self.control_path_delta_speed)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_path_delta_alpha = Endian::new(self.control_path_delta_alpha)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_path_delta_interval = Endian::new(self.control_path_delta_interval)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_path_random_interval = Endian::new(self.control_path_random_interval)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_path_rotation_x = Endian::new(self.control_path_rotation.0)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_path_rotation_y = Endian::new(self.control_path_rotation.1)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        let control_path_flag = Endian::new(self.control_path_flag)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;

        stream.write_all(&state_id.to_ne_bytes())?;
        stream.write_all(&esp_id.to_ne_bytes())?;
        stream.write_all(&texture_id.to_ne_bytes())?;
        stream.write_all(&_unknown_x03.to_ne_bytes())?; // padding
        stream.write_all(&time.to_ne_bytes())?;
        stream.write_all(&parent.to_ne_bytes())?;
        stream.write_all(&part.to_ne_bytes())?;
        stream.write_all(&flags.to_ne_bytes())?;
        stream.write_all(&position_x.to_ne_bytes())?;
        stream.write_all(&position_y.to_ne_bytes())?;
        stream.write_all(&position_z.to_ne_bytes())?;
        stream.write_all(&random_x.to_ne_bytes())?;
        stream.write_all(&random_y.to_ne_bytes())?;
        stream.write_all(&random_z.to_ne_bytes())?;
        stream.write_all(&speed_x.to_ne_bytes())?;
        stream.write_all(&speed_y.to_ne_bytes())?;
        stream.write_all(&speed_z.to_ne_bytes())?;
        stream.write_all(&delta_speed.to_ne_bytes())?;
        stream.write_all(&random_speed_x.to_ne_bytes())?;
        stream.write_all(&random_speed_y.to_ne_bytes())?;
        stream.write_all(&random_speed_z.to_ne_bytes())?;
        stream.write_all(&acceleration_x.to_ne_bytes())?;
        stream.write_all(&acceleration_y.to_ne_bytes())?;
        stream.write_all(&acceleration_z.to_ne_bytes())?;
        stream.write_all(&random_acceleration_x.to_ne_bytes())?;
        stream.write_all(&random_acceleration_y.to_ne_bytes())?;
        stream.write_all(&random_acceleration_z.to_ne_bytes())?;
        stream.write_all(&rotation_x.to_ne_bytes())?;
        stream.write_all(&rotation_y.to_ne_bytes())?;
        stream.write_all(&rotation_z.to_ne_bytes())?;
        stream.write_all(&random_rotation_x.to_ne_bytes())?;
        stream.write_all(&random_rotation_y.to_ne_bytes())?;
        stream.write_all(&random_rotation_z.to_ne_bytes())?;
        stream.write_all(&rotation_acceleration_x.to_ne_bytes())?;
        stream.write_all(&rotation_acceleration_y.to_ne_bytes())?;
        stream.write_all(&rotation_acceleration_z.to_ne_bytes())?;
        stream.write_all(&random_rotation_acceleration_x.to_ne_bytes())?;
        stream.write_all(&random_rotation_acceleration_y.to_ne_bytes())?;
        stream.write_all(&random_rotation_acceleration_z.to_ne_bytes())?;
        stream.write_all(&width.to_ne_bytes())?;
        stream.write_all(&height.to_ne_bytes())?;
        stream.write_all(&random_size.to_ne_bytes())?;
        stream.write_all(&grow.to_ne_bytes())?;
        stream.write_all(&delta_grow.to_ne_bytes())?;
        stream.write_all(&r.to_ne_bytes())?;
        stream.write_all(&g.to_ne_bytes())?;
        stream.write_all(&b.to_ne_bytes())?;
        stream.write_all(&a.to_ne_bytes())?;
        stream.write_all(&delta_r.to_ne_bytes())?;
        stream.write_all(&delta_g.to_ne_bytes())?;
        stream.write_all(&delta_b.to_ne_bytes())?;
        stream.write_all(&delta_a.to_ne_bytes())?;
        stream.write_all(&delta_color_attack.to_ne_bytes())?;
        stream.write_all(&delta_color_start_frame.to_ne_bytes())?;
        stream.write_all(&_unknown_xb4.to_ne_bytes())?;
        stream.write_all(&delta_size_start_frame.to_ne_bytes())?;
        stream.write_all(&life_time.to_ne_bytes())?;
        stream.write_all(&animation_speed.to_ne_bytes())?;
        stream.write_all(&_unknown_xbe.to_ne_bytes())?;
        stream.write_all(&release_time.to_ne_bytes())?;
        stream.write_all(&blend.to_ne_bytes())?;
        stream.write_all(&simulation_type.to_ne_bytes())?;
        stream.write_all(&simulation_power.to_ne_bytes())?;
        stream.write_all(&mask_texture_id.to_ne_bytes())?;
        stream.write_all(&value_in.to_ne_bytes())?;
        stream.write_all(&value_out.to_ne_bytes())?;
        stream.write_all(&work_0.to_ne_bytes())?;
        stream.write_all(&work_1.to_ne_bytes())?;
        stream.write_all(&work_2.to_ne_bytes())?;
        stream.write_all(&work_3.to_ne_bytes())?;
        stream.write_all(&work_4.to_ne_bytes())?;
        stream.write_all(&work_5.to_ne_bytes())?;
        stream.write_all(&work_6.to_ne_bytes())?;
        stream.write_all(&vector_0_x.to_ne_bytes())?;
        stream.write_all(&vector_0_y.to_ne_bytes())?;
        stream.write_all(&vector_0_z.to_ne_bytes())?;
        stream.write_all(&vector_1_x.to_ne_bytes())?;
        stream.write_all(&vector_1_y.to_ne_bytes())?;
        stream.write_all(&vector_1_z.to_ne_bytes())?;
        stream.write_all(&vector_2_x.to_ne_bytes())?;
        stream.write_all(&vector_2_y.to_ne_bytes())?;
        stream.write_all(&vector_2_z.to_ne_bytes())?;
        stream.write_all(&spline_0.to_ne_bytes())?;
        stream.write_all(&spline_1.to_ne_bytes())?;
        stream.write_all(&spline_2.to_ne_bytes())?;
        stream.write_all(&spline_3.to_ne_bytes())?;
        stream.write_all(&_unknown_x100.to_ne_bytes())?;
        stream.write_all(&path_own.to_ne_bytes())?;
        stream.write_all(&path_number.to_ne_bytes())?;
        stream.write_all(&path_start.to_ne_bytes())?;
        stream.write_all(&path_random.to_ne_bytes())?;
        stream.write_all(&eff_type.to_ne_bytes())?;
        stream.write_all(&control_id.to_ne_bytes())?;
        stream.write_all(&control_flag.to_ne_bytes())?;
        stream.write_all(&control_interval.to_ne_bytes())?;
        stream.write_all(&control_number.to_ne_bytes())?;
        stream.write_all(&control_rp.to_ne_bytes())?;
        stream.write_all(&_unknown_x10f.to_ne_bytes())?;
        stream.write_all(&control_life.to_ne_bytes())?;
        stream.write_all(&_unknown_x112.to_ne_bytes())?;
        stream.write_all(&_unknown_x114.to_ne_bytes())?;
        stream.write_all(&_unknown_x116.to_ne_bytes())?;
        stream.write_all(&control_path_scale_x.to_ne_bytes())?;
        stream.write_all(&control_path_scale_y.to_ne_bytes())?;
        stream.write_all(&control_path_scale_z.to_ne_bytes())?;
        stream.write_all(&control_path_delta_size.to_ne_bytes())?;
        stream.write_all(&control_path_delta_speed.to_ne_bytes())?;
        stream.write_all(&control_path_delta_alpha.to_ne_bytes())?;
        stream.write_all(&control_path_delta_interval.to_ne_bytes())?;
        stream.write_all(&control_path_random_interval.to_ne_bytes())?;
        stream.write_all(&control_path_rotation_x.to_ne_bytes())?;
        stream.write_all(&control_path_rotation_y.to_ne_bytes())?;
        stream.write_all(&control_path_flag.to_ne_bytes())?;

        Ok(())
    }
}

pub struct CurvePoint {
    pub point: (f32, f32, f32),
    pub handle_0: (f32, f32, f32),
    pub handle_1: (f32, f32, f32),
    pub unknown: f32,
}

pub struct Curve {
    pub points: Vec<CurvePoint>,
}

impl Curve {
    pub fn new<StreamT: Read + Seek>(
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Option<Curve> {
        let count = Endian::<u32>::from_stream(stream)?.cast(endianness)?;
        let mut points = Vec::<CurvePoint>::new();

        for _ in 0..count {
            let position_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
            let position_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
            let position_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
            let handle_0_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
            let handle_0_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
            let handle_0_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
            let handle_1_x = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
            let handle_1_y = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
            let handle_1_z = Endian::<f32>::from_stream(stream)?.cast(endianness)?;
            let unknown = Endian::<f32>::from_stream(stream)?.cast(endianness)?;

            points.push(CurvePoint {
                point: (position_x, position_y, position_z),
                handle_0: (handle_0_x, handle_0_y, handle_0_z),
                handle_1: (handle_1_x, handle_1_y, handle_1_z),
                unknown,
            });
        }

        Some(Curve { points })
    }

    pub fn write<StreamT: Write>(
        &self,
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Result<()> {
        let count = Endian::new(self.points.len() as u32)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        stream.write_all(&count.to_ne_bytes())?;

        for point in &self.points {
            let position_x = Endian::new(point.point.0)
                .cast(endianness)
                .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
            let position_y = Endian::new(point.point.1)
                .cast(endianness)
                .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
            let position_z = Endian::new(point.point.2)
                .cast(endianness)
                .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
            let handle_0_x = Endian::new(point.handle_0.0)
                .cast(endianness)
                .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
            let handle_0_y = Endian::new(point.handle_0.1)
                .cast(endianness)
                .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
            let handle_0_z = Endian::new(point.handle_0.2)
                .cast(endianness)
                .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
            let handle_1_x = Endian::new(point.handle_1.0)
                .cast(endianness)
                .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
            let handle_1_y = Endian::new(point.handle_1.1)
                .cast(endianness)
                .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
            let handle_1_z = Endian::new(point.handle_1.2)
                .cast(endianness)
                .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
            let unknown = Endian::new(point.unknown)
                .cast(endianness)
                .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;

            stream.write_all(&position_x.to_ne_bytes())?;
            stream.write_all(&position_y.to_ne_bytes())?;
            stream.write_all(&position_z.to_ne_bytes())?;
            stream.write_all(&handle_0_x.to_ne_bytes())?;
            stream.write_all(&handle_0_y.to_ne_bytes())?;
            stream.write_all(&handle_0_z.to_ne_bytes())?;
            stream.write_all(&handle_1_x.to_ne_bytes())?;
            stream.write_all(&handle_1_y.to_ne_bytes())?;
            stream.write_all(&handle_1_z.to_ne_bytes())?;
            stream.write_all(&unknown.to_ne_bytes())?;
        }

        Ok(())
    }
}


pub struct Eff {
    pub texture_ids: Vec<TableEntry>,
    pub core_ids: Vec<TableEntry>,
    pub ear_links: Vec<EarLink>,
    pub unknown_table: Vec<TableEntry>,
    pub model_ids: Vec<TableEntry>,
    pub tpls_metadata: Vec<TextureMetadata>,
    pub effects_0: Vec<EffectGroup>,
    pub effects_1: Vec<EffectGroup>,
    pub paths: Vec<Curve>,
}

impl Eff {

    fn load_table<StreamT: Read + Seek>(
        stream: &mut StreamT,
        offset: &u32,
        endianness: &Endian<()>,
    ) -> Option<Vec<TableEntry>> {
        let mut result = Vec::<TableEntry>::new();

        // Parts of file can be empty, if they are the offset is 0
        if *offset == 0 {
            return Some(result);
        }

        // Seek to offset
        stream.seek(SeekFrom::Start(*offset as u64)).ok()?;

        // Read element count
        let id_count = Endian::<u32>::from_stream(stream)?.cast(endianness)?;

        // Allocate memory
        result.reserve(id_count as usize);

        // Read
        for _ in 0..id_count as usize {
            result.push(TableEntry::new(stream, endianness)?);
        }

        Some(result)
    }

    fn load_ear_links<StreamT: Read + Seek>(
        stream: &mut StreamT,
        offset: &u32,
        endianness: &Endian<()>,
    ) -> Option<Vec<EarLink>> {
        let mut result = Vec::<EarLink>::new();

        // Parts of file can be empty, if they are the offset is 0
        if *offset == 0 {
            return Some(result);
        }

        // Seek to offset
        stream.seek(SeekFrom::Start(*offset as u64)).ok()?;

        // Read element count
        let id_count = Endian::<u32>::from_stream(stream)?.cast(endianness)?;

        // Allocate memory
        result.reserve(id_count as usize);

        // Read
        for _ in 0..id_count as usize {
            result.push(EarLink::new(stream, endianness)?);
        }

        Some(result)
    }

    fn load_tpls_metadata<StreamT: Read + Seek>(
        stream: &mut StreamT,
        offset: &u32,
        endianness: &Endian<()>,
    ) -> Option<Vec<TextureMetadata>> {
        let mut result = Vec::<TextureMetadata>::new();

        // Parts of file can be empty, if they are the offset is 0
        if *offset == 0 {
            return Some(result);
        }

        // Seek to offset
        stream.seek(SeekFrom::Start(*offset as u64)).ok()?;
        let offsets = Eff::load_offsets(stream, endianness)?;

        // Read element count
        let id_count = Endian::<u32>::from_stream(stream)?.cast(endianness)?;

        // Allocate memory
        result.reserve(id_count as usize);

        // Read Data
        for block_offset in offsets {
            stream.seek(SeekFrom::Start((*offset + block_offset) as u64 )).ok()?;
            result.push(TextureMetadata::new(stream, endianness)?);
        }

        Some(result)
    }

    fn load_paths<StreamT: Read + Seek>(
        stream: &mut StreamT,
        offset: &u32,
        endianness: &Endian<()>,
    ) -> Option<Vec<Curve>> {
        let mut result = Vec::<Curve>::new();

        // Parts of file can be empty, if they are the offset is 0
        if *offset == 0 {
            return Some(result);
        }

        // Seek to offset
        stream.seek(SeekFrom::Start(*offset as u64)).ok()?;
        let offsets = Eff::load_offsets(stream, endianness)?;

        // Read element count
        let id_count = Endian::<u32>::from_stream(stream)?.cast(endianness)?;

        // Allocate memory
        result.reserve(id_count as usize);

        // Read Data
        for block_offset in offsets {
            stream
                .seek(SeekFrom::Start((*offset + block_offset) as u64))
                .ok()?;
            result.push(Curve::new(stream, endianness)?);
        }

        Some(result)
    }

    fn load_effects<StreamT: Read + Seek>(
        stream: &mut StreamT,
        offset: &u32,
        endianness: &Endian<()>,
    ) -> Option<Vec<EffectGroup>> {
        let mut result = Vec::<EffectGroup>::new();

        // Parts of file can be empty, if they are the offset is 0
        if *offset == 0 {
            return Some(result);
        }

        // Seek to offset
        stream.seek(SeekFrom::Start(*offset as u64)).ok()?;
        let offsets = Eff::load_offsets(stream, endianness)?;

        // Allocate memory
        result.reserve(offsets.len());

        // Read Data
        for block_offset in offsets {
            stream
                .seek(SeekFrom::Start((*offset + block_offset) as u64))
                .ok()?;
            result.push(EffectGroup::new(stream, endianness)?);
        }

        Some(result)
    }

    fn load_offsets<StreamT: Read + Seek>(
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Option<Vec<u32>> {
        let offset_count = Endian::<u32>::from_stream(stream)?.cast(endianness)?;
        let mut offsets = vec![0u32; offset_count as usize];
        for i in 0..offset_count as usize {
            offsets[i] = Endian::<u32>::from_stream(stream)?.cast(endianness)?;
        }

        Some(offsets)
    }

    // inicio da extração do arquivo
    pub fn new<StreamT: Read + Seek>(stream: &mut StreamT, endianness: &Endian<()>) -> Option<Eff> {
        let offsets = Eff::load_offsets(stream, endianness)?;
        let texture_ids = Eff::load_table(stream, &offsets[0], endianness)?;
        let core_ids = Eff::load_table(stream, &offsets[1], endianness)?;
        let ear_links = Eff::load_ear_links(stream, &offsets[2], endianness)?;
        let unknown_table = Eff::load_table(stream, &offsets[3], endianness)?;
        let model_ids = Eff::load_table(stream, &offsets[4], endianness)?;
        let tpls_metadata = Eff::load_tpls_metadata(stream, &offsets[6], endianness)?;
        let effects_0 = Eff::load_effects(stream, &offsets[7], endianness)?;
        let effects_1 = Eff::load_effects(stream, &offsets[8], endianness)?;
        let paths = Eff::load_paths(stream, &offsets[9], endianness)?;

        Some(Eff {
            texture_ids,
            core_ids,
            ear_links,
            unknown_table,
            model_ids,
            tpls_metadata,
            effects_0,
            effects_1,
            paths,
        })
    }


    //sub funcao para gravacao em arquivo .eff
    fn byte_align(offset: u64) -> Option<u64> {
        const BYTE_ALIGNMENT: u64 = 0x20;

        if offset % 0x20 == 0 {
            return Some(offset);
        }
        // Floor division of the offset gives us current byte aligned offset
        // Adding one gives us the nest
        // Multiplication of the byte alignment gives us next byte aligned offset.
        Some(
            offset
                .checked_div(BYTE_ALIGNMENT)?
                .checked_add(1)?
                .checked_mul(BYTE_ALIGNMENT)?,
        )
    }

    //funções destinadas a gravacao de um novo arquiov .eff
    fn write_tpl_metadata<StreamT: Write + Seek>(
        &mut self,
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Result<()> {
        let mut offsets = Vec::<u32>::new();
      
        // leave space for us to add the tpl count and offsets at the end & store the current position for later
        let table_pos = stream.stream_position()?;
        stream.seek(SeekFrom::Current((self.tpls_metadata.len() as i64 + 1) * 4))?;

        // Create a variable to keep track of the aligned offsets so we can write it to the header later
        let mut current_offset = Eff::byte_align(stream.stream_position()?).ok_or(Error::new(
            ErrorKind::InvalidData,
            "Unable to update byte alignment",
        ))?;
        stream.seek(SeekFrom::Start(current_offset))?;

        for metadata in &self.tpls_metadata {
            // Write the current offset to the offset table
            offsets.push(stream.stream_position()? as u32 - table_pos as u32);
            metadata.write(stream, endianness)?;

            current_offset = Eff::byte_align(stream.stream_position()?).ok_or(Error::new(
                ErrorKind::InvalidData,
                "Unable to update byte alignment",
            ))?;
            stream.seek(SeekFrom::Start(current_offset))?;
        }

        // Seek back to the tpl table
        stream.seek(SeekFrom::Start(table_pos))?;

        // Write the tpl count
        let tpl_count = Endian::new(self.tpls_metadata.len() as u32)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        stream.write_all(&tpl_count.to_ne_bytes())?;

        // Write each of the offsets
        for offset in offsets {
            let offset = Endian::new(offset)
                .cast(endianness)
                .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
            stream.write_all(&offset.to_ne_bytes())?;
        }

        // Seek back to the end of the tpl section
        stream.seek(SeekFrom::Start(current_offset))?;
        Ok(())
    }

  
    fn write_effects<StreamT: Write + Seek>(
        effects: &[EffectGroup],
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Result<()> {
        let mut eff_offsets = Vec::<u32>::new();
        eff_offsets.reserve(effects.len());

        // leave space for us to add the tpl count and offsets at the end & store the current position for later
        let table_pos = stream.stream_position()?;
        stream.seek(SeekFrom::Current((effects.len() as i64 + 1) * 4))?;

        // Create a variable to keep track of the aligned offsets so we can write it to the header later
        let mut current_offset = Eff::byte_align(stream.stream_position()?).ok_or(Error::new(
            ErrorKind::InvalidData,
            "Unable to update byte alignment",
        ))?;
        stream.seek(SeekFrom::Start(current_offset))?;

        for effect in effects {
            // Write the current offset to the offset table
            eff_offsets.push(stream.stream_position()? as u32 - table_pos as u32);
            // Compile the tpl data
            effect.write(stream, endianness)?;

            current_offset = Eff::byte_align(stream.stream_position()?).ok_or(Error::new(
                ErrorKind::InvalidData,
                "Unable to update byte alignment",
            ))?;
            stream.seek(SeekFrom::Start(current_offset))?;
        }

        // Seek back to the tpl table
        stream.seek(SeekFrom::Start(table_pos))?;

        // Write the tpl count
        let tpl_count = Endian::new(effects.len() as u32)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        stream.write_all(&tpl_count.to_ne_bytes())?;

        // Write each of the offsets
        for offset in eff_offsets {
            let offset = Endian::new(offset)
                .cast(endianness)
                .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
            stream.write_all(&offset.to_ne_bytes())?;
        }

        // Seek back to the end of the tpl section
        stream.seek(SeekFrom::Start(current_offset))?;
        Ok(())
    }

    fn write_paths<StreamT: Write + Seek>(
        &self,
        stream: &mut StreamT,
        endianness: &Endian<()>,
    ) -> Result<()> {
        let mut offsets = Vec::<u32>::new();
        offsets.reserve(self.paths.len());

        // leave space for us to add the tpl count and offsets at the end & store the current position for later
        let table_pos = stream.stream_position()?;
        stream.seek(SeekFrom::Current((self.paths.len() as i64 + 1) * 4))?;

        // Create a variable to keep track of the aligned offsets so we can write it to the header later
        let mut current_offset = Eff::byte_align(stream.stream_position()?).ok_or(Error::new(
            ErrorKind::InvalidData,
            "Unable to update byte alignment",
        ))?;
        stream.seek(SeekFrom::Start(current_offset))?;

        for path in &self.paths {
            // Write the current offset to the offset table
            offsets.push(stream.stream_position()? as u32 - table_pos as u32);
            // Compile the tpl data
            path.write(stream, endianness)?;

            current_offset = Eff::byte_align(stream.stream_position()?).ok_or(Error::new(
                ErrorKind::InvalidData,
                "Unable to update byte alignment",
            ))?;
            stream.seek(SeekFrom::Start(current_offset))?;
        }

        // Seek back to the tpl table
        stream.seek(SeekFrom::Start(table_pos))?;

        // Write the tpl count
        let tpl_count = Endian::new(self.paths.len() as u32)
            .cast(endianness)
            .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
        stream.write_all(&tpl_count.to_ne_bytes())?;

        // Write each of the offsets
        for offset in offsets {
            let offset = Endian::new(offset)
                .cast(endianness)
                .ok_or(Error::new(ErrorKind::InvalidData, "Unable to cast value!"))?;
            stream.write_all(&offset.to_ne_bytes())?;
        }

        // Seek back to the end of the tpl section
        stream.seek(SeekFrom::Start(current_offset))?;
        Ok(())
    }


    //funcao cria novo arquivo .eff (parte final do repack)
    pub fn compile(&mut self, endianness: &Endian<()>) -> Option<Vec<u8>> {
        let buffer = Vec::<u8>::new();
        let mut stream = Cursor::new(buffer);

        // Variable to hold offsets for each block, we'll write this at the end.
        let mut current_offset = 0x40u64;
        let mut offsets = Vec::<u32>::new();
        offsets.reserve(0xB);

        offsets.push(current_offset.try_into().ok()?);
        stream.seek(SeekFrom::Start(current_offset)).ok()?;
        stream
            .write_all(
                &Endian::new(self.texture_ids.len() as u32)
                    .cast(endianness)?
                    .to_ne_bytes(),
            )
            .ok()?;
        for texture_id in &self.texture_ids {
            texture_id.write(&mut stream, endianness).ok()?;
        }

        current_offset = Eff::byte_align(stream.stream_position().ok()?.into())?;
        offsets.push(current_offset.try_into().ok()?);
        stream.seek(SeekFrom::Start(current_offset)).ok()?;
        stream
            .write_all(
                &Endian::new(self.core_ids.len() as u32)
                    .cast(endianness)?
                    .to_ne_bytes(),
            )
            .ok()?;
        for core_id in &self.core_ids {
            core_id.write(&mut stream, endianness).ok()?;
        }

        current_offset = Eff::byte_align(stream.stream_position().ok()?.into())?;
        offsets.push(current_offset.try_into().ok()?);
        stream.seek(SeekFrom::Start(current_offset)).ok()?;
        stream
            .write_all(
                &Endian::new(self.ear_links.len() as u32)
                    .cast(endianness)?
                    .to_ne_bytes(),
            )
            .ok()?;
        for ear_link in &self.ear_links {
            ear_link.write(&mut stream, endianness).ok()?;
        }

        current_offset = Eff::byte_align(stream.stream_position().ok()?.into())?;
        offsets.push(current_offset.try_into().ok()?);
        stream.seek(SeekFrom::Start(current_offset)).ok()?;
        stream
            .write_all(
                &Endian::new(self.unknown_table.len() as u32)
                    .cast(endianness)?
                    .to_ne_bytes(),
            )
            .ok()?;
        for unknown in &self.unknown_table {
            unknown.write(&mut stream, endianness).ok()?;
        }

        current_offset = Eff::byte_align(stream.stream_position().ok()?.into())?;
        offsets.push(current_offset.try_into().ok()?);
        stream.seek(SeekFrom::Start(current_offset)).ok()?;
        stream
            .write_all(
                &Endian::new(self.model_ids.len() as u32)
                    .cast(endianness)?
                    .to_ne_bytes(),
            )
            .ok()?;
        for model_id in &self.model_ids {
            model_id.write(&mut stream, endianness).ok()?;
        }

        current_offset = Eff::byte_align(stream.stream_position().ok()?.into())?;
        offsets.push(current_offset.try_into().ok()?); // offset da table05
        stream.seek(SeekFrom::Start(current_offset)).ok()?;

        //table5 padding
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());

        current_offset = Eff::byte_align(stream.stream_position().ok()?.into())?;
        offsets.push(current_offset.try_into().ok()?);
        stream.seek(SeekFrom::Start(current_offset)).ok()?;
        self.write_tpl_metadata(&mut stream, endianness).ok()?;

        current_offset = Eff::byte_align(stream.stream_position().ok()?.into())?;
        offsets.push(current_offset.try_into().ok()?);
        stream.seek(SeekFrom::Start(current_offset)).ok()?;
        Eff::write_effects(&self.effects_0, &mut stream, endianness).ok()?;

        current_offset = Eff::byte_align(stream.stream_position().ok()?.into())?;
        offsets.push(current_offset.try_into().ok()?);
        stream.seek(SeekFrom::Start(current_offset)).ok()?;
        Eff::write_effects(&self.effects_1, &mut stream, endianness).ok()?;

        current_offset = Eff::byte_align(stream.stream_position().ok()?.into())?;
        offsets.push(current_offset.try_into().ok()?);
        stream.seek(SeekFrom::Start(current_offset)).ok()?;
        self.write_paths(&mut stream, endianness).ok()?;

        current_offset = Eff::byte_align(stream.stream_position().ok()?.into())?;
        offsets.push(current_offset.try_into().ok()?); // offset da table10
        stream.seek(SeekFrom::Start(current_offset)).ok()?;

        //table10 padding
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());
        stream.write_all(&0u32.to_ne_bytes());

        //inicio
        stream.seek(SeekFrom::Start(0)).ok()?;

        stream
            .write(&Endian::new(0xbu32).cast(endianness)?.to_ne_bytes())
            .ok()?;

        for offset in offsets {
            let offset = Endian::new(offset).cast(endianness)?;
            stream.write(&offset.to_ne_bytes()).ok()?;
        }

        Some(stream.into_inner())
    }

    fn write_texture_ids(&self, path: &std::path::Path) -> Result<()> {
        std::fs::create_dir_all(path)?;
        let mut file = std::fs::File::create(path.join("Table_0_TPL_Texture_IDs.txt2"))?;

        file.write(format!("Entry Count: {}\n", self.texture_ids.len()).as_bytes())?;
        for (index, texture_id) in self.texture_ids.iter().enumerate() {
            file.write(format!("Entry {}: 0x{:X}\n", index, texture_id.id).as_bytes())?;
        }

        Ok(())
    }

    fn write_core_ids(&self, path: &std::path::Path) -> Result<()> {
        std::fs::create_dir_all(path)?;
        let mut file = std::fs::File::create(path.join("Table_1_Effect_0_Indexes.txt2"))?;

        file.write(format!("Entry Count: {}\n", self.core_ids.len()).as_bytes())?;
        for (index, texture_id) in self.core_ids.iter().enumerate() {
            file.write(format!("Entry {}: 0x{:X}\n", index, texture_id.id).as_bytes())?;
        }

        Ok(())
    }

    fn write_ear_links(&self, path: &std::path::Path) -> Result<()> {
        std::fs::create_dir_all(path)?;
        let mut file = std::fs::File::create(path.join("Table_2_EAR_Links.txt2"))?;

        file.write(format!("Entry Count: {}\n\n", self.ear_links.len()).as_bytes())?;
        for (index, link) in self.ear_links.iter().enumerate() {
            file.write(format!("Entry {} Effect Group: 0x{:X}\n",index, link.id).as_bytes())?;
            file.write(format!("Entry {} EAR Link ID: 0x{:X}\n\n", index, link.ear_link_id).as_bytes())?;
        }

        Ok(())
    }

    fn write_unknown_table(&self, path: &std::path::Path) -> Result<()> {
        std::fs::create_dir_all(path)?;
        let mut file = std::fs::File::create(path.join("Table_3_Effect_Path_IDs.txt2"))?;

        file.write(format!("Entry Count: {}\n", self.unknown_table.len()).as_bytes())?;
        for (index, unknown) in self.unknown_table.iter().enumerate() {
            file.write(format!("Entry {}: 0x{:X}\n", index, unknown.id).as_bytes())?;
        }

        Ok(())
    }

    fn write_model_ids(&self, path: &std::path::Path) -> Result<()> {
        std::fs::create_dir_all(path)?;
        let mut file = std::fs::File::create(path.join("Table_4_BIN_Model_IDs.txt2"))?;

        file.write(format!("Entry Count: {}\n", self.model_ids.len()).as_bytes())?;
        for (index, model_id) in self.model_ids.iter().enumerate() {
            file.write(format!("Entry {}: 0x{:X}\n", index, model_id.id).as_bytes())?;
        }

        Ok(())
    }


    fn write_tpl_metadata_txt(&self, path: &std::path::Path) -> Result<()> {
        std::fs::create_dir_all(path)?;
        let mut file = std::fs::File::create(path.join("Table_6_TextureData.txt2"))?;

        file.write(format!("Texture Count: {}\n\n", self.tpls_metadata.len()).as_bytes())?;
        for (index, texture_data) in self.tpls_metadata.iter().enumerate() {
            file.write(format!("Entry {}:\n", index).as_bytes())?;
            file.write(format!("Height: {}\n", texture_data.texture_height).as_bytes())?;
            file.write(format!("Width: {}\n", texture_data.texture_width).as_bytes())?;
            file.write(format!("Effect Height: {}\n", texture_data.effect_height).as_bytes())?;
            file.write(format!("Effect Width: {}\n", texture_data.effect_width).as_bytes())?;
            file.write(format!("Effect Texture Count: {}\n", texture_data.texture_count).as_bytes(),)?;
            file.write(format!("Offset[10]: {}\n", texture_data.unknown_1).as_bytes())?;
            file.write(format!("Offset[11]: {}\n\n", texture_data.unknown_2).as_bytes())?;
        }

        Ok(())
    }

    fn write_effect_to_txt(effect_group: &EffectGroup, path: &std::path::Path) -> Result<()> {
        
        let mut file = std::fs::File::create(path)?; //"Data.txt"

        file.write(format!("Effect Count: {}\n", effect_group.effects.len()).as_bytes())?;
        file.write(format!("Offset[X02]: 0x{:X}\n", effect_group._unknownX02).as_bytes())?;
        file.write(format!("Offset[X04]: 0x{:X}\n", effect_group._unknownX04).as_bytes())?;
        file.write(format!("Offset[X06]: 0x{:X}\n", effect_group._unknownX06).as_bytes())?;
        file.write(format!("Offset[X08]: 0x{:X}\n", effect_group._unknownX08).as_bytes())?;
        file.write(format!("Offset[X0A]: 0x{:X}\n", effect_group._unknownX0A).as_bytes())?;
        file.write(format!("Offset[X0B]: 0x{:X}\n", effect_group._unknownX0B).as_bytes())?;
        file.write(format!("Offset[X0C]: {}\n", effect_group._unknownX0C).as_bytes())?;
        file.write(format!("Offset[X10]: {}\n", effect_group._unknownX10).as_bytes())?;
        file.write(format!("Offset[X14]: {}\n", effect_group._unknownX14).as_bytes())?;
        file.write(format!("Offset[X18]: {}\n", effect_group._unknownX18).as_bytes())?;
        file.write(format!("Offset[X1C]: {}\n", effect_group._unknownX1C).as_bytes())?;
        file.write(format!("Offset[X20]: {}\n", effect_group._unknownX20).as_bytes())?;
        file.write(format!("Offset[X24]: 0x{:X}\n", effect_group._unknownX24).as_bytes())?;
        file.write(b"\n\n")?;

        for (index, effect) in effect_group.effects.iter().enumerate() {
            file.write(format!("Effect {}\n", index).as_bytes())?;
            file.write(format!("State ID: {}\n", effect.state_id).as_bytes())?;
            file.write(format!("ESP ID: 0x{:X}\n", effect.esp_id).as_bytes())?;
            file.write(format!("Texture ID: 0x{:X}\n", effect.texture_id).as_bytes())?;
            file.write(format!("Unknown X03: 0x{:X}\n", effect._unknown_x03).as_bytes())?;
            file.write(format!("Delay: {}\n", effect.time).as_bytes())?;
            file.write(format!("Parent: 0x{:X}\n", effect.parent).as_bytes())?;
            file.write(format!("Parent Part: 0x{:X}\n", effect.part).as_bytes())?;
            file.write(format!("Flags: 0x{:X}\n", effect.flags).as_bytes())?;
            file.write(format!("Position X: {}\n", effect.position.0).as_bytes())?;
            file.write(format!("Position Y: {}\n", effect.position.1).as_bytes())?;
            file.write(format!("Position Z: {}\n", effect.position.2).as_bytes())?;
            file.write(format!("Random Position X: {}\n", effect.random.0).as_bytes())?;
            file.write(format!("Random Position Y: {}\n", effect.random.1).as_bytes())?;
            file.write(format!("Random Position Z: {}\n", effect.random.2).as_bytes())?;
            file.write(format!("Speed X: {}\n", effect.speed.0).as_bytes())?;
            file.write(format!("Speed Y: {}\n", effect.speed.1).as_bytes())?;
            file.write(format!("Speed Z: {}\n", effect.speed.2).as_bytes())?;
            file.write(format!("Delta Speed: {}\n", effect.delta_speed).as_bytes())?;
            file.write(format!("Random Speed X: {}\n", effect.random_speed.0).as_bytes())?;
            file.write(format!("Random Speed Y: {}\n", effect.random_speed.1).as_bytes())?;
            file.write(format!("Random Speed Z: {}\n", effect.random_speed.2).as_bytes())?;
            file.write(format!("Acceleration X: {}\n", effect.acceleration.0).as_bytes())?;
            file.write(format!("Acceleration Y: {}\n", effect.acceleration.1).as_bytes())?;
            file.write(format!("Acceleration Z: {}\n", effect.acceleration.2).as_bytes())?;
            file.write(format!("Random Acceleration X: {}\n", effect.random_acceleration.0).as_bytes())?;
            file.write(format!("Random Acceleration Y: {}\n", effect.random_acceleration.1).as_bytes())?;
            file.write(format!("Random Acceleration Z: {}\n", effect.random_acceleration.2).as_bytes())?;
            file.write(format!("Rotation X: {}\n", effect.rotate.0).as_bytes())?;
            file.write(format!("Rotation Y: {}\n", effect.rotate.1).as_bytes())?;
            file.write(format!("Rotation Z: {}\n", effect.rotate.2).as_bytes())?;
            file.write(format!("Random Rotation X: {}\n", effect.random_rotate.0).as_bytes())?;
            file.write(format!("Random Rotation Y: {}\n", effect.random_rotate.1).as_bytes())?;
            file.write(format!("Random Rotation Z: {}\n", effect.random_rotate.2).as_bytes())?;
            file.write(format!("Rotation Acceleration X: {}\n", effect.rotate_acceleration.0).as_bytes())?;
            file.write(format!("Rotation Acceleration Y: {}\n", effect.rotate_acceleration.1).as_bytes())?;
            file.write(format!("Rotation Acceleration Z: {}\n", effect.rotate_acceleration.2).as_bytes())?;
            file.write(format!("Random Rotation Acceleration X: {}\n", effect.random_rotate_acceleration.0).as_bytes())?;
            file.write(format!("Random Rotation Acceleration Y: {}\n", effect.random_rotate_acceleration.1).as_bytes())?;
            file.write(format!("Random Rotation Acceleration Z: {}\n", effect.random_rotate_acceleration.2).as_bytes())?;
            file.write(format!("Width: {}\n", effect.width).as_bytes())?;
            file.write(format!("Height: {}\n", effect.height).as_bytes())?;
            file.write(format!("Random Size: {}\n", effect.random_size).as_bytes())?;
            file.write(format!("Grow: {}\n", effect.grow).as_bytes())?;
            file.write(format!("Delta Grow: {}\n", effect.delta_grow).as_bytes())?;
            file.write(format!("R: 0x{:X}\n", effect.rgba.0).as_bytes())?;
            file.write(format!("G: 0x{:X}\n", effect.rgba.1).as_bytes())?;
            file.write(format!("B: 0x{:X}\n", effect.rgba.2).as_bytes())?;
            file.write(format!("A: 0x{:X}\n", effect.rgba.3).as_bytes())?;
            file.write(format!("Delta R: {}\n", effect.delta_color.0).as_bytes())?;
            file.write(format!("Delta G: {}\n", effect.delta_color.1).as_bytes())?;
            file.write(format!("Delta B: {}\n", effect.delta_color.2).as_bytes())?;
            file.write(format!("Delta A: {}\n", effect.delta_color.3).as_bytes())?;
            file.write(format!("Delta Color Max Frame: {}\n", effect.delta_color_attack).as_bytes())?;
            file.write(format!("Delta Color Start Frame: {}\n", effect.delta_color_start_frame).as_bytes())?;
            file.write(format!("Unknown XB4: 0x{:X}\n", effect._unknown_xb4).as_bytes())?;
            file.write(format!("Delta Size Start Frame: {}\n", effect.delta_size_start_frame).as_bytes())?;
            file.write(format!("Lifetime: {}\n", effect.life_time).as_bytes())?;
            file.write(format!("Animation Speed: {}\n", effect.animation_speed).as_bytes())?;
            file.write(format!("Unknown XBE: 0x{:X}\n", effect._unknown_xbe).as_bytes())?;
            file.write(format!("Release Time: {}\n", effect.release_time).as_bytes())?;
            file.write(format!("Blend: {}\n", effect.blend).as_bytes())?;
            file.write(format!("Simulation Type: {}\n", effect.simulation_type).as_bytes())?;
            file.write(format!("Simulation Power: {}\n", effect.simulation_power).as_bytes())?;
            file.write(format!("Mask Texture ID: {}\n", effect.mask_texture_id).as_bytes())?;
            file.write(format!("Value In: {}\n", effect.value_in).as_bytes())?;
            file.write(format!("Value Out: {}\n", effect.value_out).as_bytes())?;
            file.write(format!("Work 0: {}\n", effect.work_0).as_bytes())?;
            file.write(format!("Work 1: {}\n", effect.work_1).as_bytes())?;
            file.write(format!("Work 2: {}\n", effect.work_2).as_bytes())?;
            file.write(format!("Work 3: {}\n", effect.work_3).as_bytes())?;
            file.write(format!("Work 4: {}\n", effect.work_4).as_bytes())?;
            file.write(format!("Work 5: {}\n", effect.work_5).as_bytes())?;
            file.write(format!("Work 6: {}\n", effect.work_6).as_bytes())?;
            file.write(format!("Vector 0 X: {}\n", effect.vector_0.0).as_bytes())?;
            file.write(format!("Vector 0 Y: {}\n", effect.vector_0.1).as_bytes())?;
            file.write(format!("Vector 0 Z: {}\n", effect.vector_0.2).as_bytes())?;
            file.write(format!("Vector 1 X: {}\n", effect.vector_1.0).as_bytes())?;
            file.write(format!("Vector 1 Y: {}\n", effect.vector_1.1).as_bytes())?;
            file.write(format!("Vector 1 Z: {}\n", effect.vector_1.2).as_bytes())?;
            file.write(format!("Vector 2 X: {}\n", effect.vector_2.0).as_bytes())?;
            file.write(format!("Vector 2 Y: {}\n", effect.vector_2.1).as_bytes())?;
            file.write(format!("Vector 2 Z: {}\n", effect.vector_2.2).as_bytes())?;
            file.write(format!("Spline 0: {}\n", effect.spline_0).as_bytes())?;
            file.write(format!("Spline 1: {}\n", effect.spline_1).as_bytes())?;
            file.write(format!("Spline 2: {}\n", effect.spline_2).as_bytes())?;
            file.write(format!("Spline 3: {}\n", effect.spline_3).as_bytes())?;
            file.write(format!("Unknown X100: 0x{:X}\n", effect._unknown_x100).as_bytes())?;
            file.write(format!("Path Own: {}\n", effect.path_own).as_bytes())?;
            file.write(format!("Path Number: {}\n", effect.path_number).as_bytes())?;
            file.write(format!("Path Start: {}\n", effect.path_start).as_bytes())?;
            file.write(format!("Path Random: {}\n", effect.path_random).as_bytes())?;
            file.write(format!("Effect Type: {}\n", effect.eff_type).as_bytes())?;
            file.write(format!("Control ID: {}\n", effect.control_id).as_bytes())?;
            file.write(format!("Control Flag: {}\n", effect.control_flag).as_bytes())?;
            file.write(format!("Control Interval: {}\n", effect.control_interval).as_bytes())?;
            file.write(format!("Control Number: {}\n", effect.control_number).as_bytes())?;
            file.write(format!("Control RP: {}\n", effect.control_rp).as_bytes())?;
            file.write(format!("Unknown X10F: 0x{:X}\n", effect._unknown_x10f).as_bytes())?;
            file.write(format!("Control Life: {}\n", effect.control_life).as_bytes())?;
            file.write(format!("Unknown X112: 0x{:X}\n", effect._unknown_x112).as_bytes())?;
            file.write(format!("Unknown X114: 0x{:X}\n", effect._unknown_x114).as_bytes())?;
            file.write(format!("Unknown X116: 0x{:X}\n", effect._unknown_x116).as_bytes())?;
            file.write(format!("Control Path Scale X: {}\n", effect.control_path_scale.0).as_bytes())?;
            file.write(format!("Control Path Scale Y: {}\n", effect.control_path_scale.1).as_bytes())?;
            file.write(format!("Control Path Scale Z: {}\n", effect.control_path_scale.2).as_bytes())?;
            file.write(format!("Control Path Delta Size: {}\n", effect.control_path_delta_size).as_bytes())?;
            file.write(format!("Control Path Delta Speed: {}\n", effect.control_path_delta_speed).as_bytes())?;
            file.write(format!("Control Path Delta Alpha: {}\n", effect.control_path_delta_alpha).as_bytes())?;
            file.write(format!("Control Path Delta Interval: {}\n", effect.control_path_delta_interval).as_bytes())?;
            file.write(format!("Control Path Random Interval: {}\n", effect.control_path_random_interval).as_bytes())?;
            file.write(format!("Control Path Rotation X: {}\n", effect.control_path_rotation.0).as_bytes())?;
            file.write(format!("Control Path Rotation Y: {}\n",effect.control_path_rotation.1).as_bytes())?;
            file.write(format!("Control Path Flag: {}\n\n", effect.control_path_flag).as_bytes())?;

        }

        Ok(())
    }

    fn write_effect_to_obj(
        effect_group: &EffectGroup,
        group_number: usize,
        table_number: usize,
        path: &std::path::Path,
    ) -> Result<()> {
        
        let mut file = std::fs::File::create(path)?; //"Model.obj"

        let mut face_index = 1;

        file.write(b"#For reference only, scale 1/100, Y is the height\n\n")?;
        file.write(b"vn 0.0 0.0 -1.0\n")?;

        for (index, effect) in effect_group.effects.iter().enumerate() {
            file.write(
                format!(
                    "v {} {} {}\n",
                    ((effect.position.0) / 100.0),
                    ((effect.position.1) / 100.0),
                    ((effect.position.2) / 100.0)
                )
                .as_bytes(),
            )?;
            file.write(
                format!(
                    "v {} {} {}\n",
                    ((effect.position.0) / 100.0),
                    ((effect.position.1) / 100.0) + 10.0,
                    ((effect.position.2) / 100.0) + 10.0
                )
                .as_bytes(),
            )?;
            file.write(
                format!(
                    "v {} {} {}\n",
                    ((effect.position.0) / 100.0),
                    ((effect.position.1) / 100.0) + 10.0,
                    ((effect.position.2) / 100.0) - 10.0
                )
                .as_bytes(),
            )?;

            file.write(
                format!(
                    "g Table_{}_Group_{}_EffectIndex_{}_EspID_0x{:X}_TextureID_0x{:X}\n",
                    table_number, group_number, index, effect.esp_id, effect.texture_id
                )
                .as_bytes(),
            )?;
            file.write(
                format!(
                    "f {}//1 {}//1 {}//1\n\n",
                    face_index,
                    face_index + 1,
                    face_index + 2
                )
                .as_bytes(),
            )?;
            face_index += 3;
        }

        Ok(())
    }

    fn write_effects_to_txt(effect_group: &[EffectGroup], table_number: usize, path: &std::path::Path) -> Result<()> {
        std::fs::create_dir_all(path)?;

        for (index, effect) in effect_group.iter().enumerate() {
            Eff::write_effect_to_txt(
                effect,
                &path.join(format!("Effect Group {} Data.txt2", index)),
            )?;
            Eff::write_effect_to_obj(
                effect,
                index,
                table_number,
                &path.join(format!("Effect Group {} Model.obj", index)),
            )?;
        }

        Ok(())
    }


    fn write_curves_out(&self, path: &std::path::Path) -> Result<()> {
        std::fs::create_dir_all(path)?;
        let mut file = std::fs::File::create(path.join("Table_9_Paths.txt2"))?;

        file.write(format!("Path Count: {}\n", self.paths.len()).as_bytes())?;
        for (index, path) in self.paths.iter().enumerate() {
            file.write(format!("Entry {}:\n", index).as_bytes())?;
            file.write(format!("\tPoint Count: {}\n", path.points.len()).as_bytes())?;
            for (point_index, point) in path.points.iter().enumerate() {
                file.write(format!("\tPoint {}: \n", point_index).as_bytes())?;
                file.write(format!("\t\tPosition X: {}\n", point.point.0).as_bytes())?;
                file.write(format!("\t\tPosition Y: {}\n", point.point.1).as_bytes())?;
                file.write(format!("\t\tPosition Z: {}\n", point.point.2).as_bytes())?;
                file.write(format!("\t\tHandle 0 X: {}\n", point.handle_0.0).as_bytes())?;
                file.write(format!("\t\tHandle 0 Y: {}\n", point.handle_0.1).as_bytes())?;
                file.write(format!("\t\tHandle 0 Z: {}\n", point.handle_0.2).as_bytes())?;
                file.write(format!("\t\tHandle 1 X: {}\n", point.handle_1.0).as_bytes())?;
                file.write(format!("\t\tHandle 1 Y: {}\n", point.handle_1.1).as_bytes())?;
                file.write(format!("\t\tHandle 1 Z: {}\n", point.handle_1.2).as_bytes())?;
                file.write(format!("\t\tUnknown: {}\n", point.unknown).as_bytes())?;
            }
        }

        Ok(())
    }

    fn write_ReadMe(
     path: &std::path::Path,
    ) -> Result<()> {

        let mut file = std::fs::File::create(path)?;
        file.write(b"# RE4_EFF_BLOB_RUST\n")?;
        file.write(b"# Tool By Zatarita\n")?;
        file.write(b"# Fork By JADERLINK\n")?;
        file.write(b"# Version 1.0.0")?;

     Ok(())
    }

    // cria os arqivos de txt, na parte da extracao
    pub fn write_to_text(&mut self, path: &Path) -> Result<()> {
        self.write_texture_ids(path.join("Tables").as_path())?;
        self.write_core_ids(path.join("Tables").as_path())?;
        self.write_ear_links(path.join("Tables").as_path())?;
        self.write_unknown_table(path.join("Tables").as_path())?;
        self.write_model_ids(path.join("Tables").as_path())?;
        self.write_curves_out(path.join("Tables").as_path())?;
        self.write_tpl_metadata_txt(path.join("Tables").as_path())?;
        Eff::write_effects_to_txt(&self.effects_0, 7, path.join("Effect 0").as_path())?;
        Eff::write_effects_to_txt(&self.effects_1, 8, path.join("Effect 1").as_path())?;
        let file = path.to_str().unwrap().trim_end_matches("\\").to_string() + ".EFFBLOBTXT";
        let txt = std::path::Path::new(&file);
        Eff::write_ReadMe(&txt)?;
        Ok(())
    }

    // parte do repack le os arquivos .txt

    // repack 1º funcao
    fn read_table(path: &Path) -> Option<Vec<TableEntry>> {
        let mut result = Vec::<TableEntry>::new();
        let file = std::fs::File::open(path).ok()?;

        for line in BufReader::new(file).lines() {
            let line = line.ok()?;
            let (key, value) = line.split_once(": ")?;
            match key {
                "Entry Count" => {
                    let count: u32 = value.parse().ok()?;
                    result.reserve(count as usize);
                }
                _ => {
                    let id = u32::from_str_radix(&value[2..], 16).ok()?;
                    result.push(TableEntry { id, _unknown: 0 });
                }
            }
        }

        Some(result)
    }

    //sub funcao
    fn read_line<StreamT: Read>(stream: &mut BufReader<StreamT>) -> Option<String> {
        let mut buffer = String::new();
        while buffer.is_empty() {
            stream.read_line(&mut buffer).ok()?;
            if let Some(_) = buffer.find("#") {
                buffer = buffer.split_once("#")?.0.to_owned();
            }
            if let Some(_) = buffer.find("\t") {
                buffer = buffer.split_once("\t")?.1.to_owned();
            }
            buffer.remove(buffer.find("\n")?);
        }
        Some(buffer)
    }

    // repack 5º funcao
    fn read_ear_links(path: &Path) -> Option<Vec<EarLink>> {
        let mut result = Vec::<EarLink>::new();
        let file = std::fs::File::open(path).ok()?;
        let mut reader = BufReader::new(file);

        let mut buffer = Eff::read_line(&mut reader)?;
        let (_, value) = buffer.split_once(": ")?;
        let count: u32 = u32::from_str_radix(&value, 10).ok()?;
        result.reserve(count as usize);

        for _ in 0..count {
            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let id = u16::from_str_radix(&value[2..], 16).ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let ear_link_id = u16::from_str_radix(&value[2..], 16).ok()?;

            result.push(EarLink {
                id,
                ear_link_id,
                _unknown: 0,
            });
        }

        Some(result)
    }

    // repack 6º funcao
    fn read_tpl_metadata(path: &Path) -> Option<Vec<TextureMetadata>> {
        let mut result = Vec::<TextureMetadata>::new();
        let file = std::fs::File::open(path).ok()?;
        let mut reader = BufReader::new(file);
        let mut buffer = Eff::read_line(&mut reader)?;

        let (_, value) = buffer.split_once(": ")?;
        let count: u32 = u32::from_str_radix(&value[0..], 10).ok()?;
        result.reserve(count as usize);

        for _ in 0..count {
            _ = Eff::read_line(&mut reader)?;
            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let texture_height = u16::from_str_radix(&value, 10).ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let texture_width = u16::from_str_radix(&value, 10).ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let effect_height = u16::from_str_radix(&value, 10).ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let effect_width = u16::from_str_radix(&value, 10).ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let texture_count = u16::from_str_radix(&value, 10).ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let unknown_1 = u8::from_str_radix(&value, 10).ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let unknown_2 = u8::from_str_radix(&value, 10).ok()?;

            result.push(TextureMetadata {
                texture_height,
                texture_width,
                effect_height,
                effect_width,
                texture_count,
                unknown_1,
                unknown_2,
            });
        }

        Some(result)
    }

    // repack 8º funcao
    fn read_paths_from_file(path: &Path) -> Option<Vec<Curve>> {
        let mut result = Vec::<Curve>::new();
        let file = std::fs::File::open(path).ok()?;
        let mut reader = BufReader::new(file);
        let mut buffer = Eff::read_line(&mut reader)?;

        let (_, value) = buffer.split_once(": ")?;
        let count: u32 = u32::from_str_radix(&value[0..], 10).ok()?;
        result.reserve(count as usize);

        for _ in 0..count {
            _ = Eff::read_line(&mut reader)?;
            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let point_count = usize::from_str_radix(&value, 10).ok()?;

            let mut points = Vec::<CurvePoint>::new();

            for _ in 0..point_count {
                _ = Eff::read_line(&mut reader)?;
                buffer = Eff::read_line(&mut reader)?;
                let (_, value) = buffer.split_once(": ")?;
                let position_x: f32 = value.parse().ok()?;

                buffer = Eff::read_line(&mut reader)?;
                let (_, value) = buffer.split_once(": ")?;
                let position_y: f32 = value.parse().ok()?;

                buffer = Eff::read_line(&mut reader)?;
                let (_, value) = buffer.split_once(": ")?;
                let position_z: f32 = value.parse().ok()?;

                buffer = Eff::read_line(&mut reader)?;
                let (_, value) = buffer.split_once(": ")?;
                let handle_0_x: f32 = value.parse().ok()?;

                buffer = Eff::read_line(&mut reader)?;
                let (_, value) = buffer.split_once(": ")?;
                let handle_0_y: f32 = value.parse().ok()?;

                buffer = Eff::read_line(&mut reader)?;
                let (_, value) = buffer.split_once(": ")?;
                let handle_0_z: f32 = value.parse().ok()?;

                buffer = Eff::read_line(&mut reader)?;
                let (_, value) = buffer.split_once(": ")?;
                let handle_1_x: f32 = value.parse().ok()?;

                buffer = Eff::read_line(&mut reader)?;
                let (_, value) = buffer.split_once(": ")?;
                let handle_1_y: f32 = value.parse().ok()?;

                buffer = Eff::read_line(&mut reader)?;
                let (_, value) = buffer.split_once(": ")?;
                let handle_1_z: f32 = value.parse().ok()?;

                buffer = Eff::read_line(&mut reader)?;
                let (_, value) = buffer.split_once(": ")?;
                let unknown: f32 = value.parse().ok()?;

                points.push(CurvePoint {
                    point: (position_x, position_y, position_z),
                    handle_0: (handle_0_x, handle_0_y, handle_0_z),
                    handle_1: (handle_1_x, handle_1_y, handle_1_z),
                    unknown,
                });
            }

            result.push(Curve { points });
        }

        Some(result)
    }

    // subfuncao do repack 10
    fn read_effect_from_file<StreamT: Read>(stream: &mut BufReader<StreamT>) -> Option<Effect> {
        let mut buffer = Eff::read_line(stream)?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let state_id: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let esp_id: u8 = u8::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let texture_id: u8 = u8::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let _unknown_x03: u8 = u8::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let time: u16 = u16::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let parent: u8 = u8::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let part: u8 = u8::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let flags: u32 = u32::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let position_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let position_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let position_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let speed_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let speed_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let speed_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let delta_speed: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_speed_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_speed_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_speed_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let acceleration_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let acceleration_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let acceleration_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_acceleration_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_acceleration_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_acceleration_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let rotation_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let rotation_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let rotation_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_rotation_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_rotation_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_rotation_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let rotation_acceleration_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let rotation_acceleration_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let rotation_acceleration_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_rotation_acceleration_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_rotation_acceleration_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_rotation_acceleration_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let width: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let height: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let random_size: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let grow: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let delta_grow: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let r: u8 = u8::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let g: u8 = u8::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let b: u8 = u8::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let a: u8 = u8::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let delta_r: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let delta_g: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let delta_b: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let delta_a: f32 = value.parse().ok()?;
        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let delta_color_attack: u16 = u16::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let delta_color_start_frame: u16 = u16::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let _unknown_xb4: u16 = u16::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let delta_size_start_frame: u16 = u16::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let life_time: u16 = u16::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let animation_speed: u32 = u32::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let _unknown_xbe: u16 = u16::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let release_time: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let blend: u16 = u16::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let simulation_type: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let simulation_power: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let mask_texture_id: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let value_in: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let value_out: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let work_0: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let work_1: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let work_2: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let work_3: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let work_4: u32 = u32::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let work_5: u32 = u32::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let work_6: u32 = u32::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let vector_0_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let vector_0_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let vector_0_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let vector_1_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let vector_1_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let vector_1_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let vector_2_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let vector_2_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let vector_2_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let spline_0: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let spline_1: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let spline_2: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let spline_3: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let _unknown_x100: u32 = u32::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let path_own: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let path_number: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let path_start: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let path_random: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let eff_type: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_id: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_flag: u16 = u16::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_interval: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_number: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_rp: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

         buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let _unknown_x10f: u8 = u8::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_life: u16 = u16::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let _unknown_x112: u16 = u16::from_str_radix(&value[2..], 16).ok()?;
     
        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let _unknown_x114: u16 = u16::from_str_radix(&value[2..], 16).ok()?;
        
        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let _unknown_x116: u16 = u16::from_str_radix(&value[2..], 16).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_path_scale_x: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_path_scale_y: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_path_scale_z: f32 = value.parse().ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_path_delta_size: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_path_delta_speed: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_path_delta_alpha: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_path_delta_interval: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_path_random_interval: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_path_rotation_x: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_path_rotation_y: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        buffer = Eff::read_line(stream)?;
        let (_, value) = buffer.split_once(": ")?;
        let control_path_flag: u8 = u8::from_str_radix(&value[0..], 10).ok()?;

        Some(Effect {
            state_id,
            esp_id,
            texture_id,
            _unknown_x03,
            time,
            parent,
            part,
            flags,
            position: (position_x, position_y, position_z),
            random: (random_x, random_y, random_z),
            speed: (speed_x, speed_y, speed_z),
            delta_speed,
            random_speed: (random_speed_x, random_speed_y, random_speed_z),
            acceleration: (acceleration_x, acceleration_y, acceleration_z),
            random_acceleration: (
                random_acceleration_x,
                random_acceleration_y,
                random_acceleration_z,
            ),
            rotate: (rotation_x, rotation_y, rotation_z),
            random_rotate: (random_rotation_x, random_rotation_y, random_rotation_z),
            rotate_acceleration: (
                rotation_acceleration_x,
                rotation_acceleration_y,
                rotation_acceleration_z,
            ),
            random_rotate_acceleration: (
                random_rotation_acceleration_x,
                random_rotation_acceleration_y,
                random_rotation_acceleration_z,
            ),
            width,
            height,
            random_size,
            grow,
            delta_grow,
            rgba: (r, g, b, a),
            delta_color: (delta_r, delta_g, delta_b, delta_a),
            delta_color_attack,
            delta_color_start_frame,
            _unknown_xb4,
            delta_size_start_frame,
            life_time,
            animation_speed,
            _unknown_xbe,
            release_time,
            blend,
            simulation_type,
            simulation_power,
            mask_texture_id,
            value_in,
            value_out,
            work_0,
            work_1,
            work_2,
            work_3,
            work_4,
            work_5,
            work_6,
            vector_0: (vector_0_x, vector_0_y, vector_0_z),
            vector_1: (vector_1_x, vector_1_y, vector_1_z),
            vector_2: (vector_2_x, vector_2_y, vector_2_z),
            spline_0,
            spline_1,
            spline_2,
            spline_3,
            _unknown_x100,
            path_own,
            path_number,
            path_start,
            path_random,
            eff_type,
            control_id,
            control_flag,
            control_interval,
            control_number,
            control_rp,
            _unknown_x10f,
            control_life,
            _unknown_x112,
            _unknown_x114,
            _unknown_x116,
            control_path_scale: (
                control_path_scale_x,
                control_path_scale_y,
                control_path_scale_z,
            ),
            control_path_delta_size,
            control_path_delta_speed,
            control_path_delta_alpha,
            control_path_delta_interval,
            control_path_random_interval,
            control_path_rotation: (control_path_rotation_x, control_path_rotation_y),
            control_path_flag,
        })
    }

    //repack 10º e 11º
    fn read_effect_group_from_file(path: &Path, effect_count: usize) -> Option<Vec<EffectGroup>> {
        let mut result = Vec::<EffectGroup>::new();
        for i in 0..effect_count {
            let mut effects = Vec::<Effect>::new();
            let file =
                std::fs::File::open(path.join(format!("Effect Group {} Data.txt2", i))).ok()?;
            let mut reader = BufReader::new(file);

            let mut buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let count: u32 = u32::from_str_radix(&value[0..], 10).ok()?;
            effects.reserve(count as usize);

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let _unknownX02: u16 = u16::from_str_radix(&value[2..], 16).ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let _unknownX04: u16 = u16::from_str_radix(&value[2..], 16).ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let _unknownX06: u16 = u16::from_str_radix(&value[2..], 16).ok()?;

             buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let _unknownX08: u16 = u16::from_str_radix(&value[2..], 16).ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let _unknownX0A: u8 = u8::from_str_radix(&value[2..], 16).ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let _unknownX0B: u8 = u8::from_str_radix(&value[2..], 16).ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let _unknownX0C: f32 = value.parse().ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let _unknownX10: f32 = value.parse().ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let _unknownX14: f32 = value.parse().ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let _unknownX18: f32 = value.parse().ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let _unknownX1C: f32 = value.parse().ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let _unknownX20: f32 = value.parse().ok()?;

            buffer = Eff::read_line(&mut reader)?;
            let (_, value) = buffer.split_once(": ")?;
            let _unknownX24: u8 = u8::from_str_radix(&value[2..], 16).ok()?;

            for _ in 0..count {
                effects.push(Eff::read_effect_from_file(&mut reader)?)
            }

            result.push(EffectGroup {
                _unknownX02,
                _unknownX04,
                _unknownX06,
                _unknownX08,
                _unknownX0A,
                _unknownX0B,
                _unknownX0C,
                _unknownX10,
                _unknownX14,
                _unknownX18,
                _unknownX1C,
                _unknownX20,
                _unknownX24,
                effects,
            });
        }
        Some(result)
    }

    // inicio do repack
    pub fn read_from_text(path: &Path) -> Option<Eff> {
        //variaveis
        let mut texture_ids = Vec::<TableEntry>::new();
        let mut core_ids = Vec::<TableEntry>::new();
        let mut model_ids = Vec::<TableEntry>::new();
        let mut unknown_table = Vec::<TableEntry>::new();
        let mut ear_links = Vec::<EarLink>::new();
        let mut tpls_metadata = Vec::<TextureMetadata>::new();
        let mut paths = Vec::<Curve>::new();
        let mut effects_0 = Vec::<EffectGroup>::new();
        let mut effects_1 = Vec::<EffectGroup>::new();

        //checagem
        //1
        if path.join("Tables/Table_0_TPL_Texture_IDs.txt2").as_path().is_file() 
        {
            texture_ids = Eff::read_table(path.join("Tables/Table_0_TPL_Texture_IDs.txt2").as_path())?;
        }

        //2
        if path.join("Tables/Table_1_Effect_0_Indexes.txt2").as_path().is_file()
        {
            core_ids = Eff::read_table(path.join("Tables/Table_1_Effect_0_Indexes.txt2").as_path())?;
        }

        //3
        if path.join("Tables/Table_4_BIN_Model_IDs.txt2").as_path().is_file()
        {
            model_ids = Eff::read_table(path.join("Tables/Table_4_BIN_Model_IDs.txt2").as_path())?;
        }

        //4
        if path.join("Tables/Table_3_Effect_Path_IDs.txt2").as_path().is_file()
        {
            unknown_table = Eff::read_table(path.join("Tables/Table_3_Effect_Path_IDs.txt2").as_path())?;
        }

        //5
        if path.join("Tables/Table_2_EAR_Links.txt2").as_path().is_file()
        {
            ear_links = Eff::read_ear_links(path.join("Tables/Table_2_EAR_Links.txt2").as_path())?;
        }

        //6
        if path.join("Tables/Table_6_TextureData.txt2").as_path().is_file() 
        {
            tpls_metadata = Eff::read_tpl_metadata(path.join("Tables/Table_6_TextureData.txt2").as_path())?;
        }

        //8
        if path.join("Tables/Table_9_Paths.txt2").as_path().is_file() 
        {
            paths = Eff::read_paths_from_file(path.join("Tables/Table_9_Paths.txt2").as_path())?;
        }

        //10
        if path.join("Effect 0").as_path().is_dir() 
        {
            effects_0 = Eff::read_effect_group_from_file(path.join("Effect 0").as_path(), core_ids.len())?;
        }

        //11
        if path.join("Effect 1").as_path().is_dir() 
        {
            effects_1 = Eff::read_effect_group_from_file(path.join("Effect 1").as_path(), ear_links.len())?;
        }

        Some(Eff {
            texture_ids,
            core_ids,
            ear_links,
            unknown_table,
            model_ids,
            tpls_metadata,
            effects_0,
            effects_1,
            paths,
        })
    }
}
