use sha2::{digest::DynDigest, Digest, Sha256};

use crate::seed_gen::zone_info::{generated_data::AllocType, zone_identifier::ZoneIdentifier};


#[derive(Default, Debug)]
pub struct MarkerSetHash {
    
    count: usize,
    hash: Sha256,
    
}

impl MarkerSetHash {
    
    pub fn get_count(&self) -> usize {
        self.count
    }
    
    pub fn add_to_hash(
        &mut self,
        zone_id: &ZoneIdentifier,
        alloc_type: &AllocType
    ) {
        let id_bin = bincode::serialize(zone_id)
            .unwrap();
        let alloc_bin = bincode::serialize(alloc_type)
            .unwrap();
        
        self.count += 1;
        Digest::update(&mut self.hash, &id_bin);
        Digest::update(&mut self.hash, &alloc_bin);
    }
    
}

impl Into<[u8; 32]> for MarkerSetHash {
    fn into(self) -> [u8; 32] {
        let mut result_arr = [0u8; 32];
        let _ = DynDigest::finalize_into(self.hash, &mut result_arr);
        result_arr
    }
}
