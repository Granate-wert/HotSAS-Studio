use hotsas_api::{ApiError, ApiErrorDto};

#[test]
fn api_error_exposes_structured_dto() {
    let dto = ApiError::InvalidInput("unsupported unit: Volt".to_string()).to_dto();

    assert_eq!(
        dto,
        ApiErrorDto {
            code: "invalid_input".to_string(),
            message: "invalid input: unsupported unit: Volt".to_string(),
            details: Some("unsupported unit: Volt".to_string()),
        }
    );
}
