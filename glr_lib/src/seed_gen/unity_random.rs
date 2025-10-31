use crate::seed_gen::seed_generator::SeedGenerator;

pub struct UnityRandom {
    current_id: usize,
    generator: Option<SeedGenerator>,
    seeds: [f32; 1024],
}

impl From<i32> for UnityRandom {
    fn from(value: i32) -> Self {
        UnityRandom {
            current_id: 0,
            generator: Some(SeedGenerator::from(value)),
            seeds: [0f32; 1024],
        }
    }
}

impl Iterator for UnityRandom {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(sg) = &mut self.generator {
            self.seeds[self.current_id] = sg.get_next_f32() * 0.9999f32;
        }

        let result = self.seeds[self.current_id];

        self.current_id += 1;
        if self.current_id == 1024 {
            self.generator = None;
            self.current_id = 0;
        }

        Some(result)
    }
}
