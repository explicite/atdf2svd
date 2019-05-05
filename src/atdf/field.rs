use crate::util;
use crate::atdf;
use crate::chip;
use crate::ElementExt;


pub fn parse(bitfield_el: &xmltree::Element) -> crate::Result<chip::Field> {
    bitfield_el.check_name("bitfield")?;

    let name = bitfield_el.attr("name")?.clone();
    let description = bitfield_el
        .attributes
        .get("caption")
        .and_then(|d| if d.len() != 0 { Some(d) } else { None })
        .cloned();

    // The range is defined by a mask.
    // Not that in some cases there are bits withing this range, that do not belong to this mask
    // (e.g. 0b00010010). Then the value restriction is unsafe.
    let (range, unsafe_range) = util::parse_mask(bitfield_el.attr("mask")?)?
        .ok_or_else(|| atdf::error::UnsupportedError::new("mask", bitfield_el))?;

    let restriction = if unsafe_range {
        chip::ValueRestriction::Unsafe
    } else {
        chip::ValueRestriction::Any  // TODO: Use value group if specified
    };

    let access = if let Some(access) = bitfield_el.attributes.get("rw") {
        match access.as_ref() {
            "R" => chip::AccessMode::ReadOnly,
            "RW" => chip::AccessMode::ReadWrite,
            "" => {
                log::warn!("empty access-mode on {:?}", bitfield_el);
                chip::AccessMode::ReadWrite
            }
            _ => {
                return Err(atdf::error::UnsupportedError::new(
                    format!("access-mode '{:?}'", access),
                    bitfield_el
                ).into());
            }
        }
    } else {
        chip::AccessMode::ReadWrite
    };

    Ok(chip::Field {
        name,
        description,
        range,
        access,
        restriction,
    })
}
