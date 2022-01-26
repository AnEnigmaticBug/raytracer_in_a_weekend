pub fn map(val: f32, cur_min: f32, cur_max: f32, new_min: f32, new_max: f32) -> f32 {
    let percent = (val - cur_min) / (cur_max - cur_min);
    new_min + percent * (new_max - new_min)
}

#[cfg(test)]
mod tests {
    use super::map;

    const EPS: f32 = 0.001;

    #[test]
    fn map_happy_case() {
        assert!((map(0.0, -1.0, 1.0, 0.0, 1.0) - 0.5).abs() < EPS);
    }

    #[test]
    fn map_flip_range() {
        let lhs = map(0.0, 0.0, 1.0, 1.0, 0.0);
        let rhs = 1.0;
        assert!((lhs - rhs).abs() < EPS);
    }
}
