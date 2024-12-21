#[cfg(test)]
mod tests {
    /// Given ship floating on sea
    /// And anchor released to seabed
    /// And anchor cable length is 30 meters
    /// And cable is at angle of 39 degrees
    /// When sine function is used to find depth d
    /// Then d is approximately 18.88
    #[test]
    fn seabed_depth() {
        let cable_length: f32 = 30.0; // hypotenuse
        let cable_angle: f32 = 39.0; // degree
        let cable_sin = cable_angle.to_radians().sin();
        let d: f32 = cable_sin * cable_length;
        assert_eq!(d, 18.879612)
    }
}
