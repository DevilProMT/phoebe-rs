use wicked_waifus_protocol_internal::PlayerGuides;

pub struct Guides {
    pub finished_guides: Vec<i32>,
}

impl Guides {
    pub fn load_from_save(data: PlayerGuides) -> Self {
        Guides {
            finished_guides: data.finished_guides,
        }
    }

    pub fn build_save_data(&self) -> PlayerGuides {
        PlayerGuides {
            finished_guides: self.finished_guides.clone(),
        }
    }
}

impl Default for Guides {
    fn default() -> Self {
        Self {
            finished_guides: Vec::new(),
        }
    }
}