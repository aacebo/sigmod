pub fn weighted_avg(items: &[(f32, f32)]) -> f32 {
    let total_weight: f32 = items.iter().map(|(_, w)| w).sum();

    if total_weight == 0.0 {
        return 0.0;
    }

    items.iter().map(|(score, w)| score * w).sum::<f32>() / total_weight
}
