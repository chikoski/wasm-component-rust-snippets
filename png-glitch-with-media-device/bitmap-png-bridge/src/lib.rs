#[allow(warnings)]
mod bindings;

use bindings::chikoski::glitch_art::png_glitchable::{Png, ScanLine, FilterType};
use bindings::exports::chikoski::glitch_art::bridge_to_png_glitchable::{Guest, Png as ExportedPng};
use bindings::exports::chikoski::glitch_art::png_glitchable::{GuestPng, GuestScanLine, FilterType as ExportedFilterType, ScanLine as ExportedScanLine, Guest as GuestGlitchable};

impl From<FilterType> for ExportedFilterType {
    fn from(value: FilterType) -> Self {
        match value {
            FilterType::None => ExportedFilterType::None,
            FilterType::Sub => ExportedFilterType::Sub,
            FilterType::Up => ExportedFilterType::Up,
            FilterType::Average => ExportedFilterType::Average,
            FilterType::Paeth => ExportedFilterType::Paeth,
        }
    }
}

impl From<ExportedFilterType> for FilterType {
    fn from(value: ExportedFilterType) -> Self {
        match value {
            ExportedFilterType::None => FilterType::None,
            ExportedFilterType::Sub => FilterType::Sub,
            ExportedFilterType::Up => FilterType::Up,
            ExportedFilterType::Average => FilterType::Average,
            ExportedFilterType::Paeth => FilterType::Paeth,
        }
    }
}

struct ImageData {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

impl ImageData {
    fn new(data: Vec<u8>, width: u32, height: u32) -> ImageData {
        ImageData{ data, width, height }
    }
}

struct PngImageData {
    bytes: Vec<u8>
}

impl TryFrom<ImageData> for PngImageData {
    type Error = ();

    fn try_from(value: ImageData) -> Result<Self, Self::Error> {
        let mut bytes = vec![];
        let mut encoder = png::Encoder::new(&mut bytes, value.width, value.height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|_| ())?;
        writer.write_image_data(&value.data).map_err(|_| ())?;
        let _ = writer.finish();
        
        let png_image_data = PngImageData{ bytes };
        Ok(png_image_data)
    }
}

struct ScanLineImpl {
    actual: ScanLine
}

impl ScanLineImpl {
    fn new(actual: ScanLine) -> ScanLineImpl {
        ScanLineImpl{ actual }
    }
}

impl GuestScanLine for ScanLineImpl {
    fn get_filter_type(&self) -> ExportedFilterType {
        self.actual.get_filter_type().into()
    }

    fn set_filter_type(&self, t: ExportedFilterType) {
        self.actual.set_filter_type(t.into());
    }

    fn size(&self) -> u32 {
        self.actual.size()
    }

    fn get_pixel_at(&self, index: u32) -> u8 {
        self.actual.get_pixel_at(index)
    }

    fn set_pixel_at(&self, index: u32, value: u8) {
        self.actual.set_pixel_at(index, value);
    }

    fn read(&self) -> Result<Vec<u8>, ()> {
        self.actual.read()
    }

    fn write(&self, pixels: Vec<u8>) {
        self.actual.write(&pixels);
    }
}

struct PngImpl {
    actual: Png,
}

impl PngImpl {
    fn new(actual: Png) -> PngImpl {
        PngImpl{ actual }
    }
}

impl GuestPng for PngImpl {
    fn get_scan_lines(&self) -> Vec<ExportedScanLine> {
        self.actual.get_scan_lines().into_iter().map(|actual| {
            let scanline= ScanLineImpl::new(actual);
            ExportedScanLine::new(scanline)
        }).collect()
    }

    fn read(&self) -> Result<Vec<u8>, ()> {
        self.actual.read()
    }

    fn create(data: Vec<u8>) -> Result<ExportedPng, ()> {
        let actual = Png::create(&data)?;
        let png = PngImpl::new(actual);
        let png = ExportedPng::new(png);
        Ok(png)
    }
}

impl TryFrom<PngImageData> for PngImpl {
    type Error = ();

    fn try_from(value: PngImageData) -> Result<Self, Self::Error> {
        let value = Png::create(&value.bytes)?;
        let value = PngImpl::new(value);
        Ok(value)
    }
}

impl TryFrom<ImageData> for PngImpl {
    type Error = ();

    fn try_from(value: ImageData) -> Result<Self, Self::Error> {
        let value = PngImageData::try_from(value)?;
        PngImpl::try_from(value)
    }
}

struct Component;

impl Guest for Component {
    fn create(
        data: Vec<u8>,
        width: u32,
        height: u32,
    ) -> Result<ExportedPng, ()> {
        let image_data = ImageData::new(data, width, height);
        let value = PngImpl::try_from(image_data)?;
        let value = ExportedPng::new(value);
        Ok(value)
    }
}

impl GuestGlitchable for Component {
    type ScanLine = ScanLineImpl;

    type Png = PngImpl;
}

bindings::export!(Component with_types_in bindings);
