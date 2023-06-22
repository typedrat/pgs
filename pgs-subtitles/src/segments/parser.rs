use log::warn;
use winnow::binary::{be_u16, be_u32};
use winnow::combinator::{dispatch, fail};
use winnow::multi::{length_count, length_value};
use winnow::prelude::*;
use winnow::stream::Bytes;
use winnow::token::any;
use winnow::trace::trace;

use crate::segments::types::*;
use crate::types::util::*;

fn timestamp(input: &Bytes) -> IResult<&Bytes, PGSTimestamp> {
    be_u32
        .map(PGSTimestamp::from_raw_timestamp)
        .context("PGS timestamp")
        .parse_next(input)
}

pub fn segment(input: &Bytes) -> IResult<&Bytes, PGSSegment> {
    let (rest, _) = "PG".context("magic header").parse_next(input)?;
    let (rest, presentation_timestamp) = timestamp.parse_next(rest)?;
    let (rest, decoding_timestamp) = timestamp.parse_next(rest)?;
    dispatch! {any.context("segment type");
        0x16 => length_value(be_u16.context("segment length"), trace("presentation composition segment", presentation_segment(presentation_timestamp, decoding_timestamp))).map(PGSSegment::PCS),
        0x17 => length_value(be_u16.context("segment length"), trace("window definition segment", window_segment(presentation_timestamp, decoding_timestamp))).map(PGSSegment::WDS)
        0x80 => length_value(be_u16.context("segment length"), ()).map(|_| PGSSegment::END),
        _    => fail::<_, PGSSegment, _>
    }
    .parse_next(rest)
}

//

fn presentation_segment(
    presentation_timestamp: PGSTimestamp,
    decoding_timestamp: PGSTimestamp,
) -> impl FnMut(&Bytes) -> IResult<&Bytes, PresentationControlSegment> {
    move |input: &Bytes| -> IResult<&Bytes, PresentationControlSegment> {
        let (rest, width) = be_u16.parse_next(input)?;
        let (rest, height) = be_u16.parse_next(rest)?;
        let (rest, _) = any.parse_next(rest)?;
        let (rest, composition_number) = be_u16.parse_next(rest)?;
        let (rest, composition_type) = composition_type.parse_next(rest)?;
        let (rest, is_palette_update_only) = any.map(|c| c == 0x80).parse_next(rest)?;
        let (rest, palette_id) = any.map(PaletteId::new).parse_next(rest)?;
        let (rest, composition_objs) =
            length_count(subobject_count, trace("composition subobject", subobject))
                .parse_next(rest)?;

        Ok((
            rest,
            PresentationControlSegment::new(
                presentation_timestamp,
                decoding_timestamp,
                width,
                height,
                composition_number,
                composition_type,
                is_palette_update_only,
                palette_id,
                composition_objs,
            ),
        ))
    }
}

fn composition_type(input: &Bytes) -> IResult<&Bytes, PCSCompositionType> {
    any.map(|byte: u8| match byte >> 6 {
        0b00 => PCSCompositionType::Normal,
        0b01 => PCSCompositionType::AcquisitionPoint,
        0b10 => PCSCompositionType::EpochStart,
        0b11 => PCSCompositionType::EpochContinue,
        _ => unreachable!(),
    })
    .context("composition type")
    .parse_next(input)
}

fn subobject_count(input: &Bytes) -> IResult<&Bytes, u8> {
    static MAX_OBJECT_REFS: u8 = 2;

    any.map(|count| {
        if count > MAX_OBJECT_REFS {
            warn!("Invalid number of presentation objects: {}", count);
            return MAX_OBJECT_REFS;
        } else {
            return count;
        }
    })
    .context("subobject count")
    .parse_next(input)
}

fn subobject_crop_flag(input: &Bytes) -> IResult<&Bytes, bool> {
    trace("subobject crop flag", any)
        .map(|byte| byte & 0x80 != 0)
        .parse_next(input)
}

fn subobject(input: &Bytes) -> IResult<&Bytes, PCSCompositionObject> {
    let (rest, object_id) = trace("object ID", be_u16)
        .map(ObjectId::new)
        .parse_next(input)?;
    let (rest, window_id) = trace("window ID", any)
        .map(WindowId::new)
        .parse_next(rest)?;
    let (rest, is_cropped) = subobject_crop_flag.parse_next(rest)?;
    let (rest, object_x) = trace("subobject X position", be_u16).parse_next(rest)?;
    let (rest, object_y) = trace("subobject Y position", be_u16).parse_next(rest)?;

    if is_cropped {
        let (rest, crop_x) = be_u16.parse_next(rest)?;
        let (rest, crop_y) = be_u16.parse_next(rest)?;
        let (rest, crop_width) = be_u16.parse_next(rest)?;
        let (rest, crop_height) = be_u16.parse_next(rest)?;

        Ok((
            rest,
            PCSCompositionObject::new(
                object_id,
                window_id,
                Point::new(object_x, object_y),
                Some(Rect::new(crop_x, crop_y, crop_width, crop_height)),
            ),
        ))
    } else {
        Ok((
            rest,
            PCSCompositionObject::new(object_id, window_id, Point::new(object_x, object_y), None),
        ))
    }
}

//

fn window_segment(
    presentation_timestamp: PGSTimestamp,
    decoding_timestamp: PGSTimestamp,
) -> impl FnMut(&Bytes) -> IResult<&Bytes, WindowDefinitionSegment> {
    move |input: &Bytes| -> IResult<&Bytes, WindowDefinitionSegment> {}
}
