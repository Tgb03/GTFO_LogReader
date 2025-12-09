use sha2::{digest::DynDigest, Digest, Sha256};

use crate::seed_gen::zone_info::zone_identifier::ZoneIdentifier;


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
        zone_id: &ZoneIdentifier
    ) {
        let id_bin = bincode::serialize(zone_id)
            .unwrap();
        
        self.count += 1;
        Digest::update(&mut self.hash, &id_bin);
    }
    
}

impl Into<[u8; 32]> for MarkerSetHash {
    fn into(self) -> [u8; 32] {
        let mut result_arr = [0u8; 32];
        let _ = DynDigest::finalize_into(self.hash, &mut result_arr);
        result_arr
    }
}
