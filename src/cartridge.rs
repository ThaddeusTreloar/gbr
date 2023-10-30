use crate::mbc::{MbcRead, MbcWrite};

const HEADER_START: u16 =  0x100;
const CART_LOGO_START_LOC: u16 =  0x104;
const CART_TITLE_START_LOC: u16 =  0x134;
const HEADER_VALUES_START: u16 =  0x134;
const HEADER_VALUES_END: u16 =  0x14D;

const GB_OR_CGB: u16 =  0x80;
const CGB_ONLY: u16 =  0xC0;

const RAM_NONE: u16 =  0x00;
const RAM_2KB: u16 =  0x01;
const RAM_8KB: u16 =  0x02;
const RAM_32KB: u16 =  0x03;


struct Title {
    short_title: [u8; 11],        // 0x0134 - 0x013E
    manufacturer_code: [u8; 4], // 0x013F - 0x0142
    cgb_flag: u8,               // 0x0143
}

struct Header
{
  entry_point: [u8; 4],   // 0x0100 - 0x0103
  nintendo_logo: [u8; 30], // 0x0104 - 0x0133
  title: Title, // 0x0134 - 0x0143
  new_licensee_code: [u8; 2], // 0x0144 - 0x0145
  sgb_flag: u8,                // 0x0146
  cartridge_type: u8,          // 0x0147
  rom_size: u8,                // 0x0148
  ram_size: u8,                // 0x0149
  destination_code: u8,        // 0x014A
  old_licensee_code: u8,        // 0x014B
  mask_rom_version_number: u8,   // 0x014C
  header_checksum: u8,         // 0x014D
  global_checksum: [u8; 2],  // 0x014E - 0x014F
}

struct Cartridge<T>
where T: MbcRead + MbcWrite
{
  header: Header,
  rom: u8,
  ram: u8,
  mbc: T,
}

//int loadCartridge(const char *);
//void cartCleanup(void);