use std::fs::File;
use std::io;
use std::io::{Read, Seek, SeekFrom, BufReader};
/*
use std::convert::TryInto;
*/

const PAK_FILE_SIGNATURE: &str = "PAK";
const SIGNATURE_BYTE_COUNT: u32 = 3;
const U32_BYTE_COUNT: u32 = 4;

const CATEGORY_IMG: usize = 0;
const CATEGORY_FNT: usize = 1;
const CATEGORY_SND: usize = 2;
const CATEGORY_MAP: usize = 3;
const CATEGORY_CFG: usize = 4;
const CATEGORY_SCR: usize = 5;
const CATEGORY_COUNT: usize = 6;

struct DataChunk {
    from: u64,
    length: u64
}

type DataTable = Vec<DataChunk>;

pub struct DataPakLoader {
    reader: BufReader<File>,
    data_tables: Vec<DataTable>,
    buf: Vec<u8>
}

impl DataPakLoader {
    pub fn new(file_name: &str) -> DataPakLoader {
        println!("Loading {}", file_name);
        let f = File::open(file_name).expect("Error opening bundle file!");

        let mut bundle_reader = DataPakLoader {
            reader: BufReader::new(f),
            data_tables: Vec::new(),
            buf: Vec::new()
            };

        bundle_reader.read_table_of_content().expect("Error reading table of content"); 
        bundle_reader
    }

    pub fn load_image(&mut self, index: usize) -> &[u8] {
        self.load_data(CATEGORY_IMG, index)
    }

    pub fn load_font(&mut self, index: usize) -> &[u8] {
        self.load_data(CATEGORY_FNT, index)
    }
    
    pub fn load_sound(&mut self, index: usize) -> &[u8] {
        self.load_data(CATEGORY_SND, index)
    }


    pub fn print_table_of_content(&self) {
        for i in 0..CATEGORY_COUNT {
            println!("----- Category #{} -----", i + 1);
            for chunk in &self.data_tables[i] {
                println!("{:#010X} - {:#010X}", chunk.from, chunk.from + chunk.length);
            }
        }
    }

    fn load_data(&mut self, category: usize, index: usize) -> &[u8] {
        let mut reader = self.reader.get_ref();
        let pointer = &self.data_tables[category][index];
        reader.seek(SeekFrom::Start(pointer.from))
              .expect("Error reading image data!");
        let mut chunk = reader.take(pointer.length);
        self.buf.clear();
        chunk.read_to_end(&mut self.buf);
        self.decode();

        /*
        for byte in &self.buf {
            print!("{:02X}, ", byte);
        }

        println!("");
        */

        &self.buf[..]
    }

    fn decode(&mut self) {
        for byte in &mut self.buf {
            *byte = 0xFF - *byte;
        }
    }

    fn read_table_of_content(&mut self) -> io::Result<()> {
        let mut reader = self.reader.get_ref();
        let mut signature_buf = [0u8; SIGNATURE_BYTE_COUNT as usize];
        
        // Read file signature
        reader.read_exact(&mut signature_buf).expect("Error reading file signature");
        let signature = String::from_utf8(signature_buf.iter().cloned().collect()).unwrap();
        if signature != PAK_FILE_SIGNATURE {
            panic!("Invalid DataPak File!");
        }

        let mut offsets = [0u32; CATEGORY_COUNT];

        // Read offsets to categories
        for i in 0..CATEGORY_COUNT {
            let mut buf= [0u8; U32_BYTE_COUNT as usize];

            reader.read_exact(&mut buf)
                  .expect(format!("Failed to read offset to category #{}", i + 1).as_str());
            offsets[i] = u32::from_le_bytes(buf);
        }

        // Build asset tables
        for i in 0..CATEGORY_COUNT {
            let mut buf = [0u8; U32_BYTE_COUNT as usize];
            let pos = offsets[i] as u64;
            reader.seek(SeekFrom::Start(pos)).expect("Failed to read file");
            // Read asset count
            reader.read_exact(&mut buf).expect("Failed to read asset count");
            let asset_count = u32::from_le_bytes(buf);

            // Read asset lengths
            let mut asset_lengths: Vec<u32> = Vec::new(); 

            for _j in 0..asset_count { 
                let mut buf = [0u8; U32_BYTE_COUNT as usize];
                reader.read_exact(&mut buf)?;
                asset_lengths.push(u32::from_le_bytes(buf));
            }

            // Build a table of data offsets  
            let mut asset_table: DataTable = Vec::new();
            let mut offset = reader.stream_position()?;

            for j in 0..asset_count as usize { 
                let data = DataChunk {
                    from: offset,
                    length: asset_lengths[j] as u64
                };
                asset_table.push(data);
                offset += asset_lengths[j] as u64;
            }

            self.data_tables.push(asset_table);
        }
    
        Ok(())
    }
}

/*
fn vec_to_array(v: Vec<u8>) -> [u8; 4] {
    v.try_into().unwrap_or_else(|v: Vec<u8>| panic!("Error convert vec to array!"))
}
*/
