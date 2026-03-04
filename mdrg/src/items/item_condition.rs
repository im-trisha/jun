/// The condition of an item, from worst to best
#[cfg_attr(feature = "derive-debug", derive(Debug))]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ItemCondition {
    Trashed,
    VeryHeavilyUsed,
    HeavilyUsed,
    Used,
    NearMint,
    New,
}

struct QualityDescriptor(f32, ItemCondition);

// I didn't pull those values off my ass!
// They have been provided by sheep, the lead developer :)
const QUALITY_DESCRIPTORS: [QualityDescriptor; 6] = [
    QualityDescriptor(0.0, ItemCondition::Trashed),
    QualityDescriptor(0.5, ItemCondition::VeryHeavilyUsed),
    QualityDescriptor(0.6, ItemCondition::HeavilyUsed),
    QualityDescriptor(0.75, ItemCondition::Used),
    QualityDescriptor(0.9, ItemCondition::NearMint),
    QualityDescriptor(1.0, ItemCondition::New),
];

// This is basically the C# code! :)
// Kindly provided by sheep, the lead developer
impl From<f32> for ItemCondition {
    fn from(quality: f32) -> Self {
        for descriptor in QUALITY_DESCRIPTORS.iter().rev() {
            if quality >= descriptor.0 {
                return descriptor.1;
            }
        }

        // This should never happen, but lets return the lowest quality just in case!
        QUALITY_DESCRIPTORS[0].1
    }
}

impl From<ItemCondition> for f32 {
    fn from(condition: ItemCondition) -> Self {
        match condition {
            ItemCondition::Trashed => 0.0,
            ItemCondition::VeryHeavilyUsed => 0.5,
            ItemCondition::HeavilyUsed => 0.6,
            ItemCondition::Used => 0.75,
            ItemCondition::NearMint => 0.9,
            ItemCondition::New => 1.0,
        }
    }
}
