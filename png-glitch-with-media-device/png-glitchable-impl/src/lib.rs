#[allow(warnings)]
mod bindings;

use std::cell::RefCell;
use std::io::{Read, Write};

use bindings::exports::chikoski::glitch_art::png_glitchable::{
       FilterType, Guest, GuestPng, GuestScanLine, ScanLine, Png
};

use png_glitch::FilterType as InnerFilterType;
use png_glitch::ScanLine as InnerScanLine;
use png_glitch::PngGlitch;

impl From<InnerFilterType> for FilterType {
    fn from(value: InnerFilterType) -> Self {
        match value {
           InnerFilterType::None => FilterType::None,
           InnerFilterType::Sub => FilterType::Sub,
           InnerFilterType::Up => FilterType::Up,
           InnerFilterType::Average => FilterType::Average,
           InnerFilterType::Paeth => FilterType::Paeth,
        }
    }
}

impl Into<InnerFilterType> for FilterType {
    fn into(self) ->InnerFilterType {
        match self {
            FilterType::None =>InnerFilterType::None,
            FilterType::Sub =>InnerFilterType::Sub,
            FilterType::Up =>InnerFilterType::Up,
            FilterType::Average =>InnerFilterType::Average,
            FilterType::Paeth =>InnerFilterType::Paeth,
        }
    }
}

struct GuestScanLineImpl {
    inner: RefCell<InnerScanLine>,
}

impl GuestScanLineImpl {
    fn new(inner: InnerScanLine) -> GuestScanLineImpl {
        let inner = RefCell::new(inner);
        GuestScanLineImpl{ inner }
    }
}

impl GuestScanLine for GuestScanLineImpl {
    fn get_filter_type(&self) -> FilterType {
        FilterType::from(self.inner.borrow().filter_type().clone())
    }

    fn set_filter_type(&self, t: FilterType) {
        let filter_type = t.into();
        self.inner.borrow_mut().set_filter_type(filter_type);
    }

    fn read(&self) -> Result<Vec<u8>, ()> {
        let mut buffer = vec![];
        self.inner.borrow_mut().read_to_end(&mut buffer).map_err(|_| ())?;
        Ok(buffer)
    }

    fn write(&self, pixels: Vec<u8>) {
        let _ = self.inner.borrow_mut().write(&pixels);
    }
    
    fn size(&self) -> u32 {
        self.inner.borrow().size() as u32
    }
    
    fn get_pixel_at(&self, index: u32) -> u8 {
        self.inner.borrow()[index as usize]
    }
    
    fn set_pixel_at(&self, index: u32, value: u8) {
        self.inner.borrow_mut().update(index as usize, value);
    }
}

struct GuestPngImpl {
    inner: RefCell<PngGlitch>
}

impl GuestPngImpl {
    
    fn new(data: Vec<u8>) -> Result<GuestPngImpl, ()> {
        let inner = PngGlitch::new(data).map_err(|_| ())?;
        let inner = RefCell::new(inner);
        Ok(GuestPngImpl{ inner })
    }

}


impl GuestPng for GuestPngImpl {
    fn get_scan_lines(&self) -> Vec<ScanLine> {
        self.inner.borrow_mut().scan_lines().into_iter().map(|s| ScanLine::new(GuestScanLineImpl::new(s)) ).collect()
    }

    fn read(&self) -> Result<Vec<u8>, ()> {
        let mut buffer = vec![];
        self.inner.borrow_mut().encode(&mut buffer).map_err(|_| ())?;
        Ok(buffer)
    }

    fn create(data: Vec<u8>) -> Result<Png, ()> {
        let png = GuestPngImpl::new(data)?;
        Ok(Png::new(png))
    }
    
}

struct Component;
impl Guest for Component {
    type ScanLine = GuestScanLineImpl;
    type Png = GuestPngImpl;
}

bindings::export!(Component with_types_in bindings);
